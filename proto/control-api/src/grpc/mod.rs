//! [gRPC]-based [Control API] for [Medea].
//!
//! [gRPC]: https://grpc.io
//! [Medea]: https://github.com/instrumentisto/medea
//! [Control API]: https://tinyurl.com/yxsqplq7

mod conversions;

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

use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLock;
use tonic::{transport::Channel, Response, Status};

use crate::{
    callback::Request,
    endpoint::{WebRtcPlay, WebRtcPublish},
    grpc::{
        api::{
            self as proto, control_api_server::ControlApi as GrpcControlApi,
        },
        callback::callback_client,
    },
    ControlApi, Endpoint, ErrorCode, ErrorResponse, Member, Ping, Room, Sids,
    StatefulFid,
};

pub use self::conversions::{
    CallbackUrl, CallbackUrlParseError, TryFromProtobufError,
};

/// gRPC [`CallbackClient`] for sending [`Request`]s.
///
/// [`CallbackClient`]: crate::CallbackClient
#[derive(Debug, Default)]
pub struct CallbackClient {
    /// [`tonic`] gRPC clients of [Control API Callback service].
    ///
    /// [Control API Callback service]: https://tinyurl.com/y5fajesq
    // TODO: Connections are inserted, but never removed. This shouldn't be the
    //       issue for most cases, as only 1 callback server is used. But
    //       consider switching to `HashMap`-like data-structure with expiring
    //       entries like https://docs.rs/moka/.
    clients:
        RwLock<HashMap<CallbackUrl, callback_client::CallbackClient<Channel>>>,
}

impl CallbackClient {
    /// Returns gRPC [`CallbackClient`]s.
    ///
    /// For every [`CallbackUrl`] creates a unique connection and serves all
    /// [`Event`]s through it.
    ///
    /// [`CallbackClient`]: crate::CallbackClient
    /// [`Event`]: crate::callback::Event
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait(?Send)]
impl crate::CallbackClient for CallbackClient {
    async fn send(&self, request: Request) -> Result<(), ErrorResponse> {
        let url = CallbackUrl::try_from(request.url.clone())?;
        let read_guard = self.clients.read().await;

        let mut client = if let Some(client) = read_guard.get(&url) {
            client.clone()
        } else {
            drop(read_guard);
            let mut write_guard = self.clients.write().await;
            if let Some(client) = write_guard.get(&url) {
                client.clone()
            } else {
                let client =
                    callback_client::CallbackClient::connect(url.http_addr())
                        .await
                        .map_err(|e| ErrorResponse::unexpected(&e))?;
                drop(write_guard.insert(url, client.clone()));
                client
            }
        };

        drop(
            client
                .on_event(tonic::Request::new(request.into()))
                .await
                .map_err(|e| ErrorResponse::unexpected(&e))?,
        );
        Ok(())
    }
}

#[async_trait]
impl<T> GrpcControlApi for T
where
    T: ControlApi + Send + Sync + 'static,
{
    async fn create(
        &self,
        req: tonic::Request<proto::CreateRequest>,
    ) -> Result<Response<proto::CreateResponse>, Status> {
        let fut = async {
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
                return self.create_room(Room::try_from(elem)?).await;
            }

            let parent_fid = StatefulFid::try_from(unparsed_parent_fid)?;
            match parent_fid {
                StatefulFid::Room { id } => match elem {
                    proto::create_request::El::Member(member) => {
                        let member_spec = Member::try_from(member)?;
                        Ok(self.create_room_member(id, member_spec).await?)
                    }
                    proto::create_request::El::Room(_)
                    | proto::create_request::El::WebrtcPlay(_)
                    | proto::create_request::El::WebrtcPub(_) => Err(
                        ErrorResponse::new(ErrorCode::ElementIdMismatch, &id),
                    ),
                },
                StatefulFid::Member { id, room_id } => {
                    let endpoint_spec = match elem {
                        proto::create_request::El::WebrtcPlay(play) => {
                            Endpoint::from(WebRtcPlay::try_from(play)?)
                        }
                        proto::create_request::El::WebrtcPub(publish) => {
                            Endpoint::from(WebRtcPublish::from(publish))
                        }
                        proto::create_request::El::Member(_)
                        | proto::create_request::El::Room(_) => {
                            return Err(ErrorResponse::new(
                                ErrorCode::ElementIdMismatch,
                                &StatefulFid::Member { id, room_id },
                            ));
                        }
                    };

                    Ok(self
                        .create_room_endpoint(room_id, id, endpoint_spec)
                        .await?)
                }
                StatefulFid::Endpoint { .. } => Err(ErrorResponse::new(
                    ErrorCode::ElementIdIsTooLong,
                    &parent_fid,
                )),
            }
        };

        Ok(Response::new(match fut.await {
            Ok(sids) => proto::CreateResponse {
                sid: sids
                    .into_iter()
                    .map(|(id, sid)| (id.to_string(), sid.to_string()))
                    .collect(),
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
        req: tonic::Request<proto::IdRequest>,
    ) -> Result<Response<proto::Response>, Status> {
        let ids = req
            .into_inner()
            .fid
            .into_iter()
            .map(StatefulFid::try_from)
            .collect::<Result<Vec<_>, _>>();

        let result = match ids {
            Ok(ids) => self.delete_elements(ids).await,
            Err(e) => Err(e.into()),
        };

        Ok(Response::new(match result {
            Ok(_) => proto::Response { error: None },
            Err(e) => proto::Response {
                error: Some(e.into()),
            },
        }))
    }

    async fn get(
        &self,
        req: tonic::Request<proto::IdRequest>,
    ) -> Result<Response<proto::GetResponse>, Status> {
        let ids = req
            .into_inner()
            .fid
            .into_iter()
            .map(StatefulFid::try_from)
            .collect::<Result<Vec<_>, _>>();

        let result = match ids {
            Ok(ids) => self.get_elements(ids).await,
            Err(e) => Err(e.into()),
        };

        Ok(Response::new(match result {
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
        req: tonic::Request<proto::ApplyRequest>,
    ) -> Result<Response<proto::CreateResponse>, Status> {
        let result = async {
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
                StatefulFid::Room { id } => match elem {
                    proto::apply_request::El::Room(_) => {
                        self.apply_room(Room::try_from(elem)?).await
                    }
                    proto::apply_request::El::Member(_)
                    | proto::apply_request::El::WebrtcPlay(_)
                    | proto::apply_request::El::WebrtcPub(_) => Err(
                        ErrorResponse::new(ErrorCode::ElementIdMismatch, &id),
                    ),
                },
                StatefulFid::Member { id, room_id } => match elem {
                    proto::apply_request::El::Member(member) => self
                        .apply_room_member(room_id, Member::try_from(member)?)
                        .await
                        .map(|_| Sids::new()),
                    proto::apply_request::El::Room(_)
                    | proto::apply_request::El::WebrtcPlay(_)
                    | proto::apply_request::El::WebrtcPub(_) => {
                        Err(ErrorResponse::new(
                            ErrorCode::ElementIdMismatch,
                            &StatefulFid::Member { id, room_id },
                        ))
                    }
                },
                StatefulFid::Endpoint { .. } => {
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

        Ok(Response::new(match result.await {
            Ok(sids) => proto::CreateResponse {
                sid: sids
                    .into_iter()
                    .map(|(id, sid)| (id.to_string(), sid.to_string()))
                    .collect(),
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
        request: tonic::Request<proto::Ping>,
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
                    message.push_str(&format!(": {explanation}"));
                }

                Status::unknown(message)
            })
    }
}
