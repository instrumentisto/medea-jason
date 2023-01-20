use std::ptr;

use medea_client_api_proto::stats::{
    HighResTimeStamp, KnownIceCandidatePairState, NonExhaustive,
    RtcIceCandidatePairStats,
};

use crate::{
    api::DartValueArg,
    platform::{
        rtc_stats::RTCStatsIceCandidatePairState, utils::NonNullDartValueArgExt,
    },
};

#[allow(clippy::fallible_impl_from)]
impl From<RTCIceCandidatePairFfiStats> for RtcIceCandidatePairStats {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn from(stats: RTCIceCandidatePairFfiStats) -> Self {
        Self {
            state: NonExhaustive::Known(KnownIceCandidatePairState::from(
                RTCStatsIceCandidatePairState::try_from(i64::from(
                    Option::try_from(stats.state).unwrap().unwrap(),
                ))
                .unwrap(),
            )),
            nominated: Option::try_from(unsafe { stats.nominated.unbox() })
                .unwrap()
                .unwrap(),
            bytes_sent: Option::try_from(unsafe { stats.bytes_sent.unbox() })
                .unwrap()
                .unwrap(),
            bytes_received: Option::try_from(unsafe {
                stats.bytes_received.unbox()
            })
            .unwrap()
            .unwrap(),
            total_round_trip_time: Option::try_from(unsafe {
                stats.total_round_trip_time.unbox()
            })
            .unwrap()
            .map(HighResTimeStamp),
            current_round_trip_time: Option::try_from(unsafe {
                stats.current_round_trip_time.unbox()
            })
            .unwrap()
            .map(HighResTimeStamp),
            available_outgoing_bitrate: Option::<f64>::try_from(unsafe {
                stats.available_outgoing_bitrate.unbox()
            })
            .unwrap()
            .map(|v| v as u64),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct RTCIceCandidatePairFfiStats {
    /// State of the checklist for the local
    /// and remote candidates in a pair.
    state: i32,

    /// Related to updating the nominated flag described in
    /// [Section 7.1.3.2.4 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-7.1.3.2.4
    nominated: ptr::NonNull<DartValueArg<Option<bool>>>,

    /// Total number of payload bytes sent on this candidate pair, i.e. not
    /// including headers or padding.
    bytes_sent: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Total number of payload bytes received on this candidate pair, i.e.
    /// not including headers or padding.
    bytes_received: ptr::NonNull<DartValueArg<Option<u64>>>,

    /// Sum of all round trip time measurements in seconds since
    /// the beginning of the session,
    /// based on STUN connectivity check [STUN-PATH-CHAR]
    /// responses (responsesReceived), including those that reply
    /// to requests that are sent in order to verify consent [RFC 7675].
    ///
    /// The average round trip time can be computed from
    /// [`totalRoundTripTime`][1] by dividing it
    /// by [`responsesReceived`][2].
    ///
    /// [STUN-PATH-CHAR]: https://w3.org/TR/webrtc-stats/#bib-stun-path-char
    /// [RFC 7675]: https://tools.ietf.org/html/rfc7675
    /// [1]: https://tinyurl.com/tgr543a
    /// [2]: https://tinyurl.com/r3zo2um
    total_round_trip_time: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Latest round trip time measured in seconds, computed from both STUN
    /// connectivity checks [STUN-PATH-CHAR],
    /// including those that are sent for consent verification [RFC 7675].
    ///
    /// [STUN-PATH-CHAR]: https://w3.org/TR/webrtc-stats/#bib-stun-path-char
    /// [RFC 7675]: https://tools.ietf.org/html/rfc7675
    current_round_trip_time: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Calculated by the underlying congestion control by combining the
    /// available bitrate for all the outgoing RTP streams using
    /// this candidate pair.
    /// The bitrate measurement does not count the size of the IP or
    /// other transport layers like TCP or UDP. It is similar to the TIAS
    /// defined in [RFC 3890], i.e. it is measured in bits per second and
    /// the bitrate is calculated over a 1 second window.
    ///
    /// Implementations that do not calculate a sender-side estimate
    /// MUST leave this undefined. Additionally, the value MUST be undefined
    /// for candidate pairs that were never used. For pairs in use,
    /// the estimate is normally
    /// no lower than the bitrate for the packets sent at
    /// [`lastPacketSentTimestamp`][1], but might be higher. For candidate
    /// pairs that are not currently in use but were used before,
    /// implementations MUST return undefined.
    ///
    /// [RFC 3890]: https://tools.ietf.org/html/rfc3890
    /// [1]: https://tinyurl.com/rfc72eh
    available_outgoing_bitrate: ptr::NonNull<DartValueArg<Option<f64>>>,
}
