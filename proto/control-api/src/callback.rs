//! API for receiving callbacks from a media server.

use async_trait::async_trait;
use derive_more::{Display, From};
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
    async fn on_event(&self, req: Request) -> Result<(), Self::Error>;
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

    /// [`Member`] Started traffic.
    ///
    /// [`Member`]: crate::Member
    OnStart(OnStartEvent),

    /// [`Member`] Stopped traffic.
    ///
    /// [`Member`]: crate::Member
    OnStop(OnStopEvent),
}

/// `OnStart` callback of Control API.
#[derive(Clone, Copy, Debug)]
pub struct OnStartEvent {
    /// [`MediaDirection`] of the `Endpoint` for which
    /// his callback was received.
    pub media_direction: MediaDirection,
    /// [`MediaType`] of the traffic which starts flowing in some `Endpoint`.
    pub media_type: MediaType,
}

/// Reason of why some `Endpoint` was stopped.
#[derive(Clone, Copy, Debug)]
pub enum OnStopReason {
    /// All traffic of some `Endpoint` was stopped flowing.
    TrafficNotFlowing,

    /// `Endpoint` was muted.
    Muted,

    /// Some traffic flows within `Endpoint`, but incorrectly.
    WrongTrafficFlowing,

    /// Traffic stopped because Endpoint was removed.
    EndpointRemoved,
}

/// Media type of the traffic which starts/stops flowing in some `Endpoint`.
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(u8)]
pub enum MediaType {
    /// Started/stopped audio traffic.
    Audio = 0b1,

    /// Started/stopped video traffic.
    Video = 0b10,

    /// Started/stopped video and audio traffic.
    Both = 0b11,
}

impl From<MediaType> for u8 {
    #[allow(clippy::as_conversions)] // no other way
    fn from(t: MediaType) -> Self {
        t as Self
    }
}

impl MediaType {
    /// Returns [`MediaType`] which was started based on the provided
    /// [`MediaType`]s.
    ///
    /// This [`MediaType`] should be what was before `RTCStat` update and
    /// as argument is [`MediaType`] which was got after `RTCStat` update.
    #[must_use]
    pub const fn get_started(self, after: Self) -> Option<Self> {
        match self {
            Self::Audio => match after {
                Self::Video => Some(Self::Audio),
                Self::Audio | Self::Both => None,
            },
            Self::Video => match after {
                Self::Audio => Some(Self::Video),
                Self::Video | Self::Both => None,
            },
            Self::Both => match after {
                Self::Audio => Some(Self::Video),
                Self::Video => Some(Self::Audio),
                Self::Both => None,
            },
        }
    }
}

/// Media Endpoint for which `OnStart` or `OnStop` Control API callback
/// was received.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MediaDirection {
    /// `Endpoint` is a publisher.
    Publish,

    /// `Endpoint` is a player.
    Play,
}

/// `OnStop` callback of Control API.
#[derive(Clone, Copy, Debug)]
pub struct OnStopEvent {
    /// [`MediaType`] of the traffic which stops flowing in some
    /// `Endpoint`.
    pub media_type: MediaType,

    /// [`MediaDirection`] of the `Endpoint` for which this callback was
    /// received.
    pub media_direction: MediaDirection,

    /// Reason of why `Endpoint` was stopped.
    pub reason: OnStopReason,
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
