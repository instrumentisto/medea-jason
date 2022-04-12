//! External [`Jason`] API exposing functions that can be called via FFI and
//! designed to be integrated into a [Flutter] plugin.
//!
//! [`Jason`]: crate::api::Jason
//! [Flutter]: https://flutter.dev

// TODO: Improve documentation in this module.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc,
    clippy::undocumented_unsafe_blocks,
    missing_docs
)]

pub mod audio_track_constraints;
pub mod connection_handle;
pub mod device_video_track_constraints;
pub mod display_video_track_constraints;
pub mod jason;
pub mod local_media_track;
pub mod media_device_info;
pub mod media_manager_handle;
pub mod media_stream_settings;
pub mod reconnect_handle;
pub mod remote_media_track;
pub mod room_close_reason;
pub mod room_handle;
pub mod utils;

use std::{ffi::c_void, marker::PhantomData, panic, ptr};

use dart_sys::{Dart_Handle, _Dart_Handle};
use derive_more::Display;
use libc::c_char;

use crate::{
    api::{
        dart::utils::{DartError, PtrArray},
        utils::new_panic_error,
    },
    media::{FacingMode, MediaDeviceKind, MediaKind, MediaSourceKind},
    platform,
    platform::utils::{
        dart_api::{
            Dart_DeletePersistentHandle_DL_Trampolined,
            Dart_NewPersistentHandle_DL_Trampolined,
            Dart_PropagateError_DL_Trampolined,
        },
        handle::DartHandle,
    },
};

pub use self::{
    audio_track_constraints::AudioTrackConstraints,
    connection_handle::ConnectionHandle,
    device_video_track_constraints::DeviceVideoTrackConstraints,
    display_video_track_constraints::DisplayVideoTrackConstraints,
    jason::Jason,
    local_media_track::LocalMediaTrack,
    media_device_info::MediaDeviceInfo,
    media_manager_handle::MediaManagerHandle,
    media_stream_settings::MediaStreamSettings,
    reconnect_handle::ReconnectHandle,
    remote_media_track::RemoteMediaTrack,
    room_close_reason::RoomCloseReason,
    room_handle::RoomHandle,
    utils::{
        c_str_into_string, free_dart_native_string, string_into_c_str,
        DartError as Error,
    },
};

/// Sets the provided [`Dart_Handle`] as a callback for the Rust panic hook.
#[no_mangle]
pub unsafe extern "C" fn on_panic(cb: Dart_Handle) {
    platform::set_panic_callback(platform::Function::new(cb));
}

/// Wraps the provided function to catch all the Rust panics and propagate them
/// to the Dart side.
pub fn propagate_panic<T>(f: impl FnOnce() -> T) -> T {
    let res = panic::catch_unwind(panic::AssertUnwindSafe(f));
    if let Ok(r) = res {
        r
    } else {
        unsafe {
            Dart_PropagateError_DL_Trampolined(new_panic_error());
        }
        unreachable!("`Dart_PropagateError` should do early return")
    }
}

/// Rust structure having wrapper class in Dart.
///
/// Intended to be passed through FFI boundaries as thin pointers.
pub trait ForeignClass: Sized {
    /// Consumes itself returning a wrapped raw pointer obtained via
    /// [`Box::into_raw()`].
    #[must_use]
    fn into_ptr(self) -> ptr::NonNull<Self> {
        ptr::NonNull::from(Box::leak(Box::new(self)))
    }

    /// Constructs a [`ForeignClass`] from the given raw pointer via
    /// [`Box::from_raw()`].
    ///
    /// # Safety
    ///
    /// Same as for [`Box::from_raw()`].
    #[must_use]
    unsafe fn from_ptr(this: ptr::NonNull<Self>) -> Self {
        *Box::from_raw(this.as_ptr())
    }
}

/// Marker indicating a C-style enum which can be converted from number
/// primitives.
pub trait PrimitiveEnum: TryFrom<i64> {}

/// Type-erased value that can be transferred via FFI boundaries to/from Dart.
#[allow(missing_copy_implementations)] // not trivially copyable
#[derive(Debug)]
#[repr(u8)]
pub enum DartValue {
    /// No value. It can mean `()`, `void` or [`Option::None`] basing on the
    /// contexts.
    None,

    /// Pointer to a [`Box`]ed Rust object.
    Ptr(ptr::NonNull<c_void>),

    /// Pointer to a [`Dart_Handle`] of some Dart object.
    Handle(ptr::NonNull<Dart_Handle>),

    /// Native string.
    String(ptr::NonNull<c_char>),

    /// Integer value.
    ///
    /// This can also be used to transfer boolean values and C-like enums.
    Int(i64),
}

impl From<()> for DartValue {
    fn from(_: ()) -> Self {
        Self::None
    }
}

impl<T: ForeignClass> From<T> for DartValue {
    fn from(val: T) -> Self {
        Self::Ptr(val.into_ptr().cast())
    }
}

impl<T: ForeignClass> From<Option<T>> for DartValue {
    fn from(val: Option<T>) -> Self {
        match val {
            None => Self::None,
            Some(t) => Self::from(t),
        }
    }
}

impl<T> From<PtrArray<T>> for DartValue {
    fn from(val: PtrArray<T>) -> Self {
        Self::Ptr(ptr::NonNull::from(Box::leak(Box::new(val))).cast())
    }
}

impl<T> From<Option<PtrArray<T>>> for DartValue {
    fn from(val: Option<PtrArray<T>>) -> Self {
        match val {
            None => Self::None,
            Some(arr) => Self::from(arr),
        }
    }
}

impl From<String> for DartValue {
    fn from(string: String) -> Self {
        Self::String(string_into_c_str(string))
    }
}

impl From<Option<String>> for DartValue {
    fn from(val: Option<String>) -> Self {
        match val {
            None => Self::None,
            Some(string) => Self::from(string),
        }
    }
}

impl From<Option<i64>> for DartValue {
    fn from(val: Option<i64>) -> Self {
        match val {
            None => Self::None,
            Some(i) => Self::from(i),
        }
    }
}

impl From<ptr::NonNull<Dart_Handle>> for DartValue {
    fn from(handle: ptr::NonNull<*mut _Dart_Handle>) -> Self {
        Self::Handle(handle)
    }
}

impl From<Option<ptr::NonNull<Dart_Handle>>> for DartValue {
    fn from(val: Option<ptr::NonNull<Dart_Handle>>) -> Self {
        match val {
            None => Self::None,
            Some(handle) => Self::from(handle),
        }
    }
}

impl From<Dart_Handle> for DartValue {
    fn from(handle: Dart_Handle) -> Self {
        Self::Handle(ptr::NonNull::from(Box::leak(Box::new(handle))))
    }
}

impl From<Option<Dart_Handle>> for DartValue {
    fn from(val: Option<Dart_Handle>) -> Self {
        match val {
            None => Self::None,
            Some(handle) => Self::from(handle),
        }
    }
}

impl From<DartError> for DartValue {
    fn from(err: DartError) -> Self {
        Self::Handle(err.into())
    }
}

impl From<Option<DartError>> for DartValue {
    fn from(val: Option<DartError>) -> Self {
        match val {
            None => Self::None,
            Some(err) => Self::from(err),
        }
    }
}

/// Implements [`From`] types that can by casted to `i64` for the [`DartValue`].
/// Should be called for all the integer types fitting in `2^63`.
macro_rules! impl_from_num_for_dart_value {
    ($arg:ty) => {
        impl From<$arg> for DartValue {
            fn from(val: $arg) -> Self {
                DartValue::Int(i64::from(val))
            }
        }
    };
}

impl_from_num_for_dart_value!(i8);
impl_from_num_for_dart_value!(i16);
impl_from_num_for_dart_value!(i32);
impl_from_num_for_dart_value!(i64);
impl_from_num_for_dart_value!(u8);
impl_from_num_for_dart_value!(u16);
impl_from_num_for_dart_value!(u32);
impl_from_num_for_dart_value!(bool);

/// [`DartValue`] marked by a Rust type.
///
/// There are no type parameter specific functionality, it serves purely as a
/// marker in type signatures.
#[derive(Debug)]
#[repr(transparent)]
pub struct DartValueArg<T>(DartValue, PhantomData<*const T>);

impl<F, T> From<F> for DartValueArg<T>
where
    DartValue: From<F>,
{
    fn from(from: F) -> Self {
        Self(DartValue::from(from), PhantomData)
    }
}

impl<T> TryFrom<DartValueArg<T>> for ptr::NonNull<c_void> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<T>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::Ptr(ptr) => Ok(ptr),
            DartValue::None
            | DartValue::Handle(_)
            | DartValue::String(_)
            | DartValue::Int(_) => Err(DartValueCastError {
                expectation: "NonNull<c_void>",
                value: value.0,
            }),
        }
    }
}

impl<T> TryFrom<DartValueArg<T>> for Option<ptr::NonNull<c_void>> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<T>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::Ptr(ptr) => Ok(Some(ptr)),
            DartValue::Handle(_) | DartValue::String(_) | DartValue::Int(_) => {
                Err(DartValueCastError {
                    expectation: "Option<NonNull<c_void>>",
                    value: value.0,
                })
            }
        }
    }
}

impl TryFrom<DartValueArg<Self>> for String {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Self>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::String(c_str) => unsafe { Ok(c_str_into_string(c_str)) },
            DartValue::None
            | DartValue::Ptr(_)
            | DartValue::Handle(_)
            | DartValue::Int(_) => Err(DartValueCastError {
                expectation: "String",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<()>> for () {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<()>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(()),
            DartValue::Ptr(_)
            | DartValue::Handle(_)
            | DartValue::String(_)
            | DartValue::Int(_) => Err(DartValueCastError {
                expectation: "()",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<Self>> for Option<DartHandle> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Self>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::Handle(handle) => {
                Ok(Some(unsafe { DartHandle::new(*handle.as_ptr()) }))
            }
            DartValue::Ptr(_) | DartValue::String(_) | DartValue::Int(_) => {
                Err(DartValueCastError {
                    expectation: "Option<DartHandle>",
                    value: value.0,
                })
            }
        }
    }
}

impl TryFrom<DartValueArg<Self>> for Option<String> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Self>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::String(c_str) => unsafe {
                Ok(Some(c_str_into_string(c_str)))
            },
            DartValue::Ptr(_) | DartValue::Handle(_) | DartValue::Int(_) => {
                Err(DartValueCastError {
                    expectation: "Option<String>",
                    value: value.0,
                })
            }
        }
    }
}

impl<T> TryFrom<DartValueArg<T>> for Dart_Handle {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<T>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::Handle(c_ptr) => Ok(unsafe { unbox_dart_handle(c_ptr) }),
            DartValue::None
            | DartValue::Ptr(_)
            | DartValue::String(_)
            | DartValue::Int(_) => Err(DartValueCastError {
                expectation: "Dart_Handle",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<Self>> for DartHandle {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Self>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::Handle(handle) => {
                Ok(unsafe { Self::new(unbox_dart_handle(handle)) })
            }
            DartValue::None
            | DartValue::Ptr(_)
            | DartValue::String(_)
            | DartValue::Int(_) => Err(DartValueCastError {
                expectation: "DartHandle",
                value: value.0,
            }),
        }
    }
}

impl<T> TryFrom<DartValueArg<T>> for ptr::NonNull<Dart_Handle> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<T>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::Handle(c_str) => Ok(c_str),
            DartValue::None
            | DartValue::Ptr(_)
            | DartValue::String(_)
            | DartValue::Int(_) => Err(DartValueCastError {
                expectation: "NonNull<Dart_Handle>",
                value: value.0,
            }),
        }
    }
}

impl<T> TryFrom<DartValueArg<T>> for Option<ptr::NonNull<Dart_Handle>> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<T>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::Handle(c_str) => Ok(Some(c_str)),
            DartValue::Ptr(_) | DartValue::String(_) | DartValue::Int(_) => {
                Err(DartValueCastError {
                    expectation: "Option<NonNull<Dart_Handle>>",
                    value: value.0,
                })
            }
        }
    }
}

/// Helper macro implementing [`TryFrom`]`<`[`DartValueArg`]`>` for primitive
/// types.
macro_rules! impl_primitive_dart_value_try_from {
    ($arg:ty) => {
        impl TryFrom<DartValueArg<Self>> for $arg {
            type Error = DartValueCastError;

            fn try_from(
                value: DartValueArg<Self>,
            ) -> Result<Self, Self::Error> {
                match value.0 {
                    DartValue::Int(num) => {
                        Ok(Self::try_from(num).map_err(
                            |_| DartValueCastError {
                                expectation: stringify!($arg),
                                value: value.0,
                            }
                        )?)
                    }
                    _ => Err(DartValueCastError {
                        expectation: stringify!($arg),
                        value: value.0,
                    }),
                }
            }
        }

        impl TryFrom<DartValueArg<Self>> for Option<$arg> {
            type Error = DartValueCastError;

            fn try_from(
                value: DartValueArg<Self>
            ) -> Result<Self, Self::Error> {
                match value.0 {
                    DartValue::None => Ok(None),
                    DartValue::Int(num) => {
                        Ok(Some(<$arg>::try_from(num).map_err(
                            |_| DartValueCastError {
                                expectation: concat!(
                                    "Option<",
                                    stringify!($arg),
                                    ">"
                                ),
                                value: value.0,
                            }
                        )?))
                    }
                    _ => Err(DartValueCastError {
                        expectation: concat!("Option<", stringify!($arg), ">"),
                        value: value.0,
                    }),
                }
            }
        }
    };
    ($($arg:ty),+) => {
        $(impl_primitive_dart_value_try_from!($arg);)+
    }
}

impl_primitive_dart_value_try_from![i8, i16, i32, i64, u8, u16, u32];

impl<T: PrimitiveEnum> TryFrom<DartValueArg<T>> for i64 {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<T>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::Int(num) => Ok(num),
            DartValue::None
            | DartValue::Ptr(_)
            | DartValue::Handle(_)
            | DartValue::String(_) => Err(DartValueCastError {
                expectation: "i64",
                value: value.0,
            }),
        }
    }
}

impl<T: PrimitiveEnum> TryFrom<DartValueArg<Self>> for Option<T> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Self>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::Int(num) => match T::try_from(num) {
                Ok(variant) => Ok(Some(variant)),
                Err(_) => Err(DartValueCastError {
                    expectation: "Option<i64>",
                    value: value.0,
                }),
            },
            DartValue::Ptr(_) | DartValue::Handle(_) | DartValue::String(_) => {
                Err(DartValueCastError {
                    expectation: "Option<i64>",
                    value: value.0,
                })
            }
        }
    }
}

/// Error of converting a [`DartValue`] to the concrete type.
#[derive(Debug, Display)]
#[display(fmt = "expected `{}`, but got: `{:?}`", expectation, value)]
pub struct DartValueCastError {
    /// Expected type description. Like a [`String`] or an `Option<i64>`.
    expectation: &'static str,

    /// [`DartValue`] that cannot be casted.
    value: DartValue,
}

impl DartValueCastError {
    /// Returns [`DartValue`] that could not be casted.
    fn into_value(self) -> DartValue {
        self.value
    }
}

impl PrimitiveEnum for MediaSourceKind {}
impl PrimitiveEnum for FacingMode {}

impl TryFrom<i64> for MediaSourceKind {
    type Error = i64;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Device),
            1 => Ok(Self::Display),
            _ => Err(value),
        }
    }
}

impl TryFrom<i64> for FacingMode {
    type Error = i64;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::User),
            1 => Ok(Self::Environment),
            2 => Ok(Self::Left),
            3 => Ok(Self::Right),
            _ => Err(value),
        }
    }
}

impl TryFrom<i64> for MediaKind {
    type Error = i64;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Audio),
            1 => Ok(Self::Video),
            _ => Err(value),
        }
    }
}

impl TryFrom<i64> for MediaDeviceKind {
    type Error = i64;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AudioInput),
            1 => Ok(Self::VideoInput),
            2 => Ok(Self::AudioOutput),
            _ => Err(value),
        }
    }
}

/// Returns a [`Dart_Handle`] dereferenced from the provided pointer.
#[no_mangle]
pub unsafe extern "C" fn unbox_dart_handle(
    val: ptr::NonNull<Dart_Handle>,
) -> Dart_Handle {
    *val.as_ptr()
}

/// Frees the provided [`ptr::NonNull`] pointer to a [`Dart_Handle`].
#[no_mangle]
pub unsafe extern "C" fn free_boxed_dart_handle(
    val: ptr::NonNull<Dart_Handle>,
) {
    let handle = Box::from_raw(val.as_ptr());
    Dart_DeletePersistentHandle_DL_Trampolined(*handle);
}

/// Returns a pointer to a boxed [`Dart_Handle`] created from the provided
/// [`Dart_Handle`].
#[no_mangle]
pub unsafe extern "C" fn box_dart_handle(
    val: Dart_Handle,
) -> ptr::NonNull<Dart_Handle> {
    let persisted = Dart_NewPersistentHandle_DL_Trampolined(val);
    ptr::NonNull::from(Box::leak(Box::new(persisted)))
}

/// Returns a boxed pointer to the provided [`DartValue`].
#[no_mangle]
pub unsafe extern "C" fn box_foreign_value(
    val: DartValue,
) -> ptr::NonNull<DartValue> {
    ptr::NonNull::from(Box::leak(Box::new(val)))
}

#[cfg(feature = "mockable")]
mod dart_value_extern_tests_helpers {
    use super::*;

    use crate::platform::set_panic_hook;

    #[no_mangle]
    pub unsafe extern "C" fn returns_none() -> DartValueArg<String> {
        DartValueArg::from(())
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_media_device_info_ptr(
    ) -> DartValueArg<MediaDeviceInfo> {
        DartValueArg::from(MediaDeviceInfo(0))
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_handle_ptr(
        handle: Dart_Handle,
    ) -> DartValueArg<Dart_Handle> {
        DartValueArg::from(handle)
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_string() -> DartValueArg<String> {
        DartValueArg::from(String::from("QWERTY"))
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_int() -> DartValueArg<i64> {
        DartValueArg::from(333)
    }

    #[no_mangle]
    pub unsafe extern "C" fn accepts_none(none: DartValueArg<String>) {
        assert!(matches!(none.0, DartValue::None));
    }

    #[no_mangle]
    pub unsafe extern "C" fn accepts_media_device_info_pointer(
        ptr: DartValueArg<MediaDeviceInfo>,
    ) {
        let ptr: ptr::NonNull<c_void> = ptr.try_into().unwrap();
        let info = MediaDeviceInfo::from_ptr(ptr.cast());

        assert_eq!(info.device_id(), "MediaDeviceInfo.device_id");
    }

    #[no_mangle]
    pub unsafe extern "C" fn accepts_string(str: DartValueArg<String>) {
        let string = String::try_from(str).unwrap();
        assert_eq!(string, "my string");
    }

    #[no_mangle]
    pub unsafe extern "C" fn accepts_int(int: DartValueArg<i64>) {
        let int: i64 = int.try_into().unwrap();
        assert_eq!(int, 235);
    }

    #[no_mangle]
    pub unsafe extern "C" fn fire_panic() {
        set_panic_hook();
        propagate_panic(|| panic!("Panicking"));
    }
}
