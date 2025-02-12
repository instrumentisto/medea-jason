//! Platform-agnostic functionality of a [`platform::CodecCapability`].

use derive_more::with_trait::{Display, From};
use medea_client_api_proto as proto;

use crate::{
    media::MediaKind,
    platform,
    platform::{
        send_encoding_parameters::SendEncodingParameters, CodecCapability,
    },
    utils::Caused,
};

/// Error occurred when retrieving a [`platform::CodecCapability`].
#[derive(Caused, Clone, Copy, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum CodecCapabilityError {
    /// Failed to retrieve an [RTCRtpCodecCapability][1].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
    #[display("Failed to retrieve codec capabilities")]
    FailedToGetCapabilities,

    /// Failed to retrieve an [RTCRtpCodecCapability][1]'s [MIME media
    /// type][2].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability-mimetype
    #[display("Failed to get codec's mimeType")]
    FailedToGetMimeType,
}

impl From<proto::EncodingParameters> for SendEncodingParameters {
    fn from(from: proto::EncodingParameters) -> Self {
        let proto::EncodingParameters {
            rid,
            active,
            max_bitrate,
            scale_resolution_down_by,
            scalability_mode,
            ..
        } = from;

        let enc = Self::new(rid, active);

        if let Some(b) = max_bitrate {
            enc.set_max_bitrate(b);
        }
        if let Some(s) = scale_resolution_down_by {
            enc.set_scale_resolution_down_by(s.into());
        }
        if let Some(s) = scalability_mode {
            enc.set_scalability_mode(&s.to_string());
        }

        enc
    }
}

/// Returns [`proto::Capabilities`] of the current platform.
#[must_use]
pub async fn get_capabilities() -> proto::Capabilities {
    let convert_caps = |caps: Vec<CodecCapability>| -> Vec<proto::Codec> {
        caps.into_iter()
            .map(|c| proto::Codec {
                mime_type: c.mime_type(),
                clock_rate: c.clock_rate(),
                channels: c.channels(),
                parameters: c.parameters(),
            })
            .collect::<Vec<_>>()
    };

    let audio_tx =
        CodecCapability::get_sender_codec_capabilities(MediaKind::Audio)
            .await
            .unwrap_or_default();
    let audio_rx =
        CodecCapability::get_receiver_codec_capabilities(MediaKind::Audio)
            .await
            .unwrap_or_default();
    let video_tx =
        CodecCapability::get_sender_codec_capabilities(MediaKind::Video)
            .await
            .unwrap_or_default();
    let video_rx =
        CodecCapability::get_receiver_codec_capabilities(MediaKind::Video)
            .await
            .unwrap_or_default();

    proto::Capabilities {
        audio_tx: convert_caps(audio_tx),
        audio_rx: convert_caps(audio_rx),
        video_tx: convert_caps(video_tx),
        video_rx: convert_caps(video_rx),
    }
}
