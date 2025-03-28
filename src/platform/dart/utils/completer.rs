//! Proxy for a Dart [Completer].
//!
//! Rust doesn't have a direct access to a Dart [Completer], but holds a
//! [`Dart_PersistentHandle`] to the [Completer] instance. All manipulations
//! happen on the Dart side.
//!
//! Dart side must register these function during the FFI initialization phase:
//! after Dart DL API is initialized and before any other exported Rust function
//! is called.
//!
//! [Completer]: https://api.dart.dev/dart-async/Completer-class.html

use std::{marker::PhantomData, time::Duration};

use dart_sys::{Dart_Handle, Dart_PersistentHandle};
use medea_macro::dart_bridge;

use crate::{
    api::{DartValue, Error as DartError},
    platform::dart::utils::{dart_api, dart_future::FutureFromDart},
};

#[dart_bridge("flutter/lib/src/native/ffi/completer.g.dart")]
mod completer {
    use dart_sys::Dart_Handle;

    use crate::{
        api::{DartValue, Error as DartError},
        platform::Error,
    };

    extern "C" {
        /// Returns a [`Dart_Handle`] to a new Dart [Completer].
        ///
        /// [Completer]: https://api.dart.dev/dart-async/Completer-class.html
        pub fn init() -> Result<Dart_Handle, Error>;

        /// Pointer to an extern function that invokes the [complete()] method
        /// with the provided [`DartValue`] on the provided
        /// [`Dart_Handle`] pointing to the Dart [Completer] object.
        ///
        /// [complete()]:
        /// https://api.dart.dev/dart-async/Completer/complete.html
        /// [Completer]: https://api.dart.dev/dart-async/Completer-class.html
        pub fn complete(fut: Dart_Handle, val: DartValue) -> Result<(), Error>;

        /// Invokes the [completeError()][1] method with the provided
        /// [`DartError`] on the provided [`Dart_Handle`] pointing to the Dart
        /// [Completer] object.
        ///
        /// [1]: https://api.dart.dev/dart-async/Completer/completeError.html
        /// [Completer]: https://api.dart.dev/dart-async/Completer-class.html
        pub fn complete_error(
            fut: Dart_Handle,
            val: DartError,
        ) -> Result<(), Error>;

        /// Calls the [future] getter on the provided [`Dart_Handle`] pointing
        /// to the Dart [Completer] object.
        ///
        /// This function will return [`Dart_Handle`] to the Dart [Future] which
        /// can be returned to the Dart side.
        ///
        /// [future]: https://api.dart.dev/dart-async/Completer/future.html
        /// [Completer]: https://api.dart.dev/dart-async/Completer-class.html
        /// [Future]: https://api.dart.dev/dart-async/Future-class.html
        pub fn future(fut: Dart_Handle) -> Result<Dart_Handle, Error>;

        /// Returns a [`Dart_Handle`] to the Dart `Future` waiting for the
        /// provided amount of time.
        pub fn delayed(delay_ms: i32) -> Result<Dart_Handle, Error>;
    }
}

/// [`Future`] which resolves after the provided [`Duration`].
///
/// # Panics
///
/// Panics if the `DELAYED_FUTURE_FUNCTION` isn't set by the Dart side. This
/// should be an impossible case.
pub async fn delay_for(delay: Duration) {
    #[expect( // overflow is unexpected
        clippy::cast_possible_truncation,
        reason = "overflow is unexpected",
    )]
    let delay = delay.as_millis() as i32;
    let delayed = unsafe { completer::delayed(delay) }.unwrap();
    let delayed_fut = unsafe { FutureFromDart::execute::<()>(delayed) };
    delayed_fut.await.unwrap();
}

/// Dart [Future] which can be resolved from Rust.
///
/// [Future]: https://api.dart.dev/dart-async/Future-class.html
#[derive(Debug)]
pub struct Completer<T, E> {
    /// [`Dart_PersistentHandle`] to the Dart [Completer][1] backing this
    /// [`Completer`].
    ///
    /// [1]: https://api.dart.dev/dart-async/Completer-class.html
    handle: Dart_PersistentHandle,

    /// Type with which [Future] can be successfully resolved.
    ///
    /// [Future]: https://api.dart.dev/dart-async/Future-class.html
    _success_kind: PhantomData<*const T>,

    /// Type with which [Future] can be resolved on error.
    ///
    /// [Future]: https://api.dart.dev/dart-async/Future-class.html
    _error_kind: PhantomData<*const E>,
}

impl<T, E> Completer<T, E> {
    /// Creates a new [`Dart_PersistentHandle`] for the Dart [Completer][1].
    ///
    /// Persists the created [`Dart_Handle`] so it won't be moved by the Dart VM
    /// GC.
    ///
    /// [1]: https://api.dart.dev/dart-async/Completer-class.html
    #[must_use]
    pub fn new() -> Self {
        let completer = unsafe { completer::init() }.unwrap();
        let handle = unsafe { dart_api::new_persistent_handle(completer) };
        Self { handle, _success_kind: PhantomData, _error_kind: PhantomData }
    }

    /// Returns a [`Dart_Handle`] to the Dart [Future] controlled by this
    /// [`Completer`].
    ///
    /// [Future]: https://api.dart.dev/dart-async/Future-class.html
    #[must_use]
    pub fn future(&self) -> Dart_Handle {
        let handle = unsafe { dart_api::handle_from_persistent(self.handle) };
        unsafe { completer::future(handle) }.unwrap()
    }
}

impl<T, E> Default for Completer<T, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Into<DartValue>, E> Completer<T, E> {
    /// Successfully completes the underlying Dart [Future] with the provided
    /// argument.
    ///
    /// [Future]: https://api.dart.dev/dart-async/Future-class.html
    pub fn complete(&self, arg: T) {
        let handle = unsafe { dart_api::handle_from_persistent(self.handle) };
        unsafe { completer::complete(handle, arg.into()) }.unwrap();
    }
}

impl<T> Completer<T, DartError> {
    /// Completes the underlying Dart [Future] with the provided [`DartError`].
    ///
    /// [Future]: https://api.dart.dev/dart-async/Future-class.html
    pub fn complete_error(&self, e: DartError) {
        let handle = unsafe { dart_api::handle_from_persistent(self.handle) };
        unsafe { completer::complete_error(handle, e) }.unwrap();
    }
}
