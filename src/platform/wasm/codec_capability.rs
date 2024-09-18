//! WASM side representation of an [RTCRtpCodecCapability].
//!
//! [RTCRtpCodecCapability]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability

use web_sys::{RtcRtpCodecCapability, RtcRtpSender};

use crate::{
    media::MediaKind, platform::codec_capability::CodecCapabilityError as Error,
};

/// WASM side representation of an [RTCRtpCodecCapability].
///
/// [RTCRtpCodecCapability]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
#[derive(Clone, Debug)]
pub struct CodecCapability(RtcRtpCodecCapability);

impl CodecCapability {
    /// Returns available [RTCRtpSender]'s [`CodecCapability`]s.
    ///
    /// # Errors
    ///
    /// With [`Error::FailedToGetCapabilities`] if fails to retrieve
    /// capabilities.
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#dom-rtcrtpsender
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn get_sender_codec_capabilities(
        kind: MediaKind,
    ) -> Result<Vec<Self>, Error> {
        let mut result = Vec::new();

        let Some(caps) = RtcRtpSender::get_capabilities(&kind.to_string())
        else {
            return Err(Error::FailedToGetCapabilities);
        };

        for codec in caps.get_codecs().values() {
            let Ok(codec) = codec else {
                continue;
            };
            result.push(Self(RtcRtpCodecCapability::from(codec)));
        }

        Ok(result)
    }

    /// Returns [MIME media type][2] of this [`CodecCapability`].
    ///
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability-mimetype
    #[must_use]
    pub fn mime_type(&self) -> String {
        self.0.get_mime_type()
    }

    /// Returns the underlying [`RtcRtpCodecCapability`] of this
    /// [`CodecCapability`].
    #[must_use]
    pub const fn handle(&self) -> &RtcRtpCodecCapability {
        &self.0
    }
}
