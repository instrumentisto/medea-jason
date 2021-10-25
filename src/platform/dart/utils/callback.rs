//! Functionality for converting Rust closures into callbacks that can be
//! passed to Dart and called by Dart.

use std::{convert::TryInto, fmt::Debug, ptr};

use dart_sys::Dart_Handle;

use crate::api::{DartValue, DartValueArg};

/// Pointer to an extern function that returns a [`Dart_Handle`] to a newly
/// created Dart callback that will proxy calls to the Rust callback.
type CallbackCallTwoArgProxyFunction =
    extern "C" fn(ptr::NonNull<Callback>) -> Dart_Handle;

/// Stores pointer to the [`CallbackCallTwoArgProxyFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut CALLBACK_CALL_TWO_ARG_PROXY_FUNCTION: Option<
    CallbackCallTwoArgProxyFunction,
> = None;

/// Registers the provided [`CallbackCallTwoArgProxyFunction`] as
/// [`CALLBACK_CALL_TWO_ARG_PROXY_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Callback__call_two_arg_proxy(
    f: CallbackCallTwoArgProxyFunction,
) {
    CALLBACK_CALL_TWO_ARG_PROXY_FUNCTION = Some(f);
}

/// Calls the provided [`Callback`] with the provided two [`DartValue`]s as an
/// argument.
///
/// # Safety
///
/// Provided [`Callback`] should be a valid [Callback] pointer.
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

/// Pointer to an extern function that returns a [`Dart_Handle`] to a newly
/// created Dart callback that will proxy calls to the Rust callback.
type CallbackCallProxyFunction =
    extern "C" fn(ptr::NonNull<Callback>) -> Dart_Handle;

/// Stores pointer to the [`CallbackCallProxyFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut CALLBACK_CALL_PROXY_FUNCTION: Option<CallbackCallProxyFunction> =
    None;

/// Registers the provided [`CallbackCallProxyFunction`] as
/// [`CALLBACK_CALL_PROXY_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Callback__call_proxy(
    f: CallbackCallProxyFunction,
) {
    CALLBACK_CALL_PROXY_FUNCTION = Some(f);
}

/// Calls the provided [`Callback`] with the provided [`DartValue`] as an
/// argument.
///
/// # Safety
///
/// Provided [`Callback`] should pe a valid [Callback] pointer.
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

// TODO: Fix in #10:
//       1. Requires additional parametrization or(and) wrapping.
//       2. FnOnce semantics should be reflected on Dart side somehow.
//       3. Kind::FnMut && Kind::Fn arent dropped anywhere right now.
/// Rust closure that can be called by Dart.
pub struct Callback(Kind);

/// Underlying [`Callback`] function to be called.
enum Kind {
    FnOnce(Box<dyn FnOnce(DartValue)>),
    FnMut(Box<dyn FnMut(DartValue)>),
    Fn(Box<dyn Fn(DartValue)>),
    TwoArgFnMut(Box<dyn FnMut(DartValue, DartValue)>),
}

impl Callback {
    /// Returns [`Callback`] that wraps the provided [`FnOnce`] and can be
    /// converted to [`Dart_Handle`] and passed to Dart.
    #[allow(clippy::new_ret_no_self)]
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

    /// Returns [`Callback`] that wraps the provided [`FnMut`] and can be
    /// converted to [`Dart_Handle`] and passed to Dart.
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

    /// Returns [`Callback`] that wraps the provided [`FnOnce`] and can be
    /// converted to [`Dart_Handle`] and passed to Dart.
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

    /// Returns [`Callback`] that wraps the provided [`FnMut`] with two
    /// arguments and can be converted to [`Dart_Handle`] and passed to Dart.
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

    /// Converts this [`Callback`] to [`Dart_Handle`] so it can be passed to
    /// Dart.
    ///
    /// [`Callback`] object is leaked and should be freed manually.
    #[must_use]
    pub fn into_dart(self) -> Dart_Handle {
        unsafe {
            match &self.0 {
                Kind::TwoArgFnMut(_) => CALLBACK_CALL_TWO_ARG_PROXY_FUNCTION
                    .unwrap()(
                    ptr::NonNull::from(Box::leak(Box::new(self))),
                ),
                Kind::Fn(_) | Kind::FnOnce(_) | Kind::FnMut(_) => {
                    CALLBACK_CALL_PROXY_FUNCTION.unwrap()(ptr::NonNull::from(
                        Box::leak(Box::new(self)),
                    ))
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
        Callback::from_once(move |i: DartValueArg<i64>| {
            let val: i64 = i.try_into().unwrap();
            assert_eq!(val, expects);
        })
        .into_dart()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_string(
        expects: DartValueArg<String>,
    ) -> Dart_Handle {
        let expects: String = expects.try_into().unwrap();
        Callback::from_once(move |val: DartValueArg<String>| {
            let val: String = val.try_into().unwrap();
            assert_eq!(val, expects);
        })
        .into_dart()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_optional_int(
        expects: DartValueArg<Option<i64>>,
    ) -> Dart_Handle {
        let expects: Option<i64> = expects.try_into().unwrap();
        Callback::from_once(move |val: DartValueArg<Option<i64>>| {
            let val: Option<i64> = val.try_into().unwrap();
            assert_eq!(val, expects);
        })
        .into_dart()
    }

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_optional_string(
        expects: DartValueArg<Option<String>>,
    ) -> Dart_Handle {
        let expects: Option<String> = expects.try_into().unwrap();
        Callback::from_once(move |val: DartValueArg<Option<String>>| {
            let val: Option<String> = val.try_into().unwrap();
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
        Callback::from_once(move |val: DartValueArg<Dart_Handle>| {
            let val: Dart_Handle = val.try_into().unwrap();
            unsafe { (TEST_CALLBACK_HANDLE_FUNCTION.unwrap())(val) };
        })
        .into_dart()
    }
}
