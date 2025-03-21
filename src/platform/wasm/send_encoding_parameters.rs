//! Wrapper around [RTCRtpEncodingParameters][0].
//!
//! [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters

use derive_more::{From, Into};
use web_sys::RtcRtpEncodingParameters;

/// Wrapper around [RTCRtpEncodingParameters][0] providing handy methods for its
/// direction changes.
///
/// [0]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters
#[derive(Clone, Debug, From, Into)]
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

    /// Returns [RID] of these [`SendEncodingParameters`].
    ///
    /// [RID]: https://w3.org/TR/webrtc#dom-rtcrtpcodingparameters-rid
    #[must_use]
    pub fn rid(&self) -> Option<String> {
        self.0.get_rid()
    }

    /// Sets [activeness][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-active
    pub fn set_active(&self, active: bool) {
        self.0.set_active(active);
    }

    /// Returns [activeness][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-active
    #[must_use]
    pub fn active(&self) -> bool {
        // default is true according to spec
        self.0.get_active().unwrap_or(true)
    }

    /// Sets [maxBitrate][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-maxbitrate
    pub fn set_max_bitrate(&self, max_bitrate: u32) {
        self.0.set_max_bitrate(max_bitrate);
    }

    /// Returns [maxBitrate][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpencodingparameters-maxbitrate
    #[must_use]
    pub fn max_bitrate(&self) -> Option<u32> {
        self.0.get_max_bitrate()
    }

    /// Sets [scaleResolutionDownBy][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/ypzzc75t
    pub fn set_scale_resolution_down_by(&self, scale_resolution_down_by: f32) {
        self.0.set_scale_resolution_down_by(scale_resolution_down_by);
    }

    /// Returns [scaleResolutionDownBy][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/ypzzc75t
    pub fn scale_resolution_down_by(&self) -> f64 {
        self.0.get_scale_resolution_down_by().map_or(1.0, Into::into)
    }

    /// Sets [scalabilityMode][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/3zuaee45
    #[expect(clippy::needless_pass_by_value, reason = "`cfg` code uniformity")]
    pub fn set_scalability_mode(&self, scalability_mode: String) {
        self.0.set_scalability_mode(&scalability_mode);
    }

    /// Returns [scalabilityMode][1] of these [`SendEncodingParameters`].
    ///
    /// [1]: https://tinyurl.com/3zuaee45
    #[must_use]
    pub fn scalability_mode(&self) -> Option<String> {
        self.0.get_scalability_mode()
    }
}
