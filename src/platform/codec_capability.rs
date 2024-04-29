use derive_more::{Display, From};

use crate::{platform, utils::Caused};

#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum CodecCapabilityError {
    #[display(fmt = "Failed to get codec capabilities")]
    FailedToGetCapabilities,

    #[display(fmt = "Failed to get codec's mimeType")]
    FailedToGetMimeType,
}

// /// Built [MIME "type/subtype"] string from [name] and [kind].
// ///
// /// [MIME "type/subtype"]: https://en.wikipedia.org/wiki/Media_type
// String mimeType;
//
// /// If unset, the implementation default is used.
// int? clockRate;
//
// /// Default payload type for the codec.
// ///
// /// Mainly needed for codecs that have statically assigned payload types.
// int? preferredPayloadType;
//
// /// Used to identify the codec. Equivalent to [MIME subtype][0].
// ///
// /// [0]: https://en.wikipedia.org/wiki/Media_type#Subtypes
// String name;
//
// /// [MediaKind] of this codec. Equivalent to [MIME] top-level type.
// ///
// /// [MIME]: https://en.wikipedia.org/wiki/Media_type
// MediaKind kind;
//
// /// Number of audio channels used.
// ///
// /// Unset for video codecs.
// ///
// /// If unset for audio, the implementation default is used.
// int? numChannels;
//
// /// Codec-specific parameters that must be signaled to the remote party.
// ///
// /// Corresponds to `a=fmtp` parameters in [SDP].
// ///
// /// Contrary to ORTC, these parameters are named using all lowercase strings.
// /// This helps make the mapping to [SDP] simpler, if an application is using
// /// [SDP]. Boolean values are represented by the string "1".
// ///
// /// [SDP]: https://en.wikipedia.org/wiki/Session_Description_Protocol
// Map<String, String> parameters;
