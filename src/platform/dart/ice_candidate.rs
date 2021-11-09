//! ICE candidate of a [RTCPeerConnection][1].
//!
//! [1]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection

use std::convert::{TryFrom, TryInto};

use dart_sys::Dart_Handle;
use derive_more::From;

use crate::{api::DartValueArg, platform::dart::utils::handle::DartHandle};

/// Pointer to an extern function creating a new [`IceCandidate`] with the
/// provided parameters.
type NewFunction = extern "C" fn(
    DartValueArg<String>,
    DartValueArg<Option<String>>,
    DartValueArg<Option<u16>>,
) -> Dart_Handle;

/// Pointer to an extern function returning candidate of the provided
/// [`IceCandidate`].
type CandidateFunction =
    extern "C" fn(Dart_Handle) -> DartValueArg<Option<String>>;

/// Pointer to an extern function returning SDP M line index of the provided
/// [`IceCandidate`].
type SdpMLineIndexFunction =
    extern "C" fn(Dart_Handle) -> DartValueArg<Option<u16>>;

/// Pointer to an extern function returning SDP MID of the provided
/// [`IceCandidate`].
type SdpMidFunction =
    extern "C" fn(Dart_Handle) -> DartValueArg<Option<String>>;

/// Stores pointer to the [`NewFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut NEW_FUNCTION: Option<NewFunction> = None;

/// Stores pointer to the [`CandidateFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut CANDIDATE_FUNCTION: Option<CandidateFunction> = None;

/// Stores pointer to the [`SdpMLineIndexFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut SDP_M_LINE_INDEX_FUNCTION: Option<SdpMLineIndexFunction> = None;

/// Stores pointer to the [`SdpMidFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut SDP_MID_FUNCTION: Option<SdpMidFunction> = None;

/// Registers the provided [`NewFunction`] as [`NEW_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_IceCandidate__new(f: NewFunction) {
    NEW_FUNCTION = Some(f);
}

/// Registers the provided [`CandidateFunction`] as [`CANDIDATE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_IceCandidate__candidate(
    f: CandidateFunction,
) {
    CANDIDATE_FUNCTION = Some(f);
}

/// Registers the provided [`SdpMLineIndexFunction`] as
/// [`SDP_M_LINE_INDEX_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_IceCandidate__sdp_m_line_index(
    f: SdpMLineIndexFunction,
) {
    SDP_M_LINE_INDEX_FUNCTION = Some(f);
}

/// Registers the provided [`SdpMidFunction`] as [`SDP_MID_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_IceCandidate__sdp_mid(f: SdpMidFunction) {
    SDP_MID_FUNCTION = Some(f);
}

/// Wrapper around a [`DartHandle`] representing an ICE candidate of a
/// [RTCPeerConnection][1].
///
/// [1]: https://w3.org/TR/webrtc/#dom-rtcpeerconnection
#[derive(From)]
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
            NEW_FUNCTION.unwrap()(
                candidate.to_string().into(),
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
            Option::try_from(CANDIDATE_FUNCTION.unwrap()(self.0.get()))
                .unwrap()
                .unwrap()
        }
    }

    /// Returns SDP M line index of this [`IceCandidate`].
    #[must_use]
    pub fn sdp_m_line_index(&self) -> Option<u16> {
        unsafe {
            SDP_M_LINE_INDEX_FUNCTION.unwrap()(self.0.get())
                .try_into()
                .unwrap()
        }
    }

    /// Returns SDP MID of this [`IceCandidate`].
    #[must_use]
    pub fn sdp_mid(&self) -> Option<String> {
        unsafe { SDP_MID_FUNCTION.unwrap()(self.0.get()).try_into().unwrap() }
    }
}
