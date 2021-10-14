//! Entity for the ICE candidate from the `RTCPeerConnection`'s SDP offer.

use std::{os::raw::c_char, ptr};

use dart_sys::Dart_Handle;
use derive_more::From;

use crate::{
    api::dart::string_into_c_str,
    platform::dart::utils::{
        handle::DartHandle,
        nullable::{NullableChar, NullableInt},
        option::{DartIntOption, DartStringOption},
    },
};

/// Pointer to an extern function that creates new [`IceCandidate`] with a
/// provided parameters.
type NewFunction = extern "C" fn(
    ptr::NonNull<c_char>,
    NullableChar,
    NullableInt,
) -> Dart_Handle;

/// Pointer to an extern function that returns candidate of the provided
/// [`IceCandidate`].
type CandidateFunction = extern "C" fn(Dart_Handle) -> DartStringOption;

/// Pointer to an extern function that returns SDP line index of the provided
/// [`IceCandidate`].
type SdpMLineIndexFunction = extern "C" fn(Dart_Handle) -> DartIntOption;

/// Pointer to an extern function that returns SDP MID of the provided
/// [`IceCandidate`].
type SdpMidFunction = extern "C" fn(Dart_Handle) -> DartStringOption;

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

/// Wrapper around [`DartHandle`] representing ICE candidate of the
/// `RTCPeerConnection`.
#[derive(From)]
pub struct IceCandidate(DartHandle);

impl From<Dart_Handle> for IceCandidate {
    fn from(handle: Dart_Handle) -> Self {
        Self(DartHandle::new(handle))
    }
}

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

impl IceCandidate {
    /// Returns new [`IceCandidate`] with a provided parameters.
    #[must_use]
    pub fn new(
        candidate: &str,
        sdp_m_line_index: Option<u16>,
        sdp_mid: &Option<String>,
    ) -> Self {
        let handle = unsafe {
            NEW_FUNCTION.unwrap()(
                string_into_c_str(candidate.to_owned()),
                sdp_mid.clone().into(),
                sdp_m_line_index.into(),
            )
        };
        Self(DartHandle::new(handle))
    }

    /// Returns underlying [`Dart_Handle`] of this [`IceCandidate`].
    #[must_use]
    pub fn handle(&self) -> Dart_Handle {
        self.0.get()
    }

    /// Returns candidate of this [`IceCandidate`].
    #[must_use]
    pub fn candidate(&self) -> String {
        unsafe {
            Option::from(CANDIDATE_FUNCTION.unwrap()(self.0.get())).unwrap()
        }
    }

    /// Returns SDP M line index of this [`IceCandidate`].
    #[must_use]
    pub fn sdp_m_line_index(&self) -> Option<u16> {
        unsafe {
            let index: Option<i32> =
                SDP_M_LINE_INDEX_FUNCTION.unwrap()(self.0.get()).into();
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            index.map(|i| i as u16)
        }
    }

    /// Returns SDP MID of this [`IceCandidate`].
    #[must_use]
    pub fn sdp_mid(&self) -> Option<String> {
        unsafe { SDP_MID_FUNCTION.unwrap()(self.0.get()).into() }
    }
}
