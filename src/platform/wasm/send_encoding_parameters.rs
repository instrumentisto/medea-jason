//! [SendEncodingParameters] wrapper.
//!
//! [SendEncodingParameters]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use web_sys::RtcRtpEncodingParameters;

use medea_client_api_proto::{EncodingParameters, ScalabilityMode};

/// Wrapper around [RTCRtpEncodingParameters] which provides handy methods for
/// direction changes.
///
/// [RTCRtpEncodingParameters]: https://tinyurl.com/mr3dt9ch
#[derive(Clone, Debug)]
pub struct SendEncodingParameters(RtcRtpEncodingParameters);

impl SendEncodingParameters {
    /// Creates a new [`SendEncodingParameters`].
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(rid: String, active: bool) -> Self {
        let mut params = RtcRtpEncodingParameters::new();
        _ = params.rid(&rid);
        _ = params.active(active);
        Self(params)
    }

    /// Returns underlying [`_Dart_Handle`].
    #[must_use]
    pub const fn handle(&self) -> &RtcRtpEncodingParameters {
        &self.0
    }

    /// Sets [`active`] of this [`SendEncodingParameters`].
    ///
    /// [`active`]:
    /// https://w3.org/TR/webrtc/#dom-rtcrtpencodingparameters-active
    pub fn set_active(&mut self, active: bool) {
        _ = self.0.active(active);
    }

    /// Sets [`maxBitrate`] of this [`SendEncodingParameters`].
    ///
    /// [`maxBitrate`]:
    /// https://w3.org/TR/webrtc/#dom-rtcrtpencodingparameters-maxbitrate
    pub fn set_max_bitrate(&mut self, max_bitrate: u32) {
        _ = self.0.max_bitrate(max_bitrate);
    }

    /// Sets [`scaleResolutionDownBy`] of this [`SendEncodingParameters`].
    ///
    /// [`scaleResolutionDownBy`]: https://tinyurl.com/ypzzc75t
    pub fn set_scale_resolution_down_by(
        &mut self,
        scale_resolution_down_by: f32,
    ) {
        _ = self.0.scale_resolution_down_by(scale_resolution_down_by);
    }

    /// Sets [`scalabilityMode`] of this [`SendEncodingParameters`].
    ///
    /// [`scalabilityMode`]: https://tinyurl.com/3zuaee45
    pub fn set_scalability_mode(&mut self, scalability_mode: ScalabilityMode) {
        _ = self.0.scalability_mode(&scalability_mode.to_string());
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

        let mut enc = Self::new(rid, active);

        if let Some(b) = max_bitrate {
            enc.set_max_bitrate(b);
        }
        if let Some(s) = scale_resolution_down_by {
            enc.set_scale_resolution_down_by(s.into());
        }

        enc
    }
}
