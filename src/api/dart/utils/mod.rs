pub mod arrays;
mod err;
mod frb_adapter;
mod result;
mod string;

use std::{future::Future, marker::PhantomData};

use dart_sys::Dart_Handle;

use crate::{
    api::DartValue,
    platform::{spawn, utils::Completer},
};

pub use self::{
    arrays::PtrArray,
    err::{new_panic_error, ArgumentError, DartError},
    frb_adapter::*,
    result::DartResult,
    string::{
        c_str_into_string, dart_string_into_rust, free_dart_native_string,
        string_into_c_str,
    },
};

/// Rust representation of a Dart [`Future`].
///
/// [`Future`]: https://api.dart.dev/dart-async/Future-class.html

#[derive(Debug)]
#[repr(transparent)]
pub struct DartFuture<O>(
    #[allow(unused_tuple_struct_fields)] Dart_Handle, // read by Dart side
    PhantomData<*const O>,
);

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

    // todo
    fn into_my_dart_future(self) -> MyDartFuture;
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

    fn into_my_dart_future(self) -> MyDartFuture {
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
        MyDartFuture {
            handle: dart_future,
        }
    }
}

/// Tries to convert the provided [`DartValueArg`] using [`TryInto`].
///
/// If the conversion fails, then [`ArgumentError`] is [`return`]ed as a
/// [`DartFuture`].
macro_rules! dart_arg_try_into {
    ($k:expr) => {
        match $k.try_into().map_err(|err: DartValueCastError| {
            ArgumentError::new(err.value, "kind", err.expectation)
        }) {
            Ok(s) => s,
            Err(e) => return async move { Err(e.into()) }.into_dart_future(),
        }
    };
}

pub(crate) use dart_arg_try_into;
