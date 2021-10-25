//! Definitions and implementation of the Rust side representation of the Dart
//! Futures.

use std::{convert::TryInto, fmt::Debug, future::Future, ptr};

use dart_sys::Dart_Handle;
use futures::channel::oneshot;

use crate::{
    api::{DartValue, DartValueArg},
    platform::dart::error::Error,
};

/// Pointer to an extern function that resolves provided Dart `Future` with a
/// provided [`FutureFromDart`].
type FutureFromDartCompleteProxyFunction =
    extern "C" fn(Dart_Handle, ptr::NonNull<FutureFromDart>);

/// Stores pointer to the [`FutureFromDartSpawnerFunction`] extern
/// function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut FUTURE_FROM_DART_COMPLETE_PROXY: Option<
    FutureFromDartCompleteProxyFunction,
> = None;

/// Registers the provided [`FutureFromDartSpawnerFunction`] as
/// [`FUTURE_FROM_DART_COMPLETE_PROXY`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_FutureFromDart__complete_proxy(
    f: FutureFromDartCompleteProxyFunction,
) {
    FUTURE_FROM_DART_COMPLETE_PROXY = Some(f);
}

/// Resolves provided [`FutureFromDart`] with a provided
/// [`DartValue`] as `Ok` result.
///
/// Frees provided [`FutureFromDart`].
///
/// # Safety
///
/// Provided [`FutureFromDart`] shouldn't be freed.
#[no_mangle]
pub unsafe extern "C" fn FutureFromDart__resolve_ok(
    future: ptr::NonNull<FutureFromDart>,
    val: DartValue,
) {
    let future = Box::from_raw(future.as_ptr());
    future.resolve_ok(val);
}

/// Resolves provided [`FutureFromDart`] with a provided
/// [`Error`] as `Err` result.
///
/// Frees provided [`FutureFromDart`].
///
/// # Safety
///
/// Provided [`FutureFromDart`] shouldn't be freed.
#[no_mangle]
pub unsafe extern "C" fn FutureFromDart__resolve_err(
    future: ptr::NonNull<FutureFromDart>,
    val: Dart_Handle,
) {
    let future = Box::from_raw(future.as_ptr());
    future.resolve_err(Error::from(val));
}

/// Compatibility layer for polling Dart futures in Rust.
pub struct FutureFromDart(Box<dyn FnOnce(Result<DartValue, Error>)>);

impl FutureFromDart {
    /// Converts fallible Dart side Future to the Rust's [`Future`].
    ///
    /// Returned [`Future`] will be resolved with a requested [`DartValueArg`]
    /// result on Dart side Future resolve.
    ///
    /// # Errors
    ///
    /// Errors with a [`Error`] if Dart side thrown exception.
    pub fn execute<T>(
        dart_fut: Dart_Handle,
    ) -> impl Future<Output = Result<T, Error>>
    where
        DartValueArg<T>: TryInto<T>,
        <DartValueArg<T> as TryInto<T>>::Error: Debug,
        T: 'static,
    {
        let (tx, rx) = oneshot::channel();
        let this = Self(Box::new(|res| {
            drop(tx.send(
                res.map(|val| DartValueArg::<T>::from(val).try_into().unwrap()),
            ));
        }));

        unsafe {
            FUTURE_FROM_DART_COMPLETE_PROXY.unwrap()(
                dart_fut,
                ptr::NonNull::from(Box::leak(Box::new(this))),
            );
        }

        async move { rx.await.unwrap() }
    }

    /// Resolves this [`FutureFromDart`] with a provided
    /// [`DartValue`] as `Ok` result.
    ///
    /// __Should be only called by Dart side.__
    fn resolve_ok(self, val: DartValue) {
        (self.0)(Ok(val));
    }

    /// Resolves this [`FutureFromDart`] with a provided
    /// [`Error`] as `Err` result.
    ///
    /// __Should be only called by Dart side.__
    fn resolve_err(self, err: Error) {
        (self.0)(Err(err));
    }
}

#[cfg(feature = "mockable")]
pub mod tests {
    use dart_sys::Dart_Handle;

    use crate::{
        api::{
            err::FormatException,
            utils::{DartFuture, IntoDartFuture as _},
        },
        platform::dart::utils::{
            dart_future::FutureFromDart, handle::DartHandle,
        },
    };

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
            unsafe { (TEST_FUTURE_HANDLE_FUNCTION.unwrap())(val.get().into()) }
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
            Ok(if val.is_err() { 1 } else { 0 })
        }
        .into_dart_future()
    }
}
