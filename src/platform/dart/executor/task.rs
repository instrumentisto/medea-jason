//! [`Task`] for execution by a [`platform::dart::executor`].

use std::{
    cell::{Cell, RefCell},
    mem::ManuallyDrop,
    ptr,
    rc::Rc,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    sync::atomic::Ordering
};
use std::sync::atomic::AtomicU64;

use futures::future::LocalBoxFuture;

use crate::platform::dart::executor::task_wake;

static CNT: AtomicU64 = AtomicU64::new(0);

/// Inner [`Task`]'s data.
struct Inner {
    /// An actual [`Future`] that this [`Task`] is driving.
    future: LocalBoxFuture<'static, ()>,

    /// Handle for waking up this [`Task`].
    waker: Waker,
}

/// Wrapper for a [`Future`] that can be polled by an external single threaded
/// Dart executor.
pub struct Task {
    /// [`Task`]'s inner data containing an actual [`Future`] and its
    /// [`Waker`]. Dropped on the [`Task`] completion.
    inner: RefCell<Option<Inner>>,

    pub id: u64,

    /// Indicates whether there is a [`Poll::Pending`] awake request of this
    /// [`Task`].
    is_scheduled: Cell<bool>,
}

impl Task {
    /// Spawns a new [`Task`] that will drive the given [`Future`].
    pub fn spawn(future: LocalBoxFuture<'static, ()>) {
        let this = Rc::new(Self {
            inner: RefCell::new(None),
            id: CNT.fetch_add(1, Ordering::SeqCst),
            is_scheduled: Cell::new(true),
        });

        let waker =
            unsafe { Waker::from_raw(Task::into_raw_waker(Rc::clone(&this))) };
        this.inner.borrow_mut().replace(Inner {
            future,
            waker,
        });

        // Task is leaked and must be freed manually by the external executor.
        task_wake(this);
    }

    /// Polls the underlying [`Future`].
    ///
    /// Polling after [`Future`]'s completion is no-op.
    pub fn poll(&self) -> Poll<()> {
        self.log(format!("poll start {}", self.id));
        let mut borrow = self.inner.borrow_mut();

        // Just ignore poll request if the `Future` is completed.
        let inner = match borrow.as_mut() {
            Some(inner) => inner,
            None => {
                // format!("borrow_mut stop {}", self.id);
                return Poll::Ready(())
            },
        };
        // format!("borrow_mut 222 {}", self.id);
        let poll = {
            // format!("borrow_mut 333 {}", self.id);
            let mut cx = Context::from_waker(&inner.waker);
            // format!("borrow_mut 444 {}", self.id);
            inner.future.as_mut().poll(&mut cx)
        };
        // format!("borrow_mut 555 {}", self.id);
        self.is_scheduled.set(false);
        // Cleanup resources if future is ready.
        if poll.is_ready() {
            log::error!("Dropping Waker {}", self.id);
            *borrow = None;
        }

        self.log(format!("poll end {}", self.id));
        poll
    }

    /// Calls the [`task_wake()`] function by the provided reference if this
    /// [`Task`] s incomplete and there are no [`Poll::Pending`] awake requests
    /// already.
    fn wake_by_ref(this: &Rc<Self>) {
        format!("wake_by_ref SSS {}", this.id);
        if !this.is_scheduled.get() {
            this.is_scheduled.set(true);
            // log::error!("wake_by_ref");
            task_wake(Rc::clone(&this));
        }
        // task_wake(ptr::NonNull::from(Rc::as_ref(this)));
        // format!("borrow end {}", this.id);
    }

    /// Pretty much a copy of [`std::task::Wake`] implementation but for
    /// `Rc<?Send + ?Sync>` instead of `Arc<Send + Sync>` since we are sure
    /// that everything will run on a single thread.
    #[inline(always)]
    fn into_raw_waker(this: Rc<Self>) -> RawWaker {
        // Refer to `RawWakerVTable::new()` documentation for better
        // understanding of what following functions do.
        unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr.cast::<Task>()));
            log::error!("raw_clone {}", ptr.id);
            Task::into_raw_waker(Rc::clone(&(*ptr)))
        }

        unsafe fn raw_wake(ptr: *const ()) {
            log::error!("wake_by_ref 111 {:p}", ptr);
            let ptr = Rc::from_raw(ptr.cast::<Task>());

            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_wake_by_ref(ptr: *const ()) {
            log::error!("wake_by_ref 222 {:p}", ptr);
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr.cast::<Task>()));
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_drop(ptr: *const ()) {
            let rc = Rc::from_raw(ptr.cast::<Task>());
            log::error!("raw_drop {:p} {}", ptr, Rc::strong_count(&rc));
            drop(rc);
        }

        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

        RawWaker::new(Rc::into_raw(this).cast::<()>(), &VTABLE)
    }
}
