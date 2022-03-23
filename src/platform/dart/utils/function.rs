//! Functionality for calling Dart closures from Rust.
//!
//! Dart DL API doesn't allow calling Dart closures directly. So Dart registers
//! a static function that accepts and invokes the provided Dart closures:
//! `register_fn_caller`.
//!
//! `register_fn_caller` function MUST be registered by Dart during FFI
//! initialization phase: after Dart DL API is initialized and before any other
//! exported Rust function is called.

use std::marker::PhantomData;

use dart_sys::{Dart_Handle, Dart_PersistentHandle};
use medea_macro::dart_bridge;

use crate::{api::DartValue, platform::Callback};

use super::dart_api::{
    Dart_DeletePersistentHandle_DL_Trampolined,
    Dart_HandleFromPersistent_DL_Trampolined,
    Dart_NewPersistentHandle_DL_Trampolined,
};

#[dart_bridge("flutter/lib/src/native/ffi/function.g.dart")]
mod function {
    use dart_sys::Dart_Handle;

    use crate::api::DartValue;

    /// Invokes other Dart closures that accept a [`DartValue`] argument.
    extern "C" {
        pub fn caller(f: Dart_Handle, val: DartValue);
    }
}

impl<A: Into<DartValue>> Callback<A> {
    /// Invokes the underlying [`Function`] (if any) passing the single provided
    /// argument to it.
    pub fn call1<T: Into<A>>(&self, arg: T) {
        if let Some(f) = self.0.borrow().as_ref() {
            f.call1(arg.into());
        }
    }
}

// TODO: Print exception if Dart closure throws.
/// Dart closure that can be called from Rust.
#[derive(Debug)]
pub struct Function<T> {
    /// [`Dart_PersistentHandle`] to the Dart closure that should be called.
    dart_fn: Dart_PersistentHandle,

    /// Type of this closure argument.
    _arg: PhantomData<*const T>,
}

impl<T> Function<T> {
    /// Creates a new [`Function`] from the provided [`Dart_Handle`] to a Dart
    /// closure, and persists the provided [`Dart_Handle`] so it won't be moved
    /// by the Dart VM GC.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[must_use]
    pub fn new(cb: Dart_Handle) -> Self {
        Self {
            dart_fn: unsafe { Dart_NewPersistentHandle_DL_Trampolined(cb) },
            _arg: PhantomData,
        }
    }
}

impl Function<()> {
    /// Calls the underlying Dart closure.
    pub fn call0(&self) {
        self.call1(());
    }
}

impl<T: Into<DartValue>> Function<T> {
    /// Calls the underlying Dart closure with the provided argument.
    pub fn call1(&self, arg: T) {
        unsafe {
            let fn_handle =
                Dart_HandleFromPersistent_DL_Trampolined(self.dart_fn);
            function::caller(fn_handle, arg.into());
        }
    }
}

impl<T> Drop for Function<T> {
    /// Manually deallocates saved [`Dart_PersistentHandle`] so it won't leak.
    fn drop(&mut self) {
        unsafe {
            Dart_DeletePersistentHandle_DL_Trampolined(self.dart_fn);
        }
    }
}
