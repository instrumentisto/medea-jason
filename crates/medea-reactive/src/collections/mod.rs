//! Implementations of reactive collections based on [`std::collections`].

// TODO: Needs refactoring.
#![expect(clippy::module_name_repetitions, reason = "needs refactoring")]

pub mod hash_map;
pub mod hash_set;
pub mod vec;

#[doc(inline)]
pub use {
    hash_map::{ObservableHashMap, ProgressableHashMap},
    hash_set::{ObservableHashSet, ProgressableHashSet},
    vec::{ObservableVec, ProgressableVec},
};
