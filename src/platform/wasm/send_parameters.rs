//! Representation of [RTCRtpSendParameters][0].
//!
//! [0]: https://www.w3.org/TR/webrtc/#dom-rtcrtpsendparameters

use derive_more::{From, Into};
use js_sys::Array;
use web_sys::{RtcRtpEncodingParameters, RtcRtpParameters};

use super::send_encoding_parameters::SendEncodingParameters;

/// Representation of [RTCRtpSendParameters][0].
///
/// [0]: https://www.w3.org/TR/webrtc/#dom-rtcrtpsendparameters
#[derive(Clone, Debug, From, Into)]
pub struct SendParameters(RtcRtpParameters);

impl SendParameters {
    /// Returns [`SendEncodingParameters`] of these [`SendParameters`].
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn encodings(&self) -> Vec<SendEncodingParameters> {
        let Some(current_encodings) = self.0.get_encodings() else {
            return Vec::new();
        };

        current_encodings
            .into_iter()
            .map(|e| {
                SendEncodingParameters::from(RtcRtpEncodingParameters::from(e))
            })
            .collect()
    }

    /// Sets the provided [`SendEncodingParameters`] into these
    /// [`SendParameters`].
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn set_encodings(&self, encodings: Vec<SendEncodingParameters>) {
        let js_array = Array::new();

        for enc in encodings {
            _ = js_array.push(&RtcRtpEncodingParameters::from(enc));
        }

        self.0.set_codecs(&js_array);
    }
}
