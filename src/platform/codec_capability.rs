//! Platform-agnostic functionality of a [`platform::CodecCapability`].

use derive_more::{Display, From};

use crate::{platform, utils::Caused};

/// Error occurred when retrieving a [`platform::CodecCapability`].
#[derive(Caused, Clone, Copy, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum CodecCapabilityError {
    /// Failed to retrieve an [RTCRtpCodecCapability][1].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpcodeccapability
    #[display(fmt = "Failed to retrieve codec capabilities")]
    FailedToGetCapabilities,

    /// Failed to get codec's mimeType
    #[display(fmt = "Failed to get codec's mimeType")]
    FailedToGetMimeType,
}
