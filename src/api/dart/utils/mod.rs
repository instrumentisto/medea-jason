// TODO(alexlapa): if we dont use this in api, maybe we should move this to
//                 platform?
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

impl<O> DartFuture<O> {
    /// Returns inner [`Dart_Handle`].
    #[must_use]
    pub const fn into_raw(self) -> Dart_Handle {
        self.0
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

/// Tries to convert the provided [i64] using [`TryInto`].
///
/// If the conversion fails, then [`ArgumentError`] is [`return`]ed as a
/// anyhow [`DartError`].
macro_rules! dart_enum_try_into {
    ($k:expr, $name:expr, $message:expr) => {
        if let Some(kind) = $k {
            Some(kind.try_into().map_err(|err| {
                anyhow::anyhow!(
                    "{:?}",
                    DartError::from(ArgumentError::new(err, $name, $message))
                )
            })?)
        } else {
            None
        }
    };
}

pub(crate) use dart_enum_try_into;
