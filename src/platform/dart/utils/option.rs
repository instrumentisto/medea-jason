//! Definitions and implementations of the of Dart nullable values converters to
//! the Rust side [`Option`]s.

use std::{os::raw::c_char, ptr};

use dart_sys::Dart_Handle;
use derive_more::From;

use crate::api::c_str_into_string;

/// Pointer to an extern function that checks that provided [`Dart_Handle`] is
/// not null.
type IsSomeFunction = extern "C" fn(Dart_Handle) -> i32;

/// Stores pointer to the [`IsSomeFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut IS_SOME_FUNCTION: Option<IsSomeFunction> = None;

/// Registers the provided [`IsSomeFunction`] as [`IS_SOME_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RustHandleOption__is_some(f: IsSomeFunction) {
    IS_SOME_FUNCTION = Some(f);
}

/// Pointer to an extern function that returns not null [`Dart_Handle`] from the
/// provided one.
type GetFunction = extern "C" fn(Dart_Handle) -> Dart_Handle;

/// Stores pointer to the [`IsSomeFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut GET_FUNCTION: Option<GetFunction> = None;

/// Registers the provided [`GetFunction`] as [`GET_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_RustHandleOption__get(f: GetFunction) {
    GET_FUNCTION = Some(f);
}

/// Representation of the nullable [`Dart_Handle`].
#[derive(From)]
pub struct RustHandleOption(Dart_Handle);

impl From<RustHandleOption> for Option<Dart_Handle> {
    fn from(from: RustHandleOption) -> Self {
        if unsafe { IS_SOME_FUNCTION.unwrap()(from.0) } == 1 {
            Some(unsafe { GET_FUNCTION.unwrap()(from.0) })
        } else {
            None
        }
    }
}

/// Representation of the nullable [`Dart_Handle`].
#[repr(C)]
pub struct DartOption {
    /// [`bool`] flag which indicates that this value is null.
    ///
    /// Actual type isn't [`bool`] because Dart FFI doesn't supports [`bool`]s
    /// in extern structs.
    is_some: i8,
    val: Dart_Handle,
}

impl From<DartOption> for Option<Dart_Handle> {
    fn from(from: DartOption) -> Self {
        if from.is_some == 1 {
            Some(from.val)
        } else {
            None
        }
    }
}

/// Representation of the nullable Dart [`String`].
#[repr(C)]
pub struct DartStringOption {
    /// [`bool`] flag which indicates that this value is null.
    ///
    /// Actual type isn't [`bool`] because Dart FFI doesn't supports [`bool`]s
    /// in extern structs.
    is_some: i8,

    /// Pointer to the [`String`].
    val: ptr::NonNull<c_char>,
}

impl From<DartStringOption> for Option<String> {
    fn from(from: DartStringOption) -> Self {
        if from.is_some == 1 {
            unsafe { Some(c_str_into_string(from.val)) }
        } else {
            None
        }
    }
}

/// Representation of the nullable Dart [`i32`].
#[repr(C)]
pub struct DartIntOption {
    /// [`bool`] flag which indicates that this value is null.
    ///
    /// Actual type isn't [`bool`] because Dart FFI doesn't supports [`bool`]s
    /// in extern structs.
    is_some: i8,

    /// Actual value if [`DartIntOption::is_some`] is not `false`.
    val: i32,
}

impl From<DartIntOption> for Option<i32> {
    fn from(from: DartIntOption) -> Self {
        if from.is_some == 1 {
            Some(from.val)
        } else {
            None
        }
    }
}

/// Representation of the nullable Dart [`u32`].
#[repr(C)]
pub struct DartUIntOption {
    /// [`bool`] flag which indicates that this value is null.
    ///
    /// Actual type isn't [`bool`] because Dart FFI doesn't supports [`bool`]s
    /// in extern structs.
    is_some: i8,

    /// Actual value if [`DartIntOption::is_some`] is not `false`.
    val: u32,
}

impl From<DartUIntOption> for Option<u32> {
    fn from(from: DartUIntOption) -> Self {
        if from.is_some == 1 {
            Some(from.val)
        } else {
            None
        }
    }
}
