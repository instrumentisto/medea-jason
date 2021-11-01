//! Functionality for converting Rust closures into callbacks that can be passed
//! to Dart and called by Dart.

use std::{mem, os::raw::c_void, ptr};

use dart_sys::Dart_Handle;

use crate::{
    api::{DartValue, DartValueArg},
    platform::dart::utils::dart_api::Dart_NewFinalizableHandle_DL_Trampolined,
};

/// Pointer to an extern function returning a [`Dart_Handle`] to a newly created
/// Dart callback that will proxy calls to the associated Rust callback.
type CallbackCallProxyFunction =
    extern "C" fn(ptr::NonNull<Callback>) -> Dart_Handle;

/// Stores pointer to a [`CallbackCallProxyFunction`] extern function.
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
            Kind::FnOnce(_) => {
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
}

// TODO: Fix in #10:
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
        F: FnOnce(DartValueArg<T>) + 'static,
    {
        Self(Kind::FnOnce(Box::new(move |val: DartValue| {
            let arg = DartValueArg::<T>::from(val);
            (f)(arg);
        })))
    }

    /// Returns a [`Callback`] wrapping the provided [`FnMut`], that can be
    /// converted to a [`Dart_Handle`] and passed to Dart.
    pub fn from_fn_mut<F, T>(mut f: F) -> Self
    where
        F: FnMut(DartValueArg<T>) + 'static,
    {
        Self(Kind::FnMut(Box::new(move |val: DartValue| {
            let arg = DartValueArg::<T>::from(val);
            (f)(arg);
        })))
    }

    /// Returns a [`Callback`] wrapping the provided [`Fn`], that can be
    /// converted to a [`Dart_Handle`] and passed to Dart.
    pub fn from_fn<F, T>(f: F) -> Self
    where
        F: Fn(DartValueArg<T>) + 'static,
    {
        Self(Kind::Fn(Box::new(move |val: DartValue| {
            let arg = DartValueArg::<T>::from(val);
            (f)(arg);
        })))
    }

    /// Converts this [`Callback`] into a [`Dart_Handle`], so it can be passed
    /// to Dart.
    ///
    /// [`Callback`] object is leaked and should be freed manually.
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    #[must_use]
    pub fn into_dart(self) -> Dart_Handle {
        let is_finalizable = !matches!(&self.0, Kind::FnOnce(_));
        unsafe {
            let f = ptr::NonNull::from(Box::leak(Box::new(self)));
            let handle = CALLBACK_CALL_PROXY_FUNCTION.unwrap()(f);

            if is_finalizable {
                Dart_NewFinalizableHandle_DL_Trampolined(
                    handle,
                    f.as_ptr().cast::<c_void>(),
                    mem::size_of::<*mut c_void>() as i32,
                    callback_finalizer,
                );
            }

            handle
        }
    }
}

/// Finalizer for the not [`Kind::FnOnce`] [`Callback`].
///
/// Cleans finalized [`Callback`] memory.
extern "C" fn callback_finalizer(_: *mut c_void, cb: *mut c_void) {
    drop(unsafe { Box::from_raw(cb.cast::<Callback>()) });
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
        Callback::from_fn_mut(move |i: DartValueArg<i64>| {
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
        Callback::from_fn_mut(move |val: DartValueArg<String>| {
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
        Callback::from_fn_mut(move |val: DartValueArg<Option<i64>>| {
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
        Callback::from_fn_mut(move |val: DartValueArg<Option<String>>| {
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
        Callback::from_fn_mut(move |val: DartValueArg<Dart_Handle>| {
            let val: Dart_Handle = val.try_into().unwrap();
            unsafe { (TEST_CALLBACK_HANDLE_FUNCTION.unwrap())(val) };
        })
        .into_dart()
    }
}
