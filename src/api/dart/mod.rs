//! External [`Jason`] API exposing functions that can be called via FFI and
//! designed to be integrated into a [Flutter] plugin.
//!
//! [`Jason`]: crate::api::Jason
//! [Flutter]: https://flutter.dev

// TODO: Improve documentation in this module.
#![allow(clippy::missing_safety_doc, clippy::missing_panics_doc, missing_docs)]

pub mod audio_track_constraints;
pub mod connection_handle;
pub mod device_video_track_constraints;
pub mod display_video_track_constraints;
pub mod input_device_info;
pub mod jason;
pub mod local_media_track;
pub mod media_manager_handle;
pub mod media_stream_settings;
pub mod reconnect_handle;
pub mod remote_media_track;
pub mod room_close_reason;
pub mod room_handle;
pub mod utils;

use std::{convert::TryFrom, ffi::c_void, marker::PhantomData, ptr};

use dart_sys::{Dart_Handle, _Dart_Handle};
use derive_more::Display;
use libc::c_char;

use crate::{
    api::dart::utils::{DartError, PtrArray},
    media::MediaSourceKind,
    platform::utils::{
        dart_api::{
            Dart_DeletePersistentHandle_DL_Trampolined,
            Dart_NewPersistentHandle_DL_Trampolined,
        },
        handle::DartHandle,
    },
    utils::PrimitiveEnum,
};

pub use self::{
    audio_track_constraints::AudioTrackConstraints,
    connection_handle::ConnectionHandle,
    device_video_track_constraints::DeviceVideoTrackConstraints,
    display_video_track_constraints::DisplayVideoTrackConstraints,
    input_device_info::InputDeviceInfo,
    jason::Jason,
    local_media_track::LocalMediaTrack,
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

/// Rust structure having wrapper class in Dart.
///
/// Intended to be passed through FFI boundaries as thin pointers.
pub trait ForeignClass: Sized {
    /// Consumes itself returning a wrapped raw pointer obtained via
    /// [`Box::into_raw()`].
    #[inline]
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
    #[inline]
    #[must_use]
    unsafe fn from_ptr(this: ptr::NonNull<Self>) -> Self {
        *Box::from_raw(this.as_ptr())
    }
}

/// Type-erased value that can be transferred via FFI boundaries to/from Dart.
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
    #[inline]
    fn from(_: ()) -> Self {
        Self::None
    }
}

impl<T: ForeignClass> From<T> for DartValue {
    #[inline]
    fn from(val: T) -> Self {
        Self::Ptr(val.into_ptr().cast())
    }
}

impl<T: ForeignClass> From<Option<T>> for DartValue {
    #[inline]
    fn from(val: Option<T>) -> Self {
        match val {
            None => Self::None,
            Some(t) => Self::from(t),
        }
    }
}

impl<T> From<PtrArray<T>> for DartValue {
    #[inline]
    fn from(val: PtrArray<T>) -> Self {
        Self::Ptr(ptr::NonNull::from(Box::leak(Box::new(val))).cast())
    }
}

impl<T> From<Option<PtrArray<T>>> for DartValue {
    #[inline]
    fn from(val: Option<PtrArray<T>>) -> Self {
        match val {
            None => Self::None,
            Some(arr) => Self::from(arr),
        }
    }
}

impl From<String> for DartValue {
    #[inline]
    fn from(string: String) -> Self {
        Self::String(string_into_c_str(string))
    }
}

impl From<Option<String>> for DartValue {
    #[inline]
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
    #[inline]
    fn from(handle: Dart_Handle) -> Self {
        Self::Handle(ptr::NonNull::from(Box::leak(Box::new(handle))))
    }
}

impl From<Option<Dart_Handle>> for DartValue {
    #[inline]
    fn from(val: Option<Dart_Handle>) -> Self {
        match val {
            None => Self::None,
            Some(handle) => Self::from(handle),
        }
    }
}

impl From<DartError> for DartValue {
    #[inline]
    fn from(err: DartError) -> Self {
        Self::Handle(err.into())
    }
}

impl From<Option<DartError>> for DartValue {
    #[inline]
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
            #[inline]
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
    #[inline]
    fn from(from: F) -> Self {
        Self(DartValue::from(from), PhantomData)
    }
}

impl<T> TryFrom<DartValueArg<T>> for ptr::NonNull<c_void> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<T>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::Ptr(ptr) => Ok(ptr),
            _ => Err(DartValueCastError {
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
            _ => Err(DartValueCastError {
                expectation: "Option<NonNull<c_void>>",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<String>> for String {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<String>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::String(c_str) => unsafe { Ok(c_str_into_string(c_str)) },
            _ => Err(DartValueCastError {
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
            _ => Err(DartValueCastError {
                expectation: "()",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<Option<DartHandle>>> for Option<DartHandle> {
    type Error = DartValueCastError;

    fn try_from(
        value: DartValueArg<Option<DartHandle>>,
    ) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::Handle(handle) => {
                Ok(Some(DartHandle::new(unsafe { unbox_dart_handle(handle) })))
            }
            _ => Err(DartValueCastError {
                expectation: "Option<DartHandle>",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<Option<String>>> for Option<String> {
    type Error = DartValueCastError;

    fn try_from(
        value: DartValueArg<Option<String>>,
    ) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::String(c_str) => unsafe {
                Ok(Some(c_str_into_string(c_str)))
            },
            _ => Err(DartValueCastError {
                expectation: "Option<String>",
                value: value.0,
            }),
        }
    }
}

impl<T> TryFrom<DartValueArg<T>> for Dart_Handle {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<T>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::Handle(c_ptr) => Ok(unsafe { unbox_dart_handle(c_ptr) }),
            _ => Err(DartValueCastError {
                expectation: "Dart_Handle",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<DartHandle>> for DartHandle {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<DartHandle>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::Handle(handle) => {
                Ok(DartHandle::new(unsafe { unbox_dart_handle(handle) }))
            }
            _ => Err(DartValueCastError {
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
            _ => Err(DartValueCastError {
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
            _ => Err(DartValueCastError {
                expectation: "Option<NonNull<Dart_Handle>>",
                value: value.0,
            }),
        }
    }
}

macro_rules! impl_primitive_dart_value_try_from {
    ($arg:ty) => {
        impl TryFrom<DartValueArg<$arg>> for $arg {
            type Error = DartValueCastError;

            fn try_from(
                value: DartValueArg<$arg>,
            ) -> Result<Self, Self::Error> {
                match value.0 {
                    DartValue::Int(num) => {
                        Ok(<$arg>::try_from(num).map_err(
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

        impl TryFrom<DartValueArg<Option<$arg>>> for Option<$arg> {
            type Error = DartValueCastError;

            fn try_from(
                value: DartValueArg<Option<$arg>>
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
    ($($arg:ty),*) => {
        $(impl_primitive_dart_value_try_from!($arg);)+
    }
}

impl_primitive_dart_value_try_from![
    i8, i16, i32, i128, u8, u16, u32, u64, u128
];

impl<T> TryFrom<DartValueArg<T>> for i64
where
    T: PrimitiveEnum,
{
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<T>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::Int(num) => Ok(num),
            _ => Err(DartValueCastError {
                expectation: "i64",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<i64>> for i64 {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<i64>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::Int(num) => Ok(num),
            _ => Err(DartValueCastError {
                expectation: "i64",
                value: value.0,
            }),
        }
    }
}

impl<T> TryFrom<DartValueArg<Option<T>>> for Option<i64>
where
    T: PrimitiveEnum,
{
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Option<T>>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::Int(num) => Ok(Some(num)),
            _ => Err(DartValueCastError {
                expectation: "Option<i64>",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<Option<i64>>> for Option<i64> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Option<i64>>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::Int(num) => Ok(Some(num)),
            _ => Err(DartValueCastError {
                expectation: "Option<i64>",
                value: value.0,
            }),
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

impl TryFrom<i64> for MediaSourceKind {
    type Error = i64;

    #[inline]
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Device),
            1 => Ok(Self::Display),
            _ => Err(value),
        }
    }
}

/// Returns boxed pointer to a provided [`DartValue`].
#[no_mangle]
pub unsafe extern "C" fn box_foreign_value(
    val: DartValue,
) -> ptr::NonNull<DartValue> {
    ptr::NonNull::from(Box::leak(Box::new(val)))
}

/// Returns a [`Dart_Handle`] dereferenced from the provided pointer.
#[no_mangle]
pub unsafe extern "C" fn unbox_dart_handle(
    val: ptr::NonNull<Dart_Handle>,
) -> Dart_Handle {
    *val.as_ptr()
}

/// Frees provided [`ptr::NonNull`] pointer to the [`Dart_Handle`].
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
    let val = Dart_NewPersistentHandle_DL_Trampolined(val);
    ptr::NonNull::from(Box::leak(Box::new(val)))
}

#[cfg(feature = "mockable")]
mod dart_value_extern_tests_helpers {
    use std::convert::TryInto as _;

    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn returns_none() -> DartValueArg<String> {
        DartValueArg::from(())
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_input_device_info_ptr(
    ) -> DartValueArg<InputDeviceInfo> {
        DartValueArg::from(InputDeviceInfo(0))
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
    pub unsafe extern "C" fn accepts_input_device_info_pointer(
        ptr: DartValueArg<InputDeviceInfo>,
    ) {
        let ptr: ptr::NonNull<c_void> = ptr.try_into().unwrap();
        let info = InputDeviceInfo::from_ptr(ptr.cast());

        assert_eq!(info.device_id(), "InputDeviceInfo.device_id");
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
}
