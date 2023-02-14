//! Rust-side representation of a [Dart `Future`][0].
//!
//! [0]: https://api.dart.dev/stable/dart-async/Future-class.html

use std::{fmt, future::Future, marker::PhantomData, ptr};

use dart_sys::Dart_Handle;
use flutter_rust_bridge::DartOpaque;
use futures::channel::oneshot;
use medea_macro::dart_bridge;

use crate::{
    api::{
        propagate_panic, utils::new_dart_opaque, DartValue, DartValueArg,
        Error as DartError,
    },
    platform::{
        dart::{error::Error, utils::Completer},
        spawn,
        utils::handle::DartHandle,
    },
};

#[dart_bridge("flutter/lib/src/native/ffi/future.g.dart")]
mod future_from_dart {
    use std::ptr;

    use dart_sys::Dart_Handle;

    use crate::platform::dart::utils::dart_future::FutureFromDart;

    /// Resolves the provided [Dart `Future`][0] with the provided
    /// [`FutureFromDart`].
    ///
    /// [0]: https://api.dart.dev/stable/dart-async/Future-class.html
    extern "C" {
        pub fn complete_proxy(
            fut: Dart_Handle,
            resolver: ptr::NonNull<FutureFromDart>,
        );
    }
}

/// Resolves the provided [`FutureFromDart`] with the given [`DartValue`] as
/// [`Ok`] result.
///
/// Frees the provided [`FutureFromDart`].
///
/// # Safety
///
/// The provided [`FutureFromDart`] shouldn't be previously freed.
#[no_mangle]
pub unsafe extern "C" fn FutureFromDart__resolve_ok(
    future: ptr::NonNull<FutureFromDart>,
    val: DartValue,
) {
    propagate_panic(move || {
        let future = Box::from_raw(future.as_ptr());
        future.resolve_ok(val);
    });
}

/// Resolves the provided [`FutureFromDart`] with the given [`Error`] as [`Err`]
/// result.
///
/// Frees the provided [`FutureFromDart`].
///
/// # Safety
///
/// The provided [`FutureFromDart`] shouldn't be previously freed.
#[no_mangle]
pub unsafe extern "C" fn FutureFromDart__resolve_err(
    future: ptr::NonNull<FutureFromDart>,
    val: Dart_Handle,
) {
    propagate_panic(move || {
        let future = Box::from_raw(future.as_ptr());
        future.resolve_err(Error::from_handle(val));
    });
}

/// Compatibility layer for polling [Dart `Future`s][0] in Rust.
///
/// [0]: https://api.dart.dev/stable/dart-async/Future-class.html
pub struct FutureFromDart(Box<dyn FnOnce(Result<DartValue, Error>)>);

impl fmt::Debug for FutureFromDart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("FutureFromDart")
            .field(&format!("{:p}", self.0))
            .finish()
    }
}

impl FutureFromDart {
    /// Converts a fallible [Dart `Future`s][0] into the Rust [`Future`].
    ///
    /// Returned [`Future`] will be resolved with a requested [`DartValueArg`]
    /// result on a Dart side.
    ///
    /// # Safety
    ///
    /// The provided [`Dart_Handle`] should be non-`null` and point to the
    /// correct [Dart `Future`][0].
    ///
    /// # Errors
    ///
    /// Errors with an [`Error`] if Dart side thrown an exception.
    ///
    /// [0]: https://api.dart.dev/stable/dart-async/Future-class.html
    pub unsafe fn execute<T>(
        dart_fut: Dart_Handle,
    ) -> impl Future<Output = Result<T, Error>>
    where
        DartValueArg<T>: TryInto<T>,
        <DartValueArg<T> as TryInto<T>>::Error: fmt::Debug,
        T: 'static,
    {
        let dart_fut = DartHandle::new(dart_fut);
        let (tx, rx) = oneshot::channel();
        let this = Self(Box::new(|res| {
            drop(tx.send(
                res.map(|val| DartValueArg::<T>::from(val).try_into().unwrap()),
            ));
        }));

        future_from_dart::complete_proxy(
            dart_fut.get(),
            ptr::NonNull::from(Box::leak(Box::new(this))),
        );

        async move { rx.await.unwrap() }
    }

    /// Resolves this [`FutureFromDart`] with the provided [`DartValue`] as
    /// [`Ok`] result.
    ///
    /// __Should be only called by Dart side.__
    fn resolve_ok(self, val: DartValue) {
        (self.0)(Ok(val));
    }

    /// Resolves this [`FutureFromDart`] with the provided [`Error`] as [`Err`]
    /// result.
    ///
    /// __Should be only called by Dart side.__
    fn resolve_err(self, err: Error) {
        (self.0)(Err(err));
    }
}

/// Rust representation of a Dart [`Future`].
///
/// [`Future`]: https://api.dart.dev/dart-async/Future-class.html

#[derive(Debug)]
#[repr(transparent)]
pub struct DartFuture<O>(
    #[allow(unused_tuple_struct_fields)] Dart_Handle, // read by Dart side
    PhantomData<*const O>,
);

impl<O> DartFuture<O> {
    /// Wraps the given [`DartFuture`] into a [`DartOpaque`] so it can be
    /// transferred to Dart side via `flutter_rust_bridge` bindings.
    #[must_use]
    pub fn into_dart_opaque(self) -> DartOpaque {
        unsafe { new_dart_opaque(self.0) }
    }
}

/// Extension trait for a [`Future`] allowing to convert Rust [`Future`]s to
/// [`DartFuture`]s.
pub trait IntoDartFuture {
    /// The type of the value produced on the [`DartFuture`]'s completion.
    type Output;

    /// Converts this [`Future`] into a Dart `Future`.
    ///
    /// Returns a [`Dart_Handle`] to the created Dart `Future`.
    ///
    /// __Note, that the Dart `Future` execution begins immediately and cannot
    /// be canceled.__
    fn into_dart_future(self) -> DartFuture<Self::Output>;
}

impl<Fut, Ok, Err> IntoDartFuture for Fut
where
    Fut: Future<Output = Result<Ok, Err>> + 'static,
    Ok: Into<DartValue> + 'static,
    Err: Into<DartError>,
{
    type Output = Fut::Output;

    fn into_dart_future(self) -> DartFuture<Fut::Output> {
        let completer = Completer::new();
        let dart_future = completer.future();
        spawn(async move {
            match self.await {
                Ok(ok) => {
                    completer.complete(ok);
                }
                Err(e) => {
                    completer.complete_error(e.into());
                }
            }
        });
        DartFuture(dart_future, PhantomData)
    }
}

#[cfg(feature = "mockable")]
pub mod tests {
    #![allow(clippy::missing_safety_doc)]

    use dart_sys::Dart_Handle;

    use crate::{
        api::err::FormatException,
        platform::dart::utils::{
            dart_future::FutureFromDart, handle::DartHandle,
        },
    };

    use super::{DartFuture, IntoDartFuture};

    #[no_mangle]
    pub unsafe extern "C" fn test__future_from_dart__int(
        future: Dart_Handle,
    ) -> DartFuture<Result<i64, FormatException>> {
        let future = DartHandle::new(future);
        async move {
            let val =
                FutureFromDart::execute::<i64>(future.get()).await.unwrap();
            Ok(val)
        }
        .into_dart_future()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test__future_from_dart__string(
        future: Dart_Handle,
    ) -> DartFuture<Result<String, FormatException>> {
        let future = DartHandle::new(future);
        async move {
            let val = FutureFromDart::execute::<String>(future.get())
                .await
                .unwrap();
            Ok(val)
        }
        .into_dart_future()
    }

    type TestFutureHandleFunction = extern "C" fn(Dart_Handle);

    static mut TEST_FUTURE_HANDLE_FUNCTION: Option<TestFutureHandleFunction> =
        None;

    #[no_mangle]
    pub unsafe extern "C" fn register__test__future_from_dart_handle_fn(
        f: TestFutureHandleFunction,
    ) {
        TEST_FUTURE_HANDLE_FUNCTION = Some(f);
    }

    #[no_mangle]
    pub unsafe extern "C" fn test__future_from_dart__handle(
        future: Dart_Handle,
    ) -> DartFuture<Result<(), FormatException>> {
        let future = DartHandle::new(future);
        async move {
            let val = FutureFromDart::execute::<DartHandle>(future.get())
                .await
                .unwrap();
            (TEST_FUTURE_HANDLE_FUNCTION.unwrap())(val.get());
            Ok(())
        }
        .into_dart_future()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test__future_from_dart__fails(
        future: Dart_Handle,
    ) -> DartFuture<Result<i64, FormatException>> {
        let future = DartHandle::new(future);
        async move {
            let val = FutureFromDart::execute::<i64>(future.get()).await;
            Ok(val.is_err().into())
        }
        .into_dart_future()
    }
}
