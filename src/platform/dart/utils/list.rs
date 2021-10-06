//! Definitions and implementations of the Rust side representation of the Dart
//! side `List`s.

use dart_sys::Dart_Handle;
use derive_more::From;

use crate::platform::dart::utils::handle::DartHandle;

/// Pointer to an extern function that returns element with a provided index
/// from the provided [`Dart_Handle`] `List`.
type GetFunction = extern "C" fn(Dart_Handle, i32) -> Dart_Handle;

/// Stores pointer to the [`GetFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut GET_FUNCTION: Option<GetFunction> = None;

/// Registers the provided [`GetFunction`] as
/// [`GET_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Array__get(f: GetFunction) {
    GET_FUNCTION = Some(f);
}

/// Pointer to an extern function that returns length of the Dart side `List`.
type LengthFunction = extern "C" fn(Dart_Handle) -> i32;

/// Stores pointer to the [`LengthFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut LENGTH_FUNCTION: Option<LengthFunction> = None;

/// Registers the provided [`LengthFunction`] as [`LENGTH_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_Array__length(f: LengthFunction) {
    LENGTH_FUNCTION = Some(f);
}

/// Rust side representation of the Dart side `List`s.
#[derive(From)]
pub struct DartList(DartHandle);

impl DartList {
    /// Gets [`DartHandle`] from the underlying Dart `List` with a provided
    /// index.
    #[must_use]
    pub fn get(&self, i: i32) -> Option<DartHandle> {
        unsafe {
            // TODO: make it optional
            Some(GET_FUNCTION.unwrap()(self.0.get(), i).into())
        }
    }

    /// Returns length of the underlying Dart `List`.
    #[must_use]
    pub fn length(&self) -> i32 {
        unsafe { LENGTH_FUNCTION.unwrap()(self.0.get()) }
    }
}

impl<T> From<DartList> for Vec<T>
where
    T: From<DartHandle>,
{
    fn from(list: DartList) -> Self {
        let len = list.length();
        #[allow(clippy::cast_sign_loss)]
        let mut out = Vec::with_capacity(len as usize);
        for i in 0..len {
            let val = list.get(i).unwrap();
            out.push(val.into());
        }
        out
    }
}
