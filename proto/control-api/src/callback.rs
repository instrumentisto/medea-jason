//! [`CallbackClient`] definitions.
//!
//! [`CallbackClient`]: Client

use async_trait::async_trait;
use derive_more::From;
use time::OffsetDateTime;

use crate::{ErrorResponse, StatefulFid};

/// Abstraction of a [Control API] callback client.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
#[async_trait(?Send)]
pub trait Client {
    /// Sends provided [`Request`].
    async fn send(&self, request: Request) -> Result<(), ErrorResponse>;
}

/// [Control API] callback.
///
/// Used for sending callbacks with [`Client::send`].
///
/// [Control API]: https://tinyurl.com/yxsqplq7
#[derive(Debug)]
pub struct Request {
    /// `FID` (Full `ID`) of element with which event was occurred.
    pub fid: StatefulFid,

    /// [`Member::on_join`] or [`Member::on_leave`] callback `URL`.
    ///
    /// [`Member::on_join`]: crate::Member::on_join
    /// [`Member::on_leave`]: crate::Member::on_leave
    pub url: String,

    /// [`Event`] which occurred.
    pub event: Event,

    /// Time at which event occurred.
    pub at: OffsetDateTime,
}

/// All callbacks which can happen.
#[derive(Clone, Copy, Debug, From)]
pub enum Event {
    /// Event notifying about [`Member`] joining the [`Room`].
    ///
    /// [`Room`]: crate::Room
    /// [`Member`]: crate::Member
    OnJoin(OnJoinEvent),

    /// Event notifying about [`Member`] leaving the [`Room`].
    ///
    /// [`Room`]: crate::Room
    /// [`Member`]: crate::Member
    OnLeave(OnLeaveEvent),
}

/// Event notifying about [`Member`] joining the [`Room`].
///
/// [`Room`]: crate::Room
/// [`Member`]: crate::Member
#[derive(Clone, Copy, Debug)]
pub struct OnJoinEvent;

/// Event notifying about [`Member`] leaving the [`Room`].
///
/// [`Room`]: crate::Room
/// [`Member`]: crate::Member
#[derive(Clone, Copy, Debug)]
pub struct OnLeaveEvent {
    /// Reason of why [`Member`] was lost.
    ///
    /// [`Member`]: crate::Member
    pub reason: OnLeaveReason,
}

impl OnLeaveEvent {
    /// Creates a new [`OnLeaveEvent`] with the provided [`OnLeaveReason`].
    #[must_use]
    pub const fn new(reason: OnLeaveReason) -> Self {
        Self { reason }
    }
}

/// Reason of why [`Member`] was lost.
///
/// [`Member`]: crate::Member
#[derive(Clone, Copy, Debug)]
pub enum OnLeaveReason {
    /// [`Member`] was normally disconnected.
    ///
    /// [`Member`]: crate::Member
    Disconnected,

    /// Connection with [`Member`] was lost.
    ///
    /// [`Member`]: crate::Member
    LostConnection,

    /// [`Member`] was forcibly disconnected by server.
    ///
    /// [`Member`]: crate::Member
    Kicked,

    /// Server is shutting down.
    ServerShutdown,
}
