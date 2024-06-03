//! Deserialization of [`RtcStats`].

use medea_client_api_proto::stats::RtcStat;

/// All available [`RtcStatsType`]s of a [`RtcPeerConnection`].
///
/// [`RtcStatsType`]: medea_client_api_proto::stats::RtcStatsType
/// [`RtcPeerConnection`]: crate::platform::RtcPeerConnection
#[derive(Clone, Debug)]
pub struct RtcStats(pub Vec<RtcStat>);

#[cfg(feature = "mockable")]
pub mod tests {
    #![allow(clippy::missing_safety_doc)]

    use crate::api::DartValueArg;

    #[no_mangle]
    pub unsafe extern "C" fn test_rtc_stats_parse(
        expects: DartValueArg<String>,
    ) {
        use medea_client_api_proto::stats::RtcStat;
        let expects: String = expects.try_into().unwrap();
        let stats: Vec<RtcStat> = serde_json::from_str(&expects).unwrap();
        assert!(!stats.is_empty());
    }
}
