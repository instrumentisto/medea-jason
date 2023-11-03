//! [`RtcRtpTransceiver`] wrapper.

use std::{future::Future, rc::Rc};

use derive_more::From;
use js_sys::Array;
use medea_client_api_proto::EncodingParameters;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RtcRtpEncodingParameters, RtcRtpTransceiver};

use crate::{
    media::track::local,
    platform::{Error, TransceiverDirection},
};

use super::get_property_by_name;

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
    pub fn set_recv(&self, active: bool) -> impl Future<Output = ()> + 'static {
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
    pub fn set_send(&self, active: bool) -> impl Future<Output = ()> + 'static {
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
    #[allow(clippy::unused_async)] // for platform code uniformity
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
    ) -> Result<(), Error> {
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

    pub async fn update_send_encodings(
        &self,
        encodings: Vec<EncodingParameters>,
    ) -> Result<(), Error>{
        let mut params = self.0.sender().get_parameters();
        let encs = get_property_by_name(&params, "encodings", |v| {
            v.is_array().then_some(Array::from(&v))
        })
        .unwrap();

        for mut enc in encs.iter().map(RtcRtpEncodingParameters::from) {
            let rid = get_property_by_name(&enc, "rid", |v| v.as_string())
                .unwrap();

            let Some(encoding) = encodings.iter().find(|e| e.rid == rid) else {continue;};
            
            enc.active(encoding.active);
            if let Some(max_bitrate) = encoding.max_bitrate {
                enc.max_bitrate(max_bitrate);
            }
            if let Some(scale_resolution_down_by) = encoding.scale_resolution_down_by {
                enc.scale_resolution_down_by(scale_resolution_down_by.into());
            }
        }

        drop(JsFuture::from(self.0.sender().set_parameters_with_parameters(&params))
            .await?);

        Ok(())
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
