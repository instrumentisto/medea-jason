//! [`Room`] definitions.

use std::collections::HashMap;

use derive_more::{Display, From, Into};
use ref_cast::RefCast;

use super::{member, Member};

/// Media [`Element`] representing a single space where multiple [`Member`]s can
/// interact with each other.
///
/// [`Element`]: crate::Element
/// [`Member`]: crate::Member
#[derive(Clone, Debug)]
pub struct Room {
    /// ID of this [`Room`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub id: Id,

    /// Media pipeline representing [`Member`]s of this [`Room`].
    pub pipeline: HashMap<member::Id, Member>,
}

/// ID of a [`Room`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(
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
