//! Media tracks and streams constraints functionality.

use std::{cell::RefCell, rc::Rc};

use derive_more::with_trait::Display;
use futures::stream::LocalBoxStream;
use medea_client_api_proto::{
    AudioSettings as ProtoAudioConstraints, AudioSettings, MediaSourceKind,
    MediaType as ProtoTrackConstraints, MediaType, VideoSettings,
};
use medea_reactive::ObservableCell;

use crate::{
    media::{MediaKind, track::MediaStreamTrackState},
    peer::{
        LocalStreamUpdateCriteria, MediaState, media_exchange_state, mute_state,
    },
    platform,
};

/// Describes directions that a camera can face, as seen from a user's
/// perspective.
///
/// Representation of a [VideoFacingModeEnum][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-videofacingmodeenum
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
#[repr(u8)]
pub enum FacingMode {
    /// Facing towards a user (a self-view camera).
    #[display("user")]
    User = 0,

    /// Facing away from a user (viewing an environment).
    #[display("environment")]
    Environment = 1,

    /// Facing to the left of a user.
    #[display("left")]
    Left = 2,

    /// Facing to the right of a user.
    #[display("right")]
    Right = 3,
}

/// Audio processing noise suppression aggressiveness.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum NoiseSuppressionLevel {
    /// Minimal noise suppression.
    Low = 0,

    /// Moderate level of suppression.
    Moderate = 1,

    /// Aggressive noise suppression.
    High = 2,

    /// Maximum suppression.
    VeryHigh = 3,
}

/// Local media stream for injecting into new created [`PeerConnection`]s.
///
/// [`PeerConnection`]: crate::peer::PeerConnection
#[derive(Clone, Debug, Default)]
pub struct LocalTracksConstraints(Rc<RefCell<MediaStreamSettings>>);

/// Constraints to the media received from remote. Used to disable or enable
/// media receiving.
#[expect( // intended: all fields have the same postfix `enabled`
    clippy::struct_field_names,
    reason = "intended: all fields have the same postfix `enabled`"
)]
#[derive(Debug)]
pub struct RecvConstraints {
    /// Indicator whether device audio receiving is enabled.
    audio_device_enabled: ObservableCell<bool>,

    /// Indicator whether display audio receiving is enabled.
    audio_display_enabled: ObservableCell<bool>,

    /// Indicator whether device video receiving is enabled.
    video_device_enabled: ObservableCell<bool>,

    /// Indicator whether display video receiving is enabled.
    video_display_enabled: ObservableCell<bool>,
}

impl Clone for RecvConstraints {
    fn clone(&self) -> Self {
        Self {
            audio_device_enabled: ObservableCell::new(
                self.audio_device_enabled.get(),
            ),
            audio_display_enabled: ObservableCell::new(
                self.audio_display_enabled.get(),
            ),
            video_device_enabled: ObservableCell::new(
                self.video_device_enabled.get(),
            ),
            video_display_enabled: ObservableCell::new(
                self.video_display_enabled.get(),
            ),
        }
    }
}

impl Default for RecvConstraints {
    fn default() -> Self {
        Self {
            audio_device_enabled: ObservableCell::new(true),
            audio_display_enabled: ObservableCell::new(true),
            video_device_enabled: ObservableCell::new(true),
            video_display_enabled: ObservableCell::new(true),
        }
    }
}

impl RecvConstraints {
    /// Enables or disables audio or video receiving.
    pub fn set_enabled(
        &self,
        enabled: bool,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) {
        match kind {
            MediaKind::Audio => source_kind.map_or_else(
                || {
                    self.audio_device_enabled.set(enabled);
                    self.audio_display_enabled.set(enabled);
                },
                |sk| match sk {
                    MediaSourceKind::Device => {
                        self.audio_device_enabled.set(enabled);
                    }
                    MediaSourceKind::Display => {
                        self.audio_display_enabled.set(enabled);
                    }
                },
            ),
            MediaKind::Video => source_kind.map_or_else(
                || {
                    self.video_device_enabled.set(enabled);
                    self.video_display_enabled.set(enabled);
                },
                |sk| match sk {
                    MediaSourceKind::Device => {
                        self.video_device_enabled.set(enabled);
                    }
                    MediaSourceKind::Display => {
                        self.video_display_enabled.set(enabled);
                    }
                },
            ),
        }
    }

    /// Indicates whether device audio receiving is enabled.
    pub fn is_audio_device_enabled(&self) -> bool {
        self.audio_device_enabled.get()
    }

    /// Indicates whether display audio receiving is enabled.
    pub fn is_audio_display_enabled(&self) -> bool {
        self.audio_display_enabled.get()
    }

    /// Indicates whether device video receiving is enabled.
    pub fn is_video_device_enabled(&self) -> bool {
        self.video_device_enabled.get()
    }

    /// Indicates whether display video receiving is enabled.
    pub fn is_video_display_enabled(&self) -> bool {
        self.video_display_enabled.get()
    }

    /// Returns [`LocalBoxStream`] into which all [`is_audio_device_enabled`]
    /// updates will be sent.
    ///
    /// [`is_audio_device_enabled`]: Self::is_audio_device_enabled()
    pub fn on_audio_device_enabled_change(
        &self,
    ) -> LocalBoxStream<'static, bool> {
        self.audio_device_enabled.subscribe()
    }

    /// Returns [`LocalBoxStream`] into which all [`is_audio_display_enabled`]
    /// updates will be sent.
    ///
    /// [`is_audio_display_enabled`]: Self::is_audio_display_enabled()
    pub fn on_audio_display_enabled_change(
        &self,
    ) -> LocalBoxStream<'static, bool> {
        self.audio_display_enabled.subscribe()
    }

    /// Returns [`LocalBoxStream`] into which all [`is_video_device_enabled`]
    /// updates will be sent.
    ///
    /// [`is_video_device_enabled`]: RecvConstraints::is_video_device_enabled()
    pub fn on_video_device_enabled_change(
        &self,
    ) -> LocalBoxStream<'static, bool> {
        self.video_device_enabled.subscribe()
    }

    /// Returns [`LocalBoxStream`] into which all [`is_video_display_enabled`]
    /// updates will be sent.
    ///
    /// [`is_video_display_enabled`]: RecvConstraints::is_video_display_enabled
    pub fn on_video_display_enabled_change(
        &self,
    ) -> LocalBoxStream<'static, bool> {
        self.video_display_enabled.subscribe()
    }
}

#[cfg(feature = "mockable")]
impl From<MediaStreamSettings> for LocalTracksConstraints {
    fn from(from: MediaStreamSettings) -> Self {
        Self(Rc::new(RefCell::new(from)))
    }
}

impl LocalTracksConstraints {
    /// Returns [`LocalStreamUpdateCriteria`] with [`MediaKind`] and
    /// [`MediaSourceKind`] which are different in the provided
    /// [`MediaStreamSettings`].
    #[must_use]
    pub fn calculate_kinds_diff(
        &self,
        settings: &MediaStreamSettings,
    ) -> LocalStreamUpdateCriteria {
        self.0.borrow().calculate_kinds_diff(settings)
    }

    /// Constrains the underlying [`MediaStreamSettings`] with the given `other`
    /// [`MediaStreamSettings`].
    pub fn constrain(&self, other: MediaStreamSettings) {
        self.0.borrow_mut().constrain(other);
    }

    /// Clones the underlying [`MediaStreamSettings`].
    #[must_use]
    pub fn inner(&self) -> MediaStreamSettings {
        self.0.borrow().clone()
    }

    /// Changes the underlying [`MediaStreamSettings`] basing on the provided
    /// [`MediaState`].
    pub fn set_media_state(
        &self,
        state: MediaState,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) {
        self.0.borrow_mut().set_track_media_state(state, kind, source_kind);
    }

    /// Enables/disables provided [`LocalStreamUpdateCriteria`] based on
    /// provided [`media_exchange_state`].
    pub fn set_media_exchange_state_by_kinds(
        &self,
        state: media_exchange_state::Stable,
        kinds: LocalStreamUpdateCriteria,
    ) {
        self.0.borrow_mut().set_media_exchange_state_by_kinds(state, kinds);
    }

    /// Indicates whether provided [`MediaType`] is enabled in the underlying
    /// [`MediaStreamSettings`].
    #[must_use]
    pub fn enabled(&self, kind: &MediaType) -> bool {
        self.0.borrow().enabled(kind)
    }

    /// Indicates whether provided [`MediaType`] is muted in the underlying
    /// [`MediaStreamSettings`].
    #[must_use]
    pub fn muted(&self, kind: &MediaType) -> bool {
        self.0.borrow().muted(kind)
    }

    /// Indicates whether the provided [`MediaKind`] and [`MediaSourceKind`] are
    /// enabled and constrained in this [`LocalTracksConstraints`].
    #[must_use]
    pub fn is_track_enabled_and_constrained(
        &self,
        kind: MediaKind,
        source: Option<MediaSourceKind>,
    ) -> bool {
        self.0.borrow().is_track_enabled_and_constrained(kind, source)
    }

    /// Indicates whether the provided [`MediaKind`] and [`MediaSourceKind`] are
    /// enabled in this [`LocalTracksConstraints`].
    #[must_use]
    pub fn is_track_enabled(
        &self,
        kind: MediaKind,
        source: Option<MediaSourceKind>,
    ) -> bool {
        self.0.borrow().is_track_enabled(kind, source)
    }
}

/// [MediaStreamConstraints][1] for the audio media type.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AudioMediaTracksSettings {
    /// Constraints applicable to video tracks.
    constraints: AudioTrackConstraints,

    /// Indicator whether audio is enabled and this constraints should be
    /// injected into `Peer`.
    enabled: bool,

    /// Indicator whether audio should be muted.
    muted: bool,
}

impl Default for AudioMediaTracksSettings {
    fn default() -> Self {
        Self {
            constraints: AudioTrackConstraints::default(),
            enabled: true,
            muted: false,
        }
    }
}

/// Indicates whether the provided [`platform::MediaStreamTrack`] satisfies any
/// constraints with the provided [`MediaKind`].
async fn satisfies_track(
    track: &platform::MediaStreamTrack,
    kind: MediaKind,
) -> bool {
    track.kind() == kind
        && track.ready_state().await == MediaStreamTrackState::Live
}

/// [MediaStreamConstraints][1] for the video media type.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VideoTrackConstraints<C> {
    /// Constraints applicable to video tracks.
    ///
    /// If [`None`] then this kind of video (device or display) is disabled by
    /// [`MediaStreamSettings`].
    constraints: Option<C>,

    /// Indicator whether video is enabled and this constraints should be
    /// injected into `Peer`.
    ///
    /// Any action with this flag should be performed only while disable/enable
    /// actions by [`Room`]. This flag can't be changed by
    /// [`MediaStreamSettings`] updating.
    ///
    /// [`Room`]: crate::room::Room
    enabled: bool,

    /// Indicator whether video should be muted.
    muted: bool,
}

impl<C: Default> Default for VideoTrackConstraints<C> {
    fn default() -> Self {
        Self { constraints: Some(C::default()), enabled: true, muted: false }
    }
}

impl<C> VideoTrackConstraints<C> {
    /// Returns `true` if this [`VideoTrackConstraints`] are enabled by the
    /// [`Room`] and constrained with [`VideoTrackConstraints::constraints`].
    ///
    /// [`Room`]: crate::room::Room
    const fn enabled(&self) -> bool {
        self.enabled && self.is_constrained()
    }

    /// Sets these [`VideoTrackConstraints::constraints`] to the provided
    /// `cons`.
    fn set(&mut self, cons: C) {
        self.constraints = Some(cons);
    }

    /// Resets these [`VideoTrackConstraints::constraints`] to [`None`].
    fn unconstrain(&mut self) {
        drop(self.constraints.take());
    }

    /// Returns `true` if these [`VideoTrackConstraints::constraints`] are set
    /// to [`Some`] value.
    const fn is_constrained(&self) -> bool {
        self.constraints.is_some()
    }

    /// Constraints these [`VideoTrackConstraints`] with a provided `other`
    /// [`VideoTrackConstraints`].
    fn constrain(&mut self, other: Self) {
        self.enabled &= other.enabled;
        self.constraints = other.constraints;
    }
}

impl VideoTrackConstraints<DeviceVideoTrackConstraints> {
    /// Indicates whether the provided [`platform::MediaStreamTrack`] satisfies
    /// these [`VideoTrackConstraints`].
    ///
    /// Returns `false` if these [`VideoTrackConstraints`] don't have any
    /// constraints configured.
    pub async fn satisfies<T: AsRef<platform::MediaStreamTrack>>(
        &self,
        track: T,
    ) -> bool {
        if let Some(constraints) = &self.constraints {
            self.enabled() && constraints.satisfies(track).await
        } else {
            false
        }
    }
}

impl VideoTrackConstraints<DisplayVideoTrackConstraints> {
    /// Indicates whether the provided [`platform::MediaStreamTrack`] satisfies
    /// these [`VideoTrackConstraints`].
    ///
    /// Returns `false` if these [`VideoTrackConstraints`] don't have any
    /// constraints configured.
    pub async fn satisfies<T: AsRef<platform::MediaStreamTrack>>(
        &self,
        track: T,
    ) -> bool {
        if let Some(constraints) = &self.constraints {
            self.enabled() && constraints.satisfies(track).await
        } else {
            false
        }
    }
}

/// [MediaStreamConstraints][1] wrapper.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MediaStreamSettings {
    /// [MediaStreamConstraints][1] for the device audio media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    device_audio: AudioMediaTracksSettings,

    /// [MediaStreamConstraints][1] for the display audio media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    display_audio: AudioMediaTracksSettings,

    /// [MediaStreamConstraints][1] for the device video media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    device_video: VideoTrackConstraints<DeviceVideoTrackConstraints>,

    /// [MediaStreamConstraints][1] for the display video media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    display_video: VideoTrackConstraints<DisplayVideoTrackConstraints>,
}

impl MediaStreamSettings {
    /// Creates new [`MediaStreamSettings`] with none constraints configured.
    #[must_use]
    pub fn new() -> Self {
        Self {
            device_audio: AudioMediaTracksSettings {
                constraints: AudioTrackConstraints::default(),
                enabled: false,
                muted: false,
            },
            display_audio: AudioMediaTracksSettings {
                constraints: AudioTrackConstraints::default(),
                enabled: false,
                muted: false,
            },
            display_video: VideoTrackConstraints {
                enabled: true,
                constraints: None,
                muted: false,
            },
            device_video: VideoTrackConstraints {
                enabled: true,
                constraints: None,
                muted: false,
            },
        }
    }

    /// Specifies the nature and settings of the device audio
    /// [`platform::MediaStreamTrack`].
    pub fn device_audio(&mut self, constraints: AudioTrackConstraints) {
        self.device_audio.enabled = true;
        self.device_audio.constraints = constraints;
    }

    /// Specifies the nature and settings of the display (system) audio
    /// [`platform::MediaStreamTrack`].
    pub fn display_audio(&mut self, constraints: AudioTrackConstraints) {
        self.display_audio.enabled = true;
        self.display_audio.constraints = constraints;
    }

    /// Set constraints that will be used to obtain local video sourced from
    /// media device.
    pub fn device_video(&mut self, constraints: DeviceVideoTrackConstraints) {
        self.device_video.set(constraints);
    }

    /// Set constraints that will be used to capture local video from user
    /// display.
    pub fn display_video(&mut self, constraints: DisplayVideoTrackConstraints) {
        self.display_video.set(constraints);
    }

    /// Indicates whether the provided [`platform::MediaStreamTrack`] satisfies
    /// some of the [`VideoTrackConstraints`] from this [`MediaStreamSettings`].
    ///
    /// Unconstrains [`VideoTrackConstraints`] which this
    /// [`platform::MediaStreamTrack`] satisfies.
    pub async fn unconstrain_if_satisfies_video<T>(&mut self, track: T) -> bool
    where
        T: AsRef<platform::MediaStreamTrack>,
    {
        if self.device_video.satisfies(&track).await {
            self.device_video.unconstrain();
            true
        } else if self.display_video.satisfies(&track).await {
            self.display_video.unconstrain();
            true
        } else {
            false
        }
    }

    /// Returns [`LocalStreamUpdateCriteria`] with [`MediaKind`] and
    /// [`MediaSourceKind`] which are different in the provided
    /// [`MediaStreamSettings`].
    #[must_use]
    pub fn calculate_kinds_diff(
        &self,
        another: &Self,
    ) -> LocalStreamUpdateCriteria {
        let mut kinds = LocalStreamUpdateCriteria::empty();

        if self.device_video != another.device_video {
            kinds.add(MediaKind::Video, MediaSourceKind::Device);
        }

        if self.display_video != another.display_video {
            kinds.add(MediaKind::Video, MediaSourceKind::Display);
        }

        if self.device_audio != another.device_audio {
            kinds.add(MediaKind::Audio, MediaSourceKind::Device);
        }

        if self.display_audio != another.display_audio {
            kinds.add(MediaKind::Audio, MediaSourceKind::Display);
        }

        kinds
    }

    /// Returns only device audio constraints.
    #[must_use]
    pub const fn get_device_audio(&self) -> &AudioTrackConstraints {
        &self.device_audio.constraints
    }

    /// Returns only display audio constraints.
    #[must_use]
    pub const fn get_display_audio(&self) -> &AudioTrackConstraints {
        &self.display_audio.constraints
    }

    /// Returns reference to [`DisplayVideoTrackConstraints`] from this
    /// [`MediaStreamSettings`].
    ///
    /// Returns [`None`] if [`DisplayVideoTrackConstraints`] is unconstrained.
    #[must_use]
    pub const fn get_display_video(
        &self,
    ) -> Option<&DisplayVideoTrackConstraints> {
        self.display_video.constraints.as_ref()
    }

    /// Returns reference to [`DeviceVideoTrackConstraints`] from this
    /// [`MediaStreamSettings`].
    ///
    /// Returns [`None`] if [`DeviceVideoTrackConstraints`] is unconstrained.
    #[must_use]
    pub const fn get_device_video(
        &self,
    ) -> Option<&DeviceVideoTrackConstraints> {
        self.device_video.constraints.as_ref()
    }

    /// Changes [`MediaState`] of audio or video type in this
    /// [`MediaStreamSettings`].
    ///
    /// If some type of the [`MediaStreamSettings`] is disabled, then this kind
    /// of media won't be published.
    pub fn set_track_media_state(
        &mut self,
        state: MediaState,
        kind: MediaKind,
        source_kind: Option<MediaSourceKind>,
    ) {
        match kind {
            MediaKind::Audio => match state {
                MediaState::Mute(muted) => {
                    self.set_audio_muted(
                        muted == mute_state::Stable::Muted,
                        source_kind,
                    );
                }
                MediaState::MediaExchange(media_exchange) => {
                    self.set_audio_publish(
                        media_exchange == media_exchange_state::Stable::Enabled,
                        source_kind,
                    );
                }
            },
            MediaKind::Video => match state {
                MediaState::Mute(muted) => {
                    self.set_video_muted(
                        muted == mute_state::Stable::Muted,
                        source_kind,
                    );
                }
                MediaState::MediaExchange(media_exchange) => {
                    self.set_video_publish(
                        media_exchange == media_exchange_state::Stable::Enabled,
                        source_kind,
                    );
                }
            },
        }
    }

    /// Enables/disables provided [`LocalStreamUpdateCriteria`] based on
    /// provided [`media_exchange_state`].
    pub fn set_media_exchange_state_by_kinds(
        &mut self,
        state: media_exchange_state::Stable,
        kinds: LocalStreamUpdateCriteria,
    ) {
        let enabled = state == media_exchange_state::Stable::Enabled;

        if kinds.has(MediaKind::Audio, MediaSourceKind::Device) {
            self.set_audio_publish(enabled, Some(MediaSourceKind::Device));
        }

        if kinds.has(MediaKind::Audio, MediaSourceKind::Display) {
            self.set_audio_publish(enabled, Some(MediaSourceKind::Display));
        }

        if kinds.has(MediaKind::Video, MediaSourceKind::Device) {
            self.set_video_publish(enabled, Some(MediaSourceKind::Device));
        }

        if kinds.has(MediaKind::Video, MediaSourceKind::Display) {
            self.set_video_publish(enabled, Some(MediaSourceKind::Display));
        }
    }

    /// Sets the underlying [`AudioMediaTracksSettings::muted`] to the provided
    /// value.
    const fn set_audio_muted(
        &mut self,
        muted: bool,
        source_kind: Option<MediaSourceKind>,
    ) {
        match source_kind {
            None => {
                self.display_audio.muted = muted;
                self.device_audio.muted = muted;
            }
            Some(MediaSourceKind::Device) => {
                self.device_audio.muted = muted;
            }
            Some(MediaSourceKind::Display) => {
                self.display_audio.muted = muted;
            }
        }
    }

    /// Sets the underlying [`VideoTrackConstraints::muted`] basing on the
    /// provided [`MediaSourceKind`] to the given value.
    const fn set_video_muted(
        &mut self,
        muted: bool,
        source_kind: Option<MediaSourceKind>,
    ) {
        match source_kind {
            None => {
                self.display_video.muted = muted;
                self.device_video.muted = muted;
            }
            Some(MediaSourceKind::Device) => {
                self.device_video.muted = muted;
            }
            Some(MediaSourceKind::Display) => {
                self.display_video.muted = muted;
            }
        }
    }

    /// Sets the underlying `enabled` field of these
    /// [`AudioMediaTracksSettings`] to the given value.
    pub const fn set_audio_publish(
        &mut self,
        enabled: bool,
        source_kind: Option<MediaSourceKind>,
    ) {
        match source_kind {
            None => {
                self.display_audio.enabled = enabled;
                self.device_audio.enabled = enabled;
            }
            Some(MediaSourceKind::Device) => {
                self.device_audio.enabled = enabled;
            }
            Some(MediaSourceKind::Display) => {
                self.display_audio.enabled = enabled;
            }
        }
    }

    /// Sets the underlying [`VideoTrackConstraints`] basing on the provided
    /// [`MediaSourceKind`] to the given value.
    pub const fn set_video_publish(
        &mut self,
        enabled: bool,
        source_kind: Option<MediaSourceKind>,
    ) {
        match source_kind {
            None => {
                self.display_video.enabled = enabled;
                self.device_video.enabled = enabled;
            }
            Some(MediaSourceKind::Device) => {
                self.device_video.enabled = enabled;
            }
            Some(MediaSourceKind::Display) => {
                self.display_video.enabled = enabled;
            }
        }
    }

    /// Indicates whether device audio is enabled in this
    /// [`MediaStreamSettings`].
    #[must_use]
    pub const fn is_device_audio_enabled(&self) -> bool {
        self.device_audio.enabled
    }

    /// Indicates whether display audio is enabled in this
    /// [`MediaStreamSettings`].
    #[must_use]
    pub const fn is_display_audio_enabled(&self) -> bool {
        self.display_audio.enabled
    }

    /// Returns `true` if [`DeviceVideoTrackConstraints`] are currently
    /// constrained and enabled.
    #[must_use]
    pub const fn is_device_video_enabled(&self) -> bool {
        self.device_video.enabled()
    }

    /// Returns `true` if [`DisplayVideoTrackConstraints`] are currently
    /// constrained and enabled.
    #[must_use]
    pub const fn is_display_video_enabled(&self) -> bool {
        self.display_video.enabled()
    }

    /// Indicates whether the given [`MediaType`] is enabled and constrained in
    /// this [`MediaStreamSettings`].
    #[must_use]
    pub const fn enabled(&self, kind: &MediaType) -> bool {
        match kind {
            MediaType::Video(video) => self.is_track_enabled_and_constrained(
                MediaKind::Video,
                Some(video.source_kind),
            ),
            MediaType::Audio(audio) => self.is_track_enabled_and_constrained(
                MediaKind::Audio,
                Some(audio.source_kind),
            ),
        }
    }

    /// Indicates whether the given [`MediaType`] is muted in this
    /// [`MediaStreamSettings`].
    #[must_use]
    pub const fn muted(&self, kind: &MediaType) -> bool {
        match kind {
            MediaType::Video(video) => match video.source_kind {
                MediaSourceKind::Device => self.device_video.muted,
                MediaSourceKind::Display => self.display_video.muted,
            },
            MediaType::Audio(audio) => match audio.source_kind {
                MediaSourceKind::Device => self.device_audio.muted,
                MediaSourceKind::Display => self.display_audio.muted,
            },
        }
    }

    /// Indicates whether the given [`MediaKind`] and [`MediaSourceKind`] are
    /// enabled and constrained in this [`MediaStreamSettings`].
    #[must_use]
    pub const fn is_track_enabled_and_constrained(
        &self,
        kind: MediaKind,
        source: Option<MediaSourceKind>,
    ) -> bool {
        match (kind, source) {
            (MediaKind::Video, Some(MediaSourceKind::Device)) => {
                self.device_video.enabled()
            }
            (MediaKind::Video, Some(MediaSourceKind::Display)) => {
                self.display_video.enabled()
            }
            (MediaKind::Video, None) => {
                self.display_video.enabled() && self.device_video.enabled()
            }
            (MediaKind::Audio, Some(MediaSourceKind::Device)) => {
                self.device_audio.enabled
            }
            (MediaKind::Audio, Some(MediaSourceKind::Display)) => {
                self.display_audio.enabled
            }
            (MediaKind::Audio, None) => {
                self.device_audio.enabled && self.display_audio.enabled
            }
        }
    }

    /// Indicates whether the given [`MediaKind`] and [`MediaSourceKind`] are
    /// enabled in this [`MediaStreamSettings`].
    #[must_use]
    pub const fn is_track_enabled(
        &self,
        kind: MediaKind,
        source: Option<MediaSourceKind>,
    ) -> bool {
        match (kind, source) {
            (MediaKind::Video, Some(MediaSourceKind::Device)) => {
                self.device_video.enabled
            }
            (MediaKind::Video, Some(MediaSourceKind::Display)) => {
                self.display_video.enabled
            }
            (MediaKind::Video, None) => {
                self.display_video.enabled && self.device_video.enabled
            }
            (MediaKind::Audio, Some(MediaSourceKind::Device)) => {
                self.device_audio.enabled
            }
            (MediaKind::Audio, Some(MediaSourceKind::Display)) => {
                self.display_audio.enabled
            }
            (MediaKind::Audio, None) => {
                self.device_audio.enabled && self.display_audio.enabled
            }
        }
    }

    /// Constrains this [`MediaStreamSettings`] with the given `other`
    /// [`MediaStreamSettings`].
    fn constrain(&mut self, other: Self) {
        // `&=` cause we should not enable disabled Room, but we can disable
        // enabled room.
        self.device_audio.enabled &= other.device_audio.enabled;
        self.device_audio.constraints = other.device_audio.constraints;
        self.display_audio.enabled &= other.display_audio.enabled;
        self.display_audio.constraints = other.display_audio.constraints;
        self.display_video.constrain(other.display_video);
        self.device_video.constrain(other.device_video);
    }
}

/// Wrapper around [MediaStreamConstraints][1] that specifies concrete media
/// source (device or display), and allows to group two requests with different
/// sources.
///
/// [1]: https://w3.org/TR/mediacapture-streams#mediastreamconstraints
#[derive(Debug)]
pub enum MultiSourceTracksConstraints {
    /// Only [getUserMedia()][1] request is required.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    Device(platform::MediaStreamConstraints),

    /// Only [getDisplayMedia()][1] request is required.
    ///
    /// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    Display(platform::DisplayMediaStreamConstraints),

    /// Both [getUserMedia()][1] and [getDisplayMedia()][2] are required.
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
    /// [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
    DeviceAndDisplay(
        platform::MediaStreamConstraints,
        platform::DisplayMediaStreamConstraints,
    ),
}

impl From<MediaStreamSettings> for Option<MultiSourceTracksConstraints> {
    fn from(constraints: MediaStreamSettings) -> Self {
        let is_device_video_enabled = constraints.is_device_video_enabled();
        let is_display_video_enabled = constraints.is_display_video_enabled();
        let is_device_audio_enabled = constraints.is_device_audio_enabled();
        // TODO: implement for Dart platform when `medea-flutter-webrtc` will
        //       support display audio constraints.
        #[cfg(target_family = "wasm")]
        let is_display_audio_enabled = constraints.is_display_audio_enabled();

        let mut device_cons = None;
        let mut display_cons = None;

        if is_device_video_enabled {
            if let Some(device_video_cons) =
                constraints.device_video.constraints
            {
                device_cons
                    .get_or_insert_with(platform::MediaStreamConstraints::new)
                    .video(device_video_cons);
            }
        }

        if is_display_video_enabled {
            if let Some(display_video_cons) =
                constraints.display_video.constraints
            {
                display_cons
                    .get_or_insert_with(
                        platform::DisplayMediaStreamConstraints::new,
                    )
                    .video(display_video_cons);
            }
        }

        if is_device_audio_enabled {
            device_cons
                .get_or_insert_with(platform::MediaStreamConstraints::new)
                .audio(constraints.device_audio.constraints);
        }

        // TODO: implement for Dart platform when `medea-flutter-webrtc` will
        //       support display audio constraints.
        #[cfg(target_family = "wasm")]
        if is_display_audio_enabled {
            display_cons
                .get_or_insert_with(
                    platform::DisplayMediaStreamConstraints::new,
                )
                .audio(constraints.display_audio.constraints);
        }

        match (device_cons, display_cons) {
            (Some(device_cons), Some(display_cons)) => {
                Some(MultiSourceTracksConstraints::DeviceAndDisplay(
                    device_cons,
                    display_cons,
                ))
            }
            (Some(device_cons), None) => {
                Some(MultiSourceTracksConstraints::Device(device_cons))
            }
            (None, Some(display_cons)) => {
                Some(MultiSourceTracksConstraints::Display(display_cons))
            }
            (None, None) => None,
        }
    }
}

/// Constraints for the [`MediaKind::Video`] [`local::Track`].
///
/// [`local::Track`]: crate::media::track::local::Track
#[derive(Clone, Debug)]
pub enum VideoSource {
    /// [`local::Track`] should be received from the `getUserMedia` request.
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    Device(DeviceVideoTrackConstraints),

    /// [`local::Track`] should be received from the `getDisplayMedia` request.
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    Display(DisplayVideoTrackConstraints),
}

impl VideoSource {
    /// Returns an importance of this [`VideoSource`].
    ///
    /// If this [`VideoSource`] is important then without this [`VideoSource`]
    /// call session can't be started.
    #[expect(clippy::use_self, reason = "because of `const` only")]
    #[must_use]
    pub const fn required(&self) -> bool {
        match self {
            VideoSource::Device(device) => device.required,
            VideoSource::Display(display) => display.required,
        }
    }

    /// Checks whether the provided [`platform::MediaStreamTrack`] satisfies
    /// this [`VideoSource`].
    pub async fn satisfies<T: AsRef<platform::MediaStreamTrack>>(
        &self,
        track: T,
    ) -> bool {
        match self {
            Self::Display(display) => display.satisfies(&track).await,
            Self::Device(device) => device.satisfies(track).await,
        }
    }
}

impl From<VideoSettings> for VideoSource {
    fn from(settings: VideoSettings) -> Self {
        match settings.source_kind {
            MediaSourceKind::Device => {
                Self::Device(DeviceVideoTrackConstraints {
                    device_id: None,
                    facing_mode: None,
                    width: None,
                    height: None,
                    required: settings.required,
                })
            }
            MediaSourceKind::Display => {
                Self::Display(DisplayVideoTrackConstraints {
                    height: None,
                    width: None,
                    frame_rate: None,
                    required: settings.required,
                    device_id: None,
                })
            }
        }
    }
}

/// Constraints for the [`MediaKind::Audio`] [`local::Track`].
///
/// [`local::Track`]: crate::media::track::local::Track
#[derive(Clone, Debug)]
pub enum AudioSource {
    /// [`local::Track`] should be received from the `getUserMedia` request.
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    Device(AudioTrackConstraints),

    /// [`local::Track`] should be received from the `getDisplayMedia` request.
    ///
    /// [`local::Track`]: crate::media::track::local::Track
    Display(AudioTrackConstraints),
}

impl AudioSource {
    /// Returns an importance of this [`AudioSource`].
    ///
    /// If this [`AudioSource`] is important then without this [`AudioSource`]
    /// call session can't be started.
    #[expect(clippy::use_self, reason = "because of `const` only")]
    #[must_use]
    pub const fn required(&self) -> bool {
        match self {
            AudioSource::Device(device) => device.required,
            AudioSource::Display(display) => display.required,
        }
    }

    /// Checks whether the provided [`platform::MediaStreamTrack`] satisfies
    /// this [`AudioSource`].
    pub async fn satisfies<T: AsRef<platform::MediaStreamTrack>>(
        &self,
        track: T,
    ) -> bool {
        match self {
            Self::Display(display) => display.satisfies(&track).await,
            Self::Device(device) => device.satisfies(track).await,
        }
    }
}

impl From<AudioSettings> for AudioSource {
    fn from(settings: AudioSettings) -> Self {
        match settings.source_kind {
            MediaSourceKind::Device => Self::Device(settings.into()),
            MediaSourceKind::Display => Self::Display(settings.into()),
        }
    }
}

/// Wrapper around [MediaTrackConstraints][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#media-track-constraints
#[derive(Clone, Debug)]
pub enum TrackConstraints {
    /// Audio constraints.
    Audio(AudioSource),

    /// Video constraints.
    Video(VideoSource),
}

impl TrackConstraints {
    /// Checks whether the provided [`platform::MediaStreamTrack`] satisfies
    /// these [`TrackConstraints`].
    pub async fn satisfies<T: AsRef<platform::MediaStreamTrack>>(
        &self,
        track: T,
    ) -> bool {
        match self {
            Self::Audio(audio) => audio.satisfies(&track).await,
            Self::Video(video) => video.satisfies(&track).await,
        }
    }

    /// Returns an importance of these [`TrackConstraints`].
    ///
    /// If these [`TrackConstraints`] are important then without them a session
    /// call can't be started.
    #[expect(clippy::use_self, reason = "because of `const` only")]
    #[must_use]
    pub const fn required(&self) -> bool {
        match self {
            TrackConstraints::Video(video) => video.required(),
            TrackConstraints::Audio(audio) => audio.required(),
        }
    }

    /// Returns these [`TrackConstraints`] media source kind.
    #[expect(clippy::use_self, reason = "because of `const` only")]
    #[must_use]
    pub const fn media_source_kind(&self) -> MediaSourceKind {
        match &self {
            TrackConstraints::Audio(AudioSource::Device(..))
            | TrackConstraints::Video(VideoSource::Device(..)) => {
                MediaSourceKind::Device
            }
            TrackConstraints::Audio(AudioSource::Display(..))
            | TrackConstraints::Video(VideoSource::Display(..)) => {
                MediaSourceKind::Display
            }
        }
    }

    /// Returns [`MediaKind`] of these [`TrackConstraints`].
    #[expect(clippy::use_self, reason = "because of `const` only")]
    #[must_use]
    pub const fn media_kind(&self) -> MediaKind {
        match &self {
            TrackConstraints::Audio(..) => MediaKind::Audio,
            TrackConstraints::Video(..) => MediaKind::Video,
        }
    }
}

impl From<ProtoTrackConstraints> for TrackConstraints {
    fn from(caps: ProtoTrackConstraints) -> Self {
        match caps {
            ProtoTrackConstraints::Audio(audio) => Self::Audio(audio.into()),
            ProtoTrackConstraints::Video(video) => Self::Video(video.into()),
        }
    }
}

/// Constraints applicable to audio tracks.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AudioTrackConstraints {
    /// Identifier of the device generating the content for the media track.
    pub device_id: Option<ConstrainString<String>>,

    /// Importance of this [`AudioTrackConstraints`].
    ///
    /// If `true` then without this [`AudioTrackConstraints`] call session
    /// can't be started.
    pub required: bool,

    /// Automatically manages changes in the volume of its source media to
    /// maintain a steady overall volume level.
    pub auto_gain_control: Option<ConstrainBoolean>,

    /// Indicator whether to enable noise suppression to reduce background
    /// sounds.
    pub noise_suppression: Option<ConstrainBoolean>,

    /// Sets the level of aggressiveness for noise suppression if enabled.
    ///
    /// __NOTE__: Only supported on desktop platforms.
    pub noise_suppression_level: Option<NoiseSuppressionLevel>,

    /// Indicator whether to automatically enable echo cancellation to prevent
    /// feedback.
    pub echo_cancellation: Option<ConstrainBoolean>,

    /// Indicator whether to enable a high-pass filter to eliminate
    /// low-frequency noise.
    ///
    /// __NOTE__: Only supported on desktop platforms.
    pub high_pass_filter: Option<ConstrainBoolean>,
}

impl AudioTrackConstraints {
    /// Creates new [`AudioTrackConstraints`] with none constraints configured.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an exact [deviceId][1] constraint.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
    pub fn device_id(&mut self, device_id: String) {
        self.device_id = Some(ConstrainString::Exact(device_id));
    }

    /// Checks whether the provided [`platform::MediaStreamTrack`] satisfies the
    /// contained constraints.
    pub async fn satisfies<T: AsRef<platform::MediaStreamTrack>>(
        &self,
        track: T,
    ) -> bool {
        let track = track.as_ref();

        if !satisfies_track(track, MediaKind::Audio).await {
            return false;
        }

        if !ConstrainString::satisfies(
            self.device_id.as_ref(),
            track.device_id().as_ref(),
        ) {
            return false;
        }
        if !track.is_audio_processing_available() {
            // We assume that if audio processing is not available for this
            // track, then it's not available at all.
            return true;
        }

        if let Some(ConstrainBoolean::Exact(ns_caps)) = &self.noise_suppression
        {
            if let Ok(ns_enabled) = track.is_noise_suppression_enabled().await {
                if *ns_caps != ns_enabled {
                    return false;
                }
            }
        }
        if let Some(ConstrainBoolean::Exact(aec_caps)) = &self.echo_cancellation
        {
            if let Ok(aec_enabled) = track.is_echo_cancellation_enabled().await
            {
                if *aec_caps != aec_enabled {
                    return false;
                }
            }
        }
        if let Some(ConstrainBoolean::Exact(agc_caps)) = &self.auto_gain_control
        {
            if let Ok(agc_enabled) = track.is_auto_gain_control_enabled().await
            {
                if *agc_caps != agc_enabled {
                    return false;
                }
            }
        }
        if let Some(ConstrainBoolean::Exact(hpf_caps)) = &self.high_pass_filter
        {
            if let Ok(hpf_enabled) = track.is_high_pass_filter_enabled().await {
                if *hpf_caps != hpf_enabled {
                    return false;
                }
            }
        }

        true
    }

    /// Merges these [`AudioTrackConstraints`] with `another` ones, meaning that
    /// if some constraints are not set on these ones, then they will be applied
    /// from `another`.
    pub fn merge(&mut self, another: Self) {
        if self.device_id.is_none() && another.device_id.is_some() {
            self.device_id = another.device_id;
        }
        if !self.required && another.required {
            self.required = another.required;
        }
    }

    /// Returns an importance of these [`AudioTrackConstraints`].
    ///
    /// If these [`AudioTrackConstraints`] are important then without them a
    /// session call can't be started.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

impl From<ProtoAudioConstraints> for AudioTrackConstraints {
    fn from(caps: ProtoAudioConstraints) -> Self {
        Self {
            required: caps.required,
            device_id: None,
            auto_gain_control: None,
            noise_suppression: None,
            noise_suppression_level: None,
            echo_cancellation: None,
            high_pass_filter: None,
        }
    }
}

impl AsRef<str> for FacingMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::User => "user",
            Self::Environment => "environment",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

/// Representation of a [ConstrainULong][1].
///
/// Underlying value must fit in a `[0, 4294967295]` range.
///
/// [1]: https://tinyurl.com/w3-streams#dom-constrainulong
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstrainU32 {
    /// Must be the parameter's value.
    Exact(u32),

    /// Should be used if possible.
    Ideal(u32),

    /// Parameter's value must be in this range.
    Range(u32, u32),
}

impl ConstrainU32 {
    /// Checks whether `this` [`ConstrainU32`] is satisfied with the given
    /// `setting`.
    fn satisfies(this: Option<Self>, setting: Option<u32>) -> bool {
        // It's up to `<T as Constraint>::TRACK_SETTINGS_FIELD_NAME` to
        // guarantee that such casts are safe.
        match this {
            None | Some(Self::Ideal(_)) => true,
            Some(Self::Exact(exact)) => setting.is_some_and(|val| val == exact),
            Some(Self::Range(start, end)) => {
                setting.is_some_and(|val| val >= start && val <= end)
            }
        }
    }
}

/// Representation of the [ConstrainDOMString][1].
///
/// Can set exact (must be the parameter's value) and ideal (should be used if
/// possible) constrain.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstrainString<T> {
    /// Exact value required for this property.
    Exact(T),

    /// Ideal (target) value for this property.
    Ideal(T),
}

/// Representation of a [ConstrainBoolean][1].
///
/// Can set exact (must be the parameter's value) and ideal (should be used if
/// possible) constrain.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constrainboolean
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstrainBoolean {
    /// Exact value required for this property.
    Exact(bool),

    /// Ideal (target) value for this property.
    Ideal(bool),
}

impl<T: AsRef<str>> ConstrainString<T> {
    /// Checks whether `this` [`ConstrainString`] is satisfied with the given
    /// `setting`.
    fn satisfies(this: Option<&Self>, setting: Option<&T>) -> bool {
        match this {
            None | Some(Self::Ideal(..)) => true,
            Some(Self::Exact(constrain)) => {
                setting.is_some_and(|val| val.as_ref() == constrain.as_ref())
            }
        }
    }
}

/// Constraints applicable to video tracks that are sourced from some media
/// device.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DeviceVideoTrackConstraints {
    /// Importance of this [`DeviceVideoTrackConstraints`].
    ///
    /// If `true` then without this [`DeviceVideoTrackConstraints`] call
    /// session can't be started.
    pub required: bool,

    /// Identifier of the device generating the content for the media track.
    pub device_id: Option<ConstrainString<String>>,

    /// Describes the directions that the camera can face, as seen from the
    /// user's perspective.
    pub facing_mode: Option<ConstrainString<FacingMode>>,

    /// Height of the video in pixels.
    pub height: Option<ConstrainU32>,

    /// Width of the video in pixels.
    pub width: Option<ConstrainU32>,
}

/// Constraints applicable to video tracks that are sourced from screen-capture.
impl DeviceVideoTrackConstraints {
    /// Creates new [`DeviceVideoTrackConstraints`] with none constraints
    /// configured.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets exact [deviceId][1] constraint.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
    pub fn device_id(&mut self, device_id: String) {
        self.device_id = Some(ConstrainString::Exact(device_id));
    }

    /// Sets exact [facingMode][1] constraint.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
    pub const fn exact_facing_mode(&mut self, facing_mode: FacingMode) {
        self.facing_mode = Some(ConstrainString::Exact(facing_mode));
    }

    /// Sets ideal [facingMode][1] constraint.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
    pub const fn ideal_facing_mode(&mut self, facing_mode: FacingMode) {
        self.facing_mode = Some(ConstrainString::Ideal(facing_mode));
    }

    /// Sets exact [`height`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    pub const fn exact_height(&mut self, height: u32) {
        self.height = Some(ConstrainU32::Exact(height));
    }

    /// Sets ideal [`height`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    pub const fn ideal_height(&mut self, height: u32) {
        self.height = Some(ConstrainU32::Ideal(height));
    }

    /// Sets range of [`height`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    pub const fn height_in_range(&mut self, min: u32, max: u32) {
        self.height = Some(ConstrainU32::Range(min, max));
    }

    /// Sets exact [`width`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub const fn exact_width(&mut self, width: u32) {
        self.width = Some(ConstrainU32::Exact(width));
    }

    /// Sets ideal [`width`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub const fn ideal_width(&mut self, width: u32) {
        self.width = Some(ConstrainU32::Ideal(width));
    }

    /// Sets range of [`width`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub const fn width_in_range(&mut self, min: u32, max: u32) {
        self.width = Some(ConstrainU32::Range(min, max));
    }

    /// Checks whether the provided [`platform::MediaStreamTrack`] satisfies
    /// contained [`DeviceVideoTrackConstraints`].
    pub async fn satisfies<T: AsRef<platform::MediaStreamTrack>>(
        &self,
        track: T,
    ) -> bool {
        let track = track.as_ref();
        satisfies_track(track, MediaKind::Video).await
            && ConstrainString::satisfies(
                self.device_id.as_ref(),
                track.device_id().as_ref(),
            )
            && ConstrainString::satisfies(
                self.facing_mode.as_ref(),
                track.facing_mode().as_ref(),
            )
            && ConstrainU32::satisfies(self.height, track.height())
            && ConstrainU32::satisfies(self.width, track.width())
            && !track.guess_is_from_display()
    }

    /// Merges these [`DeviceVideoTrackConstraints`] with `another` ones,
    /// meaning that if some constraints are not set on these ones, then they
    /// will be applied from `another`.
    pub fn merge(&mut self, another: Self) {
        if self.device_id.is_none() && another.device_id.is_some() {
            self.device_id = another.device_id;
        }
        if !self.required && another.required {
            self.required = another.required;
        }
        if self.facing_mode.is_none() && another.facing_mode.is_some() {
            self.facing_mode = another.facing_mode;
        }
        if self.height.is_none() && another.height.is_some() {
            self.height = another.height;
        }
        if self.width.is_none() && another.width.is_some() {
            self.width = another.width;
        }
    }

    /// Returns an importance of these [`DeviceVideoTrackConstraints`].
    ///
    /// If these [`DeviceVideoTrackConstraints`] are important then without them
    /// a session call can't be started.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}

/// Constraints applicable to video tracks sourced from a screen capturing.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DisplayVideoTrackConstraints {
    /// Importance of this [`DisplayVideoTrackConstraints`].
    ///
    /// If `true` then without these [`DisplayVideoTrackConstraints`] a session
    /// call can't be started.
    pub required: bool,

    /// Identifier of the device generating the content for the media track.
    pub device_id: Option<ConstrainString<String>>,

    /// [Height][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    pub height: Option<ConstrainU32>,

    /// [Width][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub width: Option<ConstrainU32>,

    /// [Frame rate][1] of the video.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
    pub frame_rate: Option<ConstrainU32>,
}

impl DisplayVideoTrackConstraints {
    /// Creates new [`DisplayVideoTrackConstraints`] with none constraints
    /// configured.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks whether the provided [`platform::MediaStreamTrack`] satisfies
    /// contained [`DisplayVideoTrackConstraints`].
    pub async fn satisfies<T: AsRef<platform::MediaStreamTrack>>(
        &self,
        track: T,
    ) -> bool {
        let track = track.as_ref();
        satisfies_track(track, MediaKind::Video).await
            && ConstrainString::satisfies(
                self.device_id.as_ref(),
                track.device_id().as_ref(),
            )
            && ConstrainU32::satisfies(self.height, track.height())
            && ConstrainU32::satisfies(self.width, track.width())
            && track.guess_is_from_display()
    }

    /// Merges these [`DisplayVideoTrackConstraints`] with `another` ones,
    /// meaning that if some constraints are not set on these, then they will be
    /// applied from `another`.
    pub fn merge(&mut self, another: Self) {
        if self.device_id.is_none() && another.device_id.is_some() {
            self.device_id = another.device_id;
        }
        if !self.required && another.required {
            self.required = another.required;
        }
        if self.height.is_none() && another.height.is_some() {
            self.height = another.height;
        }
        if self.width.is_none() && another.width.is_some() {
            self.width = another.width;
        }
        if self.frame_rate.is_none() && another.frame_rate.is_some() {
            self.frame_rate = another.frame_rate;
        }
    }

    /// Sets an exact [height][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    pub const fn exact_height(&mut self, height: u32) {
        self.height = Some(ConstrainU32::Exact(height));
    }

    /// Sets an ideal [height][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    pub const fn ideal_height(&mut self, height: u32) {
        self.height = Some(ConstrainU32::Ideal(height));
    }

    /// Sets an exact [width][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub const fn exact_width(&mut self, width: u32) {
        self.width = Some(ConstrainU32::Exact(width));
    }

    /// Sets an ideal [width][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub const fn ideal_width(&mut self, width: u32) {
        self.width = Some(ConstrainU32::Ideal(width));
    }

    /// Sets an exact [deviceId][1] constraint.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
    pub fn device_id(&mut self, device_id: String) {
        self.device_id = Some(ConstrainString::Exact(device_id));
    }

    /// Sets an exact [frameRate][1] constraint.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
    pub const fn exact_frame_rate(&mut self, frame_rate: u32) {
        self.frame_rate = Some(ConstrainU32::Exact(frame_rate));
    }

    /// Sets an ideal [frameRate][1] constraint.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
    pub const fn ideal_frame_rate(&mut self, frame_rate: u32) {
        self.frame_rate = Some(ConstrainU32::Ideal(frame_rate));
    }

    /// Returns an importance of this [`DisplayVideoTrackConstraints`].
    ///
    /// If these [`DisplayVideoTrackConstraints`] are important then without
    /// them a session call can't be started.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }
}
