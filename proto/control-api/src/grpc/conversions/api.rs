//! Definitions of conversions from [`ControlApi`] spec into generated from
//! `api.proto` via [`tonic-build`] and vise-versa.
//!
//! [`ControlApi`]: crate::ControlApi

use std::{collections::HashMap, time::Duration};

use crate::{
    endpoint::{
        self,
        web_rtc_play::SrcUri,
        web_rtc_publish::{AudioSettings, P2pMode, Policy, VideoSettings},
        WebRtcPlay, WebRtcPublish,
    },
    grpc::{
        api as proto,
        conversions::{CallbackUrl, TryFromProtobufError},
    },
    member::{self, Credentials},
    Element, Endpoint, ErrorResponse, Member, Room,
};

impl From<Element> for proto::Element {
    fn from(el: Element) -> Self {
        let el = match el {
            Element::Member(member) => {
                proto::element::El::Member(member.into())
            }
            Element::Room(room) => proto::element::El::Room(room.into()),
            Element::Endpoint(Endpoint::WebRtcPlay(play)) => {
                proto::element::El::WebrtcPlay(play.into())
            }
            Element::Endpoint(Endpoint::WebRtcPublish(publish)) => {
                proto::element::El::WebrtcPub(publish.into())
            }
        };

        Self { el: Some(el) }
    }
}

impl TryFrom<proto::create_request::El> for Room {
    type Error = TryFromProtobufError;

    fn try_from(proto: proto::create_request::El) -> Result<Self, Self::Error> {
        use proto::create_request::El as proto_el;

        let id = match proto {
            proto_el::Room(room) => {
                let mut pipeline = HashMap::new();
                for (id, room_element) in room.pipeline {
                    if let Some(elem) = room_element.el {
                        let member =
                            Member::try_from((member::Id(id.clone()), elem))?;
                        drop(pipeline.insert(id.into(), member));
                    } else {
                        return Err(TryFromProtobufError::EmptyElement(id));
                    }
                }

                return Ok(Self {
                    id: room.id.into(),
                    pipeline,
                });
            }
            proto_el::Member(member) => member.id,
            proto_el::WebrtcPub(webrtc_pub) => webrtc_pub.id,
            proto_el::WebrtcPlay(webrtc_play) => webrtc_play.id,
        };

        Err(TryFromProtobufError::ExpectedOtherElement(
            String::from("Room"),
            id,
        ))
    }
}

impl TryFrom<proto::apply_request::El> for Room {
    type Error = TryFromProtobufError;

    fn try_from(proto: proto::apply_request::El) -> Result<Self, Self::Error> {
        use proto::apply_request::El as proto_el;

        let id = match proto {
            proto_el::Room(room) => {
                let mut pipeline = HashMap::new();
                for (id, room_element) in room.pipeline {
                    if let Some(elem) = room_element.el {
                        let member =
                            Member::try_from((member::Id(id.clone()), elem))?;
                        drop(pipeline.insert(id.into(), member));
                    } else {
                        return Err(TryFromProtobufError::EmptyElement(id));
                    }
                }

                let pipeline = pipeline;
                return Ok(Self {
                    id: room.id.into(),
                    pipeline,
                });
            }
            proto_el::Member(member) => member.id,
            proto_el::WebrtcPub(webrtc_pub) => webrtc_pub.id,
            proto_el::WebrtcPlay(webrtc_play) => webrtc_play.id,
        };

        Err(TryFromProtobufError::ExpectedOtherElement(
            String::from("Room"),
            id,
        ))
    }
}

impl From<Member> for proto::room::Element {
    fn from(member: Member) -> Self {
        Self {
            el: Some(proto::room::element::El::Member(member.into())),
        }
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

impl TryFrom<(member::Id, proto::room::element::El)> for Member {
    type Error = TryFromProtobufError;

    fn try_from(
        (id, proto): (member::Id, proto::room::element::El),
    ) -> Result<Self, Self::Error> {
        use proto::room::element::El as proto_el;
        match proto {
            proto_el::Member(member) => Self::try_from(member),
            proto_el::WebrtcPub(_) | proto_el::WebrtcPlay(_) => {
                Err(TryFromProtobufError::ExpectedOtherElement(
                    String::from("Member"),
                    id.to_string(),
                ))
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

        let mut pipeline = HashMap::new();
        for (id, member_element) in member.pipeline {
            if let Some(elem) = member_element.el {
                let endpoint =
                    Endpoint::try_from((endpoint::Id(id.clone()), elem))?;
                drop(pipeline.insert(id.into(), endpoint));
            } else {
                return Err(TryFromProtobufError::EmptyElement(id));
            }
        }

        let credentials = member
            .credentials
            .map_or_else(Credentials::default, Credentials::from);

        let on_leave = {
            let on_leave = member.on_leave;
            if on_leave.is_empty() {
                None
            } else {
                Some(CallbackUrl::try_from(on_leave)?.to_string())
            }
        };
        let on_join = {
            let on_join = member.on_join;
            if on_join.is_empty() {
                None
            } else {
                Some(CallbackUrl::try_from(on_join)?.to_string())
            }
        };

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
            id: member::Id(member.id),
            pipeline,
            credentials,
            on_join,
            on_leave,
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
            credentials: Some(member.credentials.into()),
        }
    }
}

impl From<proto::member::Credentials> for Credentials {
    fn from(from: proto::member::Credentials) -> Self {
        use proto::member::Credentials as C;
        match from {
            C::Hash(hash) => Self::Hash(hash),
            C::Plain(plain) => Self::Plain(plain),
        }
    }
}

impl From<Credentials> for proto::member::Credentials {
    fn from(creds: Credentials) -> Self {
        match creds {
            Credentials::Hash(hash) => Self::Hash(hash),
            Credentials::Plain(plain) => Self::Plain(plain),
        }
    }
}

impl TryFrom<(endpoint::Id, proto::member::element::El)> for Endpoint {
    type Error = TryFromProtobufError;

    fn try_from(
        (_, proto): (endpoint::Id, proto::member::element::El),
    ) -> Result<Self, Self::Error> {
        use proto::member::element::El;

        match proto {
            El::WebrtcPlay(elem) => {
                let play = WebRtcPlay::try_from(elem)?;
                Ok(Self::WebRtcPlay(play))
            }
            El::WebrtcPub(elem) => {
                let publish = WebRtcPublish::from(elem);
                Ok(Self::WebRtcPublish(publish))
            }
        }
    }
}

impl From<Endpoint> for proto::member::Element {
    fn from(endpoint: Endpoint) -> Self {
        let el = match endpoint {
            Endpoint::WebRtcPlay(play) => {
                proto::member::element::El::WebrtcPlay(play.into())
            }
            Endpoint::WebRtcPublish(publish) => {
                proto::member::element::El::WebrtcPub(publish.into())
            }
        };

        Self { el: Some(el) }
    }
}

impl TryFrom<proto::WebRtcPlayEndpoint> for WebRtcPlay {
    type Error = TryFromProtobufError;

    fn try_from(value: proto::WebRtcPlayEndpoint) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.into(),
            src: SrcUri::try_from(value.src)?,
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
        Self {
            id: value.id.into(),
            p2p: P2pMode::from(
                proto::web_rtc_publish_endpoint::P2p::from_i32(value.p2p)
                    .unwrap_or_default(),
            ),
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
        Self {
            id: publish.id.into(),
            p2p: proto::web_rtc_publish_endpoint::P2p::from(publish.p2p).into(),
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
        Self {
            publish_policy:
                proto::web_rtc_publish_endpoint::PublishPolicy::from_i32(
                    from.publish_policy,
                )
                .unwrap_or_default()
                .into(),
        }
    }
}

impl From<AudioSettings> for proto::web_rtc_publish_endpoint::AudioSettings {
    fn from(settings: AudioSettings) -> Self {
        Self {
            publish_policy:
                proto::web_rtc_publish_endpoint::PublishPolicy::from(
                    settings.publish_policy,
                )
                .into(),
        }
    }
}

impl From<proto::web_rtc_publish_endpoint::VideoSettings> for VideoSettings {
    fn from(from: proto::web_rtc_publish_endpoint::VideoSettings) -> Self {
        Self {
            publish_policy:
                proto::web_rtc_publish_endpoint::PublishPolicy::from_i32(
                    from.publish_policy,
                )
                .unwrap_or_default()
                .into(),
        }
    }
}

impl From<VideoSettings> for proto::web_rtc_publish_endpoint::VideoSettings {
    fn from(settings: VideoSettings) -> Self {
        Self {
            publish_policy:
                proto::web_rtc_publish_endpoint::PublishPolicy::from(
                    settings.publish_policy,
                )
                .into(),
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

impl From<ErrorResponse> for proto::Error {
    fn from(resp: ErrorResponse) -> Self {
        let text = if let Some(additional_text) = &resp.explanation {
            format!("{} {additional_text}", resp.error_code)
        } else {
            resp.error_code.to_string()
        };
        Self {
            doc: String::new(),
            text,
            element: resp.element_id.unwrap_or_default(),
            code: resp.error_code.into(),
        }
    }
}
