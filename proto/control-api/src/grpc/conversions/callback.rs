//! Definitions of conversions from [`ControlApi`] spec into generated from
//! `callback.proto` via [`tonic-build`] and vise-versa.
//!
//! [`ControlApi`]: crate::ControlApi

use crate::{
    callback::{Event, OnJoinEvent, OnLeaveEvent, OnLeaveReason, Request},
    grpc::callback as proto,
};

impl From<Request> for proto::Request {
    fn from(req: Request) -> Self {
        Self {
            fid: req.fid.to_string(),
            at: req.at.to_string(),
            event: Some(req.event.into()),
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

impl From<OnJoinEvent> for proto::OnJoin {
    fn from(_: OnJoinEvent) -> Self {
        Self {}
    }
}

impl From<OnLeaveEvent> for proto::OnLeave {
    fn from(ev: OnLeaveEvent) -> Self {
        Self {
            reason: proto::on_leave::Reason::from(ev.reason).into(),
        }
    }
}

impl From<OnLeaveReason> for proto::on_leave::Reason {
    fn from(rsn: OnLeaveReason) -> Self {
        match rsn {
            OnLeaveReason::LostConnection => Self::LostConnection,
            OnLeaveReason::ServerShutdown => Self::ServerShutdown,
            OnLeaveReason::Disconnected => Self::Disconnected,
            OnLeaveReason::Kicked => Self::Kicked,
        }
    }
}
