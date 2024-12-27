//! [`Task`] for execution by a [`platform::dart::executor`].
//!
//! [`platform::dart::executor`]: crate::platform::executor

// #[cfg(debug_assertions)]
use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::{Context, Waker},
    thread::{self, ThreadId},
};

use derive_more::Debug;
use futures::{
    future::LocalBoxFuture,
    task::{self, ArcWake},
};

use crate::platform::dart::executor::task_wake;

/// Inner [`Task`]'s data.
#[derive(Debug)]
struct Inner {
    /// An actual [`Future`] that this [`Task`] is driving.
    ///
    /// [`Future`]: std::future::Future
    #[debug(skip)]
    future: LocalBoxFuture<'static, ()>,

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
    is_scheduled: AtomicBool,

    // #[cfg(debug_assertions)]
    /// Thread that this task was created on and must be polled on.
    thread: ThreadId,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        if !arc_self.is_scheduled.swap(true, Ordering::AcqRel) {
            task_wake(Arc::clone(arc_self));
        }
    }
}

impl Task {
    /// Spawns a new [`Task`] that will drive the given [`Future`].
    ///
    /// [`Future`]: std::future::Future
    pub fn spawn(future: LocalBoxFuture<'static, ()>) {
        let this = Arc::new(Self {
            inner: RefCell::new(None),
            is_scheduled: AtomicBool::new(false),
            // #[cfg(debug_assertions)]
            thread: thread::current().id(),
        });

        let waker = task::waker(Arc::clone(&this));
        drop(this.inner.borrow_mut().replace(Inner { future, waker }));

        Self::wake_by_ref(&this);
    }

    /// Polls the underlying [`Future`].
    ///
    /// Polling after [`Future`]'s completion is no-op.
    ///
    /// [`Future`]: std::future::Future
    pub fn poll(&self) {
        // #[cfg(debug_assertions)]
        assert_eq!(
            self.thread,
            thread::current().id(),
            "Future can only be polled on a thread it was created on"
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

// `Task` can be sent across threads safely because it ensures that
// the underlying Future will only be touched from a single thread it
// was created on.
unsafe impl Send for Task {}
unsafe impl Sync for Task {}
