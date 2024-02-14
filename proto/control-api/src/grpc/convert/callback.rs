//! Conversions between [`CallbackApi`] types and the ones generated from
//! `callback.proto` [gRPC] spec.
//!
//! [`CallbackApi`]: crate::CallbackApi
//! [gRPC]: https://grpc.io

use time::{
    format_description::well_known::Rfc3339, OffsetDateTime as DateTime,
};

use crate::{
    callback::{Event, OnJoinEvent, OnLeaveEvent, OnLeaveReason, Request},
    grpc::{callback as proto, ProtobufError},
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
