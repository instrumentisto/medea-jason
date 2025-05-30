//! `RemoteMediaTrack` JS object's representation.

use super::Error;
use crate::{browser::Statement, object::Object};

/// Media exchange direction of a `RemoteMediaTrack`.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum MediaDirection {
    /// `RemoteMediaTrack` is enabled on both receiver and sender sides.
    SendRecv = 0,

    /// `RemoteMediaTrack` is enabled on sender side only.
    SendOnly = 1,

    /// `RemoteMediaTrack` is enabled on receiver side only.
    RecvOnly = 2,

    /// `RemoteMediaTrack` is disabled on both sides.
    Inactive = 3,
}

impl From<MediaDirection> for u8 {
    // TODO: Generate by derive?
    #[expect(clippy::as_conversions, reason = "safe conversion")]
    fn from(d: MediaDirection) -> Self {
        d as Self
    }
}

/// Representation of a `RemoteMediaTrack` object.
#[derive(Clone, Copy, Debug)]
pub struct RemoteTrack;

impl Object<RemoteTrack> {
    /// Waits for this [`RemoteTrack`] being enabled.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_enabled(&self) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (track) => {
                const currentDirection = track.track.media_direction()
                if (currentDirection != 0) {
                    let waiter = new Promise((resolve) => {
                        track.onEnabledSubs.push(resolve);
                    });
                    await waiter;
                }
            }
            ",
            [],
        ))
        .await
        .map(drop)
    }

    /// Waits for this [`RemoteTrack`] being disabled, or the
    /// `RemoteMediaTrack.on_disabled()` callback to fire.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_disabled(&self) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (track) => {
                const currentDirection = track.track.media_direction()
                if (currentDirection == 0) {
                    let waiter = new Promise((resolve) => {
                        track.onDisabledSubs.push(resolve);
                    });
                    await waiter;
                }
            }
            ",
            [],
        ))
        .await
        .map(drop)
    }

    /// Indicates whether this [`RemoteTrack`]'s underlying `MediaStreamTrack`
    /// is disabled.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn disabled(&self) -> Result<bool, Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (t) => {
                const currentDirection = t.track.media_direction();
                return currentDirection != 0;
            }
            ",
            [],
        ))
        .await?
        .as_bool()
        .ok_or(Error::TypeCast)
    }

    /// Indicates whether this [`RemoteTrack`]'s underlying `MediaStreamTrack`
    /// is live.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn lived(&self) -> Result<bool, Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (t) => {
                const currentDirection = t.track.media_direction();
                return currentDirection == 0 && !t.track.stopped;
            }
            ",
            [],
        ))
        .await?
        .as_bool()
        .ok_or(Error::TypeCast)
    }

    /// Waits for the `RemoteMediaTrack.on_disabled()` callback to fire `count`
    /// times.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_on_disabled_fire_count(
        &self,
        count: u64,
    ) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (track) => {
                const [count] = args;
                while (track.on_disabled_fire_count !== count) {
                    await new Promise((resolve) => {
                        if (track.on_disabled_fire_count !== count) {
                            track.onDisabledSubs.push(resolve);
                        } else {
                            resolve();
                        }
                    });
                }
            }
            ",
            [count.into()],
        ))
        .await
        .map(drop)
    }

    /// Waits for the `RemoteMediaTrack.on_enabled()` callback to fire `count`
    /// times.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_on_enabled_fire_count(
        &self,
        count: u64,
    ) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (track) => {
                const [count] = args;
                while (track.on_enabled_fire_count !== count) {
                    await new Promise((resolve) => {
                        if (track.on_enabled_fire_count !== count) {
                            track.onEnabledSubs.push(resolve);
                        } else {
                            resolve();
                        }
                    });
                }
            }
            ",
            [count.into()],
        ))
        .await
        .map(drop)
    }

    /// Waits for the `RemoteMediaTrack.on_muted()` callback to fire `count`
    /// times.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_on_muted_fire_count(
        &self,
        count: u64,
    ) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (track) => {
                const [count] = args;
                while (track.on_muted_fire_count !== count) {
                    await new Promise((resolve) => {
                        if (track.on_muted_fire_count !== count) {
                            track.onMutedSubs.push(resolve);
                        } else {
                            resolve();
                        }
                    });
                }
            }
            ",
            [count.into()],
        ))
        .await
        .map(drop)
    }

    /// Waits for the `RemoteMediaTrack.on_unmuted()` callback to fire `count`
    /// times.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_on_unmuted_fire_count(
        &self,
        count: u64,
    ) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (track) => {
                const [count] = args;
                while (track.on_unmuted_fire_count !== count) {
                    await new Promise((resolve) => {
                        if (track.on_unmuted_fire_count !== count) {
                            track.onUnmutedSubs.push(resolve);
                        } else {
                            resolve();
                        }
                    });
                }
            }
            ",
            [count.into()],
        ))
        .await
        .map(drop)
    }

    /// Waits for the `RemoteMediaTrack.on_media_direction` with the provided
    /// [`MediaDirection`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_media_direction(
        &self,
        direction: MediaDirection,
    ) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (track) => {
                const [direction] = args;
                    if (track.track.media_direction() != direction) {
                        let waiter = new Promise((resolve) => {
                            track.onMediaDirectionChangedSubs.push((dir) => {
                                if (dir == direction) {
                                    resolve();
                                }
                            });
                    });
                    await waiter;
                }
            }
            ",
            [u8::from(direction).into()],
        ))
        .await
        .map(drop)
    }
}
