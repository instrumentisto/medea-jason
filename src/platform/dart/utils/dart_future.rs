//! Definitions and implementation of the Rust side representation of the Dart
//! Futures.

use std::future::Future;

use dart_sys::Dart_Handle;
use futures::channel::oneshot;

use crate::{
    api::{DartValue, DartValueArg},
    platform::dart::error::Error as DartError,
};
use std::{convert::TryInto, fmt::Debug};

/// Pointer to an extern function that resolves provided Dart `Future` with a
/// provided [`DartFutureResolver`].
type DartFutureResolverSpawnerFunction =
    extern "C" fn(Dart_Handle, *mut DartFutureResolver);

/// Stores pointer to the [`DartFutureResolverSpawnerFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut DART_FUTURE_RESOLVER_SPAWNER: Option<
    DartFutureResolverSpawnerFunction,
> = None;

/// Registers the provided [`DartFutureResolverSpawnerFunction`] as
/// [`DART_FUTURE_RESOLVER_SPAWNER`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_DartFutureResolver__spawner(
    f: DartFutureResolverSpawnerFunction,
) {
    DART_FUTURE_RESOLVER_SPAWNER = Some(f);
}

/// Resolves provided [`DartFutureResolver`] with a provided [`DartValue`].
///
/// Frees provided [`DartFutureResolver`].
///
/// # Safety
///
/// Provided [`DartFutureResolver`] shouldn't be freed.
#[no_mangle]
pub unsafe extern "C" fn DartFutureResolver__resolve(
    fut: *mut DartFutureResolver,
    val: DartValue,
) {
    let fut = Box::from_raw(fut);
    fut.resolve(val);
}

/// Compatibility layer of the infallible Dart side Futures with a Rust side
/// [`Future`].
pub struct DartFutureResolver(Box<dyn FnOnce(DartValue)>);

impl DartFutureResolver {
    /// Converts infallible Dart side Future to the Rust's [`Future`].
    ///
    /// Returned [`Future`] will be resolved with a requested [`DartValueArg`]
    /// result on Dart side Future resolve.
    pub fn execute<T>(dart_fut: Dart_Handle) -> impl Future<Output = T>
    where
        DartValueArg<T>: TryInto<T>,
        <DartValueArg<T> as TryInto<T>>::Error: Debug,
        T: 'static,
    {
        let (tx, rx) = oneshot::channel();
        let this = Self(Box::new(|val| {
            drop(tx.send(DartValueArg::<T>::from(val).try_into().unwrap()));
        }));

        unsafe {
            DART_FUTURE_RESOLVER_SPAWNER.unwrap()(
                dart_fut,
                Box::into_raw(Box::new(this)),
            );
        }

        async move { rx.await.unwrap() }
    }

    /// Resolves this [`DartFutureResolver`] with a provided [`DartValue`] as a
    /// result.
    ///
    /// __Should be only called by Dart side.__
    fn resolve(self, val: DartValue) {
        (self.0)(val);
    }
}

/// Pointer to an extern function that resolves provided Dart `Future` with a
/// provided [`FallibleDartFutureResolver`].
type FallibleDartFutureResolverSpawnerFunction =
    extern "C" fn(Dart_Handle, *mut FallibleDartFutureResolver);

/// Stores pointer to the [`FallibleDartFutureResolverSpawnerFunction`] extern
/// function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut FALLIBLE_DART_FUTURE_RESOLVER_SPAWNER: Option<
    FallibleDartFutureResolverSpawnerFunction,
> = None;

/// Registers the provided [`FallibleDartFutureResolverSpawnerFunction`] as
/// [`FALLIBLE_DART_FUTURE_RESOLVER_SPAWNER`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_FallibleDartFutureResolver__spawner(
    f: FallibleDartFutureResolverSpawnerFunction,
) {
    FALLIBLE_DART_FUTURE_RESOLVER_SPAWNER = Some(f);
}

/// Resolves provided [`FallibleDartFutureResolver`] with a provided
/// [`DartValue`] as `Ok` result.
///
/// Frees provided [`DartFutureResolver`].
///
/// # Safety
///
/// Provided [`FallibleDartFutureResolver`] shouldn't be freed.
#[no_mangle]
pub unsafe extern "C" fn FallibleDartFutureResolver__resolve_ok(
    fut: *mut FallibleDartFutureResolver,
    val: DartValue,
) {
    let fut = Box::from_raw(fut);
    fut.resolve_ok(val);
}

/// Resolves provided [`FallibleDartFutureResolver`] with a provided
/// [`DartError`] as `Err` result.
///
/// Frees provided [`DartFutureResolver`].
///
/// # Safety
///
/// Provided [`FallibleDartFutureResolver`] shouldn't be freed.
#[no_mangle]
pub unsafe extern "C" fn FallibleDartFutureResolver__resolve_err(
    fut: *mut FallibleDartFutureResolver,
    val: Dart_Handle,
) {
    let fut = Box::from_raw(fut);
    fut.resolve_err(DartError::from(val));
}

/// Compatibility layer of the fallible Dart side Futures with a Rust side
/// [`Future`].
pub struct FallibleDartFutureResolver(
    Box<dyn FnOnce(Result<DartValue, DartError>)>,
);

impl FallibleDartFutureResolver {
    /// Converts fallible Dart side Future to the Rust's [`Future`].
    ///
    /// Returned [`Future`] will be resolved with a requested [`DartValueArg`]
    /// result on Dart side Future resolve.
    ///
    /// # Errors
    ///
    /// Errors with a [`DartError`] if Dart side thrown exception.
    pub fn execute<T>(
        dart_fut: Dart_Handle,
    ) -> impl Future<Output = Result<T, DartError>>
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
            FALLIBLE_DART_FUTURE_RESOLVER_SPAWNER.unwrap()(
                dart_fut,
                Box::into_raw(Box::new(this)),
            );
        }

        async move { rx.await.unwrap() }
    }

    /// Resolves this [`FallibleDartFutureResolver`] with a provided
    /// [`DartValue`] as `Ok` result.
    ///
    /// __Should be only called by Dart side.__
    fn resolve_ok(self, val: DartValue) {
        (self.0)(Ok(val));
    }

    /// Resolves this [`FallibleDartFutureResolver`] with a provided
    /// [`DartError`] as `Err` result.
    ///
    /// __Should be only called by Dart side.__
    fn resolve_err(self, err: DartError) {
        (self.0)(Err(err));
    }
}

#[cfg(feature = "mockable")]
pub mod tests {
    use std::ptr;

    use dart_sys::Dart_Handle;

    use crate::{
        api::{
            err::FormatException,
            utils::{DartFuture, IntoDartFuture as _},
        },
        platform::dart::utils::{
            dart_future::FallibleDartFutureResolver, handle::DartHandle,
        },
    };

    use super::DartFutureResolver;

    #[no_mangle]
    pub unsafe extern "C" fn test__dart_future_resolver__int(
        fut: Dart_Handle,
    ) -> DartFuture<Result<i64, FormatException>> {
        let fut = DartHandle::new(fut);
        async move {
            let val = DartFutureResolver::execute::<i64>(fut.get()).await;
            Ok(val)
        }
        .into_dart_future()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test__dart_future_resolver__string(
        fut: Dart_Handle,
    ) -> DartFuture<Result<String, FormatException>> {
        let fut = DartHandle::new(fut);
        async move {
            let val = DartFutureResolver::execute::<String>(fut.get()).await;
            Ok(val)
        }
        .into_dart_future()
    }

    type TestFutureHandleFunction = extern "C" fn(ptr::NonNull<Dart_Handle>);

    static mut TEST_FUTURE_HANDLE_FUNCTION: Option<TestFutureHandleFunction> =
        None;

    #[no_mangle]
    pub unsafe extern "C" fn register__test__dart_future_resolver_handle_fn(
        f: TestFutureHandleFunction,
    ) {
        TEST_FUTURE_HANDLE_FUNCTION = Some(f);
    }

    #[no_mangle]
    pub unsafe extern "C" fn test__dart_future_resolver__handle(
        fut: Dart_Handle,
    ) -> DartFuture<Result<(), FormatException>> {
        let fut = DartHandle::new(fut);
        async move {
            let val = DartFutureResolver::execute::<ptr::NonNull<Dart_Handle>>(
                fut.get(),
            )
            .await;
            unsafe { (TEST_FUTURE_HANDLE_FUNCTION.unwrap())(val) }
            Ok(())
        }
        .into_dart_future()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test__fallible_dart_future_resolver__fails(
        fut: Dart_Handle,
    ) -> DartFuture<Result<i64, FormatException>> {
        let fut = DartHandle::new(fut);
        async move {
            let val =
                FallibleDartFutureResolver::execute::<i64>(fut.get()).await;
            Ok(if val.is_err() { 1 } else { 0 })
        }
        .into_dart_future()
    }
}
