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
        web_rtc_play::WebRtcPlay,
        web_rtc_publish::{
            self, AudioSettings, P2pMode, Policy, VideoSettings, WebRtcPublish,
        },
    },
    grpc::{api as proto, convert::ProtobufError, CallbackUrlParseError},
    member::{self, Credentials},
    room, Element, Endpoint, Fid, Member, Ping, Pong, Room,
};

impl TryFrom<(Fid, Element)> for proto::Element {
    type Error = ProtobufError;

    fn try_from((fid, spec): (Fid, Element)) -> Result<Self, Self::Error> {
        use proto::element::El;

        let el = match (fid, spec) {
            (Fid::Room { id }, Element::Room(room)) => Ok(El::Room(
                Room {
                    id,
                    spec: room.spec,
                }
                .into(),
            )),
            (Fid::Member { id, .. }, Element::Member(member)) => {
                Ok(El::Member(
                    Member {
                        id,
                        spec: member.spec,
                    }
                    .into(),
                ))
            }
            (Fid::Endpoint { id, .. }, Element::Endpoint(endpoint)) => {
                Ok(Endpoint {
                    id,
                    spec: endpoint.spec,
                }
                .into())
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
            return Room::try_from(el).map(|room| Self::Room {
                id: room.id,
                spec: room.spec,
            });
        }

        match parent_fid.parse::<Fid>()? {
            Fid::Room { id: room_id } => {
                Member::try_from(el).map(|member| Self::Member {
                    id: member.id,
                    room_id,
                    spec: Box::new(member.spec),
                })
            }
            Fid::Member {
                id: member_id,
                room_id,
            } => Endpoint::try_from(el).map(|endpoint| Self::Endpoint {
                id: endpoint.id,
                room_id,
                member_id,
                spec: endpoint.spec,
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
            Fid::Room { .. } => Room::try_from(el).map(|room| Self::Room {
                id: room.id,
                spec: room.spec,
            }),
            Fid::Member { room_id, .. } => {
                Member::try_from(el).map(|member| Self::Member {
                    id: member.id,
                    room_id,
                    spec: Box::new(member.spec),
                })
            }
            Fid::Endpoint { .. } => Err(ProtobufError::Unimplemented),
        }
    }
}

impl From<Request> for proto::CreateRequest {
    fn from(req: Request) -> Self {
        let (parent_fid, el) = match req {
            Request::Room { id, spec } => {
                (String::new(), Room { id, spec }.into())
            }
            Request::Member { id, room_id, spec } => (
                Fid::Room { id: room_id }.to_string(),
                Member { id, spec: *spec }.into(),
            ),
            Request::Endpoint {
                id,
                room_id,
                member_id,
                spec,
            } => (
                Fid::Member {
                    id: member_id,
                    room_id,
                }
                .to_string(),
                Endpoint { id, spec }.into(),
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
            Request::Room { id, spec } => {
                (Fid::Room { id: id.clone() }, Room { id, spec }.into())
            }
            Request::Member { id, room_id, spec } => (
                Fid::Member {
                    id: id.clone(),
                    room_id,
                },
                Member { id, spec: *spec }.into(),
            ),
            Request::Endpoint {
                id,
                room_id,
                member_id,
                spec,
            } => (
                Fid::Endpoint {
                    id: id.clone(),
                    room_id,
                    member_id,
                },
                Endpoint { id, spec }.into(),
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
            El::Member(member) => {
                Self::Member(Box::new(Member::try_from(member)?))
            }
            El::Room(room) => Self::Room(Room::try_from(room)?),
            El::WebrtcPlay(play) => {
                Self::Endpoint(WebRtcPlay::try_from(play)?.into())
            }
            El::WebrtcPub(publish) => {
                Self::Endpoint(WebRtcPublish::try_from(publish)?.into())
            }
        })
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
            spec: room::Spec {
                pipeline: room
                    .pipeline
                    .into_iter()
                    .map(|(id, room_el)| {
                        room_el.el.map_or(
                            Err(ProtobufError::NoElementForId(id.into())),
                            |el| {
                                Member::try_from(el)
                                    .map(|m| (m.id, m.spec.into()))
                            },
                        )
                    })
                    .collect::<Result<_, _>>()?,
            },
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
                .spec
                .pipeline
                .into_iter()
                .map(|(id, el)| match el {
                    room::PipelineSpec::Member(spec) => {
                        (id.clone().into(), Member { id, spec }.into())
                    }
                })
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
            El::Member(member) => member.try_into(),
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
            #[expect(clippy::map_err_ignore, reason = "not useful")]
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
            id: member.id.into(),
            spec: member::Spec {
                pipeline: member
                    .pipeline
                    .into_iter()
                    .map(|(id, member_el)| {
                        member_el.el.map_or(
                            Err(ProtobufError::NoElementForId(id.into())),
                            |el| Endpoint::try_from(el).map(|e| (e.id, e.spec)),
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
        })
    }
}

impl From<Member> for proto::Member {
    fn from(member: Member) -> Self {
        #[expect( // intentional
            clippy::unwrap_used,
            reason = " `Duration` values are not expected to be large"
        )]
        Self {
            id: member.id.into(),
            on_join: member
                .spec
                .on_join
                .as_ref()
                .map_or_else(String::default, ToString::to_string),
            on_leave: member
                .spec
                .on_leave
                .as_ref()
                .map_or_else(String::default, ToString::to_string),
            idle_timeout: member
                .spec
                .idle_timeout
                .map(|d| d.try_into().unwrap()),
            reconnect_timeout: member
                .spec
                .reconnect_timeout
                .map(|d| d.try_into().unwrap()),
            ping_interval: member
                .spec
                .ping_interval
                .map(|d| d.try_into().unwrap()),
            pipeline: member
                .spec
                .pipeline
                .into_iter()
                .map(|(id, spec)| {
                    (id.clone().into(), Endpoint { id, spec }.into())
                })
                .collect(),
            credentials: member.spec.credentials.map(Into::into),
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
            El::WebrtcPlay(play) => WebRtcPlay::try_from(play).map(Self::from),
            El::WebrtcPub(publish) => {
                WebRtcPublish::try_from(publish).map(Self::from)
            }
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
            El::WebrtcPub(e) => WebRtcPublish::try_from(e).map(Self::from),
            El::WebrtcPlay(e) => WebRtcPlay::try_from(e).map(Self::from),
        }
    }
}

impl From<Endpoint> for proto::member::Element {
    fn from(endpoint: Endpoint) -> Self {
        use proto::member::element::El;

        Self {
            el: Some(match endpoint.spec {
                endpoint::Spec::WebRtcPublishEndpoint(spec) => El::WebrtcPub(
                    WebRtcPublish {
                        id: String::from(endpoint.id).into(),
                        spec,
                    }
                    .into(),
                ),
                endpoint::Spec::WebRtcPlayEndpoint(spec) => El::WebrtcPlay(
                    WebRtcPlay {
                        id: String::from(endpoint.id).into(),
                        spec,
                    }
                    .into(),
                ),
            }),
        }
    }
}

impl From<Endpoint> for proto::element::El {
    fn from(endpoint: Endpoint) -> Self {
        match endpoint.spec {
            endpoint::Spec::WebRtcPublishEndpoint(spec) => Self::WebrtcPub(
                WebRtcPublish {
                    id: String::from(endpoint.id).into(),
                    spec,
                }
                .into(),
            ),
            endpoint::Spec::WebRtcPlayEndpoint(spec) => Self::WebrtcPlay(
                WebRtcPlay {
                    id: String::from(endpoint.id).into(),
                    spec,
                }
                .into(),
            ),
        }
    }
}

impl From<Endpoint> for proto::create_request::El {
    fn from(endpoint: Endpoint) -> Self {
        match endpoint.spec {
            endpoint::Spec::WebRtcPublishEndpoint(spec) => Self::WebrtcPub(
                WebRtcPublish {
                    id: String::from(endpoint.id).into(),
                    spec,
                }
                .into(),
            ),
            endpoint::Spec::WebRtcPlayEndpoint(spec) => Self::WebrtcPlay(
                WebRtcPlay {
                    id: String::from(endpoint.id).into(),
                    spec,
                }
                .into(),
            ),
        }
    }
}

impl From<Endpoint> for proto::apply_request::El {
    fn from(endpoint: Endpoint) -> Self {
        match endpoint.spec {
            endpoint::Spec::WebRtcPublishEndpoint(spec) => Self::WebrtcPub(
                WebRtcPublish {
                    id: String::from(endpoint.id).into(),
                    spec,
                }
                .into(),
            ),
            endpoint::Spec::WebRtcPlayEndpoint(spec) => Self::WebrtcPlay(
                WebRtcPlay {
                    id: String::from(endpoint.id).into(),
                    spec,
                }
                .into(),
            ),
        }
    }
}

impl TryFrom<proto::WebRtcPlayEndpoint> for WebRtcPlay {
    type Error = ProtobufError;

    fn try_from(val: proto::WebRtcPlayEndpoint) -> Result<Self, Self::Error> {
        Ok(Self {
            id: val.id.into(),
            spec: web_rtc_play::Spec {
                src: val.src.parse()?,
                force_relay: val.force_relay,
            },
        })
    }
}

impl From<WebRtcPlay> for proto::WebRtcPlayEndpoint {
    fn from(play: WebRtcPlay) -> Self {
        Self {
            id: play.id.into(),
            src: play.spec.src.to_string(),
            on_start: String::new(),
            on_stop: String::new(),
            force_relay: play.spec.force_relay,
        }
    }
}

impl TryFrom<proto::WebRtcPublishEndpoint> for WebRtcPublish {
    type Error = ProtobufError;

    fn try_from(
        val: proto::WebRtcPublishEndpoint,
    ) -> Result<Self, Self::Error> {
        use proto::web_rtc_publish_endpoint::P2p;

        Ok(Self {
            id: val.id.into(),
            spec: web_rtc_publish::Spec {
                p2p: P2p::try_from(val.p2p).unwrap_or_default().into(),
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
        })
    }
}

impl From<WebRtcPublish> for proto::WebRtcPublishEndpoint {
    fn from(publish: WebRtcPublish) -> Self {
        use proto::web_rtc_publish_endpoint::P2p;

        Self {
            id: publish.id.into(),
            p2p: P2p::from(publish.spec.p2p).into(),
            on_start: String::new(),
            on_stop: String::new(),
            force_relay: publish.spec.force_relay,
            audio_settings: Some(publish.spec.audio_settings.into()),
            video_settings: Some(publish.spec.video_settings.into()),
        }
    }
}

impl From<proto::web_rtc_publish_endpoint::AudioSettings> for AudioSettings {
    fn from(val: proto::web_rtc_publish_endpoint::AudioSettings) -> Self {
        use proto::web_rtc_publish_endpoint::PublishPolicy;

        Self {
            publish_policy: PublishPolicy::try_from(val.publish_policy)
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
            publish_policy: PublishPolicy::try_from(val.publish_policy)
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
