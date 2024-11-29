//! Deserialization of [`RtcStats`] from [`SysRtcStats`].
//!
//! [`SysRtcStats`]: web_sys::RtcStats

use std::rc::Rc;

use js_sys::{Array as JsArray, JSON};
use medea_client_api_proto::stats::{RtcStat, RtcStatsType};
use tracerr::Traced;
use wasm_bindgen::{JsCast as _, JsValue, UnwrapThrowExt as _};
use web_sys::RtcStatsReport;

use crate::platform::{self, RtcStatsError};

/// All available [`RtcStatsType`]s of a [`platform::RtcPeerConnection`].
#[derive(Clone, Debug)]
pub struct RtcStats(pub Vec<RtcStat>);

impl TryFrom<RtcStatsReport> for RtcStats {
    type Error = Traced<RtcStatsError>;

    fn try_from(stats: RtcStatsReport) -> Result<Self, Self::Error> {
        use RtcStatsError::Platform;

        let mut out = Vec::new();

        for stat in stats.entries() {
            let stat = stat.map_err(|e| {
                tracerr::new!(Platform(platform::Error::from(e)))
            })?;
            let stat = stat.unchecked_into::<JsArray>();
            let stat = RtcStatsReportEntry::try_from(stat)
                .map_err(tracerr::map_from_and_wrap!())?;
            let stat_json = JSON::stringify(&JsValue::from(&stat.0))
                .map(String::from)
                .unwrap_throw();
            let rtc_stat: RtcStat = serde_json::from_str(&stat_json)
                .map_err(Rc::new)
                .map_err(tracerr::from_and_wrap!())?;

            if matches!(rtc_stat.stats, RtcStatsType::Other) {
                continue;
            }

            out.push(rtc_stat);
        }

        Ok(Self(out))
    }
}

/// Entry of a JS RTC stats dictionary.
struct RtcStatsReportEntry(JsValue);

impl TryFrom<JsArray> for RtcStatsReportEntry {
    type Error = Traced<RtcStatsError>;

    fn try_from(value: JsArray) -> Result<Self, Self::Error> {
        use RtcStatsError::{Platform, UndefinedId, UndefinedStats};

        let id = value.get(0);
        if id.is_undefined() {
            return Err(tracerr::new!(UndefinedId));
        }

        let stats = value.get(1);
        if stats.is_undefined() {
            return Err(tracerr::new!(UndefinedStats));
        }
        let stats = stats
            .dyn_into::<JsValue>()
            .map_err(|e| tracerr::new!(Platform(platform::Error::from(e))))?;

        Ok(Self(stats))
    }
}
