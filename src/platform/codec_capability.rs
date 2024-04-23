use derive_more::{Display, From};

use crate::{platform, utils::Caused};

#[derive(Clone, Debug)]
pub struct CodecCapability {
    pub mime_type: String,
}

#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum CodecCapabilityError {
    #[display(fmt = "Failed to get codec capabilities")]
    FailedToGetCapabilities,

    #[display(fmt = "Failed to get codec's mimeType")]
    FailedToGetMimeType,
}
