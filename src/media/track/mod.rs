//! [MediaStreamTrack][1] related objects.
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack

pub mod local;
pub mod remote;

use derive_more::{Display, From, Into};
use medea_client_api_proto as proto;

use crate::{platform, utils::Caused};

/// Error returned when trying to bind to local audio track level changes.
#[derive(Caused, Clone, Debug, Display, From, Into)]
#[cause(error = platform::Error)]
#[display("Failed to calculate audio track level")]
pub struct AudioLevelError(platform::Error);

/// Error returned from media track audio processing manipulation.
#[derive(Caused, Clone, Debug, Display, From, Into)]
#[cause(error = platform::Error)]
#[display("Failed to access audio processing of a track")]
pub struct AudioProcessingError(platform::Error);

/// Liveness state of a [MediaStreamTrack][1] .
///
/// [1]: crate::platform::MediaStreamTrack
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MediaStreamTrackState {
    /// Active track (the track's underlying media source is making a
    /// best-effort attempt to provide a data in real time).
    Live,

    /// Ended track (the track's underlying media source is no longer providing
    /// any data, and will never provide more data for this track).
    ///
    /// This is a final state.
    Ended,
}

/// Media source type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum MediaSourceKind {
    /// Media is sourced from some media device (webcam or microphone).
    Device = 0,

    /// Media is obtained with screen-capture.
    Display = 1,
}

impl From<MediaSourceKind> for proto::MediaSourceKind {
    fn from(val: MediaSourceKind) -> Self {
        match val {
            MediaSourceKind::Device => Self::Device,
            MediaSourceKind::Display => Self::Display,
        }
    }
}

impl From<proto::MediaSourceKind> for MediaSourceKind {
    fn from(val: proto::MediaSourceKind) -> Self {
        match val {
            proto::MediaSourceKind::Device => Self::Device,
            proto::MediaSourceKind::Display => Self::Display,
        }
    }
}
