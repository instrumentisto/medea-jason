//! Wrapper around [RTCRtpEncodingParameters][0].
//!
//! [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters

use medea_client_api_proto::{EncodingParameters, ScalabilityMode};
use web_sys::RtcRtpEncodingParameters;

/// Wrapper around [RTCRtpEncodingParameters][0] providing handy methods for its
/// direction changes.
///
/// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
#[derive(Clone, Debug)]
pub struct SendEncodingParameters(RtcRtpEncodingParameters);

impl SendEncodingParameters {
    /// Creates new [`SendEncodingParameters`].
    #[expect(clippy::needless_pass_by_value, reason = "`cfg` code uniformity")]
    #[must_use]
    pub fn new(rid: String, active: bool) -> Self {
        let params = RtcRtpEncodingParameters::new();
        params.set_rid(&rid);
        params.set_active(active);
        Self(params)
    }

    /// Returns the underlying [`RtcRtpEncodingParameters`] of these
    /// [`SendEncodingParameters`].
    #[must_use]
    pub const fn handle(&self) -> &RtcRtpEncodingParameters {
        &self.0
    }

    /// Sets [activeness][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-active
    pub fn set_active(&self, active: bool) {
        self.0.set_active(active);
    }

    /// Sets [maxBitrate][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-maxbitrate
    pub fn set_max_bitrate(&self, max_bitrate: u32) {
        self.0.set_max_bitrate(max_bitrate);
    }

    /// Sets [scaleResolutionDownBy][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/ypzzc75t
    pub fn set_scale_resolution_down_by(&self, scale_resolution_down_by: f32) {
        self.0
            .set_scale_resolution_down_by(scale_resolution_down_by);
    }

    /// Sets [scalabilityMode][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/3zuaee45
    pub fn set_scalability_mode(&self, scalability_mode: ScalabilityMode) {
        self.0.set_scalability_mode(&scalability_mode.to_string());
    }
}

impl From<EncodingParameters> for SendEncodingParameters {
    fn from(from: EncodingParameters) -> Self {
        let EncodingParameters {
            rid,
            active,
            max_bitrate,
            scale_resolution_down_by,
        } = from;

        let enc = Self::new(rid, active);

        if let Some(b) = max_bitrate {
            enc.set_max_bitrate(b);
        }
        if let Some(s) = scale_resolution_down_by {
            enc.set_scale_resolution_down_by(s.into());
        }

        enc
    }
}
