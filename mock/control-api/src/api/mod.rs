//! REST [Control API] mock server implementation.
//!
//! [Control API]: https://tinyurl.com/yxsqplq7

pub mod endpoint;
pub mod member;
pub mod room;
pub mod ws;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix::{Addr, Recipient};
use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer,
    error::{Error as HttpError, ErrorInternalServerError as InternalError},
    middleware,
    web::{self, Data, Json, Path},
};
use derive_more::with_trait::From;
use medea_control_api_proto::grpc::api as proto;
use serde::{Deserialize, Serialize};

use self::{
    endpoint::{WebRtcPlayEndpoint, WebRtcPublishEndpoint},
    member::Member,
    room::Room,
};
use crate::{
    Cli,
    api::ws::Notification,
    callback::server::{GetCallbackItems, GrpcCallbackServer},
    client::{ControlClient, Fid},
    prelude::*,
};

/// Map of subscribers to [`Notification`]s.
pub type Subscribers =
    Arc<Mutex<HashMap<String, Vec<Recipient<Notification>>>>>;

/// Context of [`actix_web`] server.
#[derive(Debug)]
pub struct AppContext {
    /// Client for [Medea]'s [Control API].
    ///
    /// [Control API]: https://tinyurl.com/yxsqplq7
    /// [Medea]: https://github.com/instrumentisto/medea
    client: ControlClient,

    /// Map of subscribers to [`Notification`]s.
    subscribers: Subscribers,

    /// gRPC server which receives Control API callbacks.
    callback_server: Addr<GrpcCallbackServer>,
}

/// Run REST [Control API] server mock.
///
/// # Panics
///
/// If cannot bind and run HTTP server.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
pub async fn run(opts: &Cli, callback_server: Addr<GrpcCallbackServer>) {
    let subscribers = Arc::new(Mutex::new(HashMap::new()));
    let app_data = Data::new(AppContext {
        client: ControlClient::new(
            opts.medea_addr.clone().into(),
            Arc::clone(&subscribers),
        )
        .await
        .unwrap(),
        subscribers,
        callback_server,
    });

    HttpServer::new(move || {
        debug!("Running HTTP server...");
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/subscribe/{id}")
                    .route(web::get().to(ws::create_ws)),
            )
            .service(
                web::resource("/control-api/{a}")
                    .route(web::post().to(create::create1))
                    .route(web::get().to(get::get1))
                    .route(web::delete().to(delete::delete1))
                    .route(web::put().to(apply::apply1)),
            )
            .service(
                web::resource("/control-api/{a}/{b}")
                    .route(web::post().to(create::create2))
                    .route(web::get().to(get::get2))
                    .route(web::delete().to(delete::delete2))
                    .route(web::put().to(apply::apply2)),
            )
            .service(
                web::resource("/control-api/{a}/{b}/{c}")
                    .route(web::post().to(create::create3))
                    .route(web::get().to(get::get3))
                    .route(web::delete().to(delete::delete3)),
            )
            .service(
                web::resource("/callbacks").route(web::get().to(get_callbacks)),
            )
    })
    .bind(&*opts.addr)
    .unwrap()
    .run()
    .await
    .unwrap();
}

/// Generates `request` macro which will generate [`actix_web`] request handler
/// which will call some function with `Path` extracted from `Request`.
///
/// `$call_fn` - function which will be called on request;
///
/// `$resp` - type of response on this request.
macro_rules! gen_request_macro {
    ($call_fn:tt, $resp:ty) => {
        /// Generates handler with provided name and `Path` which will be
        /// passed to `$call_fn` function.
        ///
        /// `$name` - name of generated function;
        ///
        /// `$uri_tuple` - type of path which will be provided by [`actix_web`].
        macro_rules! request {
            ($name: tt,$uri_tuple: ty) => {
                pub async fn $name(
                    path: Path<$uri_tuple>,
                    state: Data<AppContext>,
                ) -> Result<HttpResponse, actix_web::Error> {
                    state
                        .client
                        .$call_fn(path.into_inner().into())
                        .await
                        .map_err(|e| {
                            actix_web::error::ErrorInternalServerError(
                                e.to_string(),
                            )
                        })
                        .map(|r| <$resp>::from(r).into())
                }
            };
        }
    };
}

/// [`actix_web`] REST API endpoint which returns all Control API Callbacks
/// received by this mock server.
///
/// # Errors
///
/// Errors if unable to send message to [`GrpcCallbackServer`] actor.
// TODO: Needs refactoring.
#[expect(clippy::missing_panics_doc, reason = "needs refactoring")]
pub async fn get_callbacks(
    state: Data<AppContext>,
) -> Result<HttpResponse, HttpError> {
    state
        .callback_server
        .send(GetCallbackItems)
        .await
        .map_err(|e| {
            InternalError(format!("`GrpcCallbackServer` mailbox error: {e}"))
        })
        .map(|callbacks| HttpResponse::Ok().json(callbacks.unwrap()))
}

/// Implementation of `Delete` requests to [Control API] mock.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
mod delete {
    use super::{AppContext, Data, HttpResponse, Path, Response};

    gen_request_macro!(delete, Response);

    request!(delete1, String);
    request!(delete2, (String, String));
    request!(delete3, (String, String, String));
}

/// Implementation of `Get` requests to [Control API] mock.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
mod get {
    use super::{AppContext, Data, HttpResponse, Path, SingleGetResponse};

    gen_request_macro!(get, SingleGetResponse);

    request!(get1, String);
    request!(get2, (String, String));
    request!(get3, (String, String, String));
}

/// Implementation of `Post` requests to [Control API] mock.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
mod create {
    use super::{
        AppContext, CreateResponse, Data, Element, Fid, HttpError,
        HttpResponse, InternalError, Json, Path,
    };

    /// Creates the given [`Element`] under the given FID represented as
    /// one-segment `path`.
    pub async fn create1(
        path: Path<String>,
        state: Data<AppContext>,
        data: Json<Element>,
    ) -> Result<HttpResponse, HttpError> {
        state
            .client
            .create(path.into_inner(), Fid::from(()), data.0)
            .await
            .map_err(|e| InternalError(e.to_string()))
            .map(|r| CreateResponse::from(r).into())
    }

    /// Creates the given [`Element`] under the given FID represented as
    /// two-segments `path`.
    pub async fn create2(
        path: Path<(String, String)>,
        state: Data<AppContext>,
        data: Json<Element>,
    ) -> Result<HttpResponse, HttpError> {
        let uri = path.into_inner();
        state
            .client
            .create(uri.1, Fid::from(uri.0), data.0)
            .await
            .map_err(|e| InternalError(e.to_string()))
            .map(|r| CreateResponse::from(r).into())
    }

    /// Creates the given [`Element`] under the given FID represented as
    /// three-segments `path`.
    pub async fn create3(
        path: Path<(String, String, String)>,
        state: Data<AppContext>,
        data: Json<Element>,
    ) -> Result<HttpResponse, HttpError> {
        let uri = path.into_inner();
        state
            .client
            .create(uri.2, Fid::from((uri.0, uri.1)), data.0)
            .await
            .map_err(|e| InternalError(e.to_string()))
            .map(|r| CreateResponse::from(r).into())
    }
}

/// Implementation of `Put` requests to [Control API] mock.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
mod apply {
    use super::{
        AppContext, CreateResponse, Data, Element, Fid, HttpError,
        HttpResponse, InternalError, Json, Path,
    };

    /// Renews the [`Element`] by its FID represented as one-segment `path`.
    pub async fn apply1(
        path: Path<String>,
        state: Data<AppContext>,
        data: Json<Element>,
    ) -> Result<HttpResponse, HttpError> {
        state
            .client
            .apply(path.clone(), Fid::from(path.into_inner()), data.0)
            .await
            .map_err(|e| InternalError(e.to_string()))
            .map(|r| CreateResponse::from(r).into())
    }

    /// Renews the [`Element`] by its FID represented as two-segments `path`.
    pub async fn apply2(
        path: Path<(String, String)>,
        state: Data<AppContext>,
        data: Json<Element>,
    ) -> Result<HttpResponse, HttpError> {
        let uri = path.into_inner();
        state
            .client
            .apply(uri.1.clone(), Fid::from((uri.0, uri.1)), data.0)
            .await
            .map_err(|e| InternalError(e.to_string()))
            .map(|r| CreateResponse::from(r).into())
    }
}

/// Error object. Returns when some error happened on [Control API]'s side.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    /// Medea's Control API error code.
    pub code: u32,

    /// Text of error.
    pub text: String,

    /// Element's ID with which error happened.
    pub element: String,
}

impl From<proto::Error> for ErrorResponse {
    fn from(e: proto::Error) -> Self {
        Self { code: e.code, text: e.text, element: e.element }
    }
}

/// Response which returns sids.
///
/// Used for create methods.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateResponse {
    /// URIs with which [Jason] can connect `Member`s.
    ///
    /// [Jason]: https://github.com/instrumentisto/medea-jason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sids: Option<HashMap<String, String>>,

    /// Error if something happened on [Control API]'s side.
    ///
    /// [Control API]: https://tinyurl.com/yxsqplq7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

/// Response which can return only error (if any).
///
/// Used for delete methods.
#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    /// Error if something happened on [Control API]'s side.
    ///
    /// [Control API]: https://tinyurl.com/yxsqplq7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

/// Macro that implements [`From`] `control-api-mock` responses for
/// [`HttpResponse`].
///
/// Implementation will check existence of `error` and if it exists then
/// [`HttpResponse`] will be `BadRequest` with this struct as response in
/// otherwise `Ok` with this struct as response.
macro_rules! impl_from_for_http_response {
    ($resp:tt) => {
        impl From<$resp> for HttpResponse {
            fn from(resp: $resp) -> Self {
                if resp.error.is_some() {
                    Self::BadRequest().json(resp)
                } else {
                    Self::Ok().json(resp)
                }
            }
        }
    };
}

impl_from_for_http_response!(CreateResponse);
impl_from_for_http_response!(Response);
impl_from_for_http_response!(SingleGetResponse);

impl From<proto::Response> for Response {
    fn from(resp: proto::Response) -> Self {
        Self { error: resp.error.map(Into::into) }
    }
}

impl From<proto::CreateResponse> for CreateResponse {
    fn from(resp: proto::CreateResponse) -> Self {
        resp.error.map_or(Self { sids: Some(resp.sid), error: None }, |error| {
            Self { sids: None, error: Some(error.into()) }
        })
    }
}

/// Union of all elements which exists in [Medea].
///
/// [Medea]: https://github.com/instrumentisto/medea
#[derive(Debug, Deserialize, From, Serialize)]
#[serde(tag = "kind")]
pub enum Element {
    /// [`Member`] element.
    Member(Box<Member>),

    /// [`WebRtcPublishEndpoint`] element.
    WebRtcPublishEndpoint(WebRtcPublishEndpoint),

    /// [`WebRtcPlayEndpoint`] element.
    WebRtcPlayEndpoint(WebRtcPlayEndpoint),

    /// [`Room`] element.
    Room(Room),
}

impl Element {
    /// Converts this [`Element`] into an appropriate [`proto::room::Element`].
    ///
    /// # Panics
    ///
    /// If a conversion for such an [`Element`] isn't implemented yet.
    #[must_use]
    pub fn into_proto(self, id: String) -> proto::room::Element {
        let el = match self {
            Self::Member(m) => {
                proto::room::element::El::Member(m.into_proto(id))
            }
            Self::WebRtcPublishEndpoint(_)
            | Self::WebRtcPlayEndpoint(_)
            | Self::Room(_) => unimplemented!(),
        };
        proto::room::Element { el: Some(el) }
    }
}

#[expect(clippy::fallible_impl_from, reason = "unrelated")]
impl From<proto::Element> for Element {
    fn from(proto: proto::Element) -> Self {
        use proto::element::El;

        match proto.el.unwrap() {
            El::Room(room) => Self::Room(room.into()),
            El::Member(member) => Self::Member(Box::new(member.into())),
            El::WebrtcPub(webrtc_pub) => {
                Self::WebRtcPublishEndpoint(webrtc_pub.into())
            }
            El::WebrtcPlay(webrtc_play) => {
                Self::WebRtcPlayEndpoint(webrtc_play.into())
            }
        }
    }
}

#[expect(clippy::fallible_impl_from, reason = "unrelated")]
impl From<proto::room::Element> for Element {
    fn from(proto: proto::room::Element) -> Self {
        match proto.el.unwrap() {
            proto::room::element::El::Member(member) => {
                Self::Member(Box::new(member.into()))
            }
            proto::room::element::El::WebrtcPlay(_)
            | proto::room::element::El::WebrtcPub(_) => unimplemented!(
                "Currently Control API mock server supports only Member \
                 element in Room pipeline.",
            ),
        }
    }
}

/// Response on request for get `Element` request.
#[derive(Debug, Deserialize, Serialize)]
pub struct SingleGetResponse {
    /// Requested element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<Element>,

    /// [`ErrorResponse`] if some error happened on [Control API]'s side.
    /// Otherwise `None`.
    ///
    /// [Control API]: https://tinyurl.com/yxsqplq7
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}

impl From<proto::GetResponse> for SingleGetResponse {
    fn from(proto: proto::GetResponse) -> Self {
        proto.error.map_or(
            Self {
                error: None,
                element: proto.elements.into_values().map(Element::from).next(),
            },
            |error| Self { element: None, error: Some(error.into()) },
        )
    }
}
