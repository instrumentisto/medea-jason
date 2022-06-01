//! Definitions of conversions from [`ControlApi`] spec into generated from
//! `api.proto` spec via [`tonic-build`] and vise-versa.
//!
//! [`ControlApi`]: crate::ControlApi

use std::time::Duration;

use crate::{
    control::{ParseFidError, Request},
    endpoint::{
        web_rtc_publish::{AudioSettings, P2pMode, Policy, VideoSettings},
        WebRtcPlay, WebRtcPublish,
    },
    grpc::{
        api as proto, conversions::TryFromProtobufError, CallbackUrlParseError,
    },
    member::{self, Credentials},
    Element, Endpoint, Fid, Member, Ping, Pong, Room,
};

impl TryFrom<proto::CreateRequest> for Request {
    type Error = TryFromProtobufError;

    fn try_from(value: proto::CreateRequest) -> Result<Self, Self::Error> {
        let parent_fid = value.parent_fid;
        let el = value.el.ok_or_else(|| {
            TryFromProtobufError::EmptyElementId(parent_fid.clone())
        })?;

        if parent_fid.is_empty() {
            return Room::try_from(el).map(Self::Room);
        }

        match parent_fid.parse::<Fid>()? {
            Fid::Room { id } => {
                Member::try_from(el).map(|member| Self::Member {
                    room_id: id,
                    member: Box::new(member),
                })
            }
            Fid::Member { id, room_id } => {
                Endpoint::try_from(el).map(|endpoint| Self::Endpoint {
                    room_id,
                    member_id: id,
                    endpoint,
                })
            }
            Fid::Endpoint { .. } => Err(TryFromProtobufError::ParseFidError(
                ParseFidError::TooManyPaths(parent_fid.into()),
            )),
        }
    }
}

impl TryFrom<proto::ApplyRequest> for Request {
    type Error = TryFromProtobufError;

    fn try_from(value: proto::ApplyRequest) -> Result<Self, Self::Error> {
        let parent_fid = value.parent_fid;
        let el = value.el.ok_or_else(|| {
            TryFromProtobufError::EmptyElementId(parent_fid.clone())
        })?;

        match parent_fid.parse::<Fid>()? {
            Fid::Room { .. } => Room::try_from(el).map(Self::Room),
            Fid::Member { room_id, .. } => {
                Member::try_from(el).map(|member| Self::Member {
                    room_id,
                    member: Box::new(member),
                })
            }
            Fid::Endpoint { .. } => {
                Err(TryFromProtobufError::UnimplementedCall)
            }
        }
    }
}

impl From<Request> for proto::CreateRequest {
    fn from(req: Request) -> Self {
        let (parent_fid, el) = match req {
            Request::Room(room) => (String::new(), room.into()),
            Request::Member { room_id, member } => (
                Fid::Room { id: room_id }.to_string(),
                member.as_ref().clone().into(),
            ),
            Request::Endpoint {
                room_id,
                member_id,
                endpoint,
            } => (
                Fid::Member {
                    id: member_id,
                    room_id,
                }
                .to_string(),
                endpoint.into(),
            ),
        };

        Self {
            parent_fid,
            el: Some(el),
        }
    }
}

impl From<Request> for proto::ApplyRequest {
    fn from(req: Request) -> Self {
        let (parent_fid, el) = match req {
            Request::Room(room) => (
                Fid::Room {
                    id: room.id.clone(),
                },
                room.into(),
            ),
            Request::Member { room_id, member } => (
                Fid::Member {
                    id: member.id.clone(),
                    room_id,
                },
                member.as_ref().clone().into(),
            ),
            Request::Endpoint {
                room_id,
                member_id,
                endpoint,
            } => (
                Fid::Endpoint {
                    id: endpoint.id(),
                    room_id,
                    member_id,
                },
                endpoint.into(),
            ),
        };

        Self {
            parent_fid: parent_fid.to_string(),
            el: Some(el),
        }
    }
}

impl From<Element> for proto::Element {
    fn from(el: Element) -> Self {
        use proto::element::El as proto_el;

        Self {
            el: Some(match el {
                Element::Member(member) => {
                    proto_el::Member(member.as_ref().clone().into())
                }
                Element::Room(room) => proto_el::Room(room.into()),
                Element::Endpoint(Endpoint::WebRtcPlay(play)) => {
                    proto_el::WebrtcPlay(play.into())
                }
                Element::Endpoint(Endpoint::WebRtcPublish(publish)) => {
                    proto_el::WebrtcPub(publish.into())
                }
            }),
        }
    }
}

impl TryFrom<proto::Element> for Element {
    type Error = TryFromProtobufError;

    fn try_from(el: proto::Element) -> Result<Self, Self::Error> {
        use proto::element::El as proto_el;

        Ok(match el.el.ok_or(TryFromProtobufError::EmptyElement)? {
            proto_el::Member(member) => {
                Self::Member(Box::new(member.try_into()?))
            }
            proto_el::Room(room) => Self::Room(room.try_into()?),
            proto_el::WebrtcPlay(play) => {
                Self::Endpoint(Endpoint::WebRtcPlay(play.try_into()?))
            }
            proto_el::WebrtcPub(publish) => {
                Self::Endpoint(Endpoint::WebRtcPublish(publish.into()))
            }
        })
    }
}

impl From<Element> for proto::create_request::El {
    fn from(el: Element) -> Self {
        match el {
            Element::Room(room) => room.into(),
            Element::Member(member) => member.as_ref().clone().into(),
            Element::Endpoint(endpoint) => endpoint.into(),
        }
    }
}

impl From<Element> for proto::apply_request::El {
    fn from(el: Element) -> Self {
        match el {
            Element::Room(room) => room.into(),
            Element::Member(member) => member.as_ref().clone().into(),
            Element::Endpoint(endpoint) => endpoint.into(),
        }
    }
}

impl TryFrom<proto::create_request::El> for Room {
    type Error = TryFromProtobufError;

    fn try_from(proto: proto::create_request::El) -> Result<Self, Self::Error> {
        use proto::create_request::El as proto_el;

        match proto {
            proto_el::Room(room) => room.try_into(),
            proto_el::Member(proto::Member { id, .. })
            | proto_el::WebrtcPub(proto::WebRtcPublishEndpoint {
                id, ..
            })
            | proto_el::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(TryFromProtobufError::ExpectedOtherElement("Room", id))
            }
        }
    }
}

impl TryFrom<proto::apply_request::El> for Room {
    type Error = TryFromProtobufError;

    fn try_from(proto: proto::apply_request::El) -> Result<Self, Self::Error> {
        use proto::apply_request::El as proto_el;

        match proto {
            proto_el::Room(room) => room.try_into(),
            proto_el::Member(proto::Member { id, .. })
            | proto_el::WebrtcPub(proto::WebRtcPublishEndpoint {
                id, ..
            })
            | proto_el::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(TryFromProtobufError::ExpectedOtherElement("Room", id))
            }
        }
    }
}

impl TryFrom<proto::Room> for Room {
    type Error = TryFromProtobufError;

    fn try_from(room: proto::Room) -> Result<Self, Self::Error> {
        Ok(Self {
            id: room.id.into(),
            pipeline: room
                .pipeline
                .into_iter()
                .map(|(id, room_element)| {
                    room_element.el.map_or(
                        Err(TryFromProtobufError::EmptyElementId(id)),
                        |el| {
                            Member::try_from(el)
                                .map(|member| (member.id.clone(), member))
                        },
                    )
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<Room> for proto::create_request::El {
    fn from(room: Room) -> Self {
        Self::Room(room.into())
    }
}

impl From<Room> for proto::apply_request::El {
    fn from(room: Room) -> Self {
        Self::Room(room.into())
    }
}

impl From<Room> for proto::Room {
    fn from(room: Room) -> Self {
        Self {
            id: room.id.into(),
            pipeline: room
                .pipeline
                .into_iter()
                .map(|(id, m)| (id.into(), m.into()))
                .collect(),
        }
    }
}

impl TryFrom<proto::create_request::El> for Member {
    type Error = TryFromProtobufError;

    fn try_from(proto: proto::create_request::El) -> Result<Self, Self::Error> {
        use proto::create_request::El as proto_el;

        match proto {
            proto_el::Member(member) => member.try_into(),
            proto_el::Room(proto::Room { id, .. })
            | proto_el::WebrtcPub(proto::WebRtcPublishEndpoint {
                id, ..
            })
            | proto_el::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(TryFromProtobufError::ExpectedOtherElement("Member", id))
            }
        }
    }
}

impl TryFrom<proto::apply_request::El> for Member {
    type Error = TryFromProtobufError;

    fn try_from(proto: proto::apply_request::El) -> Result<Self, Self::Error> {
        use proto::apply_request::El as proto_el;

        match proto {
            proto_el::Member(member) => member.try_into(),
            proto_el::Room(proto::Room { id, .. })
            | proto_el::WebrtcPub(proto::WebRtcPublishEndpoint {
                id, ..
            })
            | proto_el::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(TryFromProtobufError::ExpectedOtherElement("Member", id))
            }
        }
    }
}

impl TryFrom<proto::room::element::El> for Member {
    type Error = TryFromProtobufError;

    fn try_from(proto: proto::room::element::El) -> Result<Self, Self::Error> {
        use proto::room::element::El as proto_el;

        match proto {
            proto_el::Member(member) => Self::try_from(member),
            proto_el::WebrtcPub(proto::WebRtcPublishEndpoint {
                id, ..
            })
            | proto_el::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(TryFromProtobufError::ExpectedOtherElement("Member", id))
            }
        }
    }
}

impl TryFrom<proto::Member> for Member {
    type Error = TryFromProtobufError;

    fn try_from(member: proto::Member) -> Result<Self, Self::Error> {
        /// Tries to parse [`Duration`] and maps error to
        /// [`TryFromProtobufError`].
        fn parse_duration<T: TryInto<Duration>>(
            duration: Option<T>,
            member_id: &str,
            field: &'static str,
        ) -> Result<Option<Duration>, TryFromProtobufError> {
            #[allow(clippy::map_err_ignore)]
            duration.map(TryInto::try_into).transpose().map_err(|_| {
                TryFromProtobufError::NegativeDuration(member_id.into(), field)
            })
        }

        let idle_timeout =
            parse_duration(member.idle_timeout, &member.id, "idle_timeout")?;
        let reconnect_timeout = parse_duration(
            member.reconnect_timeout,
            &member.id,
            "reconnect_timeout",
        )?;
        let ping_interval =
            parse_duration(member.ping_interval, &member.id, "ping_interval")?;

        Ok(Self {
            id: member::Id::from(member.id),
            pipeline: member
                .pipeline
                .into_iter()
                .map(|(id, member_el)| {
                    member_el.el.map_or(
                        Err(TryFromProtobufError::EmptyElementId(id)),
                        |el| {
                            Endpoint::try_from(el)
                                .map(|endpoint| (endpoint.id(), endpoint))
                        },
                    )
                })
                .collect::<Result<_, _>>()?,
            credentials: member.credentials.map(Into::into),
            on_join: (!member.on_join.is_empty())
                .then(|| member.on_join.parse())
                .transpose()
                .map_err(CallbackUrlParseError::from)?,
            on_leave: (!member.on_leave.is_empty())
                .then(|| member.on_leave.parse())
                .transpose()
                .map_err(CallbackUrlParseError::from)?,
            idle_timeout,
            reconnect_timeout,
            ping_interval,
        })
    }
}

impl From<Member> for proto::Member {
    fn from(member: Member) -> Self {
        Self {
            id: member.id.into(),
            on_join: member
                .on_join
                .as_ref()
                .map_or_else(String::default, ToString::to_string),
            on_leave: member
                .on_leave
                .as_ref()
                .map_or_else(String::default, ToString::to_string),
            idle_timeout: member.idle_timeout.map(Into::into),
            reconnect_timeout: member.reconnect_timeout.map(Into::into),
            ping_interval: member.ping_interval.map(Into::into),
            pipeline: member
                .pipeline
                .into_iter()
                .map(|(id, e)| (id.into(), e.into()))
                .collect(),
            credentials: member.credentials.map(Into::into),
        }
    }
}

impl From<Member> for proto::create_request::El {
    fn from(member: Member) -> Self {
        Self::Member(member.into())
    }
}

impl From<Member> for proto::apply_request::El {
    fn from(member: Member) -> Self {
        Self::Member(member.into())
    }
}

impl From<Member> for proto::room::Element {
    fn from(member: Member) -> Self {
        Self {
            el: Some(proto::room::element::El::Member(member.into())),
        }
    }
}

impl From<proto::member::Credentials> for Credentials {
    fn from(from: proto::member::Credentials) -> Self {
        use proto::member::Credentials as C;

        match from {
            C::Hash(hash) => Self::Hash(hash.into()),
            C::Plain(plain) => Self::Plain(plain.into()),
        }
    }
}

impl From<Credentials> for proto::member::Credentials {
    fn from(creds: Credentials) -> Self {
        match creds {
            Credentials::Hash(hash) => Self::Hash(hash.into()),
            Credentials::Plain(plain) => Self::Plain(plain.into()),
        }
    }
}

impl TryFrom<proto::create_request::El> for Endpoint {
    type Error = TryFromProtobufError;

    fn try_from(proto: proto::create_request::El) -> Result<Self, Self::Error> {
        use proto::create_request::El as proto_el;

        match proto {
            proto_el::WebrtcPlay(play) => play.try_into().map(Self::WebRtcPlay),
            proto_el::WebrtcPub(publish) => {
                Ok(Self::WebRtcPublish(publish.into()))
            }
            proto_el::Room(proto::Room { id, .. })
            | proto_el::Member(proto::Member { id, .. }) => {
                Err(TryFromProtobufError::ExpectedOtherElement("Endpoint", id))
            }
        }
    }
}

impl TryFrom<proto::member::element::El> for Endpoint {
    type Error = TryFromProtobufError;

    fn try_from(
        proto: proto::member::element::El,
    ) -> Result<Self, Self::Error> {
        use proto::member::element::El;

        match proto {
            El::WebrtcPlay(e) => WebRtcPlay::try_from(e).map(Self::WebRtcPlay),
            El::WebrtcPub(e) => Ok(Self::WebRtcPublish(e.into())),
        }
    }
}

impl From<Endpoint> for proto::member::Element {
    fn from(endpoint: Endpoint) -> Self {
        use proto::member::element::El as proto_el;

        Self {
            el: Some(match endpoint {
                Endpoint::WebRtcPlay(e) => proto_el::WebrtcPlay(e.into()),
                Endpoint::WebRtcPublish(e) => proto_el::WebrtcPub(e.into()),
            }),
        }
    }
}

impl From<Endpoint> for proto::create_request::El {
    fn from(endpoint: Endpoint) -> Self {
        match endpoint {
            Endpoint::WebRtcPublish(publish) => Self::WebrtcPub(publish.into()),
            Endpoint::WebRtcPlay(play) => Self::WebrtcPlay(play.into()),
        }
    }
}

impl From<Endpoint> for proto::apply_request::El {
    fn from(endpoint: Endpoint) -> Self {
        match endpoint {
            Endpoint::WebRtcPublish(publish) => Self::WebrtcPub(publish.into()),
            Endpoint::WebRtcPlay(play) => Self::WebrtcPlay(play.into()),
        }
    }
}

impl TryFrom<proto::WebRtcPlayEndpoint> for WebRtcPlay {
    type Error = TryFromProtobufError;

    fn try_from(value: proto::WebRtcPlayEndpoint) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.into(),
            src: value.src.parse()?,
            force_relay: value.force_relay,
        })
    }
}

impl From<WebRtcPlay> for proto::WebRtcPlayEndpoint {
    fn from(play: WebRtcPlay) -> Self {
        Self {
            id: play.id.into(),
            src: play.src.to_string(),
            on_start: String::new(),
            on_stop: String::new(),
            force_relay: play.force_relay,
        }
    }
}

impl From<proto::WebRtcPublishEndpoint> for WebRtcPublish {
    fn from(value: proto::WebRtcPublishEndpoint) -> Self {
        use proto::web_rtc_publish_endpoint::P2p;

        Self {
            id: value.id.into(),
            p2p: P2p::from_i32(value.p2p).unwrap_or_default().into(),
            audio_settings: value
                .audio_settings
                .map(AudioSettings::from)
                .unwrap_or_default(),
            video_settings: value
                .video_settings
                .map(VideoSettings::from)
                .unwrap_or_default(),
            force_relay: value.force_relay,
        }
    }
}

impl From<WebRtcPublish> for proto::WebRtcPublishEndpoint {
    fn from(publish: WebRtcPublish) -> Self {
        use proto::web_rtc_publish_endpoint::P2p;

        Self {
            id: publish.id.into(),
            p2p: P2p::from(publish.p2p).into(),
            on_start: String::new(),
            on_stop: String::new(),
            force_relay: publish.force_relay,
            audio_settings: Some(publish.audio_settings.into()),
            video_settings: Some(publish.video_settings.into()),
        }
    }
}

impl From<proto::web_rtc_publish_endpoint::AudioSettings> for AudioSettings {
    fn from(from: proto::web_rtc_publish_endpoint::AudioSettings) -> Self {
        use proto::web_rtc_publish_endpoint::PublishPolicy;

        Self {
            publish_policy: PublishPolicy::from_i32(from.publish_policy)
                .unwrap_or_default()
                .into(),
        }
    }
}

impl From<AudioSettings> for proto::web_rtc_publish_endpoint::AudioSettings {
    fn from(settings: AudioSettings) -> Self {
        use proto::web_rtc_publish_endpoint::PublishPolicy;

        Self {
            publish_policy: PublishPolicy::from(settings.publish_policy).into(),
        }
    }
}

impl From<proto::web_rtc_publish_endpoint::VideoSettings> for VideoSettings {
    fn from(from: proto::web_rtc_publish_endpoint::VideoSettings) -> Self {
        use proto::web_rtc_publish_endpoint::PublishPolicy;

        Self {
            publish_policy: PublishPolicy::from_i32(from.publish_policy)
                .unwrap_or_default()
                .into(),
        }
    }
}

impl From<VideoSettings> for proto::web_rtc_publish_endpoint::VideoSettings {
    fn from(settings: VideoSettings) -> Self {
        use proto::web_rtc_publish_endpoint::PublishPolicy;

        Self {
            publish_policy: PublishPolicy::from(settings.publish_policy).into(),
        }
    }
}

impl From<proto::web_rtc_publish_endpoint::PublishPolicy> for Policy {
    fn from(from: proto::web_rtc_publish_endpoint::PublishPolicy) -> Self {
        use proto::web_rtc_publish_endpoint::PublishPolicy as Proto;

        match from {
            Proto::Optional => Self::Optional,
            Proto::Required => Self::Required,
            Proto::Disabled => Self::Disabled,
        }
    }
}

impl From<Policy> for proto::web_rtc_publish_endpoint::PublishPolicy {
    fn from(from: Policy) -> Self {
        match from {
            Policy::Optional => Self::Optional,
            Policy::Required => Self::Required,
            Policy::Disabled => Self::Disabled,
        }
    }
}

impl From<proto::web_rtc_publish_endpoint::P2p> for P2pMode {
    fn from(value: proto::web_rtc_publish_endpoint::P2p) -> Self {
        use proto::web_rtc_publish_endpoint::P2p;

        match value {
            P2p::Always => Self::Always,
            P2p::IfPossible => Self::IfPossible,
            P2p::Never => Self::Never,
        }
    }
}

impl From<P2pMode> for proto::web_rtc_publish_endpoint::P2p {
    fn from(value: P2pMode) -> Self {
        match value {
            P2pMode::Always => Self::Always,
            P2pMode::IfPossible => Self::IfPossible,
            P2pMode::Never => Self::Never,
        }
    }
}

impl From<proto::Ping> for Ping {
    fn from(v: proto::Ping) -> Self {
        Self(v.nonce)
    }
}

impl From<Ping> for proto::Ping {
    fn from(v: Ping) -> Self {
        Self { nonce: v.0 }
    }
}

impl From<proto::Pong> for Pong {
    fn from(v: proto::Pong) -> Self {
        Self(v.nonce)
    }
}

impl From<Pong> for proto::Pong {
    fn from(v: Pong) -> Self {
        Self { nonce: v.0 }
    }
}
