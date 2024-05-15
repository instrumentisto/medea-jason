//! [SendEncodingParameters] wrapper.
//!
//! [SendEncodingParameters]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use js_sys::{Array, JsString, Reflect};
use wasm_bindgen::JsValue;
use web_sys::RtcRtpSender;

use crate::{
    media::MediaKind, platform::codec_capability::CodecCapabilityError as Error,
};

/// WASM side representation of [RTCRtpCodecCapability].
///
/// [RTCRtpCodecCapability]: https://tinyurl.com/4jcp8m4s
#[derive(Clone, Debug)]
pub struct CodecCapability(JsValue);

impl CodecCapability {
    /// Gets available `sender`'s [`CodecCapability`]s.
    ///
    /// # Errors
    ///
    /// Errors with [`Error::FailedToGetCapabilities`] if fails to get
    /// capabilities.
    // Async needed for consistency with Dart implementation.
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

    /// Gets `mime_type` of this [`CodecCapability`]s.
    ///
    /// # Errors
    ///
    /// Never errors, but [`Result`] is needed for consistency with WASM
    /// implementation.
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
