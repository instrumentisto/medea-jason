//! [`ControlApi`] definitions.
//!
//! [`ControlApi`]: Api

// TODO: Remove once annoying false positive is fixed:
//       https://github.com/rust-lang/rust-clippy/issues/6902
#![allow(clippy::use_self)]

pub mod endpoint;
pub mod member;
pub mod room;

use std::{collections::HashMap, str::FromStr};

use async_trait::async_trait;
use derive_more::{Display, Error, From};

#[doc(inline)]
pub use self::{endpoint::Endpoint, member::Member, room::Room};

/// API allowing to control a media server dynamically, by creating, updating
/// and destroying pipelines of media [`Element`]s on it.
///
/// Both API client and API server should implement this trait.
#[async_trait]
pub trait Api {
    /// Error returned by this [`ControlApi`].
    ///
    /// [`ControlApi`]: Api
    type Error;

    /// Creates a new [`Element`] on the media server.
    ///
    /// # Non-idempotent
    ///
    /// Errors if an [`Element`] with such ID already exists.
    ///
    /// # Errors
    ///
    /// - If the [`Element`]'s parent [`Element`] (identified by a [`Fid`])
    ///   doesn't exist.
    /// - If an [`Element`] with such ID already exists.
    /// - If the media server failed to perform this request.
    async fn create(&self, req: Request) -> Result<member::Sids, Self::Error>;

    /// Applies changes to an existing [`Element`] on the media server, or
    /// creates a new one in case there is no [`Element`] with such ID.
    ///
    /// # Idempotent
    ///
    /// If no [`Element`] with such ID exists, then it will be created,
    /// otherwise it will be reconfigured. [`Element`]s that exist on the same
    /// hierarchy level, but are not specified in the provided [`Request`], will
    /// be removed.
    ///
    /// # Errors
    ///
    /// - If the [`Element`]'s parent [`Element`] (identified by a [`Fid`])
    ///   doesn't exist.
    /// - If the media server failed to perform this request.
    async fn apply(&self, req: Request) -> Result<member::Sids, Self::Error>;

    /// Removes [`Element`]s from the media server.
    ///
    /// Allows referring multiple [`Element`]s on the last two levels of a
    /// [`Fid`].
    ///
    /// # Idempotent
    ///
    /// If no [`Element`]s with such [`Fid`]s exist, then succeeds.
    ///
    /// # Errors
    ///
    /// - If no [`Fid`]s were specified.
    /// - If any [`Fid`] contains multiple [`room::Id`]s.
    /// - If the media server failed to perform this request.
    async fn delete(&self, fids: &[Fid]) -> Result<(), Self::Error>;

    /// Lookups [`Element`]s by their [`Fid`]s on the media server.
    ///
    /// If no [`Fid`]s are specified, then returns all the current [`Element`]s
    /// on the media server.
    ///
    /// If no [`Element`] exists for some [`Fid`], then it won't be present in
    /// the returned [`Elements`] collection.
    ///
    /// # Errors
    ///
    /// - If the media server failed to perform this request.
    async fn get(&self, fids: &[Fid]) -> Result<Elements, Self::Error>;

    /// Checks healthiness of the media server.
    ///
    /// Caller should assert that the returned [`Pong`] has the same nonce as
    /// the sent [`Ping`].
    ///
    /// # Errors
    ///
    /// - If the media server failed to perform this request.
    async fn healthz(&self, ping: Ping) -> Result<Pong, Self::Error>;
}

/// Request for creating or applying an [`Element`] on a media server.
#[derive(Clone, Debug)]
pub enum Request {
    /// [`Room`] to be created or to apply changes to.
    Room(Room),

    /// [`Member`] to be created or to apply changes to.
    Member {
        /// ID of the [`Room`] this [`Member`] participates in.
        room_id: room::Id,

        /// Media [`Element`] representing this [`Member`].
        member: Box<Member>,
    },

    /// [`Endpoint`] to be created or to apply changes to.
    Endpoint {
        /// ID of the [`Room`] this [`Endpoint`] belongs to.
        room_id: room::Id,

        /// ID of the [`Member`] this [`Endpoint`] belongs to.
        member_id: member::Id,

        /// Media [`Element`] representing this [`Endpoint`].
        endpoint: Endpoint,
    },
}

/// Possible media elements forming a media pipeline.
#[derive(Clone, Debug, From)]
pub enum Element {
    /// [`Room`] media element.
    Room(Room),

    /// [`Member`] media element.
    Member(Box<Member>),

    /// [`Endpoint`] media element.
    Endpoint(Endpoint),
}

/// Collection of uniquely identified [`Element`]s.
pub type Elements = HashMap<Fid, Element>;

/// FID (Full ID) is a composition of media [`Element`] IDs referring to some
/// [`Element`] on a whole media server uniquely.
#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Fid {
    /// FID of a [`Room`].
    #[display(fmt = "{}", id)]
    Room {
        /// Unique ID of the [`Room`].
        id: room::Id,
    },

    /// FID of a [`Member`].
    #[display(fmt = "{}/{}", room_id, id)]
    Member {
        /// ID of the [`Member`] in the [`Room`].
        id: member::Id,

        /// Unique ID of the [`Room`].
        room_id: room::Id,
    },

    /// FID of an [`Endpoint`].
    #[display(fmt = "{}/{}/{}", room_id, member_id, id)]
    Endpoint {
        /// ID of the [`Endpoint`] of the [`Member`].
        id: endpoint::Id,

        /// Unique ID of the [`Room`].
        room_id: room::Id,

        /// ID of the [`Member`] in the [`Room`].
        member_id: member::Id,
    },
}

impl FromStr for Fid {
    type Err = ParseFidError;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        if val.is_empty() {
            return Err(ParseFidError::Empty);
        }

        let mut splitted = val.split('/');

        let room_id = splitted.next().ok_or(ParseFidError::Empty)?;
        if room_id.is_empty() {
            return Err(ParseFidError::MissingPath(val.into()));
        }

        let member_id = if let Some(id) = splitted.next() {
            if id.is_empty() {
                return Err(ParseFidError::MissingPath(val.into()));
            }
            id
        } else {
            return Ok(Self::Room { id: room_id.into() });
        };

        let endpoint_id = if let Some(id) = splitted.next() {
            if id.is_empty() {
                return Err(ParseFidError::MissingPath(val.into()));
            }
            id
        } else {
            return Ok(Self::Member {
                id: member_id.into(),
                room_id: room_id.into(),
            });
        };

        if splitted.next().is_some() {
            Err(ParseFidError::TooManyPaths(val.into()))
        } else {
            Ok(Self::Endpoint {
                id: endpoint_id.into(),
                room_id: room_id.into(),
                member_id: member_id.into(),
            })
        }
    }
}

/// Possible errors of parsing a [`Fid`].
#[derive(Debug, Display, Error)]
pub enum ParseFidError {
    /// [`Fid`] is empty.
    #[display(fmt = "FID is empty")]
    Empty,

    /// [`Fid`] has too many paths.
    #[display(fmt = "FID has too many paths: {}", _0)]
    TooManyPaths(#[error(not(source))] Box<str>),

    /// [`Fid`] has missing paths.
    #[display(fmt = "FID has missing paths: {}", _0)]
    MissingPath(#[error(not(source))] Box<str>),
}

/// [`Ping`] message received by a media server periodically for probing its
/// healthiness.
///
/// Each new [`Ping`] should increment its nonce, starting with `0`.
#[derive(
    Clone, Copy, Debug, Display, Eq, From, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct Ping(pub u32);

/// [`Pong`] message sent by a media server in response to a received [`Ping`]
/// message.
///
/// Contains nonce of the answered [`Ping`] message.
#[derive(
    Clone, Copy, Debug, Display, Eq, From, Hash, Ord, PartialEq, PartialOrd,
)]
pub struct Pong(pub u32);
