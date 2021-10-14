//! Definitions and implementations of the Rust side representation of the
//! nullable Dart values.

use std::ptr;

use std::os::raw::c_char;

use crate::api::string_into_c_str;

// TODO: Может просто нул-пойнтер прокидывать? Ну и чекать на is_null.
//       Оно вообще точно нужно? Есть DartValue.
/// Rust side representation of the nullable Dart [`String`].
#[repr(C)]
pub struct NullableChar {
    /// [`bool`] flag which indicates that this value is null.
    ///
    /// Actual type isn't [`bool`] because Dart FFI doesn't supports [`bool`]s
    /// in extern structs.
    pub is_some: i8,

    /// Pointer to the Dart side [`String`].
    ///
    /// __SAFETY:__ do not use this pointer if [`NullableChar::is_some`] is
    /// `false`.
    pub value: *const c_char,
}

impl From<Option<String>> for NullableChar {
    fn from(from: Option<String>) -> Self {
        from.map_or_else(
            || Self {
                value: ptr::null(),
                is_some: 0,
            },
            |from| Self {
                value: string_into_c_str(from).as_ptr(),
                is_some: 1,
            },
        )
    }
}

// TODO: Оно вообще точно нужно? Есть DartValue.
/// Rust side representation of the nullable Dart [`i32`].
#[repr(C)]
pub struct NullableInt {
    /// [`bool`] flag which indicates that this value is null.
    ///
    /// Actual type isn't [`bool`] because Dart FFI doesn't supports [`bool`]s
    /// in extern structs.
    pub is_some: i8,

    /// Pointer to the Dart side [`String`].
    ///
    /// __SAFETY:__ do not use this pointer if [`NullableChar::is_some`] is
    /// `false`.
    pub value: i32,
}

impl From<Option<i32>> for NullableInt {
    fn from(from: Option<i32>) -> Self {
        from.map_or_else(
            || Self {
                value: 0,
                is_some: 0,
            },
            |from| Self {
                value: from,
                is_some: 1,
            },
        )
    }
}

impl From<Option<u16>> for NullableInt {
    fn from(from: Option<u16>) -> Self {
        from.map(i32::from).into()
    }
}
