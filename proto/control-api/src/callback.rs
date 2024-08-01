//! API for receiving callbacks from a media server.

use async_trait::async_trait;
use derive_more::From;
use time::OffsetDateTime as DateTime;

use crate::Fid;

/// API for receiving callbacks from a media server.
///
/// Both API client and API server should implement this trait.
#[async_trait]
pub trait Api {
    /// Error returned by this [`CallbackApi`].
    ///
    /// [`CallbackApi`]: Api
    type Error;

    /// Fires when a certain callback [`Event`] happens on a media server.
    async fn on_event(&self, request: Request) -> Result<(), Self::Error>;
}

/// Request with a fired callback [`Event`] and its meta information.
///
/// Used for invoking and processing callbacks via [`CallbackApi::on_event()`].
///
/// [`CallbackApi::on_event()`]: Api::on_event
#[derive(Debug)]
pub struct Request {
    /// FID (Full ID) of the media [`Element`], the occurred [`Event`] is
    /// related to.
    ///
    /// [`Element`]: crate::Element
    pub fid: Fid,

    /// Occurred [`Event`].
    pub event: Event,

    /// [`DateTime`] when the [`Event`] occurred.
    pub at: DateTime,
}

/// Possible callbacks events which may happen on a media server.
#[derive(Clone, Copy, Debug, From)]
pub enum Event {
    /// [`Member`] joined a [`Room`].
    ///
    /// [`Member`]: crate::Member
    /// [`Room`]: crate::Room
    OnJoin(OnJoinEvent),

    /// [`Member`] left its [`Room`].
    ///
    /// [`Member`]: crate::Member
    /// [`Room`]: crate::Room
    OnLeave(OnLeaveEvent),
}

/// [`Event`] notifying about a [`Member`] joining a [`Room`].
///
/// [`Member`]: crate::Member
/// [`Room`]: crate::Room
#[derive(Clone, Copy, Debug)]
pub struct OnJoinEvent;

/// [`Event`] notifying about a [`Member`] leaving its [`Room`].
///
/// [`Member`]: crate::Member
/// [`Room`]: crate::Room
#[derive(Clone, Copy, Debug)]
pub struct OnLeaveEvent {
    /// Reason of why the [`Member`] leaves.
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

/// Possible reasons of why a [`Member`] leaves its [`Room`].
///
/// [`Member`]: crate::Member
/// [`Room`]: crate::Room
#[derive(Clone, Copy, Debug)]
pub enum OnLeaveReason {
    /// [`Member`] was disconnected normally.
    ///
    /// [`Member`]: crate::Member
    Disconnected,

    /// Connection with the [`Member`] was lost.
    ///
    /// [`Member`]: crate::Member
    Lost,

    /// [`Member`] was forcibly disconnected by a media server.
    ///
    /// [`Member`]: crate::Member
    Kicked,

    /// Media server was shut down.
    Shutdown,
}
