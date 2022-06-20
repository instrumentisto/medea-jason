//! [`Room`] definitions.

use derive_more::{AsRef, Display, From, Into};
use ref_cast::RefCast;
#[cfg(feature = "serde")]
use serde::Deserialize;

use super::{member, Member, Pipeline};

/// Media [`Element`] representing a single space where multiple [`Member`]s can
/// interact with each other.
///
/// [`Element`]: crate::Element
/// [`Member`]: crate::Member
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(transparent))]
pub struct Room {
    /// Media pipeline representing [`Member`]s of this [`Room`].
    pub spec: Pipeline<member::Id, Element>,
}

/// Possible [`Element`]s of a [`Room`]'s [`Pipeline`].
#[derive(Clone, Debug, Eq, From, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize), serde(tag = "kind"))]
pub enum Element {
    /// [`Member`] media [`Element`] of the [`Room`]'s [`Pipeline`].
    Member(Member),
}

/// ID of a [`Room`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(
    AsRef,
    Clone,
    Debug,
    Display,
    Eq,
    From,
    Hash,
    Into,
    Ord,
    PartialEq,
    PartialOrd,
    RefCast,
)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[from(types(String))]
#[into(owned(types(String)))]
#[repr(transparent)]
pub struct Id(Box<str>);

// TODO: Derive via `derive::From` once it's capable to.
impl<'a> From<&'a str> for Id {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}
