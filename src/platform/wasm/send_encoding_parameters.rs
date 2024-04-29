use web_sys::RtcRtpEncodingParameters;

use medea_client_api_proto::{EncodingParameters, ScalabilityMode};

#[derive(Clone, Debug)]
pub struct SendEncodingParameters(RtcRtpEncodingParameters);

impl SendEncodingParameters {
    pub fn new(rid: String, active: bool) -> Self {
        let mut params = RtcRtpEncodingParameters::new();
        _ = params.rid(&rid);
        _ = params.active(active);
        Self(params)
    }

    pub fn handle(&self) -> &RtcRtpEncodingParameters {
        &self.0
    }

    pub fn set_active(&mut self, active: bool) {
        let _ = self.0.active(active);
    }

    pub fn set_max_bitrate(&mut self, max_bitrate: u32) {
        let _ = self.0.max_bitrate(max_bitrate);
    }

    /// Sets `scale_resolution_down_by`.
    pub fn set_scale_resolution_down_by(
        &mut self,
        scale_resolution_down_by: f32,
    ) {
        let _ = self.0.scale_resolution_down_by(scale_resolution_down_by);
    }

    /// Sets `set_scalability_mode`.
    pub fn set_scalability_mode(&mut self, scalability_mode: ScalabilityMode) {
        let _ = self.0.scalability_mode(&scalability_mode.to_string());
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

        let mut enc = SendEncodingParameters::new(rid, active);

        if let Some(b) = max_bitrate {
            enc.set_max_bitrate(b.into());
        }
        if let Some(s) = scale_resolution_down_by {
            enc.set_scale_resolution_down_by(s.into());
        }

        enc
    }
}
