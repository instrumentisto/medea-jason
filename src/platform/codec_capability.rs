//! Platform-agnostic functionality of a [`platform::CodecCapability`].

use derive_more::{Display, From};
use medea_client_api_proto::EncodingParameters;

use crate::{
    platform, platform::send_encoding_parameters::SendEncodingParameters,
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

impl From<EncodingParameters> for SendEncodingParameters {
    fn from(from: EncodingParameters) -> Self {
        let EncodingParameters {
            rid,
            active,
            max_bitrate,
            scale_resolution_down_by,
            scalability_mode,
        } = from;

        let enc = Self::new(rid, active);

        if let Some(b) = max_bitrate {
            enc.set_max_bitrate(b);
        }
        if let Some(s) = scale_resolution_down_by {
            enc.set_scale_resolution_down_by(s.into());
        }
        if let Some(s) = scalability_mode {
            enc.set_scalability_mode(s);
        }

        enc
    }
}
