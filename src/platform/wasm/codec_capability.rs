//! WASM side representation of an [RTCRtpCodecCapability].
//!
//! [RTCRtpCodecCapability]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability

use std::collections::HashMap;

use web_sys::{RtcRtpCodecCapability, RtcRtpReceiver, RtcRtpSender};

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

    /// Returns available [RTCRtpReceiver]'s [`CodecCapability`]s.
    ///
    /// # Errors
    ///
    /// With [`Error::FailedToGetCapabilities`] if fails to retrieve
    /// capabilities.
    ///
    /// [RTCRtpReceiver]: https://w3.org/TR/webrtc#dom-rtcrtpreceiver
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn get_receiver_codec_capabilities(
        kind: MediaKind,
    ) -> Result<Vec<Self>, Error> {
        let mut result = Vec::new();

        let Some(caps) = RtcRtpReceiver::get_capabilities(&kind.to_string())
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

    /// Returns [mimeType][2] of the provided [RTCRtpCodec][1].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodec
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodec-mimetype
    #[must_use]
    pub fn mime_type(&self) -> String {
        self.0.get_mime_type()
    }

    /// Returns [clockRate][2] of the provided [RTCRtpCodec][1].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodec
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodec-clockrate
    #[must_use]
    pub fn clock_rate(&self) -> u32 {
        self.0.get_clock_rate()
    }

    /// Returns [channels][2] of the provided [RTCRtpCodec][1].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodec
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodec-channels
    #[must_use]
    pub fn channels(&self) -> Option<u16> {
        self.0.get_channels()
    }

    /// Returns [sdpFmtpLine][2] of the provided [RTCRtpCodec][1].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodec
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodec-sdpfmtpline
    #[must_use]
    pub fn parameters(&self) -> HashMap<String, String> {
        self.0
            .get_sdp_fmtp_line()
            .unwrap_or_default()
            .split(';')
            .filter_map(|pair| {
                let mut kv = pair.split('=');
                match (kv.next(), kv.next()) {
                    (Some(k), Some(v)) => {
                        Some((k.trim().to_owned(), v.trim().to_owned()))
                    }
                    _ => None,
                }
            })
            .collect::<HashMap<String, String>>()
    }

    /// Returns the underlying [`RtcRtpCodecCapability`] of this
    /// [`CodecCapability`].
    #[must_use]
    pub const fn handle(&self) -> &RtcRtpCodecCapability {
        &self.0
    }
}
