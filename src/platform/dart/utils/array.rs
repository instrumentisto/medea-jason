//! Implementation and definition of Rust side representation of the Dart
//! [List].
//!
//! [List]: https://api.dart.dev/stable/2.14.1/dart-core/List-class.html

use std::slice;

// TODO: Если это прокся для листа, как написано в доке, то я бы ожидал что
//       внутри будет лежать DartHandle. Что это? Где и как используется?
//       Где тесты? Очень похоже на уже существующий PtrArray.
/// Rust side representation of the Dart [List].
///
/// [List]: https://api.dart.dev/stable/2.14.1/dart-core/List-class.html
#[repr(C)]
pub struct List<T> {
    /// Length of this [`List`].
    pub len: u64,

    /// Pointer to the Dart [List].
    ///
    /// [List]: https://api.dart.dev/stable/2.14.1/dart-core/List-class.html
    pub arr: *const *mut T,
}

impl<T> From<Vec<T>> for List<T> {
    fn from(arr: Vec<T>) -> Self {
        let out: Vec<_> = arr
            .into_iter()
            .map(|e| Box::into_raw(Box::new(e)))
            .collect();
        Self {
            len: out.len() as u64,
            arr: Box::leak(out.into_boxed_slice()).as_ptr(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        unsafe {
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_ptr_alignment
            )]
            slice::from_raw_parts_mut(self.arr as *mut i64, self.len as usize);
        }
    }
}
