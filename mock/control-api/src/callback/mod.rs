//! Control API Callback service implementation.

pub mod server;

use medea_control_api_proto::grpc::callback as proto;
use serde::{Deserialize, Serialize};

/// All callbacks which can happen.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum CallbackEvent {
    /// `OnJoin` callback of Control API.
    OnJoin(join::OnJoin),

    /// `OnLeave` callback of Control API.
    OnLeave(leave::OnLeave),

    /// `OnStart` callback of Control API.
    OnStart(start::OnStart),

    /// `OnStop` callback of Control API.
    OnStop(stop::OnStop),
}

impl From<proto::request::Event> for CallbackEvent {
    fn from(proto: proto::request::Event) -> Self {
        match proto {
            proto::request::Event::OnLeave(on_leave) => {
                Self::OnLeave(on_leave.into())
            }
            proto::request::Event::OnJoin(on_join) => {
                Self::OnJoin(on_join.into())
            }
            proto::request::Event::OnStart(on_start) => {
                Self::OnStart(on_start.into())
            }
            proto::request::Event::OnStop(on_stop) => {
                Self::OnStop(on_stop.into())
            }
        }
    }
}

/// Control API callback.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallbackItem {
    /// FID (Full ID) of element with which this event was occurred.
    pub fid: String,

    /// Event which occurred.
    pub event: CallbackEvent,

    /// Time on which callback was occurred.
    pub at: String,
}

#[allow(clippy::fallible_impl_from)] // intentional
impl From<proto::Request> for CallbackItem {
    fn from(proto: proto::Request) -> Self {
        Self {
            fid: proto.fid,
            at: proto.at,
            event: proto.event.unwrap().into(),
        }
    }
}

pub mod join {
    //! `on_join` callback's related entities and implementations.

    use medea_control_api_proto::grpc::callback as proto;
    use serde::{Deserialize, Serialize};

    /// `OnJoin` callback for Control API.
    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    pub struct OnJoin;

    impl From<proto::OnJoin> for OnJoin {
        fn from(_: proto::OnJoin) -> Self {
            Self
        }
    }
}

pub mod leave {
    //! `on_leave` callback's related entities and implementations.

    use derive_more::Display;
    use medea_control_api_proto::grpc::callback as proto;
    use serde::{Deserialize, Serialize};

    /// `OnLeave` callback of Control API.
    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    pub struct OnLeave {
        /// Reason of why `Member` leaves.
        pub reason: OnLeaveReason,
    }

    impl From<proto::OnLeave> for OnLeave {
        fn from(proto: proto::OnLeave) -> Self {
            Self {
                reason: proto::on_leave::Reason::try_from(proto.reason)
                    .unwrap_or_default()
                    .into(),
            }
        }
    }

    /// Reason of why `Member` leaves.
    #[derive(Clone, Copy, Debug, Deserialize, Display, Serialize)]
    pub enum OnLeaveReason {
        /// `Member` was normally disconnected.
        Disconnected,

        /// Connection with `Member` was lost.
        Lost,

        /// Server is shutting down.
        ServerShutdown,

        /// `Member` was forcibly disconnected by server.
        Kicked,
    }

    impl From<proto::on_leave::Reason> for OnLeaveReason {
        fn from(proto: proto::on_leave::Reason) -> Self {
            use proto::on_leave::Reason as R;

            match proto {
                R::Shutdown => Self::ServerShutdown,
                R::Lost => Self::Lost,
                R::Disconnected => Self::Disconnected,
                R::Kicked => Self::Kicked,
            }
        }
    }
}

/// Media type of the traffic which starts/stops flowing in some `Endpoint`.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum MediaType {
    /// Started/stopped audio traffic.
    Audio,

    /// Started/stopped video traffic.
    Video,

    /// Started/stopped audio and video traffic.
    Both,
}

/// Media Endpoint for which `OnStart` or `OnStop` Control API callback
/// was received.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum MediaDirection {
    /// Endpoint is a publisher.
    Publish,

    /// Endpoint is a player.
    Play,
}

impl From<proto::MediaDirection> for MediaDirection {
    fn from(end: proto::MediaDirection) -> Self {
        match end {
            proto::MediaDirection::Publish => Self::Publish,
            proto::MediaDirection::Play => Self::Play,
        }
    }
}

impl From<proto::MediaType> for MediaType {
    fn from(kind: proto::MediaType) -> Self {
        match kind {
            proto::MediaType::Audio => Self::Audio,
            proto::MediaType::Video => Self::Video,
            proto::MediaType::Both => Self::Both,
        }
    }
}

/// `on_start` callback's related entities and implementations.
mod start {
    use medea_control_api_proto::grpc::callback as proto;
    use serde::{Deserialize, Serialize};

    use super::{MediaDirection, MediaType};

    /// `OnStart` callback for Control API.
    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    pub struct OnStart {
        /// [`MediaType`] of the traffic which starts flowing in some
        /// `Endpoint`.
        media_type: MediaType,
        /// [`MediaDirection`] of the `Endpoint` for which this callback was
        /// received.
        media_direction: MediaDirection,
    }

    impl From<proto::OnStart> for OnStart {
        fn from(ev: proto::OnStart) -> Self {
            Self {
                media_type: proto::MediaType::try_from(ev.media_type)
                    .unwrap_or_default()
                    .into(),
                media_direction: proto::MediaDirection::try_from(
                    ev.media_direction,
                )
                .unwrap_or_default()
                .into(),
            }
        }
    }
}

/// `on_stop` callback's related entities and implementations.
mod stop {
    use derive_more::Display;
    use medea_control_api_proto::grpc::callback as proto;
    use serde::{Deserialize, Serialize};

    use super::{MediaDirection, MediaType};

    /// `OnStop` callback of Control API.
    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    pub struct OnStop {
        /// Reason of why `Endpoint` was stopped.
        pub reason: OnStopReason,

        /// [`MediaType`] of the traffic which starts flowing in some
        /// `Endpoint`.
        pub media_type: MediaType,

        /// [`MediaDirection`] of the `Endpoint` for which this callback was
        /// received.
        pub media_direction: MediaDirection,
    }

    impl From<proto::OnStop> for OnStop {
        fn from(proto: proto::OnStop) -> Self {
            Self {
                reason: proto::on_stop::Reason::try_from(proto.reason)
                    .unwrap_or_default()
                    .into(),
                media_type: proto::MediaType::try_from(proto.media_type)
                    .unwrap_or_default()
                    .into(),
                media_direction: proto::MediaDirection::try_from(
                    proto.media_direction,
                )
                .unwrap_or_default()
                .into(),
            }
        }
    }

    /// Reason of why some `Endpoint` was stopped.
    #[derive(Clone, Copy, Debug, Deserialize, Display, Serialize)]
    pub enum OnStopReason {
        /// All traffic of some `Endpoint` was stopped flowing.
        TrafficNotFlowing,

        /// `Endpoint` was muted.
        Muted,

        /// Source `Endpoint` of a `Endpoint` for which received this `on_stop`
        /// callback was muted.
        SrcMuted,

        /// Some traffic flows within `Endpoint`, but incorrectly.
        WrongTrafficFlowing,

        /// Traffic stopped because Endpoint was removed.
        EndpointRemoved,
    }

    impl From<proto::on_stop::Reason> for OnStopReason {
        fn from(proto: proto::on_stop::Reason) -> Self {
            use proto::on_stop::Reason as R;

            match proto {
                R::TrafficNotFlowing => Self::TrafficNotFlowing,
                R::Muted => Self::Muted,
                R::WrongTrafficFlowing => Self::WrongTrafficFlowing,
                R::EndpointRemoved => Self::EndpointRemoved,
            }
        }
    }
}
