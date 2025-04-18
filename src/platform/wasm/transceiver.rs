//! [`RtcRtpTransceiver`] wrapper.

use std::rc::Rc;

use derive_more::with_trait::From;
use js_sys::Reflect;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RtcRtpParameters, RtcRtpTransceiver, RtcRtpTransceiverInit};

use crate::{
    media::track::local,
    platform::{
        self, TransceiverDirection,
        send_encoding_parameters::SendEncodingParameters,
        send_parameters::SendParameters,
        wasm::codec_capability::CodecCapability,
    },
};

/// Wrapper around an [`RtcRtpTransceiverInit`].
#[derive(Debug)]
pub struct TransceiverInit(RtcRtpTransceiverInit);

impl TransceiverInit {
    /// Creates a new [`TransceiverInit`].
    #[must_use]
    pub fn new(direction: TransceiverDirection) -> Self {
        let init = RtcRtpTransceiverInit::new();
        init.set_direction(direction.into());
        Self(init)
    }

    /// Returns underlying [`RtcRtpTransceiverInit`].
    #[must_use]
    pub const fn handle(&self) -> &RtcRtpTransceiverInit {
        &self.0
    }

    /// Adds the provided [`SendEncodingParameters`] to this
    /// [`TransceiverInit`].
    pub fn set_send_encodings(&self, encodings: Vec<SendEncodingParameters>) {
        let send_encoding = ::js_sys::Array::new();
        for enc in encodings {
            _ = send_encoding.push(enc.handle());
        }
        self.0.set_send_encodings(&send_encoding);
    }
}

/// Wrapper around [`RtcRtpTransceiver`] which provides handy methods for
/// direction changes.
#[derive(Clone, Debug, From)]
pub struct Transceiver(RtcRtpTransceiver);

impl Transceiver {
    /// Returns current [`TransceiverDirection`] of this [`Transceiver`].
    fn direction(&self) -> TransceiverDirection {
        TransceiverDirection::from(self.0.direction())
    }

    /// Changes the receive direction of this [`Transceiver`].
    pub fn set_recv(
        &self,
        active: bool,
    ) -> impl Future<Output = ()> + 'static + use<> {
        let transceiver = self.0.clone();
        async move {
            let current_direction =
                TransceiverDirection::from(transceiver.direction());
            let new_direction = if active {
                current_direction | TransceiverDirection::RECV
            } else {
                current_direction - TransceiverDirection::RECV
            };
            transceiver.set_direction(new_direction.into());
        }
    }

    /// Changes the send direction of this [`Transceiver`].
    pub fn set_send(
        &self,
        active: bool,
    ) -> impl Future<Output = ()> + 'static + use<> {
        let transceiver = self.0.clone();
        async move {
            let current_direction =
                TransceiverDirection::from(transceiver.direction());
            let new_direction = if active {
                current_direction | TransceiverDirection::SEND
            } else {
                current_direction - TransceiverDirection::SEND
            };
            transceiver.set_direction(new_direction.into());
        }
    }

    /// Indicates whether the provided [`TransceiverDirection`] is enabled for
    /// this [`Transceiver`].
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn has_direction(&self, direction: TransceiverDirection) -> bool {
        self.direction().contains(direction)
    }

    /// Replaces [`TransceiverDirection::SEND`] [`local::Track`] of this
    /// [`Transceiver`].
    ///
    /// # Errors
    ///
    /// Errors with JS error if the underlying [`replaceTrack`][1] call fails.
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpsender-replacetrack
    pub async fn set_send_track(
        &self,
        new_track: Option<&Rc<local::Track>>,
    ) -> Result<(), platform::Error> {
        drop(
            JsFuture::from(self.0.sender().replace_track(
                new_track.map(|track| (**track).as_ref().as_ref()),
            ))
            .await?,
        );
        Ok(())
    }

    /// Returns [`mid`] of this [`Transceiver`].
    ///
    /// [`mid`]: https://w3.org/TR/webrtc#dom-rtptransceiver-mid
    #[must_use]
    pub fn mid(&self) -> Option<String> {
        self.0.mid()
    }

    /// Indicates whether the underlying [`RtcRtpTransceiver`] is stopped.
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        self.0.stopped()
    }

    /// Returns [`SendParameters`] of the underlying [RTCRtpSender].
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#rtcrtpsender-interface
    #[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
    pub async fn get_send_parameters(&self) -> SendParameters {
        SendParameters::from(self.0.sender().get_parameters())
    }

    /// Sets [`SendParameters`] into the underlying [RTCRtpSender].
    ///
    /// # Errors
    ///
    /// With [`platform::Error`] if the underlying [setParameters()][1] call
    /// fails.
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#rtcrtpsender-interface
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpsender-setparameters
    pub async fn set_send_parameters(
        &self,
        params: SendParameters,
    ) -> Result<(), platform::Error> {
        drop(
            JsFuture::from(self.0.sender().set_parameters_with_parameters(
                &RtcRtpParameters::from(params),
            ))
            .await?,
        );

        Ok(())
    }

    /// Sets the preferred [`CodecCapability`]s for this [`Transceiver`].
    pub fn set_codec_preferences(&self, codecs: Vec<CodecCapability>) {
        let is_api_available =
            Reflect::get(&self.0, &JsValue::from_str("setCodecPreferences"))
                .map_or(None, |val| (!val.is_undefined()).then_some(val))
                .is_some();

        // Unsupported on Firefox < 128.
        if is_api_available {
            let arr = ::js_sys::Array::new();
            for codec in codecs {
                _ = arr.push(codec.handle());
            }
            self.0.set_codec_preferences(&arr);
        }
    }
}

#[cfg(test)]
mod tests {
    use web_sys::RtcRtpTransceiverDirection;

    use super::TransceiverDirection;

    #[test]
    fn enable_works_correctly() {
        use TransceiverDirection as D;

        for (init, enable_dir, result) in [
            (D::INACTIVE, D::SEND, D::SEND),
            (D::INACTIVE, D::RECV, D::RECV),
            (D::SEND, D::RECV, D::RECV | D::SEND),
            (D::RECV, D::SEND, D::RECV | D::SEND),
        ] {
            assert_eq!(init | enable_dir, result);
        }
    }

    #[test]
    fn disable_works_correctly() {
        use TransceiverDirection as D;

        for (init, disable_dir, result) in [
            (D::SEND, D::SEND, D::INACTIVE),
            (D::RECV, D::RECV, D::INACTIVE),
            (D::RECV | D::SEND, D::SEND, D::RECV),
            (D::RECV | D::SEND, D::RECV, D::SEND),
        ] {
            assert_eq!(init - disable_dir, result);
        }
    }

    #[test]
    fn from_trnscvr_direction_to_sys() {
        use RtcRtpTransceiverDirection as S;
        use TransceiverDirection as D;

        for (trnscvr_dir, sys_dir) in [
            (D::SEND, S::Sendonly),
            (D::RECV, S::Recvonly),
            (D::RECV | D::SEND, S::Sendrecv),
            (D::INACTIVE, S::Inactive),
        ] {
            assert_eq!(S::from(trnscvr_dir), sys_dir);
        }
    }

    #[test]
    fn from_sys_direction_to_trnscvr() {
        use RtcRtpTransceiverDirection as S;
        use TransceiverDirection as D;

        for (sys_dir, trnscvr_dir) in [
            (S::Sendonly, D::SEND),
            (S::Recvonly, D::RECV),
            (S::Sendrecv, D::RECV | D::SEND),
            (S::Inactive, D::INACTIVE),
        ] {
            assert_eq!(D::from(sys_dir), trnscvr_dir);
        }
    }
}
