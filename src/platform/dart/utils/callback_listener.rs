//! Definitions and implementations of the Dart callback listeners.

use std::ptr;

use dart_sys::Dart_Handle;

use crate::api::{DartValue, DartValueArg};

/// Pointer to an extern function that returns a [`Dart_Handle`] to a newly
/// created Dart callback that will proxy calls to the Rust callback.
type CallbackFunction = extern "C" fn(ptr::NonNull<Callback>) -> Dart_Handle;

/// Stores pointer to the [`CallbackFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut CALLBACK_CREATE_FUNCTION: Option<CallbackFunction> = None;

/// Registers the provided [`CallbackFunction`] as [`CALLBACK_CREATE_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Callback__callback(f: CallbackFunction) {
    CALLBACK_CREATE_FUNCTION = Some(f);
}

/// Calls the provided [`Callback`] with the provided [`DartValue`] as an
/// argument.
///
/// # Safety
///
/// Provided [`Callback`] should pe a valid [Callback] pointer.
#[no_mangle]
pub unsafe extern "C" fn Callback__call(
    cb: ptr::NonNull<Callback>,
    val: DartValue,
) {
    let cb = Box::from_raw(cb.as_ptr());
    (cb.0)(val);
}

/// Rust closure that can be called by Dart.
pub struct Callback(Box<dyn FnOnce(DartValue)>);

impl Callback {
    /// Returns [`Dart_Handle`] to the Dart callback which will call th provided
    /// `f` closure when it will be called on Dart side.
    pub fn new<F, T>(f: F) -> Dart_Handle
    where
        F: FnOnce(DartValueArg<T>) + 'static,
    {
        let this = Self(Box::new(move |val: DartValue| {
            let var = DartValueArg::<T>::from(val);
            (f)(var);
        }));
        unsafe {
            CALLBACK_CREATE_FUNCTION.unwrap()(ptr::NonNull::from(Box::leak(
                Box::new(this),
            )))
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
        Callback::new(move |i: DartValueArg<i64>| {
            let val: i64 = i.try_into().unwrap();
            assert_eq!(val, expects);
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_string(
        expects: DartValueArg<String>,
    ) -> Dart_Handle {
        let expects: String = expects.try_into().unwrap();
        Callback::new(move |val: DartValueArg<String>| {
            let val: String = val.try_into().unwrap();
            assert_eq!(val, expects);
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_optional_int(
        expects: DartValueArg<Option<i64>>,
    ) -> Dart_Handle {
        let expects: Option<i64> = expects.try_into().unwrap();
        Callback::new(move |val: DartValueArg<Option<i64>>| {
            let val: Option<i64> = val.try_into().unwrap();
            assert_eq!(val, expects);
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn test_callback_listener_optional_string(
        expects: DartValueArg<Option<String>>,
    ) -> Dart_Handle {
        let expects: Option<String> = expects.try_into().unwrap();
        Callback::new(move |val: DartValueArg<Option<String>>| {
            let val: Option<String> = val.try_into().unwrap();
            assert_eq!(val, expects);
        })
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
        Callback::new(move |val: DartValueArg<Dart_Handle>| {
            let val: Dart_Handle = val.try_into().unwrap();
            unsafe { (TEST_CALLBACK_HANDLE_FUNCTION.unwrap())(val) };
        })
    }
}
