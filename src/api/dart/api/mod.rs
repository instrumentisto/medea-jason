//! External [`Jason`] API exposing functions that can be called via FFI and
//! designed to be integrated into a [Flutter] plugin.
//!
//! [Flutter]: https://flutter.dev

#![allow(
clippy::as_conversions,
clippy::doc_markdown, // TODO: From generated code in #[frb].
clippy::missing_panics_doc,
clippy::needless_pass_by_value,
clippy::undocumented_unsafe_blocks,
clippy::unwrap_used,
non_snake_case,
)]

use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    ptr,
};

use flutter_rust_bridge::{frb, DartOpaque};
use tracerr::Traced;

use crate::{
    api::{
        dart::api_bridge_generated::RustOpaque, utils::new_dart_opaque, Error,
        Error as DartError, ForeignClass,
    },
    connection,
    media::{
        self,
        constraints::{ConstrainBoolean, ConstrainU32},
        EnumerateDevicesError, EnumerateDisplaysError,
        InvalidOutputAudioDeviceIdError, MediaSourceKind, MicVolumeError,
    },
    platform::{self, utils::dart_future::IntoDartFuture},
    room::{self, ConstraintsUpdateError, RoomJoinError},
};

pub use dart_sys::Dart_Handle;

pub use crate::{
    connection::ConnectionHandle,
    jason::Jason,
    media::{
        track::{local::LocalMediaTrack, remote::Track as RemoteMediaTrack},
        MediaDeviceKind, MediaDirection, MediaKind, MediaManagerHandle,
    },
    room::{RoomCloseReason, RoomHandle},
    rpc::ReconnectHandle,
};

/// Representation of a [MediaDeviceInfo][0] ONLY for input devices.
///
/// [0]: https://w3.org/TR/mediacapture-streams#device-info
#[derive(Debug)]
pub struct ApiMediaDeviceDetails {
    /// [`MediaDeviceKind`] of this [`ApiMediaDeviceDetails`].
    ///
    /// [`MediaDeviceKind`]: MediaDeviceKind
    pub kind: MediaDeviceKind,

    /// Unique identifier of the device represented by this
    /// [`ApiMediaDeviceDetails`].
    pub device_id: String,

    /// Label describing the device represented by this
    /// [`ApiMediaDeviceDetails`] (for example, "External USB Webcam").
    pub label: String,

    /// Group identifier of the device represented by this
    /// [`ApiMediaDeviceDetails`].
    ///
    /// Two devices have the same group identifier if they belong to the same
    /// physical device. For example, the audio input and output devices
    /// representing the speaker and microphone of the same headset have the
    /// same [groupId][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
    pub group_id: Option<String>,

    /// Indicates whether the last attempt to use the provided device failed.
    pub is_failed: bool,
}

/// Representation of a display source.
#[derive(Debug)]
pub struct ApiMediaDisplayDetails {
    /// Unique identifier of the display represented by this
    /// [`ApiMediaDisplayDetails`].
    pub device_id: String,

    /// Title describing the represented display.
    pub title: Option<String>,
}

/// Constraints applicable to audio tracks.
#[derive(Debug)]
#[frb]
pub struct ApiAudioConstraints {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub device_id: Option<String>,

    /// Automatically manages changes in the volume of its source media to
    /// maintain a steady overall volume level.
    #[frb(non_final)]
    pub auto_gain_control: Option<ConstrainBoolean>,
}

impl From<ApiAudioConstraints> for media::AudioTrackConstraints {
    fn from(value: ApiAudioConstraints) -> Self {
        let mut res = Self::new();
        if let Some(id) = value.device_id {
            res.device_id(id);
        }
        if let Some(auto_gain_control) = value.auto_gain_control {
            match auto_gain_control {
                ConstrainBoolean::Exact(e) => res.exact_auto_gain_control(e),
                ConstrainBoolean::Ideal(i) => res.ideal_auto_gain_control(i),
            }
        }
        res
    }
}

/// [facingMode] constraint.
///
/// Can set exact (must be the parameter's value) and ideal (should be used if
/// possible) constrain.
///
/// [facingMode]: https://tinyurl.com/w3-streams#def-constraint-facingMode
#[derive(Copy, Clone, Debug)]
pub enum ApiConstrainFacingMode {
    /// Exact value required for this property.
    Exact(media::FacingMode),

    /// Ideal (target) value for this property.
    Ideal(media::FacingMode),
}

/// Constraints applicable to video tracks that are sourced from some media
/// device.
#[derive(Debug)]
#[frb]
pub struct ApiDeviceVideoTrackConstraints {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub device_id: Option<String>,

    /// Describes the directions that the camera can face, as seen from the
    /// user's perspective.
    #[frb(non_final)]
    pub facing_mode: Option<ApiConstrainFacingMode>,

    /// Height of the video in pixels.
    #[frb(non_final)]
    pub height: Option<ConstrainU32>,

    /// Width of the video in pixels.
    #[frb(non_final)]
    pub width: Option<ConstrainU32>,
}

impl From<ApiDeviceVideoTrackConstraints>
    for media::DeviceVideoTrackConstraints
{
    fn from(value: ApiDeviceVideoTrackConstraints) -> Self {
        let mut res = Self::new();
        if let Some(id) = value.device_id {
            res.device_id(id);
        }
        if let Some(mode) = value.facing_mode {
            match mode {
                ApiConstrainFacingMode::Exact(e) => res.exact_facing_mode(e),
                ApiConstrainFacingMode::Ideal(i) => res.ideal_facing_mode(i),
            }
        }

        if let Some(height) = value.height {
            match height {
                ConstrainU32::Exact(e) => res.exact_height(e),
                ConstrainU32::Ideal(i) => res.ideal_height(i),
                ConstrainU32::Range(min, max) => res.height_in_range(min, max),
            }
        }

        if let Some(width) = value.width {
            match width {
                ConstrainU32::Exact(e) => res.exact_width(e),
                ConstrainU32::Ideal(i) => res.ideal_width(i),
                ConstrainU32::Range(min, max) => res.width_in_range(min, max),
            }
        }
        res
    }
}

/// Constraints applicable to video tracks sourced from a screen capturing.
#[derive(Debug)]
#[frb]
pub struct ApiDisplayVideoTrackConstraints {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub device_id: Option<String>,

    /// [Height][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    #[frb(non_final)]
    pub height: Option<ConstrainU32>,

    /// [Width][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    #[frb(non_final)]
    pub width: Option<ConstrainU32>,

    /// [Frame rate][1] of the video.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
    #[frb(non_final)]
    pub frame_rate: Option<ConstrainU32>,
}

impl From<ApiDisplayVideoTrackConstraints>
    for media::DisplayVideoTrackConstraints
{
    fn from(value: ApiDisplayVideoTrackConstraints) -> Self {
        let mut res = Self::new();
        if let Some(id) = value.device_id {
            res.device_id(id);
        }

        if let Some(height) = value.height {
            match height {
                ConstrainU32::Exact(e) => res.exact_height(e),
                ConstrainU32::Ideal(i) => res.ideal_height(i),
                ConstrainU32::Range(..) => unreachable!(),
            }
        }

        if let Some(width) = value.width {
            match width {
                ConstrainU32::Exact(e) => res.exact_width(e),
                ConstrainU32::Ideal(i) => res.ideal_width(i),
                ConstrainU32::Range(..) => unreachable!(),
            }
        }

        if let Some(frame_rate) = value.frame_rate {
            match frame_rate {
                ConstrainU32::Exact(e) => res.exact_frame_rate(e),
                ConstrainU32::Ideal(i) => res.ideal_frame_rate(i),
                ConstrainU32::Range(..) => unreachable!(),
            }
        }
        res
    }
}

/// [MediaStreamConstraints][1] wrapper.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
#[derive(Debug)]
#[frb]
pub struct ApiMediaStreamSettings {
    /// [MediaStreamConstraints][1] for the audio media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    #[frb(non_final)]
    pub audio: Option<ApiAudioConstraints>,

    /// [MediaStreamConstraints][1] for the device video media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    #[frb(non_final)]
    pub device_video: Option<ApiDeviceVideoTrackConstraints>,

    /// [MediaStreamConstraints][1] for the display video media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    #[frb(non_final)]
    pub display_video: Option<ApiDisplayVideoTrackConstraints>,
}

impl From<ApiMediaStreamSettings> for media::MediaStreamSettings {
    fn from(value: ApiMediaStreamSettings) -> Self {
        let mut res = Self::new();
        if let Some(audio) = value.audio {
            res.audio(audio.into());
        }
        if let Some(device) = value.device_video {
            res.device_video(device.into());
        }
        if let Some(display) = value.display_video {
            res.display_video(display.into());
        }
        res
    }
}

impl<T> ForeignClass for Vec<T> {}

impl ForeignClass for ConnectionHandle {}
impl RefUnwindSafe for ConnectionHandle {}
impl UnwindSafe for ConnectionHandle {}
unsafe impl Send for ConnectionHandle {}
unsafe impl Sync for ConnectionHandle {}

/// Returns the [`ConnectionHandle`] from the [`ForeignClass`] address.
#[frb(sync)]
#[must_use]
pub fn connection_handle_from_ptr(ptr: usize) -> RustOpaque<ConnectionHandle> {
    unsafe {
        RustOpaque::new(ConnectionHandle::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    }
}

/// Sets a callback to be invoked once the provided `connection` is closed.
///
/// # Errors
///
/// If [`ConnectionHandle::on_close()`] errors.
#[frb(sync)]
pub fn connection_handle_on_close(
    connection: RustOpaque<ConnectionHandle>,
    f: DartOpaque,
) -> Result<(), DartOpaque> {
    connection
        .on_close(platform::Function::new(f))
        .map_err(DartError::from)?;

    Ok(())
}

/// Sets a callback to be invoked once a new [`remote::Track`] is added to the
/// provided `connection`.
///
/// # Errors
///
/// If [`ConnectionHandle::on_remote_track_added()`] errors.
///
/// [`remote::Track`]: media::track::remote::Track
#[frb(sync)]
pub fn connection_handle_on_remote_track_added(
    connection: RustOpaque<ConnectionHandle>,
    f: DartOpaque,
) -> Result<(), DartOpaque> {
    connection
        .on_remote_track_added(platform::Function::new(f))
        .map_err(DartError::from)?;

    Ok(())
}

/// Sets a callback to be invoked when a quality score of the provided
/// `connection` is updated by a server.
///
/// # Errors
///
/// If [`ConnectionHandle::on_quality_score_update()`] errors.
#[frb(sync)]
pub fn connection_handle_on_quality_score_update(
    connection: RustOpaque<ConnectionHandle>,
    f: DartOpaque,
) -> Result<(), DartOpaque> {
    connection
        .on_quality_score_update(platform::Function::new(f))
        .map_err(DartError::from)?;

    Ok(())
}

/// Returns remote `Member` ID of the provided `connection`.
///
/// # Errors
///
/// If [`ConnectionHandle::get_remote_member_id()`] errors.
#[frb(sync)]
pub fn connection_handle_get_remote_member_id(
    connection: RustOpaque<ConnectionHandle>,
) -> Result<String, DartOpaque> {
    Ok(connection.get_remote_member_id().map_err(DartError::from)?)
}

/// Enables inbound audio in the provided `connection`.
#[frb(sync)]
#[must_use]
pub fn connection_handle_enable_remote_audio(
    connection: RustOpaque<ConnectionHandle>,
) -> DartOpaque {
    async move {
        connection.enable_remote_audio().await?;

        Ok::<(), Traced<connection::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Disables inbound audio in the provided `connection`.
#[frb(sync)]
#[must_use]
pub fn connection_handle_disable_remote_audio(
    connection: RustOpaque<ConnectionHandle>,
) -> DartOpaque {
    async move {
        connection.disable_remote_audio().await?;

        Ok::<(), Traced<connection::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Enables inbound video in the provided `connection`.
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
#[frb(sync)]
#[must_use]
pub fn connection_handle_enable_remote_video(
    connection: RustOpaque<ConnectionHandle>,
    source_kind: Option<MediaSourceKind>,
) -> DartOpaque {
    let result = async move {
        connection.enable_remote_video(source_kind).await?;

        Ok::<(), Traced<connection::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque();

    result
}

/// Disables inbound video in the provided `connection`.
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
#[frb(sync)]
#[must_use]
pub fn connection_handle_disable_remote_video(
    connection: RustOpaque<ConnectionHandle>,
    source_kind: Option<MediaSourceKind>,
) -> DartOpaque {
    async move {
        connection.disable_remote_video(source_kind).await?;

        Ok::<(), Traced<connection::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

//------------------------------------------------------------------------------

impl RefUnwindSafe for Jason {}
impl UnwindSafe for Jason {}
unsafe impl Send for Jason {}
unsafe impl Sync for Jason {}

/// Sets the provided [`Dart_Handle`] as a callback for the Rust panic hook.
#[frb(sync)]
#[must_use]
pub fn on_panic(cb: DartOpaque) {
    platform::set_panic_callback(platform::Function::new(cb));
}

/// Instantiates a new [`Jason`] interface to interact with this library.
#[frb(sync)]
#[must_use]
pub fn jason_new() -> RustOpaque<Jason> {
    RustOpaque::new(Jason::new(None))
}

/// Creates a new [`Room`] and returns its [`RoomHandle`].
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn jason_init_room(jason: RustOpaque<Jason>) -> RustOpaque<RoomHandle> {
    RustOpaque::new(jason.init_room())
}

/// Returns a [`MediaManagerHandle`].
#[frb(sync)]
#[must_use]
pub fn jason_media_manager(
    jason: RustOpaque<Jason>,
) -> RustOpaque<MediaManagerHandle> {
    RustOpaque::new(jason.media_manager())
}

/// Closes the provided [`RoomHandle`].
#[frb(sync)]
#[must_use]
pub fn jason_close_room(
    jason: RustOpaque<Jason>,
    room_to_delete: RustOpaque<RoomHandle>,
) {
    let room_to_delete = room_to_delete.try_unwrap().unwrap();
    jason.close_room(&room_to_delete);
}

/// Closes the provided [`RoomHandle`].
#[frb(sync)]
#[must_use]
pub fn jason_dispose(jason: RustOpaque<Jason>) {
    let jason = jason.try_unwrap().unwrap();
    jason.dispose();
}

//------------------------------------------------------------------------------

impl ForeignClass for LocalMediaTrack {}
unsafe impl Send for LocalMediaTrack {}
unsafe impl Sync for LocalMediaTrack {}

/// Returns the [`LocalMediaTrack`] from the [`ForeignClass`] address.
#[frb(sync)]
#[must_use]
pub fn local_media_track_from_ptr(ptr: usize) -> RustOpaque<LocalMediaTrack> {
    unsafe {
        RustOpaque::new(LocalMediaTrack::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    }
}

/// Returns the [`Vec<RustOpaque<LocalMediaTrack>>`] from the [`ForeignClass`]
/// address.
#[frb(sync)]
#[must_use]
pub fn vec_local_tracks_from_ptr(
    ptr: usize,
) -> Vec<RustOpaque<LocalMediaTrack>> {
    unsafe {
        Vec::<LocalMediaTrack>::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
            .into_iter()
            .map(RustOpaque::new)
            .collect()
    }
}

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of the
/// provided [`LocalMediaTrack`].
///
/// [`MediaStreamTrack`]: platform::MediaStreamTrack
#[frb(sync)]
#[must_use]
pub fn local_media_track_get_track(
    track: RustOpaque<LocalMediaTrack>,
) -> DartOpaque {
    unsafe { new_dart_opaque(track.get_track().handle()) }
}

/// Returns a [`MediaKind::Audio`] if the provided [`LocalMediaTrack`]
/// represents an audio track, or a [`MediaKind::Video`] if it represents a
/// video track.
#[frb(sync)]
#[must_use]
pub fn local_media_track_kind(track: RustOpaque<LocalMediaTrack>) -> MediaKind {
    track.kind()
}

/// Sets callback to invoke when this [`LocalMediaTrack`] is ended.
#[frb(sync)]
#[must_use]
pub fn local_media_track_on_ended(
    track: RustOpaque<LocalMediaTrack>,
    f: DartOpaque,
) {
    track.on_ended(platform::Function::new(f));
}

/// Returns a [`media::MediaStreamTrackState::Live`] if this [`LocalMediaTrack`]
/// is active, or a [`media::MediaStreamTrackState::Ended`] if it has ended.
#[frb(sync)]
#[must_use]
pub fn local_media_track_state(
    track: RustOpaque<LocalMediaTrack>,
) -> DartOpaque {
    async move { Ok::<_, Error>(track.state().await as i64) }
        .into_dart_future()
        .into_dart_opaque()
}

/// Indicates whether an `OnAudioLevelChangedCallback` is supported for this
/// [`LocalMediaTrack`].
#[frb(sync)]
#[must_use]
pub fn is_on_audio_level_available(track: RustOpaque<LocalMediaTrack>) -> bool {
    track.is_on_audio_level_available()
}

/// Sets the provided `OnAudioLevelChangedCallback` for this
/// [`LocalMediaTrack`].
///
/// It's called for live [`LocalMediaTrack`]s when their audio level changes.
#[frb(sync)]
#[must_use]
pub fn on_audio_level_changed(
    track: RustOpaque<LocalMediaTrack>,
    f: DartOpaque,
) {
    track.on_audio_level_changed(platform::Function::new(f));
}

/// Returns a [`MediaSourceKind::Device`] if the provided [`LocalMediaTrack`] is
/// sourced from some device (webcam/microphone), or a
/// [`MediaSourceKind::Display`] if it's captured via
/// [MediaDevices.getDisplayMedia()][1].
///
/// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
#[frb(sync)]
#[must_use]
pub fn local_media_track_media_source_kind(
    track: RustOpaque<LocalMediaTrack>,
) -> MediaSourceKind {
    track.media_source_kind()
}

/// Frees the data behind the provided opaque local track.
#[frb(sync)]
#[must_use]
pub fn local_media_track_free(
    track: RustOpaque<LocalMediaTrack>,
) -> DartOpaque {
    let track = track.try_unwrap().unwrap();
    async move {
        track.maybe_stop().await;
        Ok::<_, Error>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

//------------------------------------------------------------------------------

/// Returns the [`Vec<ApiMediaDeviceDetails>`] from the [`ForeignClass`]
/// address.
#[frb(sync)]
#[must_use]
pub fn vec_media_device_details_from_ptr(
    ptr: usize,
) -> Vec<ApiMediaDeviceDetails> {
    unsafe {
        Vec::<ApiMediaDeviceDetails>::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        )
    }
}

//------------------------------------------------------------------------------

/// Returns the [`Vec<RustOpaque<ApiMediaDisplayDetails>>`] from the
/// [`ForeignClass`] address.
#[frb(sync)]
#[must_use]
pub fn vec_media_display_details_from_ptr(
    ptr: usize,
) -> Vec<ApiMediaDisplayDetails> {
    unsafe {
        Vec::<ApiMediaDisplayDetails>::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        )
    }
}

//------------------------------------------------------------------------------

impl RefUnwindSafe for MediaManagerHandle {}
impl UnwindSafe for MediaManagerHandle {}
unsafe impl Send for MediaManagerHandle {}
unsafe impl Sync for MediaManagerHandle {}

/// Returns [`LocalMediaTrack`]s objects, built from the provided
/// [`ApiMediaStreamSettings`].
#[frb(sync)]
#[must_use]
pub fn media_manager_handle_init_local_tracks(
    manager: RustOpaque<MediaManagerHandle>,
    caps: ApiMediaStreamSettings,
) -> DartOpaque {
    let manager = MediaManagerHandle::clone(&manager);

    async move { manager.init_local_tracks(caps.into()).await }
        .into_dart_future()
        .into_dart_opaque()
}

/// Returns a list of [`ApiMediaDeviceDetails`] objects representing available
/// media input and devices, such as microphones, cameras, and so forth.
#[frb(sync)]
#[must_use]
pub fn media_manager_handle_enumerate_devices(
    manager: RustOpaque<MediaManagerHandle>,
) -> DartOpaque {
    let manager = MediaManagerHandle::clone(&manager);
    let result = async move {
        Ok::<Vec<_>, Traced<EnumerateDevicesError>>(
            manager
                .enumerate_devices()
                .await?
                .into_iter()
                .map(|v| ApiMediaDeviceDetails {
                    kind: v.kind(),
                    device_id: v.device_id(),
                    label: v.label(),
                    group_id: v.group_id(),
                    is_failed: v.is_failed(),
                })
                .collect(),
        )
    }
    .into_dart_future()
    .into_dart_opaque();

    result
}

/// Returns a list of [`ApiMediaDisplayDetails`] objects representing available
/// sources that can be used for screen capturing.
#[frb(sync)]
#[must_use]
pub fn media_manager_handle_enumerate_displays(
    manager: RustOpaque<MediaManagerHandle>,
) -> DartOpaque {
    let manager = MediaManagerHandle::clone(&manager);
    async move {
        Ok::<Vec<_>, Traced<EnumerateDisplaysError>>(
            manager
                .enumerate_displays()
                .await?
                .into_iter()
                .map(|v| ApiMediaDisplayDetails {
                    device_id: v.device_id(),
                    title: v.title(),
                })
                .collect(),
        )
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Switches the current output audio device to the device with the provided
/// `device_id`.
#[frb(sync)]
#[must_use]
pub fn media_manager_handle_set_output_audio_id(
    manager: RustOpaque<MediaManagerHandle>,
    device_id: String,
) -> DartOpaque {
    let manager = MediaManagerHandle::clone(&manager);
    async move {
        manager
            .set_output_audio_id(device_id)
            .await
            .map_err(tracerr::map_from_and_wrap!())?;
        Ok::<_, Traced<InvalidOutputAudioDeviceIdError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Sets the microphone volume level in percents.
#[frb(sync)]
#[must_use]
pub fn media_manager_handle_set_microphone_volume(
    manager: RustOpaque<MediaManagerHandle>,
    level: i64,
) -> DartOpaque {
    let manager = MediaManagerHandle::clone(&manager);
    async move {
        manager
            .set_microphone_volume(level)
            .await
            .map_err(tracerr::map_from_and_wrap!())?;
        Ok::<_, Traced<MicVolumeError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Indicates whether it's possible to access microphone volume settings.
#[frb(sync)]
#[must_use]
pub fn media_manager_handle_microphone_volume_is_available(
    manager: RustOpaque<MediaManagerHandle>,
) -> DartOpaque {
    let manager = MediaManagerHandle::clone(&manager);
    async move { manager.microphone_volume_is_available().await }
        .into_dart_future()
        .into_dart_opaque()
}

/// Returns the current microphone volume level in percents.
#[frb(sync)]
#[must_use]
pub fn media_manager_handle_microphone_volume(
    manager: RustOpaque<MediaManagerHandle>,
) -> DartOpaque {
    let manager = MediaManagerHandle::clone(&manager);

    async move { manager.microphone_volume().await }
        .into_dart_future()
        .into_dart_opaque()
}

/// Subscribes onto the [`MediaManagerHandle`]'s `devicechange` event.
/// Sets an ideal [frameRate][1] constraint.
///
/// # Errors
///
/// If [`MediaManagerHandle::on_device_change()`] errors.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
#[frb(sync)]
pub fn media_manager_handle_on_device_change(
    manager: RustOpaque<MediaManagerHandle>,
    cb: DartOpaque,
) -> Result<(), DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    manager
        .on_device_change(platform::Function::new(cb))
        .map_err(DartError::from)?;

    Ok(())
}

//------------------------------------------------------------------------------

impl ForeignClass for ReconnectHandle {}
impl RefUnwindSafe for ReconnectHandle {}
impl UnwindSafe for ReconnectHandle {}
unsafe impl Send for ReconnectHandle {}
unsafe impl Sync for ReconnectHandle {}

/// Returns the [`ReconnectHandle`] from the [`ForeignClass`] address.
#[frb(sync)]
#[must_use]
pub fn reconnect_handle_from_ptr(ptr: usize) -> RustOpaque<ReconnectHandle> {
    unsafe {
        RustOpaque::new(ReconnectHandle::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    }
}

/// Tries to reconnect a [`Room`] after the provided delay in milliseconds.
///
/// If the [`Room`] is already reconnecting then new reconnection attempt won't
/// be performed. Instead, it will wait for the first reconnection attempt
/// result and use it here.
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn reconnect_handle_reconnect_with_delay(
    reconnect_handle: RustOpaque<ReconnectHandle>,
    delay_ms: u32,
) -> DartOpaque {
    let reconnect_handle = ReconnectHandle::clone(&reconnect_handle);
    async move {
        reconnect_handle.reconnect_with_delay(delay_ms).await?;
        Ok::<_, Error>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Tries to reconnect a [`Room`] in a loop with a growing backoff delay.
///
/// The first attempt will be performed immediately, and the second attempt will
/// be performed after `starting_delay_ms`.
///
/// Delay between reconnection attempts won't be greater than
/// `max_delay_ms`.
///
/// After each reconnection attempt, delay between reconnections will be
/// multiplied by the given `multiplier` until it reaches `max_delay_ms`.
///
/// If `multiplier` is a negative number then it will be considered as `0.0`.
/// This might cause a busy loop, so it's not recommended.
///
/// Max elapsed time can be limited with an optional `max_elapsed_time_ms`
/// argument.
///
/// If the [`Room`] is already reconnecting then new reconnection attempt won't
/// be performed. Instead, it will wait for the first reconnection attempt
/// result and use it here.
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn reconnect_handle_reconnect_with_backoff(
    reconnect_handle: RustOpaque<ReconnectHandle>,
    starting_delay: u32,
    multiplier: f64,
    max_delay: u32,
    max_elapsed_time_ms: Option<u32>,
) -> DartOpaque {
    let reconnect_handle = ReconnectHandle::clone(&reconnect_handle);
    async move {
        reconnect_handle
            .reconnect_with_backoff(
                starting_delay,
                multiplier,
                max_delay,
                max_elapsed_time_ms,
            )
            .await?;
        Ok::<_, DartError>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

//------------------------------------------------------------------------------

impl ForeignClass for RemoteMediaTrack {}
impl RefUnwindSafe for RemoteMediaTrack {}
impl UnwindSafe for RemoteMediaTrack {}
unsafe impl Send for RemoteMediaTrack {}
unsafe impl Sync for RemoteMediaTrack {}

/// Returns the [`RemoteMediaTrack`] from the [`ForeignClass`] address.
#[frb(sync)]
#[must_use]
pub fn remote_media_track_from_ptr(ptr: usize) -> RustOpaque<RemoteMediaTrack> {
    unsafe {
        RustOpaque::new(RemoteMediaTrack::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    }
}

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
/// [`RemoteMediaTrack`].
///
/// [`MediaStreamTrack`]: platform::MediaStreamTrack
#[frb(sync)]
#[must_use]
pub fn remote_media_track_get_track(
    track: RustOpaque<RemoteMediaTrack>,
) -> DartOpaque {
    unsafe { new_dart_opaque(track.get_track().handle()) }
}

/// Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
#[frb(sync)]
#[must_use]
pub fn remote_media_track_on_muted(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) {
    track.on_muted(platform::Function::new(f));
}

/// Sets callback to invoke when this [`RemoteMediaTrack`] is unmuted.
#[frb(sync)]
#[must_use]
pub fn remote_media_track_on_unmuted(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) {
    track.on_unmuted(platform::Function::new(f));
}

/// Sets callback to invoke when this [`RemoteMediaTrack`] is stopped.
#[frb(sync)]
#[must_use]
pub fn remote_media_track_on_stopped(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) {
    track.on_stopped(platform::Function::new(f));
}

/// Sets callback to invoke whenever this [`RemoteMediaTrack`]'s general
/// [`MediaDirection`] is changed.
#[frb(sync)]
#[must_use]
pub fn remote_media_track_on_media_direction_changed(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) {
    track.on_media_direction_changed(
        platform::Function::<MediaDirection>::new(f),
    );
}

/// Indicate whether this [`RemoteMediaTrack`] is muted.
#[frb(sync)]
#[must_use]
pub fn remote_media_track_muted(track: RustOpaque<RemoteMediaTrack>) -> bool {
    track.muted()
}

/// Returns this [`RemoteMediaTrack`]'s kind (audio/video).
#[frb(sync)]
#[must_use]
pub fn remote_media_track_kind(
    track: RustOpaque<RemoteMediaTrack>,
) -> MediaKind {
    track.kind()
}

/// Returns this [`RemoteMediaTrack`]'s media source kind.
#[frb(sync)]
#[must_use]
pub fn remote_media_track_media_source_kind(
    track: RustOpaque<RemoteMediaTrack>,
) -> MediaSourceKind {
    track.media_source_kind()
}

/// Returns the current general [`MediaDirection`] of this [`RemoteMediaTrack`].
#[frb(sync)]
#[must_use]
pub fn remote_media_track_media_direction(
    track: RustOpaque<RemoteMediaTrack>,
) -> MediaDirection {
    track.media_direction()
}

//------------------------------------------------------------------------------

impl ForeignClass for RoomCloseReason {}

/// Returns the [`RoomCloseReason`] from the [`ForeignClass`] address.
#[frb(sync)]
#[must_use]
pub fn room_close_reason_from_ptr(ptr: usize) -> RoomCloseReason {
    unsafe { RoomCloseReason::from_ptr(ptr::NonNull::new(ptr as _).unwrap()) }
}

//------------------------------------------------------------------------------

impl RefUnwindSafe for RoomHandle {}
impl UnwindSafe for RoomHandle {}
unsafe impl Send for RoomHandle {}
unsafe impl Sync for RoomHandle {}

/// Connects to a media server and joins the [`Room`] with the provided
/// authorization `token`.
///
/// Authorization token has a fixed format:
/// `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
/// (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_join(
    room_handle: RustOpaque<RoomHandle>,
    token: String,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);
    async move {
        room_handle.join(token).await?;
        Ok::<_, Traced<RoomJoinError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Updates this [`Room`]'s [`ApiMediaStreamSettings`]. This affects all the
/// [`PeerConnection`]s in this [`Room`]. If [`ApiMediaStreamSettings`] are
/// configured for some [`Room`], then this [`Room`] can only send media tracks
/// that correspond to these settings. [`ApiMediaStreamSettings`] update will
/// change media tracks in all sending peers, so that might cause a new
/// [getUserMedia()][1] request to happen.
///
/// Media obtaining/injection errors are additionally fired to
/// `on_failed_local_media` callback.
///
/// If `stop_first` set to `true` then affected local `Tracks` will be
/// dropped before new [`ApiMediaStreamSettings`] are applied. This is usually
/// required when changing video source device due to hardware limitations,
/// e.g. having an active track sourced from device `A` may hinder
/// [getUserMedia()][1] requests to device `B`.
///
/// `rollback_on_fail` option configures [`ApiMediaStreamSettings`] update
/// request to automatically rollback to previous settings if new settings
/// cannot be applied.
///
/// If recovering from fail state isn't possible then affected media types will
/// be disabled.
///
/// [`Room`]: room::Room
/// [`PeerConnection`]: crate::peer::PeerConnection
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
#[frb(sync)]
#[must_use]
pub fn room_handle_set_local_media_settings(
    room_handle: RustOpaque<RoomHandle>,
    settings: ApiMediaStreamSettings,
    stop_first: bool,
    rollback_on_fail: bool,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle
            .set_local_media_settings(
                settings.into(),
                stop_first,
                rollback_on_fail,
            )
            .await?;
        Ok::<_, ConstraintsUpdateError>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Mutes outbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_mute_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.mute_audio().await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Unmutes outbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_unmute_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.unmute_audio().await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Enables outbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_enable_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.enable_audio().await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Disables outbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_disable_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.disable_audio().await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Mutes outbound video in the provided [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// # Errors
///
/// If `source_kind` is not a [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_mute_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.mute_video(source_kind).await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Unmutes outbound video in the provided [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// # Errors
///
/// If `source_kind` is not a [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_unmute_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.unmute_video(source_kind).await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Enables outbound video in the provided [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_enable_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.enable_video(source_kind).await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Disables outbound video in the provided [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_disable_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.disable_video(source_kind).await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Enables inbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_enable_remote_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.enable_remote_audio().await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Disables inbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_disable_remote_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.disable_remote_audio().await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Enables inbound video in the provided [`Room`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_enable_remote_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.enable_remote_video(source_kind).await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Disables inbound video in the provided [`Room`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
#[frb(sync)]
#[must_use]
pub fn room_handle_disable_remote_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> DartOpaque {
    let room_handle = RoomHandle::clone(&room_handle);

    async move {
        room_handle.disable_remote_video(source_kind).await?;

        Ok::<_, Traced<room::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque()
}

/// Sets a callback to be invoked once a new [`Connection`] with some remote
/// `Peer` is established.
///
/// # Errors
///
/// If [`RoomHandle::on_new_connection()`] errors.
///
/// [`Connection`]: connection::Connection
#[frb(sync)]
pub fn room_handle_on_new_connection(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> Result<(), DartOpaque> {
    Ok(room_handle
        .on_new_connection(platform::Function::new(cb))
        .map_err(DartError::from)?)
}

/// Sets a callback to be invoked once the provided [`Room`] is closed,
/// providing a [`RoomCloseReason`].
///
/// # Errors
///
/// If [`RoomHandle::on_close()`] errors.
///
/// [`Room`]: room::Room
#[frb(sync)]
pub fn room_handle_on_close(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> Result<(), DartOpaque> {
    room_handle
        .on_close(platform::Function::new(cb))
        .map_err(DartError::from)?;

    Ok(())
}

/// Sets a callback to be invoked when a new [`LocalMediaTrack`] is added to
/// the provided [`Room`].
///
/// This might happen in such cases:
/// 1. Media server initiates a media request.
/// 2. `enable_audio`/`enable_video` is called.
/// 3. [`MediaStreamSettings`] updated via `set_local_media_settings`.
///
/// # Errors
///
/// If [`RoomHandle::on_local_track()`] errors.
///
/// [`MediaStreamSettings`]: media::MediaStreamSettings
/// [`Room`]: room::Room
#[frb(sync)]
pub fn room_handle_on_local_track(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> Result<(), DartOpaque> {
    room_handle
        .on_local_track(platform::Function::new(cb))
        .map_err(DartError::from)?;

    Ok(())
}

/// Sets a callback to be invoked once a connection with server is lost.
///
/// # Errors
///
/// If [`RoomHandle::on_connection_loss()`] errors.
#[frb(sync)]
pub fn room_handle_on_connection_loss(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> Result<(), DartOpaque> {
    room_handle
        .on_connection_loss(platform::Function::new(cb))
        .map_err(DartError::from)?;

    Ok(())
}

/// Sets a callback to be invoked on local media acquisition failures.
///
/// # Errors
///
/// If [`RoomHandle::on_failed_local_media()`] errors.
#[frb(sync)]
pub fn room_handle_on_failed_local_media(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> Result<(), DartOpaque> {
    room_handle
        .on_failed_local_media(platform::Function::new(cb))
        .map_err(DartError::from)?;

    Ok(())
}

//------------------------------------------------------------------------------

/// Logs Dart exception.
#[frb(sync)]
#[must_use]
pub fn log_dart_exception(message: String, stack_trace: String) {
    log::error!("{message}\n{stack_trace}");
}
