//! Platform-agnostic functionality of [`platform::CodecCapability`].

use derive_more::{Display, From};

use crate::{platform, utils::Caused};

/// Error occurred when getting [`platform::CodecCapability`].
#[derive(Caused, Clone, Copy, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum CodecCapabilityError {
    /// Failed to get [`RTCRtpCodecCapability`].
    ///
    /// [`RTCRtpCodecCapability`]: https://tinyurl.com/4jcp8m4s
    #[display(fmt = "Failed to get codec capabilities")]
    FailedToGetCapabilities,
}
