//! Media tracks and streams constraints functionality.

use std::{cell::RefCell, rc::Rc};

use derive_more::Display;
use futures::stream::LocalBoxStream;
use medea_client_api_proto::{
    AudioSettings as ProtoAudioConstraints, EncodingParameters,
    MediaSourceKind, MediaType as ProtoTrackConstraints, MediaType, SvcSetting,
    VideoSettings,
};
use medea_reactive::ObservableCell;

use crate::{
    media::{track::MediaStreamTrackState, MediaKind},
    peer::{
        media_exchange_state, mute_state, LocalStreamUpdateCriteria, MediaState,
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
    #[display(fmt = "user")]
    User = 0,

    /// Facing away from a user (viewing an environment).
    #[display(fmt = "environment")]
    Environment = 1,

    /// Facing to the left of a user.
    #[display(fmt = "left")]
    Left = 2,

    /// Facing to the right of a user.
    #[display(fmt = "right")]
    Right = 3,
}

/// Local media stream for injecting into new created [`PeerConnection`]s.
///
/// [`PeerConnection`]: crate::peer::PeerConnection
#[derive(Clone, Debug, Default)]
pub struct LocalTracksConstraints(Rc<RefCell<MediaStreamSettings>>);

/// Constraints to the media received from remote. Used to disable or enable
/// media receiving.
#[derive(Debug)]
pub struct RecvConstraints {
    /// Is audio receiving enabled.
    is_audio_enabled: ObservableCell<bool>,

    /// Is device video receiving enabled.
    is_video_device_enabled: ObservableCell<bool>,

    /// Is display video receiving enabled.
    is_video_display_enabled: ObservableCell<bool>,
}

impl Clone for RecvConstraints {
    fn clone(&self) -> Self {
        Self {
            is_audio_enabled: ObservableCell::new(self.is_audio_enabled.get()),
            is_video_device_enabled: ObservableCell::new(
                self.is_video_device_enabled.get(),
            ),
            is_video_display_enabled: ObservableCell::new(
                self.is_video_display_enabled.get(),
            ),
        }
    }
}

impl Default for RecvConstraints {
    fn default() -> Self {
        Self {
            is_audio_enabled: ObservableCell::new(true),
            is_video_device_enabled: ObservableCell::new(true),
            is_video_display_enabled: ObservableCell::new(true),
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
            MediaKind::Audio => {
                self.is_audio_enabled.set(enabled);
            }
            MediaKind::Video => source_kind.map_or_else(
                || {
                    self.is_video_device_enabled.set(enabled);
                    self.is_video_display_enabled.set(enabled);
                },
                |sk| match sk {
                    MediaSourceKind::Device => {
                        self.is_video_device_enabled.set(enabled);
                    }
                    MediaSourceKind::Display => {
                        self.is_video_display_enabled.set(enabled);
                    }
                },
            ),
        }
    }

    /// Returns is audio receiving enabled.
    pub fn is_audio_enabled(&self) -> bool {
        self.is_audio_enabled.get()
    }

    /// Returns is device video receiving enabled.
    pub fn is_video_device_enabled(&self) -> bool {
        self.is_video_device_enabled.get()
    }

    /// Returns is display video receiving enabled.
    pub fn is_video_display_enabled(&self) -> bool {
        self.is_video_display_enabled.get()
    }

    /// Returns [`LocalBoxStream`] into which all `is_audio_enabled` updates
    /// will be sent.
    pub fn on_audio_enabled_change(&self) -> LocalBoxStream<'static, bool> {
        self.is_audio_enabled.subscribe()
    }

    /// Returns [`LocalBoxStream`] into which all `is_video_device_enabled`
    /// updates will be sent.
    pub fn on_video_device_enabled_change(
        &self,
    ) -> LocalBoxStream<'static, bool> {
        self.is_video_device_enabled.subscribe()
    }

    /// Returns [`LocalBoxStream`] into which all `is_video_display_enabled`
    /// updates will be sent.
    pub fn on_video_display_enabled_change(
        &self,
    ) -> LocalBoxStream<'static, bool> {
        self.is_video_display_enabled.subscribe()
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
        self.0
            .borrow_mut()
            .set_track_media_state(state, kind, source_kind);
    }

    /// Enables/disables provided [`LocalStreamUpdateCriteria`] based on
    /// provided [`media_exchange_state`].
    pub fn set_media_exchange_state_by_kinds(
        &self,
        state: media_exchange_state::Stable,
        kinds: LocalStreamUpdateCriteria,
    ) {
        self.0
            .borrow_mut()
            .set_media_exchange_state_by_kinds(state, kinds);
    }

    /// Indicates whether provided [`MediaType`] is enabled in the underlying
    /// [`MediaStreamSettings`].
    #[must_use]
    pub fn enabled(&self, kind: MediaType) -> bool {
        self.0.borrow().enabled(kind)
    }

    /// Indicates whether provided [`MediaType`] is muted in the underlying
    /// [`MediaStreamSettings`].
    #[must_use]
    pub fn muted(&self, kind: MediaType) -> bool {
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
        self.0
            .borrow()
            .is_track_enabled_and_constrained(kind, source)
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
        Self {
            constraints: Some(C::default()),
            enabled: true,
            muted: false,
        }
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
    /// [MediaStreamConstraints][1] for the audio media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    audio: AudioMediaTracksSettings,

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
            audio: AudioMediaTracksSettings {
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

    /// Specifies the nature and settings of the audio
    /// [`platform::MediaStreamTrack`].
    pub fn audio(&mut self, constraints: AudioTrackConstraints) {
        self.audio.enabled = true;
        self.audio.constraints = constraints;
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
        if self.audio != another.audio {
            kinds.add(MediaKind::Audio, MediaSourceKind::Device);
        }

        kinds
    }

    /// Returns only audio constraints.
    #[must_use]
    pub const fn get_audio(&self) -> &AudioTrackConstraints {
        &self.audio.constraints
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
                    self.set_audio_muted(muted == mute_state::Stable::Muted);
                }
                MediaState::MediaExchange(media_exchange) => {
                    self.set_audio_publish(
                        media_exchange == media_exchange_state::Stable::Enabled,
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
            self.set_audio_publish(enabled);
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
    fn set_audio_muted(&mut self, muted: bool) {
        self.audio.muted = muted;
    }

    /// Sets the underlying [`VideoTrackConstraints::muted`] basing on the
    /// provided [`MediaSourceKind`] to the given value.
    fn set_video_muted(
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
    pub fn set_audio_publish(&mut self, enabled: bool) {
        self.audio.enabled = enabled;
    }

    /// Sets the underlying [`VideoTrackConstraints`] basing on the provided
    /// [`MediaSourceKind`] to the given value.
    pub fn set_video_publish(
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

    /// Indicates whether audio is enabled in this [`MediaStreamSettings`].
    #[must_use]
    pub const fn is_audio_enabled(&self) -> bool {
        self.audio.enabled
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
    pub fn enabled(&self, kind: MediaType) -> bool {
        match kind {
            MediaType::Video(video) => self.is_track_enabled_and_constrained(
                MediaKind::Video,
                Some(video.source_kind),
            ),
            MediaType::Audio(_) => self.is_track_enabled_and_constrained(
                MediaKind::Audio,
                Some(MediaSourceKind::Device),
            ),
        }
    }

    /// Indicates whether the given [`MediaType`] is muted in this
    /// [`MediaStreamSettings`].
    #[must_use]
    pub fn muted(&self, kind: MediaType) -> bool {
        match kind {
            MediaType::Video(video) => match video.source_kind {
                MediaSourceKind::Device => self.device_video.muted,
                MediaSourceKind::Display => self.display_video.muted,
            },
            MediaType::Audio(_) => self.audio.muted,
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
            (MediaKind::Audio, _) => self.audio.enabled,
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
            (MediaKind::Audio, _) => self.audio.enabled,
        }
    }

    /// Constrains this [`MediaStreamSettings`] with the given `other`
    /// [`MediaStreamSettings`].
    fn constrain(&mut self, other: Self) {
        // `&=` cause we should not enable disabled Room, but we can disable
        // enabled room.
        self.audio.enabled &= other.audio.enabled;
        self.audio.constraints = other.audio.constraints;
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
        let is_device_audio_enabled = constraints.is_audio_enabled();

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
                .audio(constraints.audio.constraints);
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
    #[allow(clippy::use_self)] // because of `const` only
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

    /// Returns configured [`EncodingParameters`].
    #[must_use]
    pub fn encodings(&self) -> Vec<EncodingParameters> {
        match self {
            Self::Device(s) => s.encodings.clone(),
            Self::Display(s) => s.encodings.clone(),
        }
    }

    /// Returns configured [`Vec<SvcSetting>`].
    #[must_use]
    pub fn svc(&self) -> Vec<SvcSetting> {
        match self {
            Self::Device(s) => s.svc.clone(),
            Self::Display(s) => s.svc.clone(),
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
                    encodings: settings.encodings,
                    svc: settings.svc,
                })
            }
            MediaSourceKind::Display => {
                Self::Display(DisplayVideoTrackConstraints {
                    height: None,
                    width: None,
                    frame_rate: None,
                    required: settings.required,
                    device_id: None,
                    encodings: settings.encodings,
                    svc: settings.svc,
                })
            }
        }
    }
}

/// Wrapper around [MediaTrackConstraints][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#media-track-constraints
#[derive(Clone, Debug)]
pub enum TrackConstraints {
    /// Audio constraints.
    Audio(AudioTrackConstraints),

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
    #[allow(clippy::use_self)] // because of `const` only
    #[must_use]
    pub const fn required(&self) -> bool {
        match self {
            TrackConstraints::Video(video) => video.required(),
            TrackConstraints::Audio(audio) => audio.required,
        }
    }

    /// Returns these [`TrackConstraints`] media source kind.
    #[allow(clippy::use_self)] // because of `const` only
    #[must_use]
    pub const fn media_source_kind(&self) -> MediaSourceKind {
        match &self {
            TrackConstraints::Audio(_) => MediaSourceKind::Device,
            TrackConstraints::Video(VideoSource::Device(_)) => {
                MediaSourceKind::Device
            }
            TrackConstraints::Video(VideoSource::Display(_)) => {
                MediaSourceKind::Display
            }
        }
    }

    /// Returns configured [`EncodingParameters`].
    #[must_use]
    pub fn encodings(&self) -> Vec<EncodingParameters> {
        match &self {
            Self::Audio(_) => Vec::new(),
            Self::Video(vs) => vs.encodings(),
        }
    }

    /// Returns configured [`SvcSetting`]s.
    #[must_use]
    pub fn svc(&self) -> Vec<SvcSetting> {
        match &self {
            Self::Audio(_) => Vec::new(),
            Self::Video(vs) => vs.svc(),
        }
    }

    /// Returns [`MediaKind`] of these [`TrackConstraints`].
    #[allow(clippy::use_self)] // because of `const` only
    #[must_use]
    pub const fn media_kind(&self) -> MediaKind {
        match &self {
            TrackConstraints::Audio(_) => MediaKind::Audio,
            TrackConstraints::Video(_) => MediaKind::Video,
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

    /// Checks whether the provided [`platform::MediaStreamTrack`] satisfies
    /// contained constraints.
    pub async fn satisfies<T: AsRef<platform::MediaStreamTrack>>(
        &self,
        track: T,
    ) -> bool {
        let track = track.as_ref();
        satisfies_track(track, MediaKind::Audio).await
            && ConstrainString::satisfies(&self.device_id, &track.device_id())
        // TODO returns Result<bool, Error>
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
            Some(Self::Exact(exact)) => {
                setting.map_or(false, |val| val == exact)
            }
            Some(Self::Range(start, end)) => {
                setting.map_or(false, |val| val >= start && val <= end)
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

impl<T: AsRef<str>> ConstrainString<T> {
    /// Checks whether `this` [`ConstrainString`] is satisfied with the given
    /// `setting`.
    fn satisfies(this: &Option<Self>, setting: &Option<T>) -> bool {
        match this {
            None | Some(Self::Ideal(_)) => true,
            Some(Self::Exact(constrain)) => setting
                .as_ref()
                .map_or(false, |val| val.as_ref() == constrain.as_ref()),
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

    /// [`EncodingParameters`] for the track.
    pub encodings: Vec<EncodingParameters>,

    /// [`SvcSetting`]s in decreasing priority.
    pub svc: Vec<SvcSetting>,
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
    pub fn exact_facing_mode(&mut self, facing_mode: FacingMode) {
        self.facing_mode = Some(ConstrainString::Exact(facing_mode));
    }

    /// Sets ideal [facingMode][1] constraint.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
    pub fn ideal_facing_mode(&mut self, facing_mode: FacingMode) {
        self.facing_mode = Some(ConstrainString::Ideal(facing_mode));
    }

    /// Sets exact [`height`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    pub fn exact_height(&mut self, height: u32) {
        self.height = Some(ConstrainU32::Exact(height));
    }

    /// Sets ideal [`height`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    pub fn ideal_height(&mut self, height: u32) {
        self.height = Some(ConstrainU32::Ideal(height));
    }

    /// Sets range of [`height`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    pub fn height_in_range(&mut self, min: u32, max: u32) {
        self.height = Some(ConstrainU32::Range(min, max));
    }

    /// Sets exact [`width`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub fn exact_width(&mut self, width: u32) {
        self.width = Some(ConstrainU32::Exact(width));
    }

    /// Sets ideal [`width`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub fn ideal_width(&mut self, width: u32) {
        self.width = Some(ConstrainU32::Ideal(width));
    }

    /// Sets range of [`width`][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub fn width_in_range(&mut self, min: u32, max: u32) {
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
            && ConstrainString::satisfies(&self.device_id, &track.device_id())
            && ConstrainString::satisfies(
                &self.facing_mode,
                &track.facing_mode(),
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

    /// [`EncodingParameters`] for the track.
    pub encodings: Vec<EncodingParameters>,

    /// [`SvcSetting`]s in decreasing priority.
    pub svc: Vec<SvcSetting>,
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
            && ConstrainString::satisfies(&self.device_id, &track.device_id())
            && ConstrainU32::satisfies(self.height, track.height())
            && ConstrainU32::satisfies(self.width, track.width())
            && track.guess_is_from_display()
    }

    /// Merges these [`DisplayVideoTrackConstraints`] with `another` ones,
    /// meaning that if some constraints are not set on these ones, then they
    /// will be applied from `another`.
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
    pub fn exact_height(&mut self, height: u32) {
        self.height = Some(ConstrainU32::Exact(height));
    }

    /// Sets an ideal [height][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    pub fn ideal_height(&mut self, height: u32) {
        self.height = Some(ConstrainU32::Ideal(height));
    }

    /// Sets an exact [width][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub fn exact_width(&mut self, width: u32) {
        self.width = Some(ConstrainU32::Exact(width));
    }

    /// Sets an ideal [width][1] constraint.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    pub fn ideal_width(&mut self, width: u32) {
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
    pub fn exact_frame_rate(&mut self, frame_rate: u32) {
        self.frame_rate = Some(ConstrainU32::Exact(frame_rate));
    }

    /// Sets an ideal [frameRate][1] constraint.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
    pub fn ideal_frame_rate(&mut self, frame_rate: u32) {
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
