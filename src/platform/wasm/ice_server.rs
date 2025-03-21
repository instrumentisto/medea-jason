//! Collection of [`RtcIceServer`][1]s.
//!
//! [1]: https://w3.org/TR/webrtc#rtciceserver-dictionary

use derive_more::with_trait::Deref;
use js_sys::Array as JsArray;
use medea_client_api_proto::IceServer;
use wasm_bindgen::JsValue;
use web_sys::RtcIceServer;

/// Collection of [`RtcIceServer`]s (see [RTCIceServer][1]).
///
/// [1]: https://w3.org/TR/webrtc#rtciceserver-dictionary
#[derive(Debug, Deref)]
pub struct RtcIceServers(JsArray);

impl<I> From<I> for RtcIceServers
where
    I: IntoIterator<Item = IceServer>,
{
    fn from(servers: I) -> Self {
        let inner = JsArray::new();

        for ice_server in servers {
            let server = RtcIceServer::new();

            let urls = JsArray::new();
            for url in ice_server.urls {
                _ = urls.push(&JsValue::from(url));
            }

            server.set_urls(&urls);

            if let Some(credential) = ice_server.credential {
                server.set_credential(credential.expose_str());
            }
            if let Some(username) = ice_server.username {
                server.set_username(&username);
            }

            _ = inner.push(&server.into());
        }

        Self(inner)
    }
}
