//! [`Object`] representing a `Room` JS object.

use std::{borrow::Cow, str::FromStr};

use super::{AwaitCompletion, Error};
use crate::{
    browser::Statement,
    object::{Object, connections_store::ConnectionStore, tracks_store},
};

/// Representation of a `Room` JS object.
#[derive(Clone, Copy, Debug)]
pub struct Room;

/// Representation of a `MediaKind` JS enum.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MediaKind {
    /// Audio media.
    Audio,

    /// Video media.
    Video,
}

impl FromStr for MediaKind {
    type Err = ParsingFailedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("audio") {
            Ok(Self::Audio)
        } else if s.contains("video") {
            Ok(Self::Video)
        } else {
            Err(ParsingFailedError)
        }
    }
}

impl MediaKind {
    /// Converts this [`MediaKind`] to the JS code for this enum variant.
    #[must_use]
    pub const fn as_js(self) -> &'static str {
        match self {
            Self::Audio => "window.rust.MediaKind.Audio",
            Self::Video => "window.rust.MediaKind.Video",
        }
    }
}

/// Representation of a `MediaSourceKind` JS enum.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MediaSourceKind {
    /// Device source of media (camera, mic, etc).
    Device,

    /// Display source of media (screen sharing, etc).
    Display,
}

impl FromStr for MediaSourceKind {
    type Err = ParsingFailedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("device") {
            Ok(Self::Device)
        } else if s.contains("display") {
            Ok(Self::Display)
        } else {
            Err(ParsingFailedError)
        }
    }
}

impl MediaSourceKind {
    /// Converts this [`MediaSourceKind`] to a JS code for this enum variant.
    #[must_use]
    pub const fn as_js(self) -> &'static str {
        match self {
            Self::Device => "window.rust.MediaSourceKind.Device",
            Self::Display => "window.rust.MediaSourceKind.Display",
        }
    }
}

impl Object<Room> {
    /// Joins a [`Room`] with the provided URI.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn join(&self, uri: String) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (room) => {
                const [uri] = args;
                await room.room.join(uri);
            }
            ",
            [uri.into()],
        ))
        .await
        .map(drop)
    }

    /// Disables media publishing for the provided [`MediaKind`] and
    /// [`MediaSourceKind`].
    ///
    /// If the provided `source_kind` is [`None`], then media publishing will be
    /// disabled for all the [`MediaSourceKind`]s.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn disable_media_send(
        &self,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
        maybe_await: AwaitCompletion,
    ) -> Result<(), Error> {
        let media_source_kind =
            source_kind.map(MediaSourceKind::as_js).unwrap_or_default();
        let disable: Cow<'_, _> = match kind {
            MediaKind::Audio => "r.room.disable_audio()".into(),
            MediaKind::Video => {
                format!("r.room.disable_video({media_source_kind})").into()
            }
        };
        self.execute(Statement::new(
            // language=JavaScript
            &format!(
                "
                async (r) => {{
                    {maybe_await} {disable};
                }}
                ",
            ),
            [],
        ))
        .await
        .map(drop)
    }

    /// Enables media publishing for the provided [`MediaKind`] and
    /// [`MediaSourceKind`].
    ///
    /// If provided [`None`] `source_kind` then media publishing will be
    /// enabled for all the [`MediaSourceKind`]s.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn enable_media_send(
        &self,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
        maybe_await: AwaitCompletion,
    ) -> Result<(), Error> {
        let media_source_kind =
            source_kind.map(MediaSourceKind::as_js).unwrap_or_default();
        let enable: Cow<'_, _> = match kind {
            MediaKind::Audio => "r.room.enable_audio()".into(),
            MediaKind::Video => {
                format!("r.room.enable_video({media_source_kind})").into()
            }
        };
        self.execute(Statement::new(
            // language=JavaScript
            &format!(
                "
                async (r) => {{
                    {maybe_await} {enable};
                }}
                ",
            ),
            [],
        ))
        .await
        .map(drop)
    }

    /// Disables remote media receiving for the provided [`MediaKind`] and
    /// [`MediaSourceKind`].
    ///
    /// If provided [`None`] `source_kind` then media receiving will be disabled
    /// for all the [`MediaSourceKind`]s.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn disable_remote_media(
        &self,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) -> Result<(), Error> {
        let media_source_kind =
            source_kind.map(MediaSourceKind::as_js).unwrap_or_default();
        let disable: Cow<'_, _> = match kind {
            MediaKind::Audio => "r.room.disable_remote_audio()".into(),
            MediaKind::Video => {
                format!("r.room.disable_remote_video({media_source_kind})")
                    .into()
            }
        };
        self.execute(Statement::new(
            // language=JavaScript
            &format!(
                "
                async (r) => {{
                    await {disable};
                }}
                ",
            ),
            [],
        ))
        .await
        .map(drop)
    }

    /// Enables remote media receiving for the provided [`MediaKind`] and
    /// [`MediaSourceKind`].
    ///
    /// If provided [`None`] `source_kind` then media receiving will be enabled
    /// for all the [`MediaSourceKind`]s.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn enable_remote_media(
        &self,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) -> Result<(), Error> {
        let media_source_kind =
            source_kind.map(MediaSourceKind::as_js).unwrap_or_default();
        let enable: Cow<'_, _> = match kind {
            MediaKind::Audio => "r.room.enable_remote_audio()".into(),
            MediaKind::Video => {
                format!("r.room.enable_remote_video({media_source_kind})")
                    .into()
            }
        };
        self.execute(Statement::new(
            // language=JavaScript
            &format!(
                "
                async (r) => {{
                    await {enable};
                }}
                ",
            ),
            [],
        ))
        .await
        .map(drop)
    }

    /// Mutes media publishing for the provided [`MediaKind`] and
    /// [`MediaSourceKind`].
    ///
    /// If provided [`None`] `source_kind` then media publishing will be muted
    /// for all the [`MediaSourceKind`]s.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn mute_media(
        &self,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
        maybe_await: AwaitCompletion,
    ) -> Result<(), Error> {
        let media_source_kind =
            source_kind.map(MediaSourceKind::as_js).unwrap_or_default();
        let mute: Cow<'_, _> = match kind {
            MediaKind::Audio => "r.room.mute_audio()".into(),
            MediaKind::Video => {
                format!("r.room.mute_video({media_source_kind})").into()
            }
        };
        self.execute(Statement::new(
            // language=JavaScript
            &format!(
                "
                async (r) => {{
                    {maybe_await} {mute};
                }}
                ",
            ),
            [],
        ))
        .await
        .map(drop)
    }

    /// Unmutes media publishing for the provided [`MediaKind`] and
    /// [`MediaSourceKind`].
    ///
    /// If provided [`None`] `source_kind` then media publishing will be unmuted
    /// for all the [`MediaSourceKind`]s.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn unmute_media(
        &self,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
        maybe_await: AwaitCompletion,
    ) -> Result<(), Error> {
        let media_source_kind =
            source_kind.map(MediaSourceKind::as_js).unwrap_or_default();
        let unmute: Cow<'_, _> = match kind {
            MediaKind::Audio => "r.room.unmute_audio()".into(),
            MediaKind::Video => {
                format!("r.room.unmute_video({media_source_kind})").into()
            }
        };
        self.execute(Statement::new(
            // language=JavaScript
            &format!(
                "
                async (r) => {{
                    {maybe_await} {unmute};
                }}
                ",
            ),
            [],
        ))
        .await
        .map(drop)
    }

    /// Returns a [`ConnectionStore`] of this [`Room`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn connections_store(
        &self,
    ) -> Result<Object<ConnectionStore>, Error> {
        self.execute_and_fetch(Statement::new(
            // language=JavaScript
            "
            async (r) => {
                let store = {
                    connections: new Map(),
                    subs: new Map(),
                };
                r.room.on_new_connection((conn) => {
                    let closeListener = {
                        isClosed: false,
                        subs: [],
                    };
                    let tracksStore = {
                        tracks: [],
                        subs: []
                    };
                    let connection = {
                        conn: conn,
                        tracksStore: tracksStore,
                        closeListener: closeListener,
                    };
                    conn.on_remote_track_added((t) => {
                        let track = {
                            track: t,
                            on_enabled_fire_count: 0,
                            on_disabled_fire_count: 0,
                            on_muted_fire_count: 0,
                            on_unmuted_fire_count: 0,
                            stopped: false,
                            onEnabledSubs: [],
                            onDisabledSubs: [],
                            onMutedSubs: [],
                            onUnmutedSubs: [],
                            onMediaDirectionChangedSubs: []
                        };
                        track.track.on_muted(() => {
                            track.on_muted_fire_count++;
                            for (sub of track.onMutedSubs) {
                                sub();
                            }
                            track.onMutedSubs = [];
                        });
                        track.track.on_unmuted(() => {
                            track.on_unmuted_fire_count++;
                            for (sub of track.onUnmutedSubs) {
                                sub();
                            }
                            track.onUnmutedSubs = [];
                        });
                        track.track.on_stopped(() => {
                            track.stopped = true;
                        });
                        track.track.on_media_direction_changed((dir) => {
                            if (dir == 0) {
                                track.on_enabled_fire_count++;
                                for (sub of track.onEnabledSubs) {
                                    sub();
                                }
                                track.onEnabledSubs = [];
                            } else {
                                track.on_disabled_fire_count++;
                                for (sub of track.onDisabledSubs) {
                                    sub(dir);
                                }
                                track.onDisabledSubs = [];
                            }

                            for (sub of track.onMediaDirectionChangedSubs) {
                                sub(dir);
                            }
                            track.onMediaDirectionChangedSubs = [];
                        });
                        tracksStore.tracks.push(track);
                        let newStoreSubs = tracksStore.subs
                            .filter((sub) => {
                                return sub(track);
                            });
                        tracksStore.subs = newStoreSubs;
                    });
                    conn.on_close(() => {
                        closeListener.isClosed = true;
                        for (sub of closeListener.subs) {
                            sub();
                        }
                    });
                    let id = conn.get_remote_member_id();
                    store.connections.set(id, connection);
                    let sub = store.subs.get(id);
                    if (sub !== undefined) {
                        sub(connection);
                    }
                });
                return store;
            }
            ",
            [],
        ))
        .await
    }

    /// Returns a [`tracks_store::Local`] of this [`Room`].
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn local_tracks(
        &self,
    ) -> Result<Object<tracks_store::Local>, Error> {
        self.execute_and_fetch(Statement::new(
            // language=JavaScript
            "async (room) => room.localTracksStore",
            [],
        ))
        .await
    }

    /// Waits for the `Room.on_close()` callback to fire.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_close(&self) -> Result<String, Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (room) => {
                if (room.closeListener.isClosed) {
                    return room.closeListener.closeReason.reason();
                } else {
                    let waiter = new Promise((resolve) => {
                        room.closeListener.subs.push(resolve);
                    });

                    let closeReason = await waiter;
                    return closeReason.reason();
                }
            }
            ",
            [],
        ))
        .await?
        .as_str()
        .ok_or(Error::TypeCast)
        .map(ToOwned::to_owned)
    }

    /// Waits for the `Room.on_connection_loss()` callback to fire.
    ///
    /// Resolves instantly if WebSocket connection currently is lost.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn wait_for_connection_loss(&self) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (room) => {
                if (!room.connLossListener.isLost) {
                    await new Promise((resolve) => {
                        room.connLossListener.subs.push(resolve);
                    });
                }
            }
            ",
            [],
        ))
        .await
        .map(drop)
    }

    /// Calls `ReconnectHandle.reconnect_with_backoff()`.
    ///
    /// # Errors
    ///
    /// Should be called only if connection was previously lost and
    /// a `ReconnectHandle` was obtained, otherwise method will error.
    pub async fn start_ws_reconnect(&self) -> Result<(), Error> {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (room) => {
                await room
                    .connLossListener
                    .reconnectHandle
                    .reconnect_with_backoff(100, 2.0, 1000, 5000);
            }
            ",
            [],
        ))
        .await
        .map(drop)
    }

    /// Enables or disables media type with a `Room.set_local_media_settings()`
    /// function call.
    ///
    /// # Errors
    ///
    /// If failed to execute JS statement.
    pub async fn set_local_media_settings(
        &self,
        video: bool,
        audio: bool,
        should_wait: bool,
    ) -> Result<(), Error> {
        self.forget_local_tracks().await;
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (room) => {
                const [video, audio, shouldWait] = args;
                let constraints = new rust.MediaStreamSettings();
                if (video) {
                    let video =
                        new window.rust.DeviceVideoTrackConstraints();
                    constraints.device_video(video);
                }
                if (audio) {
                    let audio = new window.rust.AudioTrackConstraints();
                    constraints.audio(audio);
                }
                let promise = room.room.set_local_media_settings(
                    constraints,
                    true,
                    false
                );
                if (shouldWait) {
                    await promise;
                }
            }
            ",
            [video.into(), audio.into(), should_wait.into()],
        ))
        .await
        .map(drop)
    }

    /// Waits for the `Room.on_failed_local_stream()` callback to fire the
    /// provided number of times.
    ///
    /// # Panics
    ///
    /// If failed to execute JS statement.
    pub async fn when_failed_local_stream_count(&self, count: u64) {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (room) => {
                const [count] = args;
                return await new Promise((resolve) => {
                    if (room.onFailedLocalStreamListener.count === count) {
                        resolve();
                    } else {
                        room.onFailedLocalStreamListener.subs.push(() => {
                            let failCount =
                                room.onFailedLocalStreamListener.count;
                            if (failCount === count) {
                                resolve();
                                return false;
                            } else {
                                return true;
                            }
                        });
                    }
                });
            }
            ",
            [count.into()],
        ))
        .await
        .map(drop)
        .unwrap();
    }

    /// Removes all local `LocalMediaTrack`s from the JS side.
    ///
    /// # Panics
    ///
    /// If failed to execute JS statement.
    pub async fn forget_local_tracks(&self) {
        self.execute(Statement::new(
            // language=JavaScript
            "
            async (room) => {
                room.localTracksStore.tracks.forEach((t) => t.track.free());
                room.localTracksStore.tracks = [];
            }
            ",
            [],
        ))
        .await
        .map(drop)
        .unwrap();
    }
}

/// Error of parsing a [`MediaKind`] or a [`MediaSourceKind`].
#[derive(Clone, Copy, Debug)]
pub struct ParsingFailedError;
