//! Definitions and implementation of the Rust side representation of the Dart
//! Futures.

use std::future::Future;

use dart_sys::Dart_Handle;
use futures::channel::oneshot;

use crate::api::{DartValue, DartValueArg};

type DartFutureResolverSpawnerFunction =
    extern "C" fn(Dart_Handle, *mut DartFutureResolver);

static mut DART_FUTURE_RESOLVER_SPAWNER: Option<
    DartFutureResolverSpawnerFunction,
> = None;

#[no_mangle]
pub unsafe extern "C" fn register_DartFutureResolver__spawner(
    f: DartFutureResolverSpawnerFunction,
) {
    DART_FUTURE_RESOLVER_SPAWNER = Some(f);
}

#[no_mangle]
pub unsafe extern "C" fn DartFutureResolver__resolve(
    fut: *mut DartFutureResolver,
    val: DartValue,
) {
    let fut = Box::from_raw(fut);
    fut.resolve(val);
}

pub struct DartFutureResolver(oneshot::Sender<DartValue>);

impl DartFutureResolver {
    pub fn execute<T>(
        dart_fut: Dart_Handle,
    ) -> impl Future<Output = DartValueArg<T>> {
        let (tx, rx) = oneshot::channel();
        let this = Self(tx);

        unsafe {
            DART_FUTURE_RESOLVER_SPAWNER.unwrap()(
                dart_fut,
                Box::into_raw(Box::new(this)),
            );
        }

        async move {
            let val: DartValue = rx.await.unwrap();
            DartValueArg::<T>::from(val)
        }
    }

    fn resolve(self, val: DartValue) {
        let _ = self.0.send(val);
    }
}

#[cfg(feature = "mockable")]
pub mod tests {
    use std::{convert::TryInto, ptr};

    use dart_sys::Dart_Handle;

    use crate::{
        api::{
            err::FormatException,
            utils::{DartFuture, IntoDartFuture as _},
        },
        platform::dart::utils::handle::DartHandle,
    };

    use super::DartFutureResolver;

    #[no_mangle]
    pub unsafe extern "C" fn test__dart_future_resolver__int(
        fut: Dart_Handle,
    ) -> DartFuture<Result<i64, FormatException>> {
        async move {
            let val = DartFutureResolver::execute::<i64>(fut).await;
            let val: i64 = val.try_into().unwrap();
            Ok(val)
        }
        .into_dart_future()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test__dart_future_resolver__string(
        fut: Dart_Handle,
    ) -> DartFuture<Result<String, FormatException>> {
        async move {
            let val = DartFutureResolver::execute::<String>(fut).await;
            let val: String = val.try_into().unwrap();
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
            unsafe {
                (TEST_FUTURE_HANDLE_FUNCTION.unwrap())(val.try_into().unwrap())
            }
            Ok(())
        }
        .into_dart_future()
    }
}
