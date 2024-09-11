//! Definition and implementation of [RTCRtpTransceiverDirection][1].
//!
//! [1]: https://w3.org/TR/webrtc#dom-rtcrtptransceiverdirection

use bitflags::bitflags;
use medea_client_api_proto::Direction as DirectionProto;
#[cfg(target_family = "wasm")]
use web_sys::RtcRtpTransceiverDirection;

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
        } else {
            Self::Inactive
        }
    }
}

impl From<&DirectionProto> for Direction {
    fn from(proto: &DirectionProto) -> Self {
        match proto {
            DirectionProto::Recv { .. } => Self::RECV,
            DirectionProto::Send { .. } => Self::SEND,
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
