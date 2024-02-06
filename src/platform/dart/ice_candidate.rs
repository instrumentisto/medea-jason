//! ICE candidate of a [RTCPeerConnection][1].
//!
//! [1]: https://w3.org/TR/webrtc#dom-rtcpeerconnection

use dart_sys::Dart_Handle;
use derive_more::From;
use medea_macro::dart_bridge;

use crate::platform::dart::utils::{dart_string_into_rust, handle::DartHandle};

#[dart_bridge("flutter/lib/src/native/platform/ice_candidate.g.dart")]
mod ice_candidate {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::api::DartValueArg;

    extern "C" {
        /// Creates a new [`IceCandidate`] with the provided parameters.
        pub fn init(
            candidate: DartValueArg<String>,
            sdp_mid: DartValueArg<Option<String>>,
            sdp_m_line_index: DartValueArg<Option<u16>>,
        ) -> Dart_Handle;

        /// Returns candidate of the provided [`IceCandidate`].
        pub fn candidate(ice_candidate: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns SDP line index of the provided [`IceCandidate`].
        pub fn sdp_m_line_index(ice_candidate: Dart_Handle) -> u64;

        /// Returns SDP MID of the provided [`IceCandidate`].
        pub fn sdp_mid(ice_candidate: Dart_Handle) -> ptr::NonNull<c_char>;
    }
}

#[dart_bridge("flutter/lib/src/native/platform/ice_candidate_error.g.dart")]
mod ice_candidate_error {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    extern "C" {
        /// Returns local IP address used to communicate with a STUN or TURN
        /// server.
        pub fn address(error: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns STUN or TURN URL identifying the STUN or TURN server for
        /// which the failure occurred.
        pub fn url(error: Dart_Handle) -> ptr::NonNull<c_char>;

        /// Returns numeric STUN error code returned by the STUN or TURN server.
        /// If no host candidate can reach the server, `errorCode` will
        /// be set to the value 701 which is outside the STUN error code
        /// range. This error is only fired once per server URL while in
        /// the `RTCIceGatheringState` of "gathering".
        pub fn error_code(error: Dart_Handle) -> i32;

        /// Returns STUN reason text returned by the STUN or TURN server. If the
        /// server could not be reached, `errorText` will be set to an
        /// implementation-specific value providing details about the
        /// error.
        pub fn error_text(error: Dart_Handle) -> ptr::NonNull<c_char>;
    }
}

/// Description of the error occurred with ICE candidate from a peer connection.
#[derive(Debug, From)]
pub struct IceCandidateError(DartHandle);

impl IceCandidateError {
    /// Local IP address used to communicate with a STUN or TURN server.
    pub fn address(&self) -> String {
        let address = unsafe { ice_candidate_error::address(self.0.get()) };
        unsafe { dart_string_into_rust(address) }
    }

    /// STUN or TURN URL identifying the STUN or TURN server for which the
    /// failure occurred.
    pub fn url(&self) -> String {
        let url = unsafe { ice_candidate_error::url(self.0.get()) };
        unsafe { dart_string_into_rust(url) }
    }

    /// Numeric STUN error code returned by the STUN or TURN server. If no host
    /// candidate can reach the server, `errorCode` will be set to the value 701
    /// which is outside the STUN error code range. This error is only fired
    /// once per server URL while in the `RTCIceGatheringState` of
    /// "gathering".
    pub fn error_code(&self) -> i32 {
        unsafe {
            ice_candidate_error::error_code(self.0.get())
                .try_into()
                .unwrap()
        }
    }

    /// STUN reason text returned by the STUN or TURN server. If the server
    /// could not be reached, `errorText` will be set to an
    /// implementation-specific value providing details about the error.
    pub fn error_text(&self) -> String {
        let error_text =
            unsafe { ice_candidate_error::error_text(self.0.get()) };
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
        };
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
        let candidate = unsafe { ice_candidate::candidate(self.0.get()) };
        unsafe { dart_string_into_rust(candidate) }
    }

    /// Returns SDP M line index of this [`IceCandidate`].
    #[allow(clippy::unwrap_in_result)]
    #[must_use]
    pub fn sdp_m_line_index(&self) -> Option<u16> {
        Some(unsafe {
            ice_candidate::sdp_m_line_index(self.0.get())
                .try_into()
                .unwrap()
        })
    }

    /// Returns SDP MID of this [`IceCandidate`].
    #[must_use]
    pub fn sdp_mid(&self) -> Option<String> {
        let mid = unsafe { ice_candidate::sdp_mid(self.0.get()) };
        Some(unsafe { dart_string_into_rust(mid) })
    }
}
