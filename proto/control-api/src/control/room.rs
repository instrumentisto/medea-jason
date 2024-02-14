//! [`Room`] definitions.

use derive_more::{AsRef, Display, From, Into};
use ref_cast::RefCast;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{member, Pipeline};

/// Media [`Element`] representing a single space where multiple [`Member`]s can
/// interact with each other.
///
/// [`Element`]: crate::Element
/// [`Member`]: crate::Member
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Room {
    /// ID of this [`Room`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub id: Id,

    /// [`Spec`] of this [`Room`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    pub spec: Spec,
}

/// Spec of a [`Room`] media [`Element`].
///
/// [`Element`]: crate::Element
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Spec {
    /// Media pipeline representing [`Member`]s of this [`Room`].
    ///
    /// [`Element`]: crate::Element
    /// [`Member`]: crate::Member
    pub pipeline: Pipeline<member::Id, PipelineSpec>,
}

/// Specs of [`Element`]s allowed a [`Room`]'s [`Spec::pipeline`].
///
/// [`Element`]: crate::Element
#[derive(Clone, Debug, Eq, From, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", content = "spec"))]
pub enum PipelineSpec {
    /// [`Member`] media [`Element`].
    ///
    /// [`Element`]: crate::Element
    /// [`Member`]: crate::Member
    Member(member::Spec),
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
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[from(types("&str", String))]
#[into(owned(types(String)))]
#[repr(transparent)]
pub struct Id(Box<str>);

#[cfg(feature = "client-api")]
impl From<medea_client_api_proto::RoomId> for Id {
    fn from(id: medea_client_api_proto::RoomId) -> Self {
        id.0.into()
    }
}
