use std::ptr;

use medea_client_api_proto::stats::{
    KnownCandidateType, KnownProtocol, NonExhaustive, RtcIceCandidateStats,
};

use crate::{
    api::DartValueArg,
    platform::{
        rtc_stats::{CandidateType, Protocol},
        utils::NonNullDartValueArgExt,
    },
};

#[allow(missing_copy_implementations)]
#[derive(Debug)]
#[repr(C)]
pub struct IceCandidateFfiStats {
    /// Unique ID that is associated to the object that was inspected to
    /// produce the [RTCTransportFfiStats][1] associated with this candidate.
    ///
    /// [1]: https://w3.org/TR/webrtc-stats/#transportstats-dict%2A
    transport_id: ptr::NonNull<DartValueArg<Option<String>>>,

    /// Address of the candidate, allowing for IPv4 addresses, IPv6 addresses,
    /// and fully qualified domain names (FQDNs).
    address: ptr::NonNull<DartValueArg<Option<String>>>,

    /// Port number of the candidate.
    port: ptr::NonNull<DartValueArg<Option<i32>>>,

    /// Valid values for transport is one of `udp` and `tcp`.
    protocol: i32,

    /// Type of the ICE candidate.
    candidate_type: i32,

    /// Calculated as defined in [Section 15.1 of RFC 5245][1].
    ///
    /// [1]: https://tools.ietf.org/html/rfc5245#section-15.1
    priority: ptr::NonNull<DartValueArg<Option<i32>>>,

    /// For local candidates this is the URL of the ICE server from which the
    /// candidate was obtained. It is the same as the
    /// [url surfaced in the RTCPeerConnectionIceEvent][1].
    ///
    /// `None` for remote candidates.
    ///
    /// [1]: https://w3.org/TR/webrtc#rtcpeerconnectioniceevent
    url: ptr::NonNull<DartValueArg<Option<String>>>,

    /// Protocol used by the endpoint to communicate with the TURN server.
    ///
    /// Only present for local candidates.
    relay_protocol: ptr::NonNull<DartValueArg<Option<i32>>>,
}

// Type-erased value that can be transferred via Ffi boundaries to/from Dart.
#[allow(missing_copy_implementations, dead_code)] // not trivially copyable
#[derive(Debug)]
#[repr(u8)]
pub enum RTCIceCandidateFfiStats {
    /// Remote candidate variant.
    Remote(ptr::NonNull<IceCandidateFfiStats>),
    /// Remote candidate variant.
    Local(ptr::NonNull<IceCandidateFfiStats>),
}

#[allow(clippy::fallible_impl_from)]
impl From<IceCandidateFfiStats> for RtcIceCandidateStats {
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn from(stats: IceCandidateFfiStats) -> Self {
        let protocol = NonExhaustive::Known(KnownProtocol::from(
            Protocol::try_from(i64::from(
                Option::try_from(stats.protocol).unwrap().unwrap(),
            ))
            .unwrap(),
        ));

        let relay_protocol =
            Option::<i32>::try_from(unsafe { stats.relay_protocol.unbox() })
                .unwrap()
                .and_then(|v| Protocol::try_from(i64::from(v)).ok())
                .map(KnownProtocol::from)
                .map(NonExhaustive::Known);

        let candidate_type = NonExhaustive::Known(KnownCandidateType::from(
            CandidateType::try_from(i64::from(
                Option::try_from(stats.candidate_type).unwrap().unwrap(),
            ))
            .unwrap(),
        ));

        let port =
            Option::<i32>::try_from(unsafe { stats.port.unbox() }).unwrap();

        let priority =
            Option::<i32>::try_from(unsafe { stats.priority.unbox() })
                .unwrap()
                .unwrap() as u32;

        Self {
            transport_id: Option::try_from(unsafe {
                stats.transport_id.unbox()
            })
            .unwrap(),
            address: Option::try_from(unsafe { stats.address.unbox() })
                .unwrap(),
            port: port.unwrap() as u16,
            protocol,
            candidate_type,
            priority,
            url: Option::try_from(unsafe { stats.url.unbox() }).unwrap(),
            relay_protocol,
        }
    }
}
