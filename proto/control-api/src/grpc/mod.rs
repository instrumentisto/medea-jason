//! [gRPC]-based [Control API] for [Medea].
//!
//! [gRPC]: https://grpc.io
//! [Medea]: https://github.com/instrumentisto/medea
//! [Control API]: https://tinyurl.com/yxsqplq7

#[allow(
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#[rustfmt::skip]
pub mod api;
#[allow(
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#[rustfmt::skip]
pub mod callback;

pub mod adapter {
    use std::{collections::HashMap, fmt, time::Duration};

    use async_trait::async_trait;
    use derive_more::{Display, Error, From, Into};
    use tonic::{Request, Response, Status};
    use url::Url;

    use crate::{
        endpoint,
        endpoint::{
            web_rtc_play::SrcUri,
            web_rtc_publish::{
                AudioSettings, P2pMode, PublishPolicy, VideoSettings,
            },
        },
        grpc::{
            api::{
                self as proto, control_api_server::ControlApi as GrpcControlApi,
            },
            callback::{self, callback_server::Callback as GrpcCallback},
        },
        member,
        member::Credentials,
        room, Callback, ControlApi, Element, Endpoint, ErrorCode,
        ErrorResponse, Fid, Member, Ping, Room, Sids, StatefulFid, ToEndpoint,
        ToMember, ToRoom, WebRtcPlay, WebRtcPublish,
    };

    #[derive(Clone, Debug, Display, Eq, Hash, Into, PartialEq)]
    #[display(fmt = "grpc://{}", _0)]
    pub struct GrpcCallbackUrl(pub String);

    impl TryFrom<String> for GrpcCallbackUrl {
        type Error = CallbackUrlParseError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            let url = Url::parse(&value)?;
            let url_scheme = url.scheme();
            let host = url.host().ok_or(CallbackUrlParseError::MissingHost)?;
            let host = url.port().map_or_else(
                || host.to_string(),
                |port| format!("{host}:{port}"),
            );

            match url_scheme {
                "grpc" => Ok(GrpcCallbackUrl(host)),
                _ => Err(CallbackUrlParseError::UnsupportedScheme),
            }
        }
    }

    pub type OnJoin = GrpcCallbackUrl;

    pub type OnLeave = GrpcCallbackUrl;

    #[async_trait]
    impl<T> GrpcControlApi for T
    where
        T: ControlApi<OnJoin, OnLeave> + Send + Sync + 'static,
    {
        async fn create(
            &self,
            req: Request<proto::CreateRequest>,
        ) -> Result<Response<proto::CreateResponse>, Status> {
            let res = async {
                let req = req.into_inner();
                let unparsed_parent_fid = req.parent_fid;
                let elem = if let Some(elem) = req.el {
                    elem
                } else {
                    return Err(ErrorResponse::new(
                        ErrorCode::NoElement,
                        &unparsed_parent_fid,
                    ));
                };

                if unparsed_parent_fid.is_empty() {
                    return Ok(self.create_room(Room::try_from(elem)?).await?);
                }

                let parent_fid = StatefulFid::try_from(unparsed_parent_fid)?;
                match parent_fid {
                    StatefulFid::Room(parent_fid) => match elem {
                        proto::create_request::El::Member(member) => {
                            let member_spec = Member::try_from(member)?;
                            Ok(self
                                .create_room_member(parent_fid, member_spec)
                                .await?)
                        }
                        proto::create_request::El::Room(_)
                        | proto::create_request::El::WebrtcPlay(_)
                        | proto::create_request::El::WebrtcPub(_) => {
                            Err(ErrorResponse::new(
                                ErrorCode::ElementIdMismatch,
                                &parent_fid,
                            ))
                        }
                    },
                    StatefulFid::Member(parent_fid) => {
                        let endpoint_spec = match elem {
                            proto::create_request::El::WebrtcPlay(play) => {
                                Endpoint::from(WebRtcPlay::try_from(&play)?)
                            }
                            proto::create_request::El::WebrtcPub(publish) => {
                                Endpoint::from(WebRtcPublish::from(&publish))
                            }
                            proto::create_request::El::Member(_)
                            | proto::create_request::El::Room(_) => {
                                return Err(ErrorResponse::new(
                                    ErrorCode::ElementIdMismatch,
                                    &parent_fid,
                                ));
                            }
                        };

                        Ok(self
                            .create_room_endpoint(parent_fid, endpoint_spec)
                            .await?)
                    }
                    StatefulFid::Endpoint(_) => Err(ErrorResponse::new(
                        ErrorCode::ElementIdIsTooLong,
                        &parent_fid,
                    )),
                }
            };

            Ok(Response::new(match res.await {
                Ok(sids) => proto::CreateResponse {
                    sid: proto_sids(sids),
                    error: None,
                },
                Err(e) => proto::CreateResponse {
                    sid: HashMap::new(),
                    error: Some(e.into()),
                },
            }))
        }

        async fn delete(
            &self,
            req: Request<proto::IdRequest>,
        ) -> Result<Response<proto::Response>, Status> {
            let ids = req
                .into_inner()
                .fid
                .into_iter()
                .map(|id| StatefulFid::try_from(id))
                .collect::<Result<Vec<_>, _>>();

            let res = match ids {
                Ok(ids) => self.delete_elements(ids).await,
                Err(e) => Err(e.into()),
            };

            Ok(Response::new(match res {
                Ok(_) => proto::Response { error: None },
                Err(e) => proto::Response {
                    error: Some(e.into()),
                },
            }))
        }

        async fn get(
            &self,
            req: Request<proto::IdRequest>,
        ) -> Result<Response<proto::GetResponse>, Status> {
            let ids = req
                .into_inner()
                .fid
                .into_iter()
                .map(|id| StatefulFid::try_from(id))
                .collect::<Result<Vec<_>, _>>();

            let res = match ids {
                Ok(ids) => self.get(ids).await,
                Err(e) => Err(e.into()),
            };

            Ok(Response::new(match res {
                Ok(elements) => proto::GetResponse {
                    elements: elements
                        .into_iter()
                        .map(|(id, el)| (id.to_string(), el.into()))
                        .collect(),
                    error: None,
                },
                Err(e) => proto::GetResponse {
                    elements: HashMap::new(),
                    error: Some(e.into()),
                },
            }))
        }

        async fn apply(
            &self,
            req: Request<proto::ApplyRequest>,
        ) -> Result<Response<proto::CreateResponse>, Status> {
            let res = async {
                let req = req.into_inner();
                let unparsed_fid = req.parent_fid;
                let elem = if let Some(elem) = req.el {
                    elem
                } else {
                    return Err(ErrorResponse::new(
                        ErrorCode::NoElement,
                        &unparsed_fid,
                    ));
                };

                let parent_fid = StatefulFid::try_from(unparsed_fid)?;
                match parent_fid {
                    StatefulFid::Room(fid) => match elem {
                        proto::apply_request::El::Room(_) => {
                            self.apply_room(Room::try_from(elem)?).await
                        }
                        proto::apply_request::El::Member(_)
                        | proto::apply_request::El::WebrtcPlay(_)
                        | proto::apply_request::El::WebrtcPub(_) => {
                            Err(ErrorResponse::new(
                                ErrorCode::ElementIdMismatch,
                                &fid,
                            ))
                        }
                    },
                    StatefulFid::Member(fid) => match elem {
                        proto::apply_request::El::Member(member) => self
                            .apply_room_member(
                                fid.into(),
                                Member::try_from(member)?,
                            )
                            .await
                            .map(|_| Sids::new()),
                        proto::apply_request::El::Room(_)
                        | proto::apply_request::El::WebrtcPlay(_)
                        | proto::apply_request::El::WebrtcPub(_) => {
                            Err(ErrorResponse::new(
                                ErrorCode::ElementIdMismatch,
                                &fid,
                            ))
                        }
                    },
                    StatefulFid::Endpoint(_) => {
                        Err(ErrorResponse::with_explanation(
                            ErrorCode::UnimplementedCall,
                            String::from(
                                "Apply method for Endpoints is not \
                                 currently supported.",
                            ),
                            None,
                        ))
                    }
                }
            };

            Ok(Response::new(match res.await {
                Ok(sid) => proto::CreateResponse {
                    sid: proto_sids(sid),
                    error: None,
                },
                Err(e) => proto::CreateResponse {
                    sid: HashMap::new(),
                    error: Some(e.into()),
                },
            }))
        }

        async fn healthz(
            &self,
            request: Request<proto::Ping>,
        ) -> Result<Response<proto::Pong>, Status> {
            self.healthz(Ping(request.into_inner().nonce))
                .await
                .map(|pong| Response::new(proto::Pong { nonce: pong.0 }))
                .map_err(|e| {
                    let mut message = e
                        .element_id
                        .map_or_else(String::new, |id| format!("{id}: "));
                    message.push_str(&e.error_code.to_string());
                    if let Some(explanation) = e.explanation {
                        message.push_str(&format!(": {explanation}"))
                    }

                    Status::unknown(message)
                })
        }
    }

    /// Converts [`Sids`] to a [`HashMap`] of [`String`]s for gRPC Control API
    /// protocol.
    fn proto_sids(sids: Sids) -> HashMap<String, String> {
        sids.into_iter()
            .map(|(id, sid)| (id.to_string(), sid.to_string()))
            .collect()
    }

    impl TryFrom<proto::create_request::El> for Room<OnJoin, OnLeave> {
        type Error = TryFromProtobufError;

        fn try_from(
            proto: proto::create_request::El,
        ) -> Result<Self, Self::Error> {
            use proto::create_request::El as proto_el;

            let id = match proto {
                proto_el::Room(room) => {
                    let mut pipeline = HashMap::new();
                    for (id, room_element) in room.pipeline {
                        if let Some(elem) = room_element.el {
                            let member = Member::try_from((
                                member::Id(id.clone()),
                                elem,
                            ))?;
                            drop(pipeline.insert(id.into(), member.into()));
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

    impl TryFrom<proto::apply_request::El> for Room<OnJoin, OnLeave> {
        type Error = TryFromProtobufError;

        fn try_from(
            proto: proto::apply_request::El,
        ) -> Result<Self, Self::Error> {
            use proto::apply_request::El as proto_el;

            let id = match proto {
                proto_el::Room(room) => {
                    let mut pipeline = HashMap::new();
                    for (id, room_element) in room.pipeline {
                        if let Some(elem) = room_element.el {
                            let member = Member::try_from((
                                member::Id(id.clone()),
                                elem,
                            ))?;
                            drop(pipeline.insert(id.into(), member.into()));
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

    impl TryFrom<(member::Id, proto::room::element::El)>
        for Member<OnJoin, OnLeave>
    {
        type Error = TryFromProtobufError;

        fn try_from(
            (id, proto): (member::Id, proto::room::element::El),
        ) -> Result<Self, Self::Error> {
            use proto::room::element::El as proto_el;
            match proto {
                proto_el::Member(member) => Self::try_from(member),
                _ => Err(TryFromProtobufError::ExpectedOtherElement(
                    String::from("Member"),
                    id.to_string(),
                )),
            }
        }
    }

    impl TryFrom<proto::Member> for Member<OnJoin, OnLeave> {
        type Error = TryFromProtobufError;

        fn try_from(member: proto::Member) -> Result<Self, Self::Error> {
            fn parse_duration<T: TryInto<Duration>>(
                duration: Option<T>,
                member_id: &str,
                field: &'static str,
            ) -> Result<Option<Duration>, TryFromProtobufError> {
                #[allow(clippy::map_err_ignore)]
                duration.map(TryInto::try_into).transpose().map_err(|_| {
                    TryFromProtobufError::NegativeDuration(
                        member_id.into(),
                        field,
                    )
                })
            }

            let mut pipeline = HashMap::new();
            for (id, member_element) in member.pipeline {
                if let Some(elem) = member_element.el {
                    let endpoint =
                        Endpoint::try_from((endpoint::Id(id.clone()), elem))?;
                    drop(pipeline.insert(id.into(), endpoint.into()));
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
                    Some(GrpcCallbackUrl::try_from(on_leave)?)
                }
            };
            let on_join = {
                let on_join = member.on_join;
                if on_join.is_empty() {
                    None
                } else {
                    Some(GrpcCallbackUrl::try_from(on_join)?)
                }
            };

            let idle_timeout = parse_duration(
                member.idle_timeout,
                &member.id,
                "idle_timeout",
            )?;
            let reconnect_timeout = parse_duration(
                member.reconnect_timeout,
                &member.id,
                "reconnect_timeout",
            )?;
            let ping_interval = parse_duration(
                member.ping_interval,
                &member.id,
                "ping_interval",
            )?;

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

    impl TryFrom<(endpoint::Id, proto::member::element::El)> for Endpoint {
        type Error = TryFromProtobufError;

        fn try_from(
            (_, proto): (endpoint::Id, proto::member::element::El),
        ) -> Result<Self, Self::Error> {
            use proto::member::element::El;

            match proto {
                El::WebrtcPlay(elem) => {
                    let play = WebRtcPlay::try_from(&elem)?;
                    Ok(Self::WebRtcPlay(play))
                }
                El::WebrtcPub(elem) => {
                    let publish = WebRtcPublish::from(&elem);
                    Ok(Self::WebRtcPublish(publish))
                }
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

    impl TryFrom<String> for SrcUri {
        type Error = SrcParseError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            let local_uri = StatefulLocalUri::try_from(value)
                .map_err(SrcParseError::LocalUriParseError)?;

            match local_uri {
                StatefulLocalUri::Room(uri) => {
                    Err(SrcParseError::NotSrcUri(uri.to_string()))
                }
                StatefulLocalUri::Member(uri) => {
                    Err(SrcParseError::NotSrcUri(uri.to_string()))
                }
                StatefulLocalUri::Endpoint(uri) => Ok(uri.into()),
            }
        }
    }

    impl TryFrom<&proto::WebRtcPlayEndpoint> for WebRtcPlay {
        type Error = TryFromProtobufError;

        fn try_from(
            value: &proto::WebRtcPlayEndpoint,
        ) -> Result<Self, Self::Error> {
            Ok(Self {
                id: value.id.clone().into(),
                src: SrcUri::try_from(value.src.clone())?,
                force_relay: value.force_relay,
            })
        }
    }

    impl From<&proto::WebRtcPublishEndpoint> for WebRtcPublish {
        fn from(value: &proto::WebRtcPublishEndpoint) -> Self {
            Self {
                id: value.id.clone().into(),
                p2p: P2pMode::from(
                    proto::web_rtc_publish_endpoint::P2p::from_i32(value.p2p)
                        .unwrap_or_default(),
                ),
                audio_settings: value
                    .audio_settings
                    .as_ref()
                    .map(AudioSettings::from)
                    .unwrap_or_default(),
                video_settings: value
                    .video_settings
                    .as_ref()
                    .map(VideoSettings::from)
                    .unwrap_or_default(),
                force_relay: value.force_relay,
            }
        }
    }

    #[rustfmt::skip]
    impl From<&proto::web_rtc_publish_endpoint::AudioSettings>
        for AudioSettings
    {
        fn from(from: &proto::web_rtc_publish_endpoint::AudioSettings) -> Self {
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

    #[rustfmt::skip]
    impl From<&proto::web_rtc_publish_endpoint::VideoSettings>
        for VideoSettings
    {
        fn from(from: &proto::web_rtc_publish_endpoint::VideoSettings) -> Self {
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

    #[rustfmt::skip]
    impl From<proto::web_rtc_publish_endpoint::PublishPolicy>
        for PublishPolicy
    {
        fn from(from: proto::web_rtc_publish_endpoint::PublishPolicy) -> Self {
            use proto::web_rtc_publish_endpoint::PublishPolicy as Proto;
            match from {
                Proto::Optional => Self::Optional,
                Proto::Required => Self::Required,
                Proto::Disabled => Self::Disabled,
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

    impl TryFrom<String> for StatefulFid {
        type Error = ParseFidError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.is_empty() {
                return Err(ParseFidError::Empty);
            }

            let mut splitted = value.split('/');
            let room_id = if let Some(room_id) = splitted.next() {
                if room_id.is_empty() {
                    return Err(ParseFidError::MissingPath(value));
                }
                room_id
            } else {
                return Err(ParseFidError::Empty);
            };

            let member_id = if let Some(member_id) = splitted.next() {
                if member_id.is_empty() {
                    return Err(ParseFidError::MissingPath(value));
                }
                member_id
            } else {
                return Ok(Fid::<ToRoom>::new(room_id.to_owned().into()).into());
            };

            let endpoint_id = if let Some(endpoint_id) = splitted.next() {
                if endpoint_id.is_empty() {
                    return Err(ParseFidError::MissingPath(value));
                }
                endpoint_id
            } else {
                return Ok(Fid::<ToMember>::new(
                    room_id.to_owned().into(),
                    member_id.to_owned().into(),
                )
                .into());
            };

            if splitted.next().is_some() {
                Err(ParseFidError::TooManyPaths(value))
            } else {
                Ok(Fid::<ToEndpoint>::new(
                    room_id.to_owned().into(),
                    member_id.to_owned().into(),
                    endpoint_id.to_owned().into(),
                )
                .into())
            }
        }
    }

    impl From<Element<OnJoin, OnLeave>> for proto::Element {
        fn from(el: Element<OnJoin, OnLeave>) -> Self {
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

    impl From<Member<OnJoin, OnLeave>> for proto::Member {
        fn from(member: Member<OnJoin, OnLeave>) -> Self {
            proto::Member {
                id: member.id.into(),
                on_join: member
                    .on_join
                    .map_or_else(String::default, String::from),
                on_leave: member
                    .on_leave
                    .map_or_else(String::default, String::from),
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

    impl From<Member<OnJoin, OnLeave>> for proto::room::Element {
        fn from(member: Member<OnJoin, OnLeave>) -> Self {
            Self {
                el: Some(proto::room::element::El::Member(member.into())),
            }
        }
    }

    impl From<Room<OnJoin, OnLeave>> for proto::Room {
        fn from(room: Room<OnJoin, OnLeave>) -> Self {
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

            proto::member::Element { el: Some(el) }
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

    impl From<WebRtcPublish> for proto::WebRtcPublishEndpoint {
        fn from(publish: WebRtcPublish) -> Self {
            Self {
                id: publish.id.into(),
                #[allow(clippy::as_conversions)]
                p2p: publish.p2p as i32,
                on_start: String::new(),
                on_stop: String::new(),
                force_relay: publish.force_relay,
                audio_settings: Some(publish.audio_settings.into()),
                video_settings: Some(publish.video_settings.into()),
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

    #[rustfmt::skip]
    impl From<AudioSettings>
        for proto::web_rtc_publish_endpoint::AudioSettings
    {
        fn from(settings: AudioSettings) -> Self {
            #[allow(clippy::as_conversions)]
            Self { publish_policy: settings.publish_policy as i32 }
        }
    }

    #[rustfmt::skip]
    impl From<VideoSettings>
        for proto::web_rtc_publish_endpoint::VideoSettings
    {
        fn from(settings: VideoSettings) -> Self {
            #[allow(clippy::as_conversions)]
            Self { publish_policy: settings.publish_policy as i32 }
        }
    }

    /// Errors which can happen while parsing [`Fid`].
    #[derive(Debug, Display, Error)]
    pub enum ParseFidError {
        #[display(fmt = "Fid is empty")]
        Empty,

        #[display(fmt = "Too many paths [fid = {}]", _0)]
        TooManyPaths(#[error(not(source))] String),

        #[display(fmt = "Missing paths [fid = {}]", _0)]
        MissingPath(#[error(not(source))] String),
    }

    /// URI in format `local://room_id/member_id/endpoint_id`.
    ///
    /// This kind of URI used for pointing to some element in spec ([`Room`],
    /// [`Member`], [`WebRtcPlayEndpoint`], [`WebRtcPublishEndpoint`], etc)
    /// based on state.
    ///
    /// [`LocalUri`] can be in three states: [`ToRoom`], [`ToMember`],
    /// [`ToRoom`]. This is used for compile time guarantees that some
    /// [`LocalUri`] have all mandatory fields.
    ///
    /// You also can take value from [`LocalUri`] without clone, but you have to
    /// do it consistently. For example, if you wish to get [`RoomId`],
    /// [`MemberId`] and [`Endpoint`] ID from [`LocalUri`] in [`ToEndpoint`]
    /// state you should make this steps:
    ///
    /// ```
    /// # use medea::api::control::refs::{LocalUri, ToEndpoint};
    /// # use medea::api::control::{RoomId, MemberId, EndpointId};
    /// #
    /// let orig_room_id = RoomId::from("room");
    /// let orig_member_id = MemberId::from("member");
    /// let orig_endpoint_id = EndpointId::from("endpoint");
    ///
    /// // Create new LocalUri for endpoint.
    /// let local_uri = LocalUri::<ToEndpoint>::new(
    ///     orig_room_id.clone(),
    ///     orig_member_id.clone(),
    ///     orig_endpoint_id.clone()
    /// );
    /// let local_uri_clone = local_uri.clone();
    ///
    /// // We can get reference to room_id from this LocalUri
    /// // without taking ownership:
    /// assert_eq!(local_uri.room_id(), &orig_room_id);
    ///
    /// // If you want to take all IDs ownership, you should do this steps:
    /// let (endpoint_id, member_uri) = local_uri.take_endpoint_id();
    /// assert_eq!(endpoint_id, orig_endpoint_id);
    ///
    /// let (member_id, room_uri) = member_uri.take_member_id();
    /// assert_eq!(member_id, orig_member_id);
    ///
    /// let room_id = room_uri.take_room_id();
    /// assert_eq!(room_id, orig_room_id);
    ///
    /// // Or simply
    /// let (room_id, member_id, endpoint_id) = local_uri_clone.take_all();
    /// ```
    ///
    /// This is necessary so that it is not possible to get the address in the
    /// wrong state (`local://room_id//endpoint_id` for example).
    ///
    /// [`Member`]: crate::signalling::elements::Member
    /// [`Room`]: crate::signalling::room::Room
    /// [`WebRtcPlayEndpoint`]:
    /// crate::signalling::elements::endpoints::webrtc::WebRtcPlayEndpoint
    /// [`WebRtcPublishEndpoint`]:
    /// crate::signalling::elements::endpoints::webrtc::WebRtcPublishEndpoint
    /// [`Endpoint`]: crate::signalling::elements::endpoints::Endpoint
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct LocalUri<T> {
        /// Element indication this [`LocalUri`] references to.
        state: T,
    }

    impl From<SrcUri> for LocalUri<ToEndpoint> {
        fn from(uri: SrcUri) -> Self {
            Self::new(uri.room_id, uri.member_id, uri.endpoint_id.into())
        }
    }

    impl From<LocalUri<ToEndpoint>> for SrcUri {
        fn from(uri: LocalUri<ToEndpoint>) -> Self {
            let (room_id, member_id, endpoint_id) = uri.take_all();

            Self {
                room_id,
                member_id,
                endpoint_id: endpoint_id.into(),
            }
        }
    }

    impl LocalUri<ToEndpoint> {
        /// Creates new reference in [`ToEndpoint`] state.
        ///
        /// [`ToEndpoint`]: crate::api::control::refs::ToEndpoint
        #[must_use]
        pub const fn new(
            room_id: room::Id,
            member_id: member::Id,
            endpoint_id: endpoint::Id,
        ) -> Self {
            Self {
                state: ToEndpoint(room_id, member_id, endpoint_id),
            }
        }

        /// Returns borrowed [`RoomId`].
        ///
        /// [`RoomId`]: medea_client_api_proto::RoomId
        #[must_use]
        pub const fn room_id(&self) -> &room::Id {
            &self.state.0
        }

        /// Returns borrowed [`MemberId`].
        ///
        /// [`MemberId`]: medea_client_api_proto::MemberId
        #[must_use]
        pub const fn member_id(&self) -> &member::Id {
            &self.state.1
        }

        /// Returns borrowed [`EndpointId`].
        ///
        /// [`EndpointId`]: crate::api::control::EndpointId
        #[must_use]
        pub const fn endpoint_id(&self) -> &endpoint::Id {
            &self.state.2
        }

        /// Returns [`Endpoint`] id and reference in [`ToMember`] state.
        ///
        /// [`Endpoint`]: crate::signalling::elements::endpoints::Endpoint
        /// [`ToMember`]: crate::api::control::refs::ToMember
        #[must_use]
        pub fn take_endpoint_id(self) -> (endpoint::Id, LocalUri<ToMember>) {
            (
                self.state.2,
                LocalUri::<ToMember>::new(self.state.0, self.state.1),
            )
        }

        /// Returns [`EndpointId`], [`RoomId`] and [`MemberId`].
        ///
        /// [`EndpointId`]: crate::api::control::EndpointId
        /// [`RoomId`]: medea_client_api_proto::RoomId
        /// [`MemberId`]: medea_client_api_proto::MemberId
        #[must_use]
        pub fn take_all(self) -> (room::Id, member::Id, endpoint::Id) {
            let (endpoint_id, member_url) = self.take_endpoint_id();
            let (member_id, room_url) = member_url.take_member_id();

            (room_url.take_room_id(), member_id, endpoint_id)
        }
    }

    impl LocalUri<ToRoom> {
        #[doc = "Create new reference in [`ToRoom`] state."]
        #[must_use]
        pub const fn new(room_id: room::Id) -> Self {
            Self {
                state: ToRoom(room_id),
            }
        }

        /// Returns borrowed [`RoomId`].
        ///
        /// [`RoomId`]: medea_client_api_proto::RoomId
        #[must_use]
        pub const fn room_id(&self) -> &room::Id {
            &self.state.0
        }

        /// Returns [`RoomId`].
        ///
        /// [`RoomId`]: medea_client_api_proto::RoomId
        #[must_use]
        pub fn take_room_id(self) -> room::Id {
            self.state.0
        }

        /// Pushes [`MemberId`] to the end of URI and returns
        /// reference in [`ToMember`] state.
        ///
        /// [`MemberId`]: medea_client_api_proto::MemberId
        /// [`ToMember`]: crate::api::control::refs::ToMember
        #[must_use]
        pub fn push_member_id(
            self,
            member_id: member::Id,
        ) -> LocalUri<ToMember> {
            LocalUri::<ToMember>::new(self.state.0, member_id)
        }
    }

    impl LocalUri<ToMember> {
        /// Create new reference in [`ToMember`] state.
        ///
        /// [`ToMember`]: crate::api::control::refs::ToMember
        #[must_use]
        pub const fn new(room_id: room::Id, member_id: member::Id) -> Self {
            Self {
                state: ToMember(room_id, member_id),
            }
        }

        /// Returns borrowed [`RoomId`].
        ///
        /// [`RoomId`]: medea_client_api_proto::RoomId
        #[must_use]
        pub const fn room_id(&self) -> &room::Id {
            &self.state.0
        }

        /// Returns borrowed [`MemberId`].
        ///
        /// [`MemberId`]: medea_client_api_proto::MemberId
        #[must_use]
        pub const fn member_id(&self) -> &member::Id {
            &self.state.1
        }

        /// Return [`MemberId`] and reference in state [`ToRoom`].
        ///
        /// [`MemberId`]: medea_client_api_proto::MemberId
        /// [`ToRoom`]: crate::api::control::refs::ToRoom
        #[allow(clippy::missing_const_for_fn)]
        #[must_use]
        pub fn take_member_id(self) -> (member::Id, LocalUri<ToRoom>) {
            (self.state.1, LocalUri::<ToRoom>::new(self.state.0))
        }

        /// Push endpoint ID to the end of URI and returns
        /// reference in [`ToEndpoint`] state.
        ///
        /// [`ToEndpoint`]: crate::api::control::refs::ToEndpoint
        #[must_use]
        pub fn push_endpoint_id(
            self,
            endpoint_id: endpoint::Id,
        ) -> LocalUri<ToEndpoint> {
            let (member_id, room_uri) = self.take_member_id();
            let room_id = room_uri.take_room_id();
            LocalUri::<ToEndpoint>::new(room_id, member_id, endpoint_id)
        }
    }

    impl fmt::Display for LocalUri<ToRoom> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "local://{}", self.state.0)
        }
    }

    impl fmt::Display for LocalUri<ToMember> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "local://{}/{}", self.state.0, self.state.1)
        }
    }

    impl fmt::Display for LocalUri<ToEndpoint> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "local://{}/{}/{}",
                self.state.0, self.state.1, self.state.2
            )
        }
    }

    /// Enum for storing [`LocalUri`]s in all states.
    #[derive(Debug, Hash, PartialEq, Eq, Clone, Display, From)]
    pub enum StatefulLocalUri {
        /// Stores [`LocalUri`] in [`ToRoom`] state.
        Room(LocalUri<ToRoom>),

        /// Stores [`LocalUri`] in [`ToMember`] state.
        Member(LocalUri<ToMember>),

        /// Stores [`LocalUri`] in [`ToEndpoint`] state.
        Endpoint(LocalUri<ToEndpoint>),
    }

    impl TryFrom<String> for StatefulLocalUri {
        type Error = LocalUriParseError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.is_empty() {
                return Err(LocalUriParseError::Empty);
            }

            let url = match Url::parse(&value) {
                Ok(url) => url,
                Err(err) => {
                    return Err(LocalUriParseError::UrlParseErr(value, err))
                }
            };

            if url.scheme() != "local" {
                return Err(LocalUriParseError::NotLocal(value));
            }

            let room_uri = match url.host() {
                Some(host) => {
                    let host = host.to_string();
                    if host.is_empty() {
                        return Err(LocalUriParseError::MissingPaths(value));
                    }
                    LocalUri::<ToRoom> { state: host.into() }
                }
                None => return Err(LocalUriParseError::MissingPaths(value)),
            };

            let mut path = match url.path_segments() {
                Some(path) => path,
                None => return Ok(room_uri.into()),
            };

            let member_id = path
                .next()
                .filter(|id| !id.is_empty())
                .map(|id| member::Id(id.into()));

            let endpoint_id = path
                .next()
                .filter(|id| !id.is_empty())
                .map(ToString::to_string);

            if path.next().is_some() {
                return Err(LocalUriParseError::TooManyPaths(value));
            }

            if let Some(member_id) = member_id {
                let member_uri = room_uri.push_member_id(member_id);
                if let Some(endpoint_id) = endpoint_id {
                    Ok(member_uri.push_endpoint_id(endpoint_id.into()).into())
                } else {
                    Ok(member_uri.into())
                }
            } else if endpoint_id.is_some() {
                Err(LocalUriParseError::MissingPaths(value))
            } else {
                Ok(room_uri.into())
            }
        }
    }

    /// Errors which may occur while deserializing protobuf spec.
    #[derive(Debug, Display, Error)]
    pub enum TryFromProtobufError {
        /// Error while parsing [`SrcUri`] of [`WebRtcPlayEndpoint`].
        ///
        /// [`WebRtcPlayEndpoint`]:
        /// crate::api::control::endpoints::WebRtcPlayEndpoint
        /// [`SrcUri`]: crate::api::control::refs::SrcUri
        #[display(fmt = "Src uri parse error: {:?}", _0)]
        SrcUriError(SrcParseError),

        /// `Room` element doesn't have `Member` element. Currently this is
        /// unimplemented.
        #[display(
            fmt = "Room(id: {}) doesn't have Member element (currently, this \
                   is unimplemented)",
            _0
        )]
        NotMemberElementInRoomElement(#[error(not(source))] String),

        /// `Room` element doesn't have `Member` element. Currently this is
        /// unimplemented.
        #[display(fmt = "Expected element of type [{}]. Id [{}]", _0, _1)]
        ExpectedOtherElement(String, String),

        /// Element is `None`, but expected `Some`.
        #[display(fmt = "Element is None, expected Some. Id [{}]", _0)]
        EmptyElement(#[error(not(source))] String),

        /// Provided `Endpoint` is unimplemented.
        #[display(fmt = "Endpoint is unimplemented. Id [{}]", _0)]
        UnimplementedEndpoint(#[error(not(source))] String),

        /// Error while [`CallbackUrl`] parsing.
        ///
        /// [`CallbackUrl`]: crate::api::control::callback::CallbackUrl
        #[display(fmt = "Error while parsing callback URL. {:?}", _0)]
        CallbackUrlParseErr(CallbackUrlParseError),

        /// Some element from a spec contains negative [`Duration`], but it's
        /// not
        /// supported.
        ///
        /// [`Duration`]: std::time::Duration
        #[display(
            fmt = "Element(id: {}) contains negative duration field `{}`",
            _0,
            _1
        )]
        NegativeDuration(String, &'static str),
    }

    impl From<TryFromProtobufError> for ErrorResponse {
        fn from(err: TryFromProtobufError) -> Self {
            use TryFromProtobufError as E;

            match err {
                E::SrcUriError(e) => e.into(),
                E::CallbackUrlParseErr(e) => e.into(),
                E::NotMemberElementInRoomElement(id) => Self::with_explanation(
                    ErrorCode::UnimplementedCall,
                    String::from(
                        "Not Member elements in Room element currently is \
                     unimplemented.",
                    ),
                    Some(id),
                ),
                E::UnimplementedEndpoint(id) => Self::with_explanation(
                    ErrorCode::UnimplementedCall,
                    String::from("Endpoint is not implemented."),
                    Some(id),
                ),
                E::ExpectedOtherElement(element, id) => Self::with_explanation(
                    ErrorCode::ElementIdMismatch,
                    format!(
                        "Provided fid can not point to element of type \
                         [{element}]",
                    ),
                    Some(id),
                ),
                E::EmptyElement(id) => Self::with_explanation(
                    ErrorCode::NoElement,
                    String::from("No element was provided"),
                    Some(id),
                ),
                E::NegativeDuration(id, f) => Self::with_explanation(
                    ErrorCode::NegativeDuration,
                    format!(
                        "Element(id: {id}) contains negative duration field \
                         `{f}`",
                    ),
                    Some(id),
                ),
            }
        }
    }

    impl From<SrcParseError> for ErrorResponse {
        fn from(err: SrcParseError) -> Self {
            use SrcParseError::{LocalUriParseError, NotSrcUri};

            match err {
                NotSrcUri(text) => Self::new(ErrorCode::NotSourceUri, &text),
                LocalUriParseError(err) => err.into(),
            }
        }
    }

    impl From<LocalUriParseError> for ErrorResponse {
        fn from(err: LocalUriParseError) -> Self {
            use LocalUriParseError as E;

            match err {
                E::NotLocal(text) => {
                    Self::new(ErrorCode::ElementIdIsNotLocal, &text)
                }
                E::TooManyPaths(text) => {
                    Self::new(ErrorCode::ElementIdIsTooLong, &text)
                }
                E::Empty => Self::without_id(ErrorCode::EmptyElementId),
                E::MissingPaths(text) => {
                    Self::new(ErrorCode::MissingFieldsInSrcUri, &text)
                }
                E::UrlParseErr(id, _) => {
                    Self::new(ErrorCode::InvalidSrcUri, &id)
                }
            }
        }
    }

    impl From<ParseFidError> for ErrorResponse {
        fn from(err: ParseFidError) -> Self {
            use ParseFidError::{Empty, MissingPath, TooManyPaths};

            match err {
                TooManyPaths(text) => {
                    Self::new(ErrorCode::ElementIdIsTooLong, &text)
                }
                Empty => Self::without_id(ErrorCode::EmptyElementId),
                MissingPath(text) => Self::new(ErrorCode::MissingPath, &text),
            }
        }
    }

    impl From<CallbackUrlParseError> for ErrorResponse {
        fn from(err: CallbackUrlParseError) -> Self {
            use CallbackUrlParseError::{
                MissingHost, UnsupportedScheme, UrlParseErr,
            };

            match err {
                MissingHost => {
                    Self::without_id(ErrorCode::MissingHostInCallbackUrl)
                }
                UnsupportedScheme => {
                    Self::without_id(ErrorCode::UnsupportedCallbackUrlProtocol)
                }
                UrlParseErr(_) => {
                    Self::without_id(ErrorCode::InvalidCallbackUrl)
                }
            }
        }
    }

    impl From<SrcParseError> for TryFromProtobufError {
        fn from(from: SrcParseError) -> Self {
            Self::SrcUriError(from)
        }
    }

    impl From<CallbackUrlParseError> for TryFromProtobufError {
        fn from(from: CallbackUrlParseError) -> Self {
            Self::CallbackUrlParseErr(from)
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

    /// Errors which can happen while parsing [`SrcUri`] from [Control API]
    /// specs.
    ///
    /// [Control API]: https://tinyurl.com/yxsqplq7
    #[derive(Debug, Display, Error)]
    pub enum SrcParseError {
        /// Provided not source URI.
        #[display(fmt = "Provided not src URI: {}", _0)]
        NotSrcUri(#[error(not(source))] String),

        /// Error from [`LocalUri`] parser. This is general errors for
        /// [`SrcUri`] parsing because [`SrcUri`] parses with
        /// [`LocalUri`] parser.
        #[display(fmt = "Local URI parse error: {}", _0)]
        LocalUriParseError(LocalUriParseError),
    }

    /// Error which can happen while [`LocalUri`] parsing.
    #[derive(Debug, Display, Error)]
    pub enum LocalUriParseError {
        /// Protocol of provided URI is not "local://".
        #[display(fmt = "Provided URIs protocol is not `local://`")]
        NotLocal(#[error(not(source))] String),

        /// Too many paths in provided URI.
        ///
        /// `local://room_id/member_id/endpoint_id/redundant_path` for example.
        #[display(fmt = "Too many paths in provided URI ({})", _0)]
        TooManyPaths(#[error(not(source))] String),

        /// Some paths is missing in URI.
        ///
        /// `local://room_id//qwerty` for example.
        #[display(fmt = "Missing fields: {}", _0)]
        MissingPaths(#[error(not(source))] String),

        /// Error while parsing URI by [`url::Url`].
        #[display(fmt = "Error while parsing URL: {}", _0)]
        UrlParseErr(String, #[error(source)] url::ParseError),

        /// Provided empty URI.
        #[display(fmt = "Provided empty local URI")]
        Empty,
    }

    /// Error of [`CallbackUrl`] parsing.
    #[derive(Clone, Copy, Debug, Display, Error, From)]
    pub enum CallbackUrlParseError {
        /// Failed to parse URL.
        UrlParseErr(url::ParseError),

        /// URL is missing host.
        #[display(fmt = "Missing host")]
        MissingHost,

        /// URL contains unsupported scheme.
        #[display(fmt = "Unsupported URL scheme")]
        UnsupportedScheme,
    }
}
