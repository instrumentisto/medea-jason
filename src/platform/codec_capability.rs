use derive_more::{Display, From};

use crate::{platform, utils::Caused};

#[derive(Caused, Clone, Copy, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum CodecCapabilityError {
    #[display(fmt = "Failed to get codec capabilities")]
    FailedToGetCapabilities,

    #[display(fmt = "Failed to get codec's mimeType")]
    FailedToGetMimeType,
}
