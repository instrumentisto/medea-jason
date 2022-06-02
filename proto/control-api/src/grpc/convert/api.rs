//! Conversions between [`ControlApi`] types and the ones generated from
//! `api.proto` [gRPC] spec.
//!
//! [`ControlApi`]: crate::ControlApi
//! [gRPC]: https://grpc.io

use std::time::Duration;

use crate::{
    control::{ParseFidError, Request},
    endpoint::{
        web_rtc_publish::{AudioSettings, P2pMode, Policy, VideoSettings},
        WebRtcPlay, WebRtcPublish,
    },
    grpc::{api as proto, convert::ProtobufError, CallbackUrlParseError},
    member::{self, Credentials},
    Element, Endpoint, Fid, Member, Ping, Pong, Room,
};

impl TryFrom<proto::CreateRequest> for Request {
    type Error = ProtobufError;

    fn try_from(req: proto::CreateRequest) -> Result<Self, Self::Error> {
        let parent_fid = req.parent_fid;
        let el = req.el.ok_or_else(|| {
            ProtobufError::NoElementForId(parent_fid.as_str().into())
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
            Fid::Endpoint { .. } => Err(ProtobufError::ParseFidErr(
                ParseFidError::TooManyPaths(parent_fid.into()),
            )),
        }
    }
}

impl TryFrom<proto::ApplyRequest> for Request {
    type Error = ProtobufError;

    fn try_from(req: proto::ApplyRequest) -> Result<Self, Self::Error> {
        let parent_fid = req.parent_fid;
        let el = req.el.ok_or_else(|| {
            ProtobufError::NoElementForId(parent_fid.as_str().into())
        })?;

        match parent_fid.parse::<Fid>()? {
            Fid::Room { .. } => Room::try_from(el).map(Self::Room),
            Fid::Member { room_id, .. } => {
                Member::try_from(el).map(|member| Self::Member {
                    room_id,
                    member: Box::new(member),
                })
            }
            Fid::Endpoint { .. } => Err(ProtobufError::Unimplemented),
        }
    }
}

impl From<Request> for proto::CreateRequest {
    fn from(req: Request) -> Self {
        let (parent_fid, el) = match req {
            Request::Room(room) => (String::new(), room.into()),
            Request::Member { room_id, member } => {
                (Fid::Room { id: room_id }.to_string(), (*member).into())
            }
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
                    id: endpoint.id().clone(),
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
        use proto::element::El;

        Self {
            el: Some(match el {
                Element::Member(member) => El::Member((*member).into()),
                Element::Room(room) => El::Room(room.into()),
                Element::Endpoint(Endpoint::WebRtcPlay(play)) => {
                    El::WebrtcPlay(play.into())
                }
                Element::Endpoint(Endpoint::WebRtcPublish(publish)) => {
                    El::WebrtcPub(publish.into())
                }
            }),
        }
    }
}

impl TryFrom<proto::Element> for Element {
    type Error = ProtobufError;

    fn try_from(el: proto::Element) -> Result<Self, Self::Error> {
        use proto::element::El;

        Ok(match el.el.ok_or(ProtobufError::NoElement)? {
            El::Member(member) => Self::Member(Box::new(member.try_into()?)),
            El::Room(room) => Self::Room(room.try_into()?),
            El::WebrtcPlay(play) => {
                Self::Endpoint(Endpoint::WebRtcPlay(play.try_into()?))
            }
            El::WebrtcPub(publish) => {
                Self::Endpoint(Endpoint::WebRtcPublish(publish.into()))
            }
        })
    }
}

impl From<Element> for proto::create_request::El {
    fn from(el: Element) -> Self {
        match el {
            Element::Room(room) => room.into(),
            Element::Member(member) => (*member).into(),
            Element::Endpoint(endpoint) => endpoint.into(),
        }
    }
}

impl From<Element> for proto::apply_request::El {
    fn from(el: Element) -> Self {
        match el {
            Element::Room(room) => room.into(),
            Element::Member(member) => (*member).into(),
            Element::Endpoint(endpoint) => endpoint.into(),
        }
    }
}

impl TryFrom<proto::create_request::El> for Room {
    type Error = ProtobufError;

    fn try_from(val: proto::create_request::El) -> Result<Self, Self::Error> {
        use proto::create_request::El;

        match val {
            El::Room(room) => room.try_into(),
            El::Member(proto::Member { id, .. })
            | El::WebrtcPub(proto::WebRtcPublishEndpoint { id, .. })
            | El::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(ProtobufError::ExpectedElement("Room", id.into()))
            }
        }
    }
}

impl TryFrom<proto::apply_request::El> for Room {
    type Error = ProtobufError;

    fn try_from(val: proto::apply_request::El) -> Result<Self, Self::Error> {
        use proto::apply_request::El;

        match val {
            El::Room(room) => room.try_into(),
            El::Member(proto::Member { id, .. })
            | El::WebrtcPub(proto::WebRtcPublishEndpoint { id, .. })
            | El::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(ProtobufError::ExpectedElement("Room", id.into()))
            }
        }
    }
}

impl TryFrom<proto::Room> for Room {
    type Error = ProtobufError;

    fn try_from(room: proto::Room) -> Result<Self, Self::Error> {
        Ok(Self {
            id: room.id.into(),
            pipeline: room
                .pipeline
                .into_iter()
                .map(|(id, room_element)| {
                    room_element.el.map_or(
                        Err(ProtobufError::NoElementForId(id.into())),
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
    type Error = ProtobufError;

    fn try_from(val: proto::create_request::El) -> Result<Self, Self::Error> {
        use proto::create_request::El;

        match val {
            El::Member(member) => member.try_into(),
            El::Room(proto::Room { id, .. })
            | El::WebrtcPub(proto::WebRtcPublishEndpoint { id, .. })
            | El::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(ProtobufError::ExpectedElement("Member", id.into()))
            }
        }
    }
}

impl TryFrom<proto::apply_request::El> for Member {
    type Error = ProtobufError;

    fn try_from(val: proto::apply_request::El) -> Result<Self, Self::Error> {
        use proto::apply_request::El;

        match val {
            El::Member(member) => member.try_into(),
            El::Room(proto::Room { id, .. })
            | El::WebrtcPub(proto::WebRtcPublishEndpoint { id, .. })
            | El::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(ProtobufError::ExpectedElement("Member", id.into()))
            }
        }
    }
}

impl TryFrom<proto::room::element::El> for Member {
    type Error = ProtobufError;

    fn try_from(val: proto::room::element::El) -> Result<Self, Self::Error> {
        use proto::room::element::El;

        match val {
            El::Member(member) => Self::try_from(member),
            El::WebrtcPub(proto::WebRtcPublishEndpoint { id, .. })
            | El::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(ProtobufError::ExpectedElement("Member", id.into()))
            }
        }
    }
}

impl TryFrom<proto::Member> for Member {
    type Error = ProtobufError;

    fn try_from(member: proto::Member) -> Result<Self, Self::Error> {
        /// Tries to parse [`Duration`] and maps its error into a
        /// [`TryFromProtobufError`].
        fn parse_duration<T: TryInto<Duration>>(
            duration: Option<T>,
            member_id: &str,
            field: &'static str,
        ) -> Result<Option<Duration>, ProtobufError> {
            #[allow(clippy::map_err_ignore)]
            duration.map(TryInto::try_into).transpose().map_err(|_| {
                ProtobufError::InvalidDuration(member_id.into(), field)
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
                        Err(ProtobufError::NoElementForId(id.into())),
                        |el| {
                            Endpoint::try_from(el)
                                .map(|endpnt| (endpnt.id().clone(), endpnt))
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
    fn from(val: proto::member::Credentials) -> Self {
        use proto::member::Credentials as Creds;

        match val {
            Creds::Hash(hash) => Self::Hash(hash.into()),
            Creds::Plain(plain) => Self::Plain(plain.into()),
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
    type Error = ProtobufError;

    fn try_from(val: proto::create_request::El) -> Result<Self, Self::Error> {
        use proto::create_request::El;

        match val {
            El::WebrtcPlay(play) => play.try_into().map(Self::WebRtcPlay),
            El::WebrtcPub(publish) => Ok(Self::WebRtcPublish(publish.into())),
            El::Room(proto::Room { id, .. })
            | El::Member(proto::Member { id, .. }) => {
                Err(ProtobufError::ExpectedElement("Endpoint", id.into()))
            }
        }
    }
}

impl TryFrom<proto::member::element::El> for Endpoint {
    type Error = ProtobufError;

    fn try_from(val: proto::member::element::El) -> Result<Self, Self::Error> {
        use proto::member::element::El;

        match val {
            El::WebrtcPlay(e) => WebRtcPlay::try_from(e).map(Self::WebRtcPlay),
            El::WebrtcPub(e) => Ok(Self::WebRtcPublish(e.into())),
        }
    }
}

impl From<Endpoint> for proto::member::Element {
    fn from(val: Endpoint) -> Self {
        use proto::member::element::El;

        Self {
            el: Some(match val {
                Endpoint::WebRtcPlay(e) => El::WebrtcPlay(e.into()),
                Endpoint::WebRtcPublish(e) => El::WebrtcPub(e.into()),
            }),
        }
    }
}

impl From<Endpoint> for proto::create_request::El {
    fn from(val: Endpoint) -> Self {
        match val {
            Endpoint::WebRtcPublish(publish) => Self::WebrtcPub(publish.into()),
            Endpoint::WebRtcPlay(play) => Self::WebrtcPlay(play.into()),
        }
    }
}

impl From<Endpoint> for proto::apply_request::El {
    fn from(val: Endpoint) -> Self {
        match val {
            Endpoint::WebRtcPublish(publish) => Self::WebrtcPub(publish.into()),
            Endpoint::WebRtcPlay(play) => Self::WebrtcPlay(play.into()),
        }
    }
}

impl TryFrom<proto::WebRtcPlayEndpoint> for WebRtcPlay {
    type Error = ProtobufError;

    fn try_from(val: proto::WebRtcPlayEndpoint) -> Result<Self, Self::Error> {
        Ok(Self {
            id: val.id.into(),
            src: val.src.parse()?,
            force_relay: val.force_relay,
        })
    }
}

impl From<WebRtcPlay> for proto::WebRtcPlayEndpoint {
    fn from(val: WebRtcPlay) -> Self {
        Self {
            id: val.id.into(),
            src: val.src.to_string(),
            on_start: String::new(),
            on_stop: String::new(),
            force_relay: val.force_relay,
        }
    }
}

impl From<proto::WebRtcPublishEndpoint> for WebRtcPublish {
    fn from(val: proto::WebRtcPublishEndpoint) -> Self {
        use proto::web_rtc_publish_endpoint::P2p;

        Self {
            id: val.id.into(),
            p2p: P2p::from_i32(val.p2p).unwrap_or_default().into(),
            audio_settings: val
                .audio_settings
                .map(AudioSettings::from)
                .unwrap_or_default(),
            video_settings: val
                .video_settings
                .map(VideoSettings::from)
                .unwrap_or_default(),
            force_relay: val.force_relay,
        }
    }
}

impl From<WebRtcPublish> for proto::WebRtcPublishEndpoint {
    fn from(val: WebRtcPublish) -> Self {
        use proto::web_rtc_publish_endpoint::P2p;

        Self {
            id: val.id.into(),
            p2p: P2p::from(val.p2p).into(),
            on_start: String::new(),
            on_stop: String::new(),
            force_relay: val.force_relay,
            audio_settings: Some(val.audio_settings.into()),
            video_settings: Some(val.video_settings.into()),
        }
    }
}

impl From<proto::web_rtc_publish_endpoint::AudioSettings> for AudioSettings {
    fn from(val: proto::web_rtc_publish_endpoint::AudioSettings) -> Self {
        use proto::web_rtc_publish_endpoint::PublishPolicy;

        Self {
            publish_policy: PublishPolicy::from_i32(val.publish_policy)
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
    fn from(val: proto::web_rtc_publish_endpoint::VideoSettings) -> Self {
        use proto::web_rtc_publish_endpoint::PublishPolicy;

        Self {
            publish_policy: PublishPolicy::from_i32(val.publish_policy)
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
    fn from(val: proto::web_rtc_publish_endpoint::PublishPolicy) -> Self {
        use proto::web_rtc_publish_endpoint::PublishPolicy as Proto;

        match val {
            Proto::Optional => Self::Optional,
            Proto::Required => Self::Required,
            Proto::Disabled => Self::Disabled,
        }
    }
}

impl From<Policy> for proto::web_rtc_publish_endpoint::PublishPolicy {
    fn from(val: Policy) -> Self {
        match val {
            Policy::Optional => Self::Optional,
            Policy::Required => Self::Required,
            Policy::Disabled => Self::Disabled,
        }
    }
}

impl From<proto::web_rtc_publish_endpoint::P2p> for P2pMode {
    fn from(val: proto::web_rtc_publish_endpoint::P2p) -> Self {
        use proto::web_rtc_publish_endpoint::P2p;

        match val {
            P2p::Always => Self::Always,
            P2p::IfPossible => Self::IfPossible,
            P2p::Never => Self::Never,
        }
    }
}

impl From<P2pMode> for proto::web_rtc_publish_endpoint::P2p {
    fn from(val: P2pMode) -> Self {
        match val {
            P2pMode::Always => Self::Always,
            P2pMode::IfPossible => Self::IfPossible,
            P2pMode::Never => Self::Never,
        }
    }
}

impl From<proto::Ping> for Ping {
    fn from(val: proto::Ping) -> Self {
        Self(val.nonce)
    }
}

impl From<Ping> for proto::Ping {
    fn from(val: Ping) -> Self {
        Self { nonce: val.0 }
    }
}

impl From<proto::Pong> for Pong {
    fn from(val: proto::Pong) -> Self {
        Self(val.nonce)
    }
}

impl From<Pong> for proto::Pong {
    fn from(val: Pong) -> Self {
        Self { nonce: val.0 }
    }
}
