/// WASM side representation of an [RTCRtpCodecCapability].
///
/// [RTCRtpCodecCapability]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
use js_sys::{Array, JsString, Reflect};
use wasm_bindgen::JsValue;
use web_sys::RtcRtpSender;

use crate::{
    media::MediaKind, platform::codec_capability::CodecCapabilityError as Error,
};

/// WASM side representation of an [RTCRtpCodecCapability].
///
/// [RTCRtpCodecCapability]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
#[derive(Clone, Debug)]
pub struct CodecCapability(JsValue);

impl CodecCapability {
    /// Returns available [RTCRtpSender]'s [`CodecCapability`]s.
    ///
    /// # Errors
    ///
    /// With [`Error::FailedToGetCapabilities`] if fails to retrieve
    /// capabilities.
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#dom-rtcrtpsender
    // Async is needed for consistency with Dart implementation.
    #[allow(clippy::unused_async)]
    pub async fn get_sender_codec_capabilities(
        kind: MediaKind,
    ) -> Result<Vec<Self>, Error> {
        let codecs = RtcRtpSender::get_capabilities(&kind.to_string())
            .and_then(|capabs| {
                Reflect::get(&capabs, &JsString::from("codecs")).ok()
            })
            .ok_or(Error::FailedToGetCapabilities)?;

        Ok(Array::from(&codecs).iter().map(Self).collect())
    }

    /// Returns [MIME media type][2] of this [`CodecCapability`].
    ///
    /// # Errors
    ///
    /// With [`Error::FailedToGetMimeType`] if fails to retrieve codec's
    /// [MIME media type][2].
    ///
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability-mimetype
    pub fn mime_type(&self) -> Result<String, Error> {
        Reflect::get(&self.0, &JsString::from("mimeType"))
            .ok()
            .and_then(|a| a.as_string())
            .ok_or(Error::FailedToGetMimeType)
    }

    /// Returns the underlying [`JsValue`] of this [`CodecCapability`].
    #[must_use]
    pub const fn handle(&self) -> &JsValue {
        &self.0
    }
}
