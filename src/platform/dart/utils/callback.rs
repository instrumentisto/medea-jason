//! Functionality for converting Rust closures into callbacks that can be passed
//! to Dart and called by Dart.

use std::{convert::TryInto, fmt::Debug, ptr};

use dart_sys::Dart_Handle;
use medea_macro::dart_bridge;

use crate::api::{DartValue, DartValueArg};

#[dart_bridge("flutter/lib/src/native/ffi/callback.g.dart")]
mod callback {
    use std::ptr;

    use dart_sys::Dart_Handle;

    use crate::platform::dart::utils::callback::Callback;

    extern "C" {
        /// Returns a [`Dart_Handle`] to a newly created Dart callback accepting
        /// 2 arguments that will proxy calls to the given Rust callback.
        pub fn call_two_arg_proxy(cb: ptr::NonNull<Callback>) -> Dart_Handle;

        /// Returns a [`Dart_Handle`] to a newly created Dart callback that will
        /// proxy calls to the associated Rust callback.
        pub fn call_proxy(cb: ptr::NonNull<Callback>) -> Dart_Handle;
    }
}

/// Calls the provided [`Callback`] with the provided two [`DartValue`]s as
/// arguments.
///
/// # Safety
///
/// Provided callback should be a valid [`Callback`] pointer.
#[no_mangle]
pub unsafe extern "C" fn Callback__call_two_arg(
    mut cb: ptr::NonNull<Callback>,
    first: DartValue,
    second: DartValue,
) {
    match &mut cb.as_mut().0 {
        Kind::TwoArgFnMut(func) => (func)(first, second),
        _ => unreachable!(),
    }
}

/// Calls the provided [`Callback`] with the provided [`DartValue`] as an
/// argument.
///
/// # Safety
///
/// Provided [`Callback`] should be a valid [`Callback`] pointer.
#[no_mangle]
pub unsafe extern "C" fn Callback__call(
    mut cb: ptr::NonNull<Callback>,
    val: DartValue,
) {
    if matches!(cb.as_ref().0, Kind::FnOnce(_)) {
        let cb = Box::from_raw(cb.as_ptr());
        if let Kind::FnOnce(func) = cb.0 {
            (func)(val);
        }
    } else {
        match &mut cb.as_mut().0 {
            Kind::FnMut(func) => {
                (func)(val);
            }
            Kind::Fn(func) => {
                (func)(val);
            }
            Kind::FnOnce(_) | Kind::TwoArgFnMut(_) => {
                unreachable!();
            }
        }
    }
}

/// Possible kinds of an underlying [`Callback`] function to be called.
enum Kind {
    FnOnce(Box<dyn FnOnce(DartValue)>),
    FnMut(Box<dyn FnMut(DartValue)>),
    Fn(Box<dyn Fn(DartValue)>),
    TwoArgFnMut(Box<dyn FnMut(DartValue, DartValue)>),
}

// TODO: Fix in #13:
//       1. Requires additional parametrization or(and) wrapping.
//       2. `FnOnce` semantics should be reflected on Dart side somehow.
//       3. `Kind::FnMut` and `Kind::Fn` aren't dropped anywhere right now.
/// Rust closure which can be called by Dart.
#[must_use]
pub struct Callback(Kind);

impl Callback {
    /// Returns a [`Callback`] wrapping the provided [`FnOnce`], that can be
    /// converted to a [`Dart_Handle`] and passed to Dart.
    pub fn from_once<F, T>(f: F) -> Self
    where
        F: FnOnce(T) + 'static,
        DartValueArg<T>: TryInto<T>,
        <DartValueArg<T> as TryInto<T>>::Error: Debug,
        T: 'static,
    {
        Self(Kind::FnOnce(Box::new(move |val: DartValue| {
            let arg = DartValueArg::<T>::from(val);
            (f)(arg.try_into().unwrap());
        })))
    }

    /// Returns a [`Callback`] wrapping the provided [`FnMut`], that can be
    /// converted to a [`Dart_Handle`] and passed to Dart.
    pub fn from_fn_mut<F, T>(mut f: F) -> Self
    where
        F: FnMut(T) + 'static,
        DartValueArg<T>: TryInto<T>,
        <DartValueArg<T> as TryInto<T>>::Error: Debug,
        T: 'static,
    {
        Self(Kind::FnMut(Box::new(move |val: DartValue| {
            let arg = DartValueArg::<T>::from(val);
            (f)(arg.try_into().unwrap());
        })))
    }

    /// Returns a [`Callback`] wrapping the provided [`Fn`], that can be
    /// converted to a [`Dart_Handle`] and passed to Dart.
    pub fn from_fn<F, T>(f: F) -> Self
    where
        F: Fn(T) + 'static,
        DartValueArg<T>: TryInto<T>,
        <DartValueArg<T> as TryInto<T>>::Error: Debug,
        T: 'static,
    {
        Self(Kind::Fn(Box::new(move |val: DartValue| {
            let arg = DartValueArg::<T>::from(val);
            (f)(arg.try_into().unwrap());
        })))
    }

    /// Returns a [`Callback`] wrapping the provided [`FnMut`] with two
    /// arguments, that can be converted to a [`Dart_Handle`] and passed to
    /// Dart.
    pub fn from_two_arg_fn_mut<F, T, S>(mut f: F) -> Self
    where
        F: FnMut(T, S) + 'static,
        DartValueArg<T>: TryInto<T>,
        <DartValueArg<T> as TryInto<T>>::Error: Debug,
        T: 'static,
        DartValueArg<S>: TryInto<S>,
        <DartValueArg<S> as TryInto<S>>::Error: Debug,
        S: 'static,
    {
        Self(Kind::TwoArgFnMut(Box::new(
            move |first: DartValue, second: DartValue| {
                let first = DartValueArg::<T>::from(first);
                let second = DartValueArg::<S>::from(second);
                (f)(first.try_into().unwrap(), second.try_into().unwrap());
            },
        )))
    }

    /// Converts this [`Callback`] into a [`Dart_Handle`], so it can be passed
    /// to Dart.
    ///
    /// [`Callback`] object is leaked and should be freed manually.
    #[must_use]
    pub fn into_dart(self) -> Dart_Handle {
        unsafe {
            match &self.0 {
                Kind::TwoArgFnMut(_) => callback::call_two_arg_proxy(
                    ptr::NonNull::from(Box::leak(Box::new(self))),
                ),
                Kind::Fn(_) | Kind::FnOnce(_) | Kind::FnMut(_) => {
                    callback::call_proxy(ptr::NonNull::from(Box::leak(
                        Box::new(self),
                    )))
                }
            }
        }
    }
}

#[cfg(feature = "mockable")]
pub mod tests {
    use std::convert::TryInto;

    use dart_sys::Dart_Handle;

    use crate::api::DartValueArg;

    use super::Callback;

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_int(
        expects: DartValueArg<i64>,
    ) -> Dart_Handle {
        let expects: i64 = expects.try_into().unwrap();
        Callback::from_once(move |val: i64| {
            assert_eq!(val, expects);
        })
        .into_dart()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_string(
        expects: DartValueArg<String>,
    ) -> Dart_Handle {
        let expects: String = expects.try_into().unwrap();
        Callback::from_once(move |val: String| {
            assert_eq!(val, expects);
        })
        .into_dart()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_optional_int(
        expects: DartValueArg<Option<i64>>,
    ) -> Dart_Handle {
        let expects: Option<i64> = expects.try_into().unwrap();
        Callback::from_once(move |val: Option<i64>| {
            assert_eq!(val, expects);
        })
        .into_dart()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_optional_string(
        expects: DartValueArg<Option<String>>,
    ) -> Dart_Handle {
        let expects: Option<String> = expects.try_into().unwrap();
        Callback::from_once(move |val: Option<String>| {
            assert_eq!(val, expects);
        })
        .into_dart()
    }

    type TestCallbackHandleFunction = extern "C" fn(Dart_Handle);

    static mut TEST_CALLBACK_HANDLE_FUNCTION: Option<
        TestCallbackHandleFunction,
    > = None;

    #[no_mangle]
    pub unsafe extern "C" fn register__test__test_callback_handle_function(
        f: TestCallbackHandleFunction,
    ) {
        TEST_CALLBACK_HANDLE_FUNCTION = Some(f);
    }

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_dart_handle() -> Dart_Handle
    {
        Callback::from_once(move |val: Dart_Handle| {
            unsafe { (TEST_CALLBACK_HANDLE_FUNCTION.unwrap())(val) };
        })
        .into_dart()
    }
}
