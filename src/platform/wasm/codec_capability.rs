//! WASM side representation of an [RTCRtpCodecCapability].
//!
//! [RTCRtpCodecCapability]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability

use js_sys::{Array, JsString, Reflect};
use web_sys::{RtcRtpCodecCapability, RtcRtpSender};

use crate::{
    media::MediaKind, platform::codec_capability::CodecCapabilityError as Error,
};

/// WASM side representation of an [RTCRtpCodecCapability].
///
/// [RTCRtpCodecCapability]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
#[derive(Clone, Debug)]
pub struct CodecCapability {
    /// An actual JS-side [`RtcRtpCodecCapability`].
    codec_cap: RtcRtpCodecCapability,

    /// The codec MIME media type/subtype.
    mime_type: String,
}

impl CodecCapability {
    /// Returns available [RTCRtpSender]'s [`CodecCapability`]s.
    ///
    /// # Errors
    ///
    /// With [`Error::FailedToGetCapabilities`] if fails to retrieve
    /// capabilities.
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#dom-rtcrtpsender
    #[allow(clippy::unused_async)] // for platform code uniformity
    pub async fn get_sender_codec_capabilities(
        kind: MediaKind,
    ) -> Result<Vec<Self>, Error> {
        let mut result = Vec::new();

        let Some(caps) = RtcRtpSender::get_capabilities(&kind.to_string())
        else {
            return Err(Error::FailedToGetCapabilities);
        };

        // TODO: Get rid of reflection in #183 which updates web-sys to 0.3.70.
        let Ok(codecs) = Reflect::get(&caps, &JsString::from("codecs")) else {
            return Err(Error::FailedToGetCapabilities);
        };

        for codec in Array::from(&codecs).values() {
            let Ok(codec) = codec else {
                continue;
            };

            let codec_cap = RtcRtpCodecCapability::from(codec);
            let Some(mime_type) =
                Reflect::get(&codec_cap, &JsString::from("mimeType"))
                    .ok()
                    .and_then(|v| v.as_string())
            else {
                return Err(Error::FailedToGetCapabilities);
            };

            result.push(Self {
                codec_cap,
                mime_type,
            });
        }

        Ok(result)
    }

    /// Returns [MIME media type][2] of this [`CodecCapability`].
    ///
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability-mimetype
    #[must_use]
    pub fn mime_type(&self) -> String {
        self.mime_type.clone()
    }

    /// Returns the underlying [`RtcRtpCodecCapability`] of this
    /// [`CodecCapability`].
    #[must_use]
    pub const fn handle(&self) -> &RtcRtpCodecCapability {
        &self.codec_cap
    }
}
