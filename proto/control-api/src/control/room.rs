//! [`Room`] definitions.

use std::collections::HashMap;

use derive_more::{Display, From, Into};

use super::{member, Member};

/// `ID` of a [`Room`].
#[derive(
    Clone, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
pub struct Id(pub String);

/// [`ControlApi`]'s `Room` element specification.
///
/// [`ControlApi`]: crate::ControlApi
#[derive(Clone, Debug)]
pub struct Room {
    /// `ID` of this [`Room`].
    pub id: Id,

    /// [`Member`]s of this [`Room`].
    pub pipeline: HashMap<member::Id, Member>,
}
