use web_sys::RtcRtpEncodingParameters;

use medea_client_api_proto::ScalabilityMode;

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
        self.0.active(active);
    }

    pub fn set_max_bitrate(&mut self, max_bitrate: u32) {
        self.0.max_bitrate(max_bitrate);
    }

    /// Sets `scale_resolution_down_by`.
    pub fn set_scale_resolution_down_by(
        &mut self,
        scale_resolution_down_by: f32,
    ) {
        self.0.scale_resolution_down_by(scale_resolution_down_by);
    }

    /// Sets `set_scalability_mode`.
    pub fn set_scalability_mode(&mut self, scalability_mode: ScalabilityMode) {
        self.0.scalability_mode(&scalability_mode.to_string());
    }
}
