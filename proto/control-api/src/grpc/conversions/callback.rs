//! Definitions of conversions from [`ControlApi`] spec into generated from
//! `callback.proto` spec via [`tonic-build`] and vise-versa.
//!
//! [`ControlApi`]: crate::ControlApi

use time::{
    format_description::well_known::Rfc3339, OffsetDateTime as DateTime,
};

use crate::{
    callback::{Event, OnJoinEvent, OnLeaveEvent, OnLeaveReason, Request},
    grpc::{callback as proto, TryFromProtobufError},
};

impl TryFrom<proto::Request> for Request {
    type Error = TryFromProtobufError;

    fn try_from(value: proto::Request) -> Result<Self, Self::Error> {
        Ok(Self {
            fid: value.fid.parse()?,
            event: value
                .event
                .map(Into::into)
                .ok_or(TryFromProtobufError::EmptyElementId(value.fid))?,
            at: DateTime::parse(&value.at, &Rfc3339)?,
        })
    }
}

impl TryFrom<Request> for proto::Request {
    type Error = TryFromProtobufError;

    fn try_from(req: Request) -> Result<Self, Self::Error> {
        Ok(Self {
            fid: req.fid.to_string(),
            at: req.at.format(&Rfc3339)?,
            event: Some(req.event.into()),
        })
    }
}

impl From<proto::request::Event> for Event {
    fn from(ev: proto::request::Event) -> Self {
        use proto::request::Event as Ev;

        match ev {
            Ev::OnJoin(on_join) => Self::OnJoin(on_join.into()),
            Ev::OnLeave(on_leave) => Self::OnLeave(on_leave.into()),
        }
    }
}

impl From<Event> for proto::request::Event {
    fn from(ev: Event) -> Self {
        match ev {
            Event::OnJoin(on_join) => Self::OnJoin(on_join.into()),
            Event::OnLeave(on_leave) => Self::OnLeave(on_leave.into()),
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
            reason: proto::on_leave::Reason::from_i32(ev.reason)
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

impl From<proto::on_leave::Reason> for OnLeaveReason {
    fn from(rsn: proto::on_leave::Reason) -> Self {
        match rsn {
            proto::on_leave::Reason::LostConnection => Self::Lost,
            proto::on_leave::Reason::ServerShutdown => Self::Shutdown,
            proto::on_leave::Reason::Disconnected => Self::Disconnected,
            proto::on_leave::Reason::Kicked => Self::Kicked,
        }
    }
}

impl From<OnLeaveReason> for proto::on_leave::Reason {
    fn from(rsn: OnLeaveReason) -> Self {
        match rsn {
            OnLeaveReason::Lost => Self::LostConnection,
            OnLeaveReason::Shutdown => Self::ServerShutdown,
            OnLeaveReason::Disconnected => Self::Disconnected,
            OnLeaveReason::Kicked => Self::Kicked,
        }
    }
}
