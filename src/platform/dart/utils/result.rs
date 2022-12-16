//! FFI-compatible [`Result`] for Dart.

use crate::{api::{dart::DartValue, Error}};

/// FFI-compatible [`Result`] for Dart.
#[allow(variant_size_differences)] // that's totally OK here
#[derive(Debug)]
#[repr(u8)]
pub enum DartResult {
    /// Success [`DartValue`].
    Ok(DartValue),

    /// [`DartError`] value.
    Err(Error),
}

impl<T: Into<DartValue>> From<Result<T, Error>> for DartResult {
    fn from(res: Result<T, Error>) -> Self {
        match res {
            Ok(val) => Self::Ok(val.into()),
            Err(e) => Self::Err(e),
        }
    }
}

impl<T: Into<Error>> From<T> for DartResult {
    fn from(err: T) -> Self {
        Self::Err(err.into())
    }
}
