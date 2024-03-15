//! Conversions between [`CallbackApi`] types and the ones generated from
//! `callback.proto` [gRPC] spec.
//!
//! [`CallbackApi`]: crate::CallbackApi
//! [gRPC]: https://grpc.io

use time::{
    format_description::well_known::Rfc3339, OffsetDateTime as DateTime,
};

use crate::{
    callback::{
        Event, MediaDirection, MediaType, OnJoinEvent, OnLeaveEvent,
        OnLeaveReason, OnStartEvent, OnStopEvent, OnStopReason, Request,
    },
    grpc::{
        callback::{self as proto},
        ProtobufError,
    },
};

impl TryFrom<proto::Request> for Request {
    type Error = ProtobufError;

    fn try_from(value: proto::Request) -> Result<Self, Self::Error> {
        Ok(Self {
            fid: value.fid.parse()?,
            event: value
                .event
                .ok_or_else(|| ProtobufError::NoElementForId(value.fid.into()))?
                .into(),
            at: DateTime::parse(&value.at, &Rfc3339)?,
        })
    }
}

impl From<Request> for proto::Request {
    fn from(req: Request) -> Self {
        Self {
            fid: req.fid.to_string(),
            at: req
                .at
                .format(&Rfc3339)
                .unwrap_or_else(|e| unreachable!("{e}")),
            event: Some(req.event.into()),
        }
    }
}

impl From<proto::request::Event> for Event {
    fn from(ev: proto::request::Event) -> Self {
        use proto::request::Event;

        match ev {
            Event::OnJoin(on_join) => Self::OnJoin(on_join.into()),
            Event::OnLeave(on_leave) => Self::OnLeave(on_leave.into()),
            Event::OnStart(on_start) => Self::OnStart(on_start.into()),
            Event::OnStop(on_stop) => Self::OnStop(on_stop.into()),
        }
    }
}

impl From<Event> for proto::request::Event {
    fn from(ev: Event) -> Self {
        match ev {
            Event::OnJoin(on_join) => Self::OnJoin(on_join.into()),
            Event::OnLeave(on_leave) => Self::OnLeave(on_leave.into()),
            Event::OnStart(on_start) => Self::OnStart(on_start.into()),
            Event::OnStop(on_stop) => Self::OnStop(on_stop.into()),
        }
    }
}

impl From<proto::OnJoin> for OnJoinEvent {
    fn from(_: proto::OnJoin) -> Self {
        Self {}
    }
}

impl From<OnJoinEvent> for proto::OnJoin {
    fn from(_: OnJoinEvent) -> Self {
        Self {}
    }
}

impl From<proto::OnLeave> for OnLeaveEvent {
    fn from(ev: proto::OnLeave) -> Self {
        Self {
            reason: proto::on_leave::Reason::try_from(ev.reason)
                .unwrap_or_default()
                .into(),
        }
    }
}

impl From<OnLeaveEvent> for proto::OnLeave {
    fn from(ev: OnLeaveEvent) -> Self {
        Self {
            reason: proto::on_leave::Reason::from(ev.reason).into(),
        }
    }
}

impl From<OnStartEvent> for proto::OnStart {
    fn from(ev: OnStartEvent) -> Self {
        Self {
            media_type: proto::MediaType::from(ev.media_type).into(),
            media_direction: proto::MediaDirection::from(ev.media_direction)
                .into(),
        }
    }
}

impl From<proto::OnStart> for OnStartEvent {
    fn from(ev: proto::OnStart) -> Self {
        Self {
            media_direction: proto::MediaDirection::try_from(
                ev.media_direction,
            )
            .unwrap_or_default()
            .into(),
            media_type: proto::MediaType::try_from(ev.media_type)
                .unwrap_or_default()
                .into(),
        }
    }
}

impl From<OnStopEvent> for proto::OnStop {
    fn from(ev: OnStopEvent) -> Self {
        Self {
            reason: proto::on_stop::Reason::from(ev.reason).into(),
            media_type: proto::MediaType::from(ev.media_type).into(),
            media_direction: proto::MediaDirection::from(ev.media_direction)
                .into(),
        }
    }
}

impl From<proto::OnStop> for OnStopEvent {
    fn from(ev: proto::OnStop) -> Self {
        Self {
            reason: proto::on_stop::Reason::try_from(ev.reason)
                .unwrap_or_default()
                .into(),
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

impl From<proto::on_leave::Reason> for OnLeaveReason {
    fn from(rsn: proto::on_leave::Reason) -> Self {
        use proto::on_leave::Reason;

        match rsn {
            Reason::Lost => Self::Lost,
            Reason::Shutdown => Self::Shutdown,
            Reason::Disconnected => Self::Disconnected,
            Reason::Kicked => Self::Kicked,
        }
    }
}

impl From<OnLeaveReason> for proto::on_leave::Reason {
    fn from(rsn: OnLeaveReason) -> Self {
        match rsn {
            OnLeaveReason::Lost => Self::Lost,
            OnLeaveReason::Shutdown => Self::Shutdown,
            OnLeaveReason::Disconnected => Self::Disconnected,
            OnLeaveReason::Kicked => Self::Kicked,
        }
    }
}

impl From<proto::on_stop::Reason> for OnStopReason {
    fn from(rsn: proto::on_stop::Reason) -> Self {
        use proto::on_stop::Reason;

        match rsn {
            Reason::TrafficNotFlowing => Self::TrafficNotFlowing,
            Reason::Muted => Self::Muted,
            Reason::WrongTrafficFlowing => Self::WrongTrafficFlowing,
            Reason::EndpointRemoved => Self::EndpointRemoved,
        }
    }
}

impl From<OnStopReason> for proto::on_stop::Reason {
    fn from(rsn: OnStopReason) -> Self {
        match rsn {
            OnStopReason::TrafficNotFlowing => Self::TrafficNotFlowing,
            OnStopReason::Muted => Self::Muted,
            OnStopReason::WrongTrafficFlowing => Self::WrongTrafficFlowing,
            OnStopReason::EndpointRemoved => Self::EndpointRemoved,
        }
    }
}

impl From<MediaDirection> for proto::MediaDirection {
    fn from(end: MediaDirection) -> Self {
        match end {
            MediaDirection::Publish => Self::Publish,
            MediaDirection::Play => Self::Play,
        }
    }
}

impl From<MediaType> for proto::MediaType {
    fn from(kind: MediaType) -> Self {
        match kind {
            MediaType::Audio => Self::Audio,
            MediaType::Video => Self::Video,
            MediaType::Both => Self::Both,
        }
    }
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
