//! [`Task`] for execution by a [`platform::dart::executor`].
//!
//! [`platform::dart::executor`]: crate::platform::executor

use std::{
    cell::RefCell,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Waker},
    thread::{self, ThreadId},
};

use derive_more::with_trait::Debug;
use futures::{
    future::LocalBoxFuture,
    task::{self, ArcWake},
};

use crate::platform::dart::executor::task_wake;

/// Inner [`Task`]'s data.
#[derive(Debug)]
struct Inner {
    /// An actual [`Future`] that this [`Task`] is driving.
    #[debug(skip)]
    future: LocalBoxFuture<'static, ()>,

    /// Handle for waking up this [`Task`].
    waker: Waker,
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
    is_scheduled: AtomicBool,

    /// ID of the thread this [`Task`] was created on and must be polled on.
    thread_id: ThreadId,
}

/// [`Task`] can be sent across threads safely because it ensures that the
/// underlying [`Future`] will only be touched from a single thread it was
/// created on.
unsafe impl Send for Task {}
/// [`Task`] can be shared across threads safely because it ensures that the
/// underlying [`Future`] will only be touched from a single thread it was
/// created on.
unsafe impl Sync for Task {}

impl ArcWake for Task {
    /// Commands an external Dart executor to poll this [`Task`] if it's
    /// incomplete and there are no [`Poll::Pending`] awake requests already.
    ///
    /// [`Poll::Pending`]: task::Poll::Pending
    fn wake_by_ref(arc_self: &Arc<Self>) {
        if !arc_self.is_scheduled.swap(true, Ordering::AcqRel) {
            task_wake(Arc::clone(arc_self));
        }
    }
}

impl Task {
    /// Spawns a new [`Task`] that will drive the given [`Future`].
    ///
    /// Must be called on the same thread where the [`Task`] will be polled,
    /// otherwise polling will panic.
    pub fn spawn(future: LocalBoxFuture<'static, ()>) {
        let this = Arc::new(Self {
            inner: RefCell::new(None),
            is_scheduled: AtomicBool::new(false),
            thread_id: thread::current().id(),
        });

        let waker = task::waker(Arc::clone(&this));
        drop(this.inner.borrow_mut().replace(Inner { future, waker }));

        Self::wake_by_ref(&this);
    }

    /// Polls the underlying [`Future`].
    ///
    /// Polling after [`Future`]'s completion is no-op.
    ///
    /// # Panics
    ///
    /// If called not on the same thread where this [`Task`] was originally
    /// created.
    pub fn poll(&self) {
        assert_eq!(
            self.thread_id,
            thread::current().id(),
            "`dart::executor::Task` can only be polled on the same thread \
             where it was originally created",
        );

        let mut borrow = self.inner.borrow_mut();

        // Just ignore poll request if the `Future` is completed.
        let Some(inner) = borrow.as_mut() else {
            return;
        };

        self.is_scheduled.store(false, Ordering::Release);

        let poll = {
            let mut cx = Context::from_waker(&inner.waker);
            inner.future.as_mut().poll(&mut cx)
        };

        // Cleanup resources if future is ready.
        if poll.is_ready() {
            *borrow = None;
        }
    }
}
