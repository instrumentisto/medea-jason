//! Definitions and implementation of the Rust side representation of the Dart
//! Futures.

use std::{future::Future, os::raw::c_char, ptr};

use dart_sys::Dart_Handle;
use futures::channel::oneshot;

use crate::{
    api::c_str_into_string,
    platform::dart::{error::DartError, utils::handle::DartHandle},
};

/// Pointer to an extern function that spawns Dart Future which returns `Int32`
/// on the Dart side and resolves provided [`IntFuture`] when it's resolved on
/// Dart.
type IntFutureSpawnerFunction = extern "C" fn(Dart_Handle, *mut IntFuture);

/// Stores pointer to the [`IntFutureSpawnerFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut INT_FUTURE_SPAWNER_FUNCTION: Option<IntFutureSpawnerFunction> = None;

/// Registers the provided [`IntFutureSpawnerFunction`] as
/// [`INT_FUTURE_SPAWNER_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_int_future_spawner_function(
    f: IntFutureSpawnerFunction,
) {
    INT_FUTURE_SPAWNER_FUNCTION = Some(f);
}

/// Resolves provided [`IntFuture`] with a provided `val`.
///
/// # Safety
///
/// Should be called only when provided pointer to [`IntFuture`] is not freed.
#[no_mangle]
pub unsafe extern "C" fn IntFuture__resolve(fut: *mut IntFuture, val: i32) {
    let fut = Box::from_raw(fut);
    fut.resolve(val);
}

/// Compatibility layer of the Dart side Futures which resolves into `Int32`
/// with a Rust side [`Future`].
pub struct IntFuture(oneshot::Sender<i32>);

impl IntFuture {
    /// Converts Dart side Future to the Rust's [`Future`].
    ///
    /// Returned [`Future`] will be resolved with a [`i32`] result on Dart side
    /// Future resolve.
    pub fn execute(dart_fut: Dart_Handle) -> impl Future<Output = i32> {
        let (tx, rx) = oneshot::channel();
        let this = Self(tx);

        unsafe {
            INT_FUTURE_SPAWNER_FUNCTION.unwrap()(
                dart_fut,
                Box::into_raw(Box::new(this)),
            );
        }

        async move { rx.await.unwrap() }
    }

    /// Resolves this [`IntFuture`] with a provided `val`.
    ///
    /// __Should be only called by Dart side.__
    pub fn resolve(self, val: i32) {
        let _ = self.0.send(val);
    }
}

/// Compatibility layer of the Dart side Futures which resolved into `String`
/// with a Rust side [`Future`].
pub struct StringFuture(oneshot::Sender<String>);

impl StringFuture {
    /// Converts Dart side Future to the Rust's [`Future`].
    ///
    /// Returned [`Future`] will be resolved with a [`String`] result on Dart
    /// side Future resolve.
    pub fn execute(dart_fut: Dart_Handle) -> impl Future<Output = String> {
        let (tx, rx) = oneshot::channel();
        let this = Self(tx);

        unsafe {
            STRING_FUTURE_SPAWNER_FUNCTION.unwrap()(
                dart_fut,
                Box::into_raw(Box::new(this)),
            );
        }

        async move { rx.await.unwrap() }
    }

    /// Resolves this [`StringFuture`] with a provided `val`.
    ///
    /// __Should be only called by Dart side.__
    pub fn resolve(self, val: ptr::NonNull<c_char>) {
        drop(self.0.send(unsafe { c_str_into_string(val) }));
    }
}

/// Compatibility layer of the Dart side Futures which resolves into
/// [`Dart_Handle`] or throws exception with a Rust side [`Future`].
pub struct DartFuture(oneshot::Sender<Result<DartHandle, DartError>>);

impl DartFuture {
    /// Converts Dart side Future to the Rust's [`Future`].
    ///
    /// Returned [`Future`] will be resolved with a [`DartHandle`] or
    /// [`DartError`] result on Dart side Future resolve.
    ///
    /// # Errors
    ///
    /// Returns [`DartError`] if provided Dart Future throws exception.
    pub fn execute(
        dart_fut: Dart_Handle,
    ) -> impl Future<Output = Result<DartHandle, DartError>> {
        let (tx, rx) = oneshot::channel();
        let this = Self(tx);

        unsafe {
            FUTURE_SPAWNER_FUNCTION.unwrap()(
                dart_fut,
                Box::into_raw(Box::new(this)),
            );
        }

        async move { rx.await.unwrap() }
    }

    /// Resolves this [`DartFuture`] with a provided `val` as `Ok` variant of
    /// [`Result`].
    ///
    /// __Should be only called by Dart side.__
    fn resolve_ok(self, val: Dart_Handle) {
        drop(self.0.send(Ok(DartHandle::new(val))));
    }

    /// Resolves this [`DartFuture`] with a provided `val` as `Err` variant of
    /// [`Result`].
    ///
    /// __Should be only called by Dart side.__
    fn resolve_err(self, val: Dart_Handle) {
        drop(self.0.send(Err(DartError::from(val))));
    }
}

/// Compatibility layer of the Dart side Futures which resolves into `void` with
/// a Rust side [`Future`].
pub struct VoidDartFuture(oneshot::Sender<()>);

impl VoidDartFuture {
    /// Converts Dart side Future to the Rust's [`Future`].
    ///
    /// Returned [`Future`] will be resolved  on Dart side Future resolve.
    pub fn execute(dart_fut: Dart_Handle) -> impl Future<Output = ()> {
        let (tx, rx) = oneshot::channel();
        let this = Self(tx);

        unsafe {
            VOID_FUTURE_SPAWNER_FUNCTION.unwrap()(
                dart_fut,
                Box::into_raw(Box::new(this)),
            );
        }

        async move {
            rx.await.unwrap();
        }
    }

    /// Resolves this [`VoidDartFuture`] with a provided `val`.
    ///
    /// __Should be only called by Dart side.__
    pub fn resolve(self) {
        #[allow(clippy::drop_copy)]
        drop(self.0.send(()));
    }
}

/// Pointer to an extern function that spawns Dart Future on the Dart side and
/// resolves provided [`VoidFuture`] when it's resolved on Dart.
type VoidFutureSpawnerFunction =
    extern "C" fn(Dart_Handle, *mut VoidDartFuture);

/// Stores pointer to the [`VoidFutureSpawnerFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut VOID_FUTURE_SPAWNER_FUNCTION: Option<VoidFutureSpawnerFunction> =
    None;

/// Registers the provided [`VoidFutureSpawnerFunction`] as
/// [`VOID_FUTURE_SPAWNER_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_void_future_spawner_function(
    f: VoidFutureSpawnerFunction,
) {
    VOID_FUTURE_SPAWNER_FUNCTION = Some(f);
}

/// Resolves provided [`VoidDartFuture`].
///
/// # Safety
///
/// Should be called only when provided pointer to [`VoidDartFuture`] is not
/// freed.
#[no_mangle]
pub unsafe extern "C" fn VoidDartFuture__resolve(fut: *mut VoidDartFuture) {
    let fut = Box::from_raw(fut);
    fut.resolve();
}

/// Pointer to an extern function that spawns Dart Future which returns `String`
/// on the Dart side and resolves provided [`VoidFuture`] when it's resolved on
/// Dart.
type StringFutureSpawnerFunction =
    extern "C" fn(Dart_Handle, *mut StringFuture);

/// Stores pointer to the [`StringFutureSpawnerFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut STRING_FUTURE_SPAWNER_FUNCTION: Option<StringFutureSpawnerFunction> =
    None;

/// Registers the provided [`StringFutureSpawnerFunction`] as
/// [`STRING_FUTURE_SPAWNER_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_string_future_spawner_function(
    f: StringFutureSpawnerFunction,
) {
    STRING_FUTURE_SPAWNER_FUNCTION = Some(f);
}

/// Resolves provided [`StringFuture`] with a provided `val`.
///
/// # Safety
///
/// Should be called only when provided pointer to [`StringFuture`] and
/// [`c_char`] pointer is not freed.
#[no_mangle]
pub unsafe extern "C" fn StringFuture__resolve(
    fut: *mut StringFuture,
    val: ptr::NonNull<c_char>,
) {
    let fut = Box::from_raw(fut);
    fut.resolve(val);
}

/// Pointer to an extern function that spawns Dart Future which returns
/// [`Dart_Handle`] on the Dart side and resolves provided [`DartFuture`] when
/// it's resolved on Dart.
type FutureSpawnerFunction = extern "C" fn(Dart_Handle, *mut DartFuture);

/// Stores pointer to the [`FutureSpawnerFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut FUTURE_SPAWNER_FUNCTION: Option<FutureSpawnerFunction> = None;

/// Registers the provided [`FutureSpawnerFunction`] as
/// [`FUTURE_SPAWNER_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_spawn_dart_future_function(
    f: FutureSpawnerFunction,
) {
    FUTURE_SPAWNER_FUNCTION = Some(f);
}

/// Resolves provided [`DartFuture`] with a provided `val` as `Ok` [`Result`]
/// variant.
///
/// # Safety
///
/// Should be called only when provided pointer to [`DartFuture`] and
/// [`Dart_Handle`] pointer is not freed.
#[no_mangle]
pub unsafe extern "C" fn DartFuture__resolve_ok(
    fut: *mut DartFuture,
    val: Dart_Handle,
) {
    let fut = Box::from_raw(fut);
    fut.resolve_ok(val);
}

/// Resolves provided [`DartFuture`] with a provided `val` as `Err` [`Result`]
/// variant.
///
/// # Safety
///
/// Should be called only when provided pointer to [`DartFuture`] and
/// [`Dart_Handle`] pointer is not freed.
#[no_mangle]
pub unsafe extern "C" fn DartFuture__resolve_err(
    fut: *mut DartFuture,
    val: Dart_Handle,
) {
    let fut = Box::from_raw(fut);
    fut.resolve_err(val);
}
