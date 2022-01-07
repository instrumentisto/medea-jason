//! ICE candidate of a [RTCPeerConnection][1].
//!
//! [1]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection

use dart_sys::Dart_Handle;
use derive_more::From;
use medea_macro::dart_bridge;

use crate::platform::dart::utils::{
    handle::DartHandle, NonNullDartValueArgExt,
};

#[dart_bridge("flutter/lib/src/native/platform/ice_candidate.g.dart")]
mod ice_candidate {
    use std::ptr;

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
        pub fn candidate(
            ice_candidate: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;

        /// Returns SDP line index of the provided [`IceCandidate`].
        pub fn sdp_m_line_index(
            ice_candidate: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<u16>>>;

        /// Returns SDP MID of the provided [`IceCandidate`].
        pub fn sdp_mid(
            ice_candidate: Dart_Handle,
        ) -> ptr::NonNull<DartValueArg<Option<String>>>;
    }
}

/// Wrapper around a [`DartHandle`] representing an ICE candidate of a
/// [RTCPeerConnection][1].
///
/// [1]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection
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
        Self(DartHandle::new(handle))
    }

    /// Returns the underlying [`Dart_Handle`] of this [`IceCandidate`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }

    /// Returns candidate of this [`IceCandidate`].
    #[must_use]
    pub fn candidate(&self) -> String {
        unsafe {
            Option::try_from(ice_candidate::candidate(self.0.get()).unbox())
                .unwrap()
                .unwrap()
        }
    }

    /// Returns SDP M line index of this [`IceCandidate`].
    #[must_use]
    pub fn sdp_m_line_index(&self) -> Option<u16> {
        unsafe {
            ice_candidate::sdp_m_line_index(self.0.get())
                .unbox()
                .try_into()
                .unwrap()
        }
    }

    /// Returns SDP MID of this [`IceCandidate`].
    #[must_use]
    pub fn sdp_mid(&self) -> Option<String> {
        unsafe {
            ice_candidate::sdp_mid(self.0.get())
                .unbox()
                .try_into()
                .unwrap()
        }
    }
}
