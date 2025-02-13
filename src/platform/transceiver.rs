//! Definition and implementation of [RTCRtpTransceiverDirection][1].
//!
//! [1]: https://w3.org/TR/webrtc#dom-rtcrtptransceiverdirection

use bitflags::bitflags;
use derive_more::{Display, From};
use medea_client_api_proto as proto;
#[cfg(target_family = "wasm")]
use web_sys::RtcRtpTransceiverDirection;

use crate::{media::MediaKind, platform, platform::Transceiver};

bitflags! {
    /// Representation of [RTCRtpTransceiverDirection][1].
    ///
    /// [`sendrecv` direction][2] can be represented by
    /// [`Direction::SEND`]` | `[`Direction::RECV`] bitflag.
    ///
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtptransceiverdirection
    /// [2]: https://w3.org/TR/webrtc#dom-rtcrtptransceiverdirection-sendrecv
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Direction: u8 {
        /// [`inactive` direction][1] of transceiver.
        ///
        /// [1]: https://tinyurl.com/y2zslyw2
        const INACTIVE = 0b000;

        /// [`sendonly` direction][1] of transceiver.
        ///
        /// [1]: https://tinyurl.com/y6y2ye97
        const SEND = 0b001;

        /// [`recvonly` direction][1] of transceiver.
        ///
        /// [1]: https://tinyurl.com/y2nlxpzf
        const RECV = 0b010;

        /// [`stopped` direction][1] of transceiver.
        ///
        /// [1]: https://tinyurl.com/39ddy5z2
        const STOPPED = 0b100;
    }
}

#[expect(clippy::allow_attributes, reason = "`#[expect]` is not considered")]
#[allow(clippy::multiple_inherent_impl, reason = "multiplatform structure")]
impl Transceiver {
    /// Updates parameters of `encodings` for the underlying [RTCRtpSender] of
    /// this [`Transceiver`].
    ///
    /// # Errors
    ///
    /// With [`platform::Error`] if the underlying [setParameters()][1] call
    /// fails.
    ///
    /// [RTCRtpSender]: https://w3.org/TR/webrtc#dom-rtcrtpsender
    /// [1]: https://w3.org/TR/webrtc#dom-rtcrtpsender-setparameters
    pub async fn update_send_encodings(
        &self,
        updated_encodings: &[proto::EncodingParameters],
    ) -> Result<(), UpdateSendEncodingError> {
        let params = self.get_send_parameters().await;
        let current_encodings = params.encodings();

        if updated_encodings.len() != current_encodings.len() {
            // No encodings can be removed or added via setParameters
            // according to spec:
            // https://www.w3.org/TR/webrtc/#dom-rtcrtpsender-setparameters
            return Err(UpdateSendEncodingError::EncodingsLengthsMismatch {
                current: current_encodings.len(),
                new: updated_encodings.len(),
            });
        }

        for (i, enc) in current_encodings.iter().enumerate() {
            let updated_enc = &updated_encodings[i];
            // Not updating RID cause spec:
            // RID is not modifiable via setParameters. It can only be set or
            // modified in addTransceiver on the sending side.
            // https://www.w3.org/TR/webrtc/#dom-rtcrtpcodingparameters-rid

            enc.set_active(updated_enc.active);
            if let Some(max_bitrate) = updated_enc.max_bitrate {
                enc.set_max_bitrate(max_bitrate);
            }
            if let Some(scale_resolution_down_by) =
                updated_enc.scale_resolution_down_by
            {
                enc.set_scale_resolution_down_by(
                    scale_resolution_down_by.into(),
                );
            }
            if let Some(scalability_mode) = updated_enc.scalability_mode {
                enc.set_scalability_mode(scalability_mode.to_string());
            }
        }

        self.set_send_parameters(params).await?;

        Ok(())
    }
}

/// [`Transceiver::update_send_encodings`] error.
#[derive(Clone, Debug, Display, From)]
pub enum UpdateSendEncodingError {
    /// [`EncodingParameters`] list cannot be modified via
    /// [`Transceiver::set_send_parameters`].
    #[display(
        "SendParameters.encodings length can not be changed. Tried to \
        change from {current} to {new}"
    )]
    EncodingsLengthsMismatch {
        /// Number of [`proto::EncodingParameters`] stored in sender.
        current: usize,

        /// Length of updated [`proto::EncodingParameters`] list.
        new: usize,
    },

    /// [RTCRtpSender.setParameters][0] error.
    ///
    /// [0]: https://www.w3.org/TR/webrtc/#dom-rtcrtpsender-setparameters
    #[display("RTCRtpSender.set_parameters error: {_0:?}")]
    SetSenderParameters(platform::Error),
}

/// Constructs codec preferences list based on the provided target
/// [`proto::Codec`]s.
pub async fn probe_target_codecs(
    target_codecs: impl IntoIterator<Item = &proto::Codec>,
) -> Option<Vec<platform::CodecCapability>> {
    /// List of required "codecs" for every [`MediaKind::Video`] of a
    /// [`platform::Transceiver`].
    const REQUIRED_CODECS: [&str; 3] =
        ["video/rtx", "video/red", "video/ulpfec"];

    const DEFAULT_PARAMS: [(&str, &str); 6] = [
        ("profile-id", "0"), // VP9
        ("packetization-mode", "0"),
        ("profile-level-id", "42001f"), // H264
        ("profile", "0"),
        ("level-idx", "5"),
        ("tier", "0"), // AV1
    ];

    let caps = platform::CodecCapability::get_sender_codec_capabilities(
        MediaKind::Video,
    )
    .await
    .ok()?;

    let mut result = Vec::new();
    for target in target_codecs {
        'cap: for cap in &caps {
            if cap.mime_type() != target.mime_type
                || cap.channels() != target.channels
                || cap.clock_rate() != target.clock_rate
            {
                continue;
            }

            let cap_params = cap.parameters();

            #[expect(
                clippy::iter_over_hash_type,
                reason = "order doesn't matter"
            )]
            for (k, v) in &target.parameters {
                if cap_params.get(k) != Some(v)
                    && !DEFAULT_PARAMS.iter().any(|(dk, dv)| k == dk && v == dv)
                {
                    continue 'cap;
                }
            }

            result.push(cap.clone());
        }
    }
    if result.is_empty() {
        None
    } else {
        for cap in caps {
            if REQUIRED_CODECS.contains(&cap.mime_type().as_str()) {
                result.push(cap);
            }
        }

        Some(result)
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<Direction> for i64 {
    fn from(from: Direction) -> Self {
        use Direction as D;

        if from.contains(D::STOPPED) {
            4
        } else if from.contains(D::SEND) && from.contains(D::RECV) {
            0
        } else if from.contains(D::SEND) {
            1
        } else if from.contains(D::RECV) {
            2
        } else {
            3
        }
    }
}

#[cfg(target_family = "wasm")]
impl From<RtcRtpTransceiverDirection> for Direction {
    fn from(direction: RtcRtpTransceiverDirection) -> Self {
        use RtcRtpTransceiverDirection as D;

        match direction {
            D::Sendonly => Self::SEND,
            D::Recvonly => Self::RECV,
            D::Inactive => Self::INACTIVE,
            D::Sendrecv => Self::SEND | Self::RECV,
            D::Stopped => Self::STOPPED,
            _ => {
                unreachable!("unexpected transceiver direction")
            }
        }
    }
}

#[cfg(target_family = "wasm")]
impl From<Direction> for RtcRtpTransceiverDirection {
    #[inline]
    fn from(direction: Direction) -> Self {
        use Direction as D;

        if direction.contains(D::RECV) && direction.contains(D::SEND) {
            Self::Sendrecv
        } else if direction.contains(D::RECV) {
            Self::Recvonly
        } else if direction.contains(D::SEND) {
            Self::Sendonly
        } else if direction.contains(D::STOPPED) {
            Self::Stopped
        } else {
            Self::Inactive
        }
    }
}

impl From<&proto::Direction> for Direction {
    fn from(proto: &proto::Direction) -> Self {
        match proto {
            proto::Direction::Recv { .. } => Self::RECV,
            proto::Direction::Send { .. } => Self::SEND,
        }
    }
}

impl From<i32> for Direction {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::SEND | Self::RECV,
            1 => Self::SEND,
            2 => Self::RECV,
            3 => Self::INACTIVE,
            4 => Self::STOPPED,
            _ => {
                unreachable!("Unknown `Direction` enum variant {i}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_family = "wasm")]
    use web_sys::RtcRtpTransceiverDirection as S;

    use super::Direction as D;

    #[test]
    fn enable_works_correctly() {
        for (init, enable_dir, result) in [
            (D::INACTIVE, D::SEND, D::SEND),
            (D::INACTIVE, D::RECV, D::RECV),
            (D::SEND, D::RECV, D::SEND | D::RECV),
            (D::RECV, D::SEND, D::SEND | D::RECV),
            (D::RECV, D::STOPPED, D::STOPPED),
            (D::SEND, D::STOPPED, D::STOPPED),
            (D::STOPPED, D::RECV, D::STOPPED),
            (D::STOPPED, D::SEND, D::STOPPED),
        ] {
            assert_eq!(init | enable_dir, result);
        }
    }

    #[test]
    fn disable_works_correctly() {
        for (init, disable_dir, result) in [
            (D::SEND, D::SEND, D::INACTIVE),
            (D::RECV, D::RECV, D::INACTIVE),
            (D::SEND | D::RECV, D::SEND, D::RECV),
            (D::SEND | D::RECV, D::RECV, D::SEND),
            (D::STOPPED, D::RECV, D::STOPPED),
            (D::STOPPED, D::SEND, D::STOPPED),
        ] {
            assert_eq!(init - disable_dir, result);
        }
    }

    #[cfg(target_family = "wasm")]
    #[test]
    fn from_trnscvr_direction_to_sys() {
        for (trnscvr_dir, sys_dir) in [
            (D::SEND, S::Sendonly),
            (D::RECV, S::Recvonly),
            (D::SEND | D::RECV, S::Sendrecv),
            (D::INACTIVE, S::Inactive),
        ] {
            assert_eq!(S::from(trnscvr_dir), sys_dir);
        }
    }

    #[cfg(target_family = "wasm")]
    #[test]
    fn from_sys_direction_to_trnscvr() {
        for (sys_dir, trnscvr_dir) in [
            (S::Sendonly, D::SEND),
            (S::Recvonly, D::RECV),
            (S::Sendrecv, D::SEND | D::RECV),
            (S::Inactive, D::INACTIVE),
        ] {
            assert_eq!(D::from(sys_dir), trnscvr_dir);
        }
    }
}
