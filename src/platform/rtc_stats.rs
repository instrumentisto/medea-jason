//! Platform-agnostic functionality of [`platform::RtcStats`].

use std::rc::Rc;

use derive_more::{Display, From};

use crate::{platform, utils::Caused};

/// Errors which can occur during deserialization of a [`RtcStatsType`].
///
/// [`RtcStatsType`]: medea_client_api_proto::stats::RtcStatsType
#[derive(Caused, Clone, Debug, Display, From)]
#[cause(error = platform::Error)]
pub enum RtcStatsError {
    /// [RTCStats.id][1] is undefined.
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcstats-id
    #[display("RTCStats.id is undefined")]
    UndefinedId,

    /// [RTCStats.stats] are undefined.
    ///
    /// [1]: https://w3.org/TR/webrtc-stats/#dfn-stats-object
    #[display("RTCStats.stats are undefined")]
    UndefinedStats,

    /// Some platform error occurred.
    #[display("Unexpected platform error: {}", _0)]
    Platform(platform::Error),

    /// [`platform::RtcStats`] deserialization error.
    #[display("Failed to deserialize into RtcStats: {}", _0)]
    ParseError(Rc<serde_json::Error>),
}
