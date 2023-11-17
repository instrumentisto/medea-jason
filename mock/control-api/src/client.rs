//! Implementation of client for [Medea]'s gRPC [Control API].
//!
//! [Medea]: https://github.com/instrumentisto/medea
//! [Control API]: https://tinyurl.com/yxsqplq7

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use actix::Recipient;
use derive_more::{AsRef, From, Into};
use medea_control_api_proto::grpc::api as proto;
use proto::control_api_client::ControlApiClient;
use tonic::{transport::Channel, Status};

use crate::{
    api::{ws::Notification, Element, Subscribers},
    prelude::*,
};

/// Fid to `Room` element.
#[derive(Clone, Debug, AsRef, From, Into)]
#[as_ref(forward)]
pub struct Fid(String);

impl Fid {
    /// Returns `Room`'s ID from this [`Fid`].
    fn room_id(&self) -> &str {
        // PANIC: Slicing is OK here, as the index is taken from the source.
        #[allow(clippy::string_slice)]
        self.0.find('/').map_or(self.0.as_str(), |i| &self.0[..i])
    }
}

impl From<()> for Fid {
    fn from((): ()) -> Self {
        Self(String::new())
    }
}

impl From<(String, String)> for Fid {
    fn from(path: (String, String)) -> Self {
        Self(format!("{}/{}", path.0, path.1))
    }
}

impl From<(String, String, String)> for Fid {
    fn from(path: (String, String, String)) -> Self {
        Self(format!("{}/{}/{}", path.0, path.1, path.2))
    }
}

/// Returns new [`proto::IdRequest`] with provided FIDs.
const fn id_request(ids: Vec<String>) -> proto::IdRequest {
    proto::IdRequest { fid: ids }
}

/// Client for [Medea]'s [Control API].
///
/// [Medea]: https://github.com/instrumentisto/medea
/// [Control API]: https://tinyurl.com/yxsqplq7
#[derive(Clone, Debug)]
pub struct ControlClient {
    /// Map of subscribers to [`Notification`]s.
    subscribers: Subscribers,

    /// [`tonic`] gRPC client for Medea Control API.
    grpc_client: ControlApiClient<Channel>,
}

impl ControlClient {
    /// Creates a new client for Medea's Control API.
    ///
    /// __Note that call of this function doesn't checks availability of Control
    /// API gRPC server. Availability will be checked only on sending request to
    /// gRPC server.__
    ///
    ///
    /// # Errors
    ///
    /// Errors if unable to resolve the provided `medea_addr`.
    pub async fn new(
        medea_addr: String,
        subscribers: Arc<Mutex<HashMap<String, Vec<Recipient<Notification>>>>>,
    ) -> Result<Self, tonic::transport::Error> {
        let grpc_client = {
            /// Max number of retries when connection medea.
            const MAX_RETRIES: u64 = 5;

            let mut current_try = 0;
            loop {
                current_try += 1;
                let client =
                    ControlApiClient::connect(medea_addr.clone()).await;

                match client {
                    Ok(client) => {
                        break client;
                    }
                    Err(err) => {
                        if current_try == MAX_RETRIES {
                            error!("Error connection to medea: {}", err);
                            return Err(err);
                        }
                        error!("Error connection to medea: {}, retrying", err);
                        actix::clock::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        };

        Ok(Self {
            subscribers,
            grpc_client,
        })
    }

    /// Returns [`ControlApiClient`] of this [`ControlClient`].
    fn get_client(&self) -> ControlApiClient<Channel> {
        self.grpc_client.clone()
    }

    /// Creates provided element with gRPC Control API.
    ///
    /// # Errors
    ///
    /// Errors if gRPC request fails.
    #[allow(clippy::missing_panics_doc)]
    pub async fn create(
        &self,
        id: String,
        fid: Fid,
        element: Element,
    ) -> Result<proto::CreateResponse, Status> {
        use proto::create_request::El;

        let room_id = if fid.0.is_empty() {
            id.clone()
        } else {
            fid.room_id().to_owned()
        };

        let notification = Notification::created(&fid, &element);
        let el = match element {
            Element::Room(room) => El::Room(room.into_proto(id)),
            Element::Member(member) => El::Member(member.into_proto(id)),
            Element::WebRtcPlayEndpoint(webrtc_play) => {
                El::WebrtcPlay(webrtc_play.into_proto(id))
            }
            Element::WebRtcPublishEndpoint(webrtc_pub) => {
                El::WebrtcPub(webrtc_pub.into_proto(id))
            }
        };
        let req = proto::CreateRequest {
            parent_fid: fid.into(),
            el: Some(el),
        };

        let response = self.get_client().create(tonic::Request::new(req)).await;

        if response.is_ok() {
            if let Some(subs) = self.subscribers.lock().unwrap().get(&room_id) {
                for sub in subs {
                    sub.do_send(notification.clone());
                }
            };
        }

        response.map(tonic::Response::into_inner)
    }

    /// Applies the provided element via gRPC Control API.
    ///
    /// # Errors
    ///
    /// Errors if gRPC request fails.
    pub async fn apply(
        &self,
        id: String,
        fid: Fid,
        element: Element,
    ) -> Result<proto::CreateResponse, Status> {
        use proto::apply_request::El;

        let el = match element {
            Element::Room(room) => El::Room(room.into_proto(id)),
            Element::Member(member) => El::Member(member.into_proto(id)),
            Element::WebRtcPlayEndpoint(webrtc_play) => {
                El::WebrtcPlay(webrtc_play.into_proto(id))
            }
            Element::WebRtcPublishEndpoint(webrtc_pub) => {
                El::WebrtcPub(webrtc_pub.into_proto(id))
            }
        };
        let req = proto::ApplyRequest {
            parent_fid: fid.into(),
            el: Some(el),
        };

        let response = self.get_client().apply(tonic::Request::new(req)).await;
        response.map(tonic::Response::into_inner)
    }

    /// Gets element from Control API by FID.
    ///
    /// # Errors
    ///
    /// Errors if gRPC request fails.
    pub async fn get(&self, fid: Fid) -> Result<proto::GetResponse, Status> {
        let req = id_request(vec![fid.into()]);
        self.get_client()
            .get(tonic::Request::new(req))
            .await
            .map(tonic::Response::into_inner)
    }

    /// Deletes element from Control API by FID.
    ///
    /// # Errors
    ///
    /// Errors if gRPC request fails.
    #[allow(clippy::missing_panics_doc)]
    pub async fn delete(&self, fid: Fid) -> Result<proto::Response, Status> {
        let req = id_request(vec![fid.clone().into()]);
        let response = self.get_client().delete(tonic::Request::new(req)).await;

        if response.is_ok() {
            if let Some(subs) =
                self.subscribers.lock().unwrap().get(fid.room_id())
            {
                let notification = Notification::deleted(&fid);
                for sub in subs {
                    sub.do_send(notification.clone());
                }
            };
        }
        response.map(tonic::Response::into_inner)
    }
}
