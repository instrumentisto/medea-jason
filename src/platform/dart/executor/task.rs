//! [`Task`] for execution by a [`platform::dart::executor`].

use std::{
    cell::{Cell, RefCell},
    fmt,
    mem::ManuallyDrop,
    panic::AssertUnwindSafe,
    rc::Rc,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use futures::future::LocalBoxFuture;

use crate::platform::dart::executor::task_wake;

/// Inner [`Task`]'s data.
struct Inner {
    /// An actual [`Future`] that this [`Task`] is driving.
    future: LocalBoxFuture<'static, ()>,

    /// Handle for waking up this [`Task`].
    waker: Waker,
}

impl fmt::Debug for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inner")
            .field("waker", &self.waker)
            .finish_non_exhaustive()
    }
}

/// Wrapper for a [`Future`] that can be polled by an external single threaded
/// Dart executor.
#[derive(Debug)]
pub struct Task {
    /// [`Task`]'s inner data containing an actual [`Future`] and its
    /// [`Waker`]. Dropped on the [`Task`] completion.
    inner: RefCell<Option<Inner>>,

    /// Indicates whether there is a [`Poll::Pending`] awake request of this
    /// [`Task`].
    is_scheduled: Cell<bool>,
}

impl Task {
    /// Spawns a new [`Task`] that will drive the given [`Future`].
    pub fn spawn(future: LocalBoxFuture<'static, ()>) {
        let this = Rc::new(Self {
            inner: RefCell::new(None),
            is_scheduled: Cell::new(false),
        });

        let waker =
            unsafe { Waker::from_raw(Self::into_raw_waker(Rc::clone(&this))) };
        drop(this.inner.borrow_mut().replace(Inner { future, waker }));

        Self::wake_by_ref(&this);
    }

    /// Polls the underlying [`Future`].
    ///
    /// Polling after [`Future`]'s completion is no-op.
    pub fn poll(&self) -> Poll<()> {
        let mut borrow = self.inner.borrow_mut();

        // Just ignore poll request if the `Future` is completed.
        let inner = match borrow.as_mut() {
            Some(inner) => inner,
            None => return Poll::Ready(()),
        };

        let poll = {
            let mut cx = Context::from_waker(&inner.waker);
            let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                inner.future.as_mut().poll(&mut cx)
            }));
            if let Ok(poll) = res {
                poll
            } else {
                self.is_scheduled.set(false);
                *borrow = None;
                return Poll::Ready(());
            }
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
            task_wake(Rc::clone(this));
        }
    }

    /// Pretty much a copy of [`std::task::Wake`] implementation but for
    /// `Rc<?Send + ?Sync>` instead of `Arc<Send + Sync>` since we are sure
    /// that everything will run on a single thread.
    fn into_raw_waker(this: Rc<Self>) -> RawWaker {
        #![allow(clippy::missing_docs_in_private_items)]

        // Refer to `RawWakerVTable::new()` documentation for better
        // understanding of what the following functions do.

        unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr.cast::<Task>()));
            Task::into_raw_waker(Rc::clone(&(*ptr)))
        }

        unsafe fn raw_wake(ptr: *const ()) {
            let ptr = Rc::from_raw(ptr.cast::<Task>());
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_wake_by_ref(ptr: *const ()) {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr.cast::<Task>()));
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_drop(ptr: *const ()) {
            drop(Rc::from_raw(ptr.cast::<Task>()));
        }

        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

        RawWaker::new(Rc::into_raw(this).cast::<()>(), &VTABLE)
    }
}
