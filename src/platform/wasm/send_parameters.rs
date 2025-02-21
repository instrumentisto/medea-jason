//! Representation of [RTCRtpSendParameters][0].
//!
//! [0]: https://www.w3.org/TR/webrtc/#dom-rtcrtpsendparameters

use derive_more::{From, Into};
use web_sys::{RtcRtpEncodingParameters, RtcRtpParameters};

use super::send_encoding_parameters::SendEncodingParameters;

/// Representation of [RTCRtpSendParameters][0].
///
/// [0]: https://www.w3.org/TR/webrtc/#dom-rtcrtpsendparameters
#[derive(Clone, Debug, From, Into)]
pub struct SendParameters(RtcRtpParameters);

impl SendParameters {
    /// Returns [`SendEncodingParameters`] of these [`SendParameters`].
    #[must_use]
    pub fn encodings(&self) -> Box<[SendEncodingParameters]> {
        let Some(current_encodings) = self.0.get_encodings() else {
            return Box::new([]);
        };

        current_encodings
            .into_iter()
            .map(|e| {
                SendEncodingParameters::from(RtcRtpEncodingParameters::from(e))
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}
