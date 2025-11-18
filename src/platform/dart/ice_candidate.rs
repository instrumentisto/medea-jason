//! ICE candidate of a [RTCPeerConnection][1].
//!
//! [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnection

use dart_sys::Dart_Handle;
use derive_more::with_trait::From;
use medea_macro::dart_bridge;

use crate::platform::dart::utils::{dart_string_into_rust, handle::DartHandle};

#[dart_bridge("flutter/lib/src/native/platform/ice_candidate.g.dart")]
mod ice_candidate {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::Error};

    extern "C" {
        /// Creates a new [`IceCandidate`] with the provided parameters.
        pub fn init(
            candidate: DartValueArg<String>,
            sdp_mid: DartValueArg<Option<String>>,
            sdp_m_line_index: DartValueArg<Option<u16>>,
        ) -> Result<Dart_Handle, Error>;

        /// Returns candidate of the provided [`IceCandidate`].
        pub fn candidate(
            ice_candidate: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;

        /// Returns SDP line index of the provided [`IceCandidate`].
        pub fn sdp_m_line_index(
            ice_candidate: Dart_Handle,
        ) -> Result<u64, Error>;

        /// Returns SDP MID of the provided [`IceCandidate`].
        pub fn sdp_mid(
            ice_candidate: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;
    }
}

#[dart_bridge("flutter/lib/src/native/platform/ice_candidate_error.g.dart")]
mod ice_candidate_error {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::platform::Error;

    extern "C" {
        /// Returns the local IP address used to communicate with a
        /// [STUN]/[TURN] server.
        ///
        /// [STUN]: https://webrtcglossary.com/stun
        /// [TURN]: https://webrtcglossary.com/turn
        pub fn address(
            error: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;

        /// Returns the port used to communicate with a [STUN]/[TURN] server.
        ///
        /// [STUN]: https://webrtcglossary.com/stun
        /// [TURN]: https://webrtcglossary.com/turn
        pub fn port(error: Dart_Handle) -> Result<u32, Error>;

        /// Returns the URL identifying the [STUN]/[TURN] server for which the
        /// failure occurred.
        ///
        /// [STUN]: https://webrtcglossary.com/stun
        /// [TURN]: https://webrtcglossary.com/turn
        pub fn url(error: Dart_Handle) -> Result<ptr::NonNull<c_char>, Error>;

        /// Returns the Numeric [STUN] error code returned by the [STUN]/[TURN]
        /// server.
        ///
        /// If no host candidate can reach the server, this error code will be
        /// set to the value `701`, which is outside the [STUN] error code
        /// range. This error is only fired once per server URL while in the
        /// `RTCIceGatheringState` of "gathering".
        ///
        /// [STUN]: https://webrtcglossary.com/stun
        /// [TURN]: https://webrtcglossary.com/turn
        pub fn error_code(error: Dart_Handle) -> Result<i32, Error>;

        /// [STUN] reason text returned by the [STUN]/[TURN] server.
        ///
        /// If the server could not be reached, this reason test will be set to
        /// an implementation-specific value providing details about the error.
        ///
        /// [STUN]: https://webrtcglossary.com/stun
        /// [TURN]: https://webrtcglossary.com/turn
        pub fn error_text(
            error: Dart_Handle,
        ) -> Result<ptr::NonNull<c_char>, Error>;
    }
}

/// Error occurred with an [ICE] candidate from a [`PeerConnection`].
///
/// [`PeerConnection`]: crate::peer::PeerConnection
/// [ICE]: https://webrtcglossary.com/ice
#[derive(Debug, From)]
pub struct IceCandidateError(DartHandle);

impl IceCandidateError {
    /// Returns the local IP address used to communicate with a [STUN]/[TURN]
    /// server.
    ///
    /// [STUN]: https://webrtcglossary.com/stun
    /// [TURN]: https://webrtcglossary.com/turn
    #[must_use]
    pub fn address(&self) -> String {
        let address =
            unsafe { ice_candidate_error::address(self.0.get()) }.unwrap();
        unsafe { dart_string_into_rust(address) }
    }

    /// Returns the port used to communicate with a [STUN]/[TURN] server.
    ///
    /// [STUN]: https://webrtcglossary.com/stun
    /// [TURN]: https://webrtcglossary.com/turn
    #[must_use]
    pub fn port(&self) -> u32 {
        unsafe { ice_candidate_error::port(self.0.get()) }.unwrap()
    }

    /// Returns the URL identifying the [STUN]/[TURN] server for which the
    /// failure occurred.
    ///
    /// [STUN]: https://webrtcglossary.com/stun
    /// [TURN]: https://webrtcglossary.com/turn
    #[must_use]
    pub fn url(&self) -> String {
        let url = unsafe { ice_candidate_error::url(self.0.get()) }.unwrap();
        unsafe { dart_string_into_rust(url) }
    }

    /// Returns the Numeric [STUN] error code returned by the [STUN]/[TURN]
    /// server.
    ///
    /// If no host candidate can reach the server, this error code will be set
    /// to the value `701`, which is outside the [STUN] error code range. This
    /// error is only fired once per server URL while in the
    /// `RTCIceGatheringState` of "gathering".
    ///
    /// [STUN]: https://webrtcglossary.com/stun
    /// [TURN]: https://webrtcglossary.com/turn
    #[must_use]
    pub fn error_code(&self) -> i32 {
        unsafe { ice_candidate_error::error_code(self.0.get()) }.unwrap()
    }

    /// [STUN] reason text returned by the [STUN]/[TURN] server.
    ///
    /// If the server could not be reached, this reason test will be set to an
    /// implementation-specific value providing details about the error.
    ///
    /// [STUN]: https://webrtcglossary.com/stun
    /// [TURN]: https://webrtcglossary.com/turn
    #[must_use]
    pub fn error_text(&self) -> String {
        let error_text =
            unsafe { ice_candidate_error::error_text(self.0.get()) }.unwrap();
        unsafe { dart_string_into_rust(error_text) }
    }
}

/// Wrapper around a [`DartHandle`] representing an ICE candidate of a
/// [RTCPeerConnection][1].
///
/// [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
#[derive(Debug, From)]
pub struct IceCandidate(DartHandle);

impl IceCandidate {
    /// Returns a new [`IceCandidate`] with the provided parameters.
    #[must_use]
    pub fn new(
        candidate: &str,
        sdp_m_line_index: Option<u16>,
        sdp_mid: &Option<String>,
    ) -> Self {
        let handle = unsafe {
            ice_candidate::init(
                candidate.to_owned().into(),
                sdp_mid.clone().into(),
                sdp_m_line_index.map(i64::from).into(),
            )
        }
        .unwrap();
        Self(unsafe { DartHandle::new(handle) })
    }

    /// Returns the underlying [`Dart_Handle`] of this [`IceCandidate`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }

    /// Returns candidate of this [`IceCandidate`].
    #[must_use]
    pub fn candidate(&self) -> String {
        let candidate =
            unsafe { ice_candidate::candidate(self.0.get()) }.unwrap();
        unsafe { dart_string_into_rust(candidate) }
    }

    /// Returns SDP M line index of this [`IceCandidate`].
    #[must_use]
    pub fn sdp_m_line_index(&self) -> Option<u16> {
        Some(unsafe {
            ice_candidate::sdp_m_line_index(self.0.get())
                .unwrap()
                .try_into()
                .unwrap()
        })
    }

    /// Returns SDP MID of this [`IceCandidate`].
    #[must_use]
    pub fn sdp_mid(&self) -> Option<String> {
        let mid = unsafe { ice_candidate::sdp_mid(self.0.get()) }.unwrap();
        Some(unsafe { dart_string_into_rust(mid) })
    }
}
