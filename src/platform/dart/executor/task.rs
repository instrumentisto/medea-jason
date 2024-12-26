//! [`Task`] for execution by a [`platform::dart::executor`].
//!
//! [`platform::dart::executor`]: crate::platform::executor

use std::{
    cell::{BorrowError, Cell, Ref, RefCell},
    mem::ManuallyDrop,
    rc::Rc,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use derive_more::Debug;
use futures::future::LocalBoxFuture;

use crate::platform::dart::executor::task_wake;

/// Inner [`Task`]'s data.
#[derive(Debug)]
struct Inner {
    /// An actual [`Future`] that this [`Task`] is driving.
    ///
    /// [`Future`]: std::future::Future
    #[debug(skip)]
    future: LocalBoxFuture<'static, ()>,

    id: u64,

    /// Handle for waking up this [`Task`].
    waker: Waker,
}

/// Wrapper for a [`Future`] that can be polled by an external single threaded
/// Dart executor.
///
/// [`Future`]: std::future::Future
#[derive(Debug)]
pub struct Task {
    /// [`Task`]'s inner data containing an actual [`Future`] and its
    /// [`Waker`]. Dropped on the [`Task`] completion.
    ///
    /// [`Future`]: std::future::Future
    inner: RefCell<Option<Inner>>,

    /// Indicates whether there is a [`Poll::Pending`] awake request of this
    /// [`Task`].
    is_scheduled: Cell<bool>,
}

impl Task {
    /// Spawns a new [`Task`] that will drive the given [`Future`].
    ///
    /// [`Future`]: std::future::Future
    pub fn spawn(future: LocalBoxFuture<'static, ()>, id: u64) {
        let this = Rc::new(Self {
            inner: RefCell::new(None),
            is_scheduled: Cell::new(false),
        });

        let waker =
            unsafe { Waker::from_raw(Self::into_raw_waker(Rc::clone(&this))) };
        drop(this.inner.borrow_mut().replace(Inner { future, id, waker }));

        Self::wake_by_ref(&this);
    }

    /// Polls the underlying [`Future`].
    ///
    /// Polling after [`Future`]'s completion is no-op.
    ///
    /// [`Future`]: std::future::Future
    pub fn poll(&self) -> Poll<()> {
        let mut borrow = self.inner.borrow_mut();

        // Just ignore poll request if the `Future` is completed.
        let Some(inner) = borrow.as_mut() else {
            return Poll::Ready(());
        };

        let poll = {
            let mut cx = Context::from_waker(&inner.waker);
            inner.future.as_mut().poll(&mut cx)
        };
        self.is_scheduled.set(false);

        // Cleanup resources if future is ready.
        if poll.is_ready() {
            *borrow = None;
        }

        poll
    }

    /// Calls the [`task_wake()`] function by the provided reference if this
    /// [`Task`] s incomplete and there are no [`Poll::Pending`] awake requests
    /// already.
    fn wake_by_ref(this: &Rc<Self>) {
        if !this.is_scheduled.replace(true) {
            // let id = {
            //     match this.inner.try_borrow() {
            //         Ok(v) => match &*v {
            //             None => String::from("None"),
            //             Some(i) => i.id.to_string(),
            //         },
            //         Err(_) => String::from("BorrowErr"),
            //     }
            // };
            // println!("task_wake {:?}, {id}", std::thread::current().id());

            task_wake(Rc::clone(this));
        }
    }

    /// Pretty much a copy of [`std::task::Wake`] implementation but for
    /// `Rc<?Send + ?Sync>` instead of `Arc<Send + Sync>` since we are sure
    /// that everything will run on a single thread.
    fn into_raw_waker(this: Rc<Self>) -> RawWaker {
        #![expect( // not visible
            clippy::missing_docs_in_private_items,
            reason = "not visible at all"
        )]

        // Refer to `RawWakerVTable::new()` documentation for better
        // understanding of what the following functions do.

        unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
            let ptr =
                ManuallyDrop::new(unsafe { Rc::from_raw(ptr.cast::<Task>()) });
            Task::into_raw_waker(Rc::clone(&(*ptr)))
        }

        unsafe fn raw_wake(ptr: *const ()) {
            let ptr = unsafe { Rc::from_raw(ptr.cast::<Task>()) };
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_wake_by_ref(ptr: *const ()) {
            let ptr =
                ManuallyDrop::new(unsafe { Rc::from_raw(ptr.cast::<Task>()) });
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_drop(ptr: *const ()) {
            drop(unsafe { Rc::from_raw(ptr.cast::<Task>()) });
        }

        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

        RawWaker::new(Rc::into_raw(this).cast::<()>(), &VTABLE)
    }
}
