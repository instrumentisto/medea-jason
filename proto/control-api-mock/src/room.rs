use std::collections::HashMap;

use crate::member::Member;

use serde::{Deserialize, Serialize};

/// [Control API]'s `Room` representation.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
#[derive(Debug, Deserialize, Serialize)]
pub struct Room {
    /// ID of this [`Room`].
    #[serde(skip_deserializing)]
    pub id: String,

    /// Pipeline of this [`Room`].
    pub pipeline: HashMap<String, RoomElement>,
}

/// Element of [`Room`]'s pipeline.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind")]
pub enum RoomElement {
    Member(Member),
}
