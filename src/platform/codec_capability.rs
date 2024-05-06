//! Platform-agnostic functionality of [`platform::CodecCapability`].

use derive_more::{Display, From};

use crate::{platform, utils::Caused};

/// Error occurred when getting [`platform::CodecCapability`].
#[derive(Caused, Clone, Copy, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum CodecCapabilityError {
    /// Failed to get codec capabilities
    #[display(fmt = "Failed to get codec capabilities")]
    FailedToGetCapabilities,

    /// Failed to get codec's mimeType
    #[display(fmt = "Failed to get codec's mimeType")]
    FailedToGetMimeType,
}
