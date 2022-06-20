//! Conversions between [`ControlApi`] types and the ones generated from
//! `api.proto` [gRPC] spec.
//!
//! [`ControlApi`]: crate::ControlApi
//! [gRPC]: https://grpc.io

use std::time::Duration;

use crate::{
    control::{ParseFidError, Request},
    endpoint::{
        self, web_rtc_play,
        web_rtc_publish::{
            self, AudioSettings, P2pMode, Policy, VideoSettings,
        },
        WebRtcPlay, WebRtcPublish,
    },
    grpc::{api as proto, convert::ProtobufError, CallbackUrlParseError},
    member::{self, Credentials},
    room, Element, Endpoint, Fid, Member, Ping, Pong, Room,
};

impl TryFrom<(Fid, Element)> for proto::Element {
    type Error = ProtobufError;

    fn try_from((fid, el): (Fid, Element)) -> Result<Self, Self::Error> {
        use proto::element::El;

        let el = match (fid, el) {
            (Fid::Room { id }, Element::Room(room)) => {
                Ok(El::Room((id, room).into()))
            }
            (Fid::Member { id, .. }, Element::Member(member)) => {
                Ok(El::Member((id, *member).into()))
            }
            (Fid::Endpoint { id, .. }, Element::Endpoint(endpoint)) => {
                Ok(match endpoint {
                    Endpoint::WebRtcPlayEndpoint { spec } => {
                        El::WebrtcPlay((String::from(id).into(), spec).into())
                    }
                    Endpoint::WebRtcPublishEndpoint { spec } => {
                        El::WebrtcPub((String::from(id).into(), spec).into())
                    }
                })
            }
            (id @ Fid::Room { .. }, _) => Err(ProtobufError::ExpectedElement(
                "Room",
                id.to_string().into(),
            )),
            (id @ Fid::Member { .. }, _) => Err(
                ProtobufError::ExpectedElement("Member", id.to_string().into()),
            ),

            (id @ Fid::Endpoint { .. }, _) => {
                Err(ProtobufError::ExpectedElement(
                    "Endpoint",
                    id.to_string().into(),
                ))
            }
        }?;

        Ok(Self { el: Some(el) })
    }
}

impl TryFrom<proto::CreateRequest> for Request {
    type Error = ProtobufError;

    fn try_from(req: proto::CreateRequest) -> Result<Self, Self::Error> {
        let parent_fid = req.parent_fid;
        let el = req.el.ok_or_else(|| {
            ProtobufError::NoElementForId(parent_fid.as_str().into())
        })?;

        if parent_fid.is_empty() {
            return el.try_into().map(|(id, room)| Self::Room { id, room });
        }

        match parent_fid.parse::<Fid>()? {
            Fid::Room { id: room_id } => {
                el.try_into().map(|(id, member)| Self::Member {
                    id,
                    room_id,
                    member: Box::new(member),
                })
            }
            Fid::Member {
                id: member_id,
                room_id,
            } => el.try_into().map(|(id, endpoint)| Self::Endpoint {
                id,
                room_id,
                member_id,
                endpoint,
            }),
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
            Fid::Room { .. } => {
                el.try_into().map(|(id, room)| Self::Room { id, room })
            }
            Fid::Member { room_id, .. } => {
                el.try_into().map(|(id, member)| Self::Member {
                    id,
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
            Request::Room { id, room } => (String::new(), (id, room).into()),
            Request::Member {
                id,
                room_id,
                member,
            } => (Fid::Room { id: room_id }.to_string(), (id, *member).into()),
            Request::Endpoint {
                id,
                room_id,
                member_id,
                endpoint,
            } => (
                Fid::Member {
                    id: member_id,
                    room_id,
                }
                .to_string(),
                (id, endpoint).into(),
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
            Request::Room { id, room } => {
                (Fid::Room { id: id.clone() }, (id, room).into())
            }
            Request::Member {
                id,
                room_id,
                member,
            } => (
                Fid::Member {
                    id: id.clone(),
                    room_id,
                },
                (id, *member).into(),
            ),
            Request::Endpoint {
                id,
                room_id,
                member_id,
                endpoint,
            } => (
                Fid::Endpoint {
                    id: id.clone(),
                    room_id,
                    member_id,
                },
                (id, endpoint).into(),
            ),
        };

        Self {
            parent_fid: parent_fid.to_string(),
            el: Some(el),
        }
    }
}

impl TryFrom<proto::Element> for Element {
    type Error = ProtobufError;

    fn try_from(el: proto::Element) -> Result<Self, Self::Error> {
        use proto::element::El;

        Ok(match el.el.ok_or(ProtobufError::NoElement)? {
            El::Member(member) => Self::Member(Box::new(
                <(member::Id, Member)>::try_from(member)?.1,
            )),
            El::Room(room) => Self::Room(<(room::Id, Room)>::try_from(room)?.1),
            El::WebrtcPlay(play) => {
                Self::Endpoint(Endpoint::WebRtcPlayEndpoint {
                    spec: <(web_rtc_play::Id, WebRtcPlay)>::try_from(play)?.1,
                })
            }
            El::WebrtcPub(publish) => {
                Self::Endpoint(Endpoint::WebRtcPublishEndpoint {
                    spec: <(web_rtc_publish::Id, WebRtcPublish)>::from(publish)
                        .1,
                })
            }
        })
    }
}

impl TryFrom<proto::create_request::El> for (room::Id, Room) {
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

impl TryFrom<proto::apply_request::El> for (room::Id, Room) {
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

impl TryFrom<proto::Room> for (room::Id, Room) {
    type Error = ProtobufError;

    fn try_from(room: proto::Room) -> Result<Self, Self::Error> {
        Ok((
            room.id.into(),
            Room {
                spec: room
                    .pipeline
                    .into_iter()
                    .map(|(id, room_element)| {
                        room_element.el.map_or(
                            Err(ProtobufError::NoElementForId(id.into())),
                            TryInto::try_into,
                        )
                    })
                    .collect::<Result<_, _>>()?,
            },
        ))
    }
}

impl From<(room::Id, Room)> for proto::create_request::El {
    fn from(room: (room::Id, Room)) -> Self {
        Self::Room(room.into())
    }
}

impl From<(room::Id, Room)> for proto::apply_request::El {
    fn from(room: (room::Id, Room)) -> Self {
        Self::Room(room.into())
    }
}

#[allow(clippy::shadow_unrelated)]
impl From<(room::Id, Room)> for proto::Room {
    fn from((id, room): (room::Id, Room)) -> Self {
        Self {
            id: id.into(),
            pipeline: room
                .spec
                .into_iter()
                .map(|(id, m)| (id.clone().into(), (id, m).into()))
                .collect(),
        }
    }
}

impl From<(member::Id, room::Element)> for proto::room::Element {
    fn from((id, value): (member::Id, room::Element)) -> Self {
        let room::Element::Member(member) = value;

        proto::room::Element {
            el: Some(proto::room::element::El::Member((id, member).into())),
        }
    }
}

impl From<(member::Id, room::Element)> for proto::Member {
    fn from((id, value): (member::Id, room::Element)) -> Self {
        let room::Element::Member(member) = value;
        (id, member).into()
    }
}

impl TryFrom<proto::create_request::El> for (member::Id, Member) {
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

impl TryFrom<proto::apply_request::El> for (member::Id, Member) {
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

impl TryFrom<proto::room::element::El> for (member::Id, room::Element) {
    type Error = ProtobufError;

    fn try_from(val: proto::room::element::El) -> Result<Self, Self::Error> {
        val.try_into().map(|(id, m)| (id, room::Element::Member(m)))
    }
}

impl TryFrom<proto::room::element::El> for (member::Id, Member) {
    type Error = ProtobufError;

    fn try_from(val: proto::room::element::El) -> Result<Self, Self::Error> {
        use proto::room::element::El;

        match val {
            El::Member(member) => member.try_into(),
            El::WebrtcPub(proto::WebRtcPublishEndpoint { id, .. })
            | El::WebrtcPlay(proto::WebRtcPlayEndpoint { id, .. }) => {
                Err(ProtobufError::ExpectedElement("Member", id.into()))
            }
        }
    }
}

impl TryFrom<proto::Member> for (member::Id, Member) {
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

        Ok((
            member.id.into(),
            Member {
                spec: member
                    .pipeline
                    .into_iter()
                    .map(|(id, member_el)| {
                        member_el.el.map_or(
                            Err(ProtobufError::NoElementForId(id.into())),
                            TryInto::try_into,
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
            },
        ))
    }
}

#[allow(clippy::shadow_unrelated)]
impl From<(member::Id, Member)> for proto::Member {
    fn from((id, member): (member::Id, Member)) -> Self {
        Self {
            id: id.into(),
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
                .spec
                .into_iter()
                .map(|(id, e)| (id.clone().into(), (id, e).into()))
                .collect(),
            credentials: member.credentials.map(Into::into),
        }
    }
}

impl From<(member::Id, Member)> for proto::create_request::El {
    fn from(member: (member::Id, Member)) -> Self {
        Self::Member(member.into())
    }
}

impl From<(member::Id, Member)> for proto::apply_request::El {
    fn from(member: (member::Id, Member)) -> Self {
        Self::Member(member.into())
    }
}

impl From<(member::Id, Member)> for proto::room::Element {
    fn from(member: (member::Id, Member)) -> Self {
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

impl TryFrom<proto::create_request::El> for (endpoint::Id, Endpoint) {
    type Error = ProtobufError;

    fn try_from(val: proto::create_request::El) -> Result<Self, Self::Error> {
        use proto::create_request::El;

        match val {
            El::WebrtcPlay(play) => play.try_into().map(|(id, spec)| {
                (id.into(), Endpoint::WebRtcPlayEndpoint { spec })
            }),
            El::WebrtcPub(publish) => {
                let (id, spec) = publish.into();
                Ok((id.into(), Endpoint::WebRtcPublishEndpoint { spec }))
            }
            El::Room(proto::Room { id, .. })
            | El::Member(proto::Member { id, .. }) => {
                Err(ProtobufError::ExpectedElement("Endpoint", id.into()))
            }
        }
    }
}

impl TryFrom<proto::member::element::El> for (endpoint::Id, Endpoint) {
    type Error = ProtobufError;

    fn try_from(val: proto::member::element::El) -> Result<Self, Self::Error> {
        use proto::member::element::El;

        match val {
            El::WebrtcPlay(e) => <(web_rtc_play::Id, WebRtcPlay)>::try_from(e)
                .map(|(id, spec)| {
                    (id.into(), Endpoint::WebRtcPlayEndpoint { spec })
                }),
            El::WebrtcPub(e) => {
                let (id, spec) =
                    <(web_rtc_publish::Id, WebRtcPublish)>::from(e);

                Ok((id.into(), Endpoint::WebRtcPublishEndpoint { spec }))
            }
        }
    }
}

impl From<(endpoint::Id, Endpoint)> for proto::member::Element {
    fn from((id, val): (endpoint::Id, Endpoint)) -> Self {
        use proto::member::element::El;

        Self {
            el: Some(match val {
                Endpoint::WebRtcPlayEndpoint { spec } => {
                    El::WebrtcPlay((String::from(id).into(), spec).into())
                }
                Endpoint::WebRtcPublishEndpoint { spec } => {
                    El::WebrtcPub((String::from(id).into(), spec).into())
                }
            }),
        }
    }
}

impl From<(endpoint::Id, Endpoint)> for proto::create_request::El {
    fn from((id, val): (endpoint::Id, Endpoint)) -> Self {
        match val {
            Endpoint::WebRtcPublishEndpoint { spec } => {
                Self::WebrtcPub((String::from(id).into(), spec).into())
            }
            Endpoint::WebRtcPlayEndpoint { spec } => {
                Self::WebrtcPlay((String::from(id).into(), spec).into())
            }
        }
    }
}

impl From<(endpoint::Id, Endpoint)> for proto::apply_request::El {
    fn from((id, val): (endpoint::Id, Endpoint)) -> Self {
        match val {
            Endpoint::WebRtcPublishEndpoint { spec } => {
                Self::WebrtcPub((String::from(id).into(), spec).into())
            }
            Endpoint::WebRtcPlayEndpoint { spec } => {
                Self::WebrtcPlay((String::from(id).into(), spec).into())
            }
        }
    }
}

impl TryFrom<proto::WebRtcPlayEndpoint> for (web_rtc_play::Id, WebRtcPlay) {
    type Error = ProtobufError;

    fn try_from(val: proto::WebRtcPlayEndpoint) -> Result<Self, Self::Error> {
        Ok((
            val.id.into(),
            WebRtcPlay {
                src: val.src.parse()?,
                force_relay: val.force_relay,
            },
        ))
    }
}

impl From<(web_rtc_play::Id, WebRtcPlay)> for proto::WebRtcPlayEndpoint {
    fn from((id, val): (web_rtc_play::Id, WebRtcPlay)) -> Self {
        Self {
            id: id.into(),
            src: val.src.to_string(),
            on_start: String::new(),
            on_stop: String::new(),
            force_relay: val.force_relay,
        }
    }
}

impl From<proto::WebRtcPublishEndpoint>
    for (web_rtc_publish::Id, WebRtcPublish)
{
    fn from(val: proto::WebRtcPublishEndpoint) -> Self {
        use proto::web_rtc_publish_endpoint::P2p;

        (
            val.id.into(),
            WebRtcPublish {
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
            },
        )
    }
}

impl From<(web_rtc_publish::Id, WebRtcPublish)>
    for proto::WebRtcPublishEndpoint
{
    fn from((id, val): (web_rtc_publish::Id, WebRtcPublish)) -> Self {
        use proto::web_rtc_publish_endpoint::P2p;

        Self {
            id: id.into(),
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
