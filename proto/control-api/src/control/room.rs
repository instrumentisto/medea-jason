//! [`Room`] definitions.

use derive_more::{AsRef, Display, From, Into};
use ref_cast::RefCast;

use super::{member, Member, Pipeline};

/// Media [`Element`] representing a single space where multiple [`Member`]s can
/// interact with each other.
///
/// [`Element`]: crate::Element
/// [`Member`]: crate::Member
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(transparent))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Room {
    /// Media pipeline representing [`Member`]s of this [`Room`].
    pub spec: Pipeline<member::Id, Element>,
}

/// Elements of `Room`'s [`Pipeline`].
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(tag = "kind"))]
#[derive(Clone, Debug, Eq, From, PartialEq)]
pub enum Element {
    /// Represent `Member`
    Member(Member),
}

/// ID of a [`Room`] media [`Element`].
///
/// [`Element`]: crate::Element
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
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
