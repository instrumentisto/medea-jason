//! [`ControlApi`] definitions.
//!
//! [`ControlApi`]: Api

// TODO: Remove once annoying false positive is fixed:
//       https://github.com/rust-lang/rust-clippy/issues/6902
#![allow(clippy::use_self)]

pub mod endpoint;
pub mod member;
pub mod room;

use std::collections::HashMap;

use async_trait::async_trait;
use derive_more::From;

pub use self::{endpoint::Endpoint, member::Member, room::Room};

/// [Control API] used to control [Medea] server.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
/// [Medea]: https://git.instrumentisto.com/streaming/medea
#[async_trait(?Send)]
pub trait Api {
    /// Error of this [`ControlApi`].
    ///
    /// [`ControlApi`]: Api
    type Error;

    /// Creates a new [`Element`].
    ///
    /// # Errors
    ///
    /// - If [`Element`]'s parent [`Fid`] doesn't exist;
    /// - If [`Element`] with same ID already exists.
    async fn create(&self, req: Request) -> Result<member::Sids, Self::Error>;

    /// Applies changes to the existing [`Element`] or creates a new one, in
    /// case the is no [`Element`] with the provided `ID`.
    ///
    /// - If [`Element`]'s parent [`Fid`] doesn't exist.
    async fn apply(&self, req: Request) -> Result<member::Sids, Self::Error>;

    /// Deletes [`Elements`] with provided [`Fid`]s.
    ///
    /// # Errors
    ///
    /// - If `fids` is empty;
    /// - If `fids` contains multiple [`room::Id`]s.
    async fn delete_elements(
        &self,
        fids: Vec<Fid>,
    ) -> Result<member::Sids, Self::Error>;

    /// Returns [`Elements`] by their [`Fid`]s.
    ///
    /// # Errors
    ///
    /// - If an [`Element`] with the provided [`Fid`] doesn't exist.
    async fn get_elements(
        &self,
        fids: Vec<Fid>,
    ) -> Result<Elements, Self::Error>;

    /// Checks healthiness of this media server.
    async fn healthz(&self, ping: Ping) -> Result<Pong, Self::Error>;
}

/// Request for creating a new [`Element`] or applying changes to the existing
/// one.
#[allow(variant_size_differences, clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum Request {
    /// Creates a new [`Room`] or applies changes to the exising one.
    Room(Room),

    /// Creates a new [`Member`] in some [`Room`] or applies changes to the
    /// exising one.
    Member {
        /// ID of the [`Room`], [`Member`] is a participant of.
        room_id: room::Id,

        /// [`Member`] media element.
        member: Member,
    },

    /// Creates a new [`Endpoint`] for some [`Member`] or applies changes to
    /// the exising one.
    Endpoint {
        /// ID of the [`Room`], [`Member`] is a participant of.
        room_id: room::Id,

        /// ID of the [`Member`], [`Endpoint`] belongs to.
        member_id: member::Id,

        /// [`Element`] media element.
        endpoint: Endpoint,
    },
}

/// Possible media elements forming a media pipeline.
#[allow(variant_size_differences, clippy::large_enum_variant)]
#[derive(Clone, Debug, From)]
pub enum Element {
    /// [`Room`] media element.
    Room(Room),

    /// [`Member`] media element.
    Member(Member),

    /// [`Endpoint`] media element.
    Endpoint(Endpoint),
}

/// Collection of uniquely identified [`Element`]s.
pub type Elements = HashMap<Fid, Element>;

/// FID (Full ID) is a composition of media [`Element`] IDs referring to some
/// [`Element`] on a whole media server uniquely.
#[derive(Clone, Debug)]
pub enum Fid {
    /// FID of a [`Room`].
    Room {
        /// Unique ID of the [`Room`].
        id: room::Id,
    },

    /// FID of a [`Member`].
    Member {
        /// ID of the [`Member`] in the [`Room`].
        id: member::Id,

        /// Unique ID of the [`Room`].
        room_id: room::Id,
    },

    /// FID of an [`Endpoint`].
    Endpoint {
        /// ID of the [`Endpoint`] of the [`Member`].
        id: endpoint::Id,

        /// Unique ID of the [`Room`].
        room_id: room::Id,

        /// ID of the [`Member`] in the [`Room`].
        member_id: member::Id,
    },
}

/// [`Ping`] message received by a media server periodically for probing its
/// healthiness.
///
/// Each new [`Ping`] should increase its nonce, starting with `0`.
#[derive(Clone, Copy, Debug)]
pub struct Ping(pub u32);

/// [`Pong`] message send by a media server in response to a received [`Ping`]
/// message.
///
/// Contains nonce of the answered [`Ping`] message.
#[derive(Clone, Copy, Debug)]
pub struct Pong(pub u32);
