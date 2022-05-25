//! [`Room`] definitions.

use std::collections::HashMap;

use derive_more::{Display, From, Into};

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

/// ID of a [`Room`] [`Element`].
///
/// [`Element`]: crate::Element
#[derive(
    Clone, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
#[from(types(String))]
#[into(owned(types(String)))]
pub struct Id(Box<str>);
