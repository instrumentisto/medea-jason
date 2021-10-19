//! Definitions and implementations of the Dart callback listeners.

use dart_sys::Dart_Handle;

use crate::api::{DartValue, DartValueArg};

/// Listener for the Dart callback with `void` as argument.
pub struct Callback(Box<dyn FnOnce(DartValue)>);

impl Callback {
    /// Returns [`Dart_Handle`] to the Dart callback which will call provided
    /// `f` closure when it will be called on Dart side.
    pub fn callback<F, T>(f: F) -> Dart_Handle
    where
        F: FnOnce(DartValueArg<T>) + 'static,
    {
        let this = Self(Box::new(move |val: DartValue| {
            let var = DartValueArg::<T>::from(val);
            (f)(var);
        }));
        unsafe { CALLBACK_FUNCTION.unwrap()(Box::into_raw(Box::new(this))) }
    }
}

/// Pointer to an extern function that returns a [`Dart_Handle`] to a newly
/// created Dart callback which will call Rust side callback when Dart side
/// callback will be fired.
type CallbackFunction = extern "C" fn(*mut Callback) -> Dart_Handle;

/// Stores pointer to the [`CallbackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut CALLBACK_FUNCTION: Option<CallbackFunction> = None;

/// Registers the provided [`CallbackFunction`] as [`CALLBACK_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Callback__callback(f: CallbackFunction) {
    CALLBACK_FUNCTION = Some(f);
}

/// Calls provided [`Callback`] with a provided [`DartValue`] as argument.
///
/// # Safety
///
/// Provided [`Callback`] shouldn't be freed.
#[no_mangle]
pub unsafe extern "C" fn Callback__call(cb: *mut Callback, val: DartValue) {
    let cb = Box::from_raw(cb);
    (cb.0)(val);
}

#[cfg(feature = "mockable")]
pub mod tests {
    use std::{convert::TryInto, ptr};

    use dart_sys::Dart_Handle;

    use crate::api::DartValueArg;

    use super::Callback;

    #[no_mangle]
    pub unsafe extern "C" fn test__callback_listener__int(
        expects: DartValueArg<i64>,
    ) -> Dart_Handle {
        let expects: i64 = expects.try_into().unwrap();
        Callback::callback(move |i: DartValueArg<i64>| {
            let val: i64 = i.try_into().unwrap();
            assert_eq!(val, expects);
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn test__callback_listener__string(
        expects: DartValueArg<String>,
    ) -> Dart_Handle {
        let expects: String = expects.try_into().unwrap();
        Callback::callback(move |val: DartValueArg<String>| {
            let val: String = val.try_into().unwrap();
            assert_eq!(val, expects);
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn test__callback_listener__optional_int(
        expects: DartValueArg<Option<i64>>,
    ) -> Dart_Handle {
        let expects: Option<i64> = expects.try_into().unwrap();
        Callback::callback(move |val: DartValueArg<Option<i64>>| {
            let val: Option<i64> = val.try_into().unwrap();
            assert_eq!(val, expects);
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn test__callback_listener__optional_string(
        expects: DartValueArg<Option<String>>,
    ) -> Dart_Handle {
        let expects: Option<String> = expects.try_into().unwrap();
        Callback::callback(move |val: DartValueArg<Option<String>>| {
            let val: Option<String> = val.try_into().unwrap();
            assert_eq!(val, expects);
        })
    }

    type TestCallbackHandleFunction = extern "C" fn(ptr::NonNull<Dart_Handle>);

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
    pub unsafe extern "C" fn test__callback_listener__dart_handle(
    ) -> Dart_Handle {
        Callback::callback(move |val: DartValueArg<Dart_Handle>| {
            unsafe {
                (TEST_CALLBACK_HANDLE_FUNCTION.unwrap())(
                    val.try_into().unwrap(),
                )
            };
        })
    }
}
