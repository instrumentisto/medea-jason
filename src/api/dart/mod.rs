//! External [`Jason`] API exposing functions that can be called via FFI and
//! designed to be integrated into a [Flutter] plugin.
//!
//! [Flutter]: https://flutter.dev

// TODO: Improve documentation in this module.
#![allow(
    clippy::as_conversions,
    clippy::missing_docs_in_private_items,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc,
    clippy::undocumented_unsafe_blocks,
    clippy::unwrap_used,
    missing_docs
)]

pub use crate::jason_api;
pub mod api_struct;
pub mod utils;
pub use api_struct::*;

use std::{
    ffi::{c_void, CString},
    marker::PhantomData,
    panic, ptr,
};

use dart_sys::{Dart_Handle, _Dart_Handle};
use derive_more::Display;
use libc::c_char;

use crate::{
    api::dart::utils::new_panic_error,
    media::{FacingMode, MediaDeviceKind, MediaKind, MediaSourceKind},
    platform::utils::{
        c_str_into_string,
        dart_api::{
            Dart_DeletePersistentHandle_DL_Trampolined,
            Dart_NewPersistentHandle_DL_Trampolined,
            Dart_PropagateError_DL_Trampolined,
        },
        free_dart_native_string,
        handle::DartHandle,
        string_into_c_str,
    },
};

pub use crate::media::MediaDirection;

pub use self::{
    jason_api::{
        ConnectionHandle, Jason, LocalMediaTrack, MediaManagerHandle,
        ReconnectHandle, RemoteMediaTrack, RoomCloseReason, RoomHandle,
    },
    utils::{ArgumentError, DartError as Error},
};

/// Wraps the provided function to catch all the Rust panics and propagate them
/// to the Dart side.
pub fn propagate_panic<T>(f: impl FnOnce() -> T) -> T {
    panic::catch_unwind(panic::AssertUnwindSafe(f)).unwrap_or_else(|_| {
        unsafe {
            Dart_PropagateError_DL_Trampolined(new_panic_error());
        }
        unreachable!("`Dart_PropagateError` should do early return")
    })
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

/// Owner of some allocated memory.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum MemoryOwner {
    /// Memory is allocated on Rust side.
    Rust = 0,

    /// Memory is allocated on Dart side.
    Dart = 1,
}

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
    String(ptr::NonNull<c_char>, MemoryOwner),

    /// Integer value.
    ///
    /// This can also be used to transfer boolean values and C-like enums.
    Int(i64),

    /// Float value.
    Float(f64),

    /// Boolean value.
    Bool(bool),
}

impl Drop for DartValue {
    fn drop(&mut self) {
        match self {
            Self::Float(_)
            | Self::Bool(_)
            | Self::Int(_)
            | Self::Ptr(_)
            | Self::Handle(_)
            | Self::None => {}
            Self::String(ptr, MemoryOwner::Dart) => unsafe {
                free_dart_native_string(*ptr);
            },
            Self::String(ptr, MemoryOwner::Rust) => unsafe {
                drop(CString::from_raw(ptr.as_ptr()));
            },
        }
    }
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
        val.map_or(Self::None, |t| Self::from(t))
    }
}

impl From<String> for DartValue {
    fn from(string: String) -> Self {
        Self::String(string_into_c_str(string), MemoryOwner::Rust)
    }
}

impl From<Option<String>> for DartValue {
    fn from(val: Option<String>) -> Self {
        val.map_or(Self::None, Self::from)
    }
}

impl From<Option<i64>> for DartValue {
    fn from(val: Option<i64>) -> Self {
        val.map_or(Self::None, Self::from)
    }
}

impl From<ptr::NonNull<Dart_Handle>> for DartValue {
    fn from(handle: ptr::NonNull<*mut _Dart_Handle>) -> Self {
        Self::Handle(handle)
    }
}

impl From<Option<ptr::NonNull<Dart_Handle>>> for DartValue {
    fn from(val: Option<ptr::NonNull<Dart_Handle>>) -> Self {
        val.map_or(Self::None, Self::from)
    }
}

impl From<Dart_Handle> for DartValue {
    fn from(handle: Dart_Handle) -> Self {
        Self::Handle(ptr::NonNull::from(Box::leak(Box::new(handle))))
    }
}

impl From<Option<Dart_Handle>> for DartValue {
    fn from(val: Option<Dart_Handle>) -> Self {
        val.map_or(Self::None, Self::from)
    }
}

impl From<Error> for DartValue {
    fn from(err: Error) -> Self {
        Self::Handle(err.into())
    }
}

impl From<Option<Error>> for DartValue {
    fn from(val: Option<Error>) -> Self {
        val.map_or(Self::None, Self::from)
    }
}

impl From<MediaDirection> for DartValue {
    fn from(val: MediaDirection) -> Self {
        Self::from(val as u8)
    }
}

impl From<bool> for DartValue {
    fn from(val: bool) -> Self {
        Self::Bool(val)
    }
}

impl From<f32> for DartValue {
    fn from(val: f32) -> Self {
        Self::Float(f64::from(val))
    }
}

impl From<f64> for DartValue {
    fn from(val: f64) -> Self {
        Self::Float(val)
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
            | DartValue::String(_, _)
            | DartValue::Int(_)
            | DartValue::Bool(_)
            | DartValue::Float(_) => Err(DartValueCastError {
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
            DartValue::Handle(_)
            | DartValue::String(_, _)
            | DartValue::Float(_)
            | DartValue::Bool(_)
            | DartValue::Int(_) => Err(DartValueCastError {
                expectation: "Option<NonNull<c_void>>",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<Self>> for String {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Self>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::String(c_str, _) => unsafe {
                Ok(c_str_into_string(c_str))
            },
            DartValue::None
            | DartValue::Ptr(_)
            | DartValue::Handle(_)
            | DartValue::Float(_)
            | DartValue::Bool(_)
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
            | DartValue::String(_, _)
            | DartValue::Float(_)
            | DartValue::Bool(_)
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
            DartValue::Ptr(_)
            | DartValue::Bool(_)
            | DartValue::Float(_)
            | DartValue::String(_, _)
            | DartValue::Int(_) => Err(DartValueCastError {
                expectation: "Option<DartHandle>",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<Self>> for Option<String> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Self>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::String(c_str, _) => unsafe {
                Ok(Some(c_str_into_string(c_str)))
            },
            DartValue::Ptr(_)
            | DartValue::Bool(_)
            | DartValue::Float(_)
            | DartValue::Handle(_)
            | DartValue::Int(_) => Err(DartValueCastError {
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
            DartValue::None
            | DartValue::Ptr(_)
            | DartValue::String(_, _)
            | DartValue::Float(_)
            | DartValue::Bool(_)
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
            | DartValue::String(_, _)
            | DartValue::Float(_)
            | DartValue::Bool(_)
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
            | DartValue::String(_, _)
            | DartValue::Float(_)
            | DartValue::Bool(_)
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
            DartValue::Ptr(_)
            | DartValue::Bool(_)
            | DartValue::Float(_)
            | DartValue::String(_, _)
            | DartValue::Int(_) => Err(DartValueCastError {
                expectation: "Option<NonNull<Dart_Handle>>",
                value: value.0,
            }),
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
            | DartValue::Float(_)
            | DartValue::Bool(_)
            | DartValue::String(_, _) => Err(DartValueCastError {
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
            DartValue::Ptr(_)
            | DartValue::Float(_)
            | DartValue::Bool(_)
            | DartValue::Handle(_)
            | DartValue::String(_, _) => Err(DartValueCastError {
                expectation: "Option<i64>",
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<Self>> for Option<f64> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Self>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::Float(num) => Ok(Some(num)),
            DartValue::Ptr(_)
            | DartValue::Handle(_)
            | DartValue::String(..)
            | DartValue::Int(_)
            | DartValue::Bool(_) => Err(DartValueCastError {
                expectation: concat!("Option<f64>"),
                value: value.0,
            }),
        }
    }
}

impl TryFrom<DartValueArg<Self>> for Option<bool> {
    type Error = DartValueCastError;

    fn try_from(value: DartValueArg<Self>) -> Result<Self, Self::Error> {
        match value.0 {
            DartValue::None => Ok(None),
            DartValue::Bool(num) => Ok(Some(num)),
            DartValue::Ptr(_)
            | DartValue::Handle(_)
            | DartValue::String(..)
            | DartValue::Int(_)
            | DartValue::Float(_) => Err(DartValueCastError {
                expectation: concat!("Option<f64>"),
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

impl PrimitiveEnum for MediaSourceKind {}
impl PrimitiveEnum for FacingMode {}
impl PrimitiveEnum for MediaDirection {}

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

impl TryFrom<i64> for MediaDirection {
    type Error = i64;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SendRecv),
            1 => Ok(Self::SendOnly),
            2 => Ok(Self::RecvOnly),
            3 => Ok(Self::Inactive),
            _ => Err(value),
        }
    }
}

/// Returns a [`Dart_Handle`] dereferenced from the provided pointer.
// false positive: dereferencing raw mutable pointers in const is unstable
#[allow(clippy::missing_const_for_fn)]
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
    pub unsafe extern "C" fn fire_panic() {
        set_panic_hook();
        propagate_panic(|| panic!("Panicking"));
    }
}
