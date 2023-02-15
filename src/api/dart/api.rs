//! External [`Jason`] API exposing functions that can be called via FFI and
//! designed to be integrated into a [Flutter] plugin.
//!
//! [Flutter]: https://flutter.dev

use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    ptr,
};

pub use dart_sys::Dart_Handle;
use flutter_rust_bridge::{frb, DartOpaque, RustOpaque, SyncReturn};
use tracerr::Traced;

pub use crate::{
    connection::ConnectionHandle,
    jason::Jason,
    media::{
        track::{local::LocalMediaTrack, remote::Track as RemoteMediaTrack},
        MediaManagerHandle,
    },
    room::{RoomCloseReason, RoomHandle},
    rpc::ReconnectHandle,
};

use crate::{
    api::{utils::new_dart_opaque, Error, Error as DartError, ForeignClass},
    connection,
    media::{
        self, constraints::ConstrainU32, EnumerateDevicesError,
        EnumerateDisplaysError, InvalidOutputAudioDeviceIdError,
        MediaDirection, MediaKind, MediaSourceKind, MicVolumeError,
    },
    platform::{self, utils::dart_future::IntoDartFuture},
    room::{self, ConstraintsUpdateError, RoomJoinError},
};

/// Representation of a [`ApiMediaDeviceInfo`][0] ONLY for input devices.
///
/// [0]: https://w3.org/TR/mediacapture-streams#device-info
#[derive(Debug)]
pub struct ApiMediaDeviceInfo {
    /// [`MediaDeviceKind`] of this [`ApiMediaDeviceInfo`].
    pub(crate) kind: media::MediaDeviceKind,

    /// Unique identifier of the device represented by this
    /// [`ApiMediaDeviceInfo`].
    pub(crate) device_id: String,

    /// Label describing the device represented by this
    /// [`ApiMediaDeviceInfo`] (for example, "External USB Webcam").
    pub(crate) label: String,

    /// Group identifier of the device represented by this
    /// [`ApiMediaDeviceInfo`]
    ///
    /// Two devices have the same group identifier if they belong to the same
    /// physical device. For example, the audio input and output devices
    /// representing the speaker and microphone of the same headset have the
    /// same [groupId][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
    pub(crate) group_id: Option<String>,

    /// Indicates whether the last attempt to use the provided device
    /// failed.
    pub(crate) is_failed: bool,
}

/// Representation of a display source.
#[derive(Debug)]
pub struct ApiMediaDisplayInfo {
    /// Unique identifier of the display represented by this
    /// [`ApiMediaDisplayInfo`].
    pub(crate) device_id: String,

    /// Title describing the represented display.
    pub(crate) title: Option<String>,
}

#[derive(Debug)]
#[frb]
pub struct ApiAudioConstraints {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub(crate) device_id: Option<String>,
}

impl From<ApiAudioConstraints> for media::AudioTrackConstraints {
    fn from(value: ApiAudioConstraints) -> Self {
        let mut res = Self::new();
        if let Some(id) = value.device_id {
            res.device_id(id);
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
#[frb]
#[derive(Debug)]
pub struct ApiDeviceVideoTrackConstraints {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub(crate) device_id: Option<String>,

    /// Describes the directions that the camera can face, as seen from the
    /// user's perspective.
    #[frb(non_final)]
    pub(crate) facing_mode: Option<ApiConstrainFacingMode>,

    /// Height of the video in pixels.
    #[frb(non_final)]
    pub(crate) height: Option<ConstrainU32>,

    /// Width of the video in pixels.
    #[frb(non_final)]
    pub(crate) width: Option<ConstrainU32>,
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
    pub(crate) device_id: Option<String>,

    /// [Height][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    #[frb(non_final)]
    pub(crate) height: Option<ConstrainU32>,

    /// [Width][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    #[frb(non_final)]
    pub(crate) width: Option<ConstrainU32>,

    /// [Frame rate][1] of the video.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
    #[frb(non_final)]
    pub(crate) frame_rate: Option<ConstrainU32>,
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
    pub(crate) audio: Option<ApiAudioConstraints>,

    /// [MediaStreamConstraints][1] for the device video media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    #[frb(non_final)]
    pub(crate) device_video: Option<ApiDeviceVideoTrackConstraints>,

    /// [MediaStreamConstraints][1] for the display video media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    #[frb(non_final)]
    pub(crate) display_video: Option<ApiDisplayVideoTrackConstraints>,
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

/// Returns the [`ConnectionHandle`] from the address [`ForeignClass`].
#[must_use]
pub fn connection_handle_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<ConnectionHandle>> {
    SyncReturn(unsafe {
        RustOpaque::new(ConnectionHandle::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    })
}

/// Sets callback, invoked when this `Connection` will close.
///
/// # Errors
///
/// If [`ConnectionHandle::on_close`] returns error.

pub fn connection_handle_on_close(
    connection: RustOpaque<ConnectionHandle>,
    f: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    let f = unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw().cast())
    };

    connection
        .on_close(f)
        .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?;

    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a new [`remote::Track`] is added to this
/// [`Connection`].
///
/// [`remote::Track`]: crate::media::track::remote::Track
/// [`Connection`]: crate::connection::Connection
///
/// # Errors
///
/// If [`ConnectionHandle::on_remote_track_added`] returns error.

pub fn connection_handle_on_remote_track_added(
    connection: RustOpaque<ConnectionHandle>,
    f: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    let f = unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw().cast())
    };

    connection
        .on_remote_track_added(f)
        .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?;

    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a connection quality score is updated by
/// a server.
///
/// # Errors
///
/// If [`ConnectionHandle::on_quality_score_update`] returns error.
pub fn connection_handle_on_quality_score_update(
    connection: RustOpaque<ConnectionHandle>,
    f: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    let f = unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw().cast())
    };

    connection
        .on_quality_score_update(f)
        .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?;

    Ok(SyncReturn(()))
}

/// Returns remote `Member` ID.
///
/// # Errors
///
/// If [`ConnectionHandle::get_remote_member_id`] returns error.
pub fn connection_handle_get_remote_member_id(
    connection: RustOpaque<ConnectionHandle>,
) -> anyhow::Result<SyncReturn<String>> {
    Ok(SyncReturn(connection.get_remote_member_id().map_err(
        |err| anyhow::anyhow!("{:?}", DartError::from(err)),
    )?))
}

/// Enables inbound audio in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[must_use]
pub fn connection_handle_enable_remote_audio(
    connection: RustOpaque<ConnectionHandle>,
) -> SyncReturn<DartOpaque> {
    SyncReturn(
        async move {
            connection.enable_remote_audio().await?;

            Ok::<(), Traced<connection::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    )
}

/// Disables inbound audio in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[must_use]
pub fn connection_handle_disable_remote_audio(
    connection: RustOpaque<ConnectionHandle>,
) -> SyncReturn<DartOpaque> {
    SyncReturn(
        async move {
            connection.disable_remote_audio().await?;

            Ok::<(), Traced<connection::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    )
}

/// Enables inbound video in this [`ConnectionHandle`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[must_use]
pub fn connection_handle_enable_remote_video(
    connection: RustOpaque<ConnectionHandle>,
    source_kind: Option<MediaSourceKind>,
) -> SyncReturn<DartOpaque> {
    let result = async move {
        connection.enable_remote_video(source_kind).await?;

        Ok::<(), Traced<connection::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque();

    SyncReturn(result)
}

/// Disables inbound video in this [`ConnectionHandle`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[must_use]
pub fn connection_handle_disable_remote_video(
    connection: RustOpaque<ConnectionHandle>,
    source_kind: Option<MediaSourceKind>,
) -> SyncReturn<DartOpaque> {
    let result = async move {
        connection.disable_remote_video(source_kind).await?;

        Ok::<(), Traced<connection::ChangeMediaStateError>>(())
    }
    .into_dart_future()
    .into_dart_opaque();

    SyncReturn(result)
}

// -------------------------------------------------------------------

impl RefUnwindSafe for Jason {}
impl UnwindSafe for Jason {}

/// Sets the provided [`Dart_Handle`] as a callback for the Rust panic hook.
#[must_use]
pub fn on_panic(cb: DartOpaque) -> SyncReturn<()> {
    platform::set_panic_callback(unsafe {
        platform::Function::new(cb.try_unwrap().unwrap().into_raw().cast())
    });

    SyncReturn(())
}

/// Instantiates a new [`Jason`] interface to interact with this library.
#[must_use]
pub fn jason_new() -> SyncReturn<RustOpaque<Jason>> {
    SyncReturn(RustOpaque::new(Jason::new()))
}

/// Creates a new [`Room`] and returns its [`RoomHandle`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn jason_init_room(
    jason: RustOpaque<Jason>,
) -> SyncReturn<RustOpaque<RoomHandle>> {
    SyncReturn(RustOpaque::new(jason.init_room()))
}

/// Returns a [`MediaManagerHandle`].
#[must_use]
pub fn jason_media_manager(
    jason: RustOpaque<Jason>,
) -> SyncReturn<RustOpaque<MediaManagerHandle>> {
    SyncReturn(RustOpaque::new(jason.media_manager()))
}

/// Closes the provided [`RoomHandle`].
#[must_use]
pub fn jason_close_room(
    jason: RustOpaque<Jason>,
    room_to_delete: RustOpaque<RoomHandle>,
) -> SyncReturn<()> {
    let room_to_delete = room_to_delete.try_unwrap().unwrap();
    jason.close_room(room_to_delete);

    SyncReturn(())
}

/// Closes the provided [`RoomHandle`].
#[must_use]
pub fn jason_dispose(jason: RustOpaque<Jason>) -> SyncReturn<()> {
    let jason = jason.try_unwrap().unwrap();
    jason.dispose();

    SyncReturn(())
}

// -------------------------------------------------------------------

impl ForeignClass for LocalMediaTrack {}

/// Returns the [`LocalMediaTrack`] from the address [`ForeignClass`].
#[must_use]
pub fn local_media_track_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<LocalMediaTrack>> {
    SyncReturn(unsafe {
        RustOpaque::new(LocalMediaTrack::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    })
}

/// Returns the [`Vec<RustOpaque<LocalMediaTrack>>`] from the address
/// [`ForeignClass`].
#[must_use]
pub fn vec_local_tracks_from_ptr(
    ptr: usize,
) -> SyncReturn<Vec<RustOpaque<LocalMediaTrack>>> {
    SyncReturn(unsafe {
        Vec::<LocalMediaTrack>::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
            .into_iter()
            .map(RustOpaque::new)
            .collect()
    })
}

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
/// [`LocalMediaTrack`].
///
/// [`MediaStreamTrack`]: crate::platform::MediaStreamTrack
#[must_use]
pub fn local_media_track_get_track(
    track: RustOpaque<LocalMediaTrack>,
) -> SyncReturn<DartOpaque> {
    SyncReturn(unsafe { new_dart_opaque(track.get_track().handle()) })
}

/// Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
/// audio track, or a [`MediaKind::Video`] if it represents a video track.
///
/// [`MediaKind::Audio`]: crate::media::MediaKind::Audio
/// [`MediaKind::Video`]: crate::media::MediaKind::Video
#[must_use]
pub fn local_media_track_kind(
    track: RustOpaque<LocalMediaTrack>,
) -> SyncReturn<MediaKind> {
    SyncReturn(track.kind())
}

/// Returns a [`MediaSourceKind::Device`] if this [`LocalMediaTrack`] is
/// sourced from some device (webcam/microphone), or a
/// [`MediaSourceKind::Display`] if it's captured via
/// [MediaDevices.getDisplayMedia()][1].
///
/// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
/// [`MediaSourceKind::Device`]: crate::media::MediaSourceKind::Device
/// [`MediaSourceKind::Display`]: crate::media::MediaSourceKind::Display
#[must_use]
pub fn local_media_track_media_source_kind(
    track: RustOpaque<LocalMediaTrack>,
) -> SyncReturn<MediaSourceKind> {
    SyncReturn(track.media_source_kind())
}

/// Frees the data behind the provided opaque local track.
#[must_use]
pub fn local_media_track_free(
    track: RustOpaque<LocalMediaTrack>,
) -> SyncReturn<DartOpaque> {
    let track = track.try_unwrap().unwrap();
    SyncReturn(
        async move {
            track.maybe_stop().await;
            Ok::<_, Error>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    )
}

// -------------------------------------------------------------------

/// Returns the [`Vec<MediaDeviceInfo>`] from the address
/// [`ForeignClass`].
#[must_use]
pub fn vec_media_device_info_from_ptr(
    ptr: usize,
) -> SyncReturn<Vec<ApiMediaDeviceInfo>> {
    SyncReturn(unsafe {
        Vec::<ApiMediaDeviceInfo>::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        )
    })
}

// -------------------------------------------------------------------

/// Returns the [`Vec<RustOpaque<MediaDisplayInfo>>`] from the address
/// [`ForeignClass`].
#[must_use]
pub fn vec_media_display_info_from_ptr(
    ptr: usize,
) -> SyncReturn<Vec<ApiMediaDisplayInfo>> {
    SyncReturn(unsafe {
        Vec::<ApiMediaDisplayInfo>::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        )
    })
}

// -------------------------------------------------------------------

impl RefUnwindSafe for MediaManagerHandle {}
impl UnwindSafe for MediaManagerHandle {}

/// Returns [`LocalMediaTrack`]s objects, built from the provided
/// [`ApiMediaStreamSettings`].
///
/// [`LocalMediaTrack`]: crate::media::track::local::LocalMediaTrack
#[must_use]
pub fn media_manager_handle_init_local_tracks(
    manager: RustOpaque<MediaManagerHandle>,
    caps: ApiMediaStreamSettings,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);

    SyncReturn(
        async move { manager.init_local_tracks(caps.into()).await }
            .into_dart_future()
            .into_dart_opaque(),
    )
}

/// Returns a list of [`ApiMediaDeviceInfo`] objects representing available
/// media input and devices, such as microphones, cameras, and so forth.
#[must_use]
pub fn media_manager_handle_enumerate_devices(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let result = async move {
        Ok::<Vec<_>, Traced<EnumerateDevicesError>>(
            manager
                .enumerate_devices()
                .await?
                .into_iter()
                .map(|v| ApiMediaDeviceInfo {
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

    SyncReturn(result)
}

/// Returns a list of [`ApiMediaDisplayInfo`] objects representing available
/// sources that can be used for screen capturing.
#[must_use]
pub fn media_manager_handle_enumerate_displays(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let result = async move {
        Ok::<Vec<_>, Traced<EnumerateDisplaysError>>(
            manager
                .enumerate_displays()
                .await?
                .into_iter()
                .map(|v| ApiMediaDisplayInfo {
                    device_id: v.device_id(),
                    title: v.title(),
                })
                .collect(),
        )
    }
    .into_dart_future()
    .into_dart_opaque();

    SyncReturn(result)
}

/// Switches the current output audio device to the device with the provided
/// `device_id`.
#[must_use]
pub fn media_manager_handle_set_output_audio_id(
    manager: RustOpaque<MediaManagerHandle>,
    device_id: String,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let result = async move {
        manager
            .set_output_audio_id(device_id)
            .await
            .map_err(tracerr::map_from_and_wrap!())?;
        Ok::<_, Traced<InvalidOutputAudioDeviceIdError>>(())
    }
    .into_dart_future()
    .into_dart_opaque();

    SyncReturn(result)
}

/// Sets the microphone volume level in percents.
#[must_use]
pub fn media_manager_handle_set_microphone_volume(
    manager: RustOpaque<MediaManagerHandle>,
    level: i64,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let result = async move {
        manager
            .set_microphone_volume(level)
            .await
            .map_err(tracerr::map_from_and_wrap!())?;
        Ok::<_, Traced<MicVolumeError>>(())
    }
    .into_dart_future()
    .into_dart_opaque();

    SyncReturn(result)
}

/// Indicates whether it's possible to access microphone volume settings.
#[must_use]
pub fn media_manager_handle_microphone_volume_is_available(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let result = async move { manager.microphone_volume_is_available().await }
        .into_dart_future()
        .into_dart_opaque();

    SyncReturn(result)
}

/// Returns the current microphone volume level in percents.
#[must_use]
pub fn media_manager_handle_microphone_volume(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);

    SyncReturn(
        async move { manager.microphone_volume().await }
            .into_dart_future()
            .into_dart_opaque(),
    )
}

/// Subscribes onto the [`MediaManagerHandle`]'s `devicechange` event.
/// Sets an ideal [frameRate][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
///
/// # Errors
///
/// If [`MediaManagerHandle::on_device_change`] returns error.
pub fn media_manager_handle_on_device_change(
    manager: RustOpaque<MediaManagerHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    let manager = MediaManagerHandle::clone(&manager);
    manager
        .on_device_change(unsafe {
            platform::Function::new(cb.try_unwrap().unwrap().into_raw().cast())
        })
        .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?;

    Ok(SyncReturn(()))
}

// -------------------------------------------------------------------

impl ForeignClass for ReconnectHandle {}
impl RefUnwindSafe for ReconnectHandle {}
impl UnwindSafe for ReconnectHandle {}

/// Returns the [`ReconnectHandle`] from the address
/// [`ForeignClass`].
#[must_use]
pub fn reconnect_handle_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<ReconnectHandle>> {
    SyncReturn(unsafe {
        RustOpaque::new(ReconnectHandle::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    })
}

/// Tries to reconnect a [`Room`] after the provided delay in milliseconds.
///
/// If the [`Room`] is already reconnecting then new reconnection attempt won't
/// be performed. Instead, it will wait for the first reconnection attempt
/// result and use it here..
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn reconnect_handle_reconnect_with_delay(
    reconnect_handle: RustOpaque<ReconnectHandle>,
    delay_ms: u32,
) -> SyncReturn<DartOpaque> {
    let reconnect_handle = ReconnectHandle::clone(&reconnect_handle);
    let result = async move {
        reconnect_handle.reconnect_with_delay(delay_ms).await?;
        Ok::<_, Error>(())
    }
    .into_dart_future()
    .into_dart_opaque();

    SyncReturn(result)
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
/// [`Room`]: crate::room::Room
#[must_use]
pub fn reconnect_handle_reconnect_with_backoff(
    reconnect_handle: RustOpaque<ReconnectHandle>,
    starting_delay: u32,
    multiplier: f64,
    max_delay: u32,
    max_elapsed_time_ms: Option<u32>,
) -> SyncReturn<DartOpaque> {
    let reconnect_handle = ReconnectHandle::clone(&reconnect_handle);
    let result = async move {
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
    .into_dart_opaque();

    SyncReturn(result)
}

// -------------------------------------------------------------------

impl ForeignClass for RemoteMediaTrack {}
impl RefUnwindSafe for RemoteMediaTrack {}
impl UnwindSafe for RemoteMediaTrack {}

/// Returns the [`RemoteMediaTrack`] from the address
/// [`ForeignClass`].
#[must_use]
pub fn remote_media_track_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<RemoteMediaTrack>> {
    SyncReturn(unsafe {
        RustOpaque::new(RemoteMediaTrack::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    })
}

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
/// [`RemoteMediaTrack`].
///
/// [`MediaStreamTrack`]: platform::MediaStreamTrack
#[must_use]
pub fn remote_media_track_get_track(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<DartOpaque> {
    SyncReturn(unsafe { new_dart_opaque(track.get_track().handle()) })
}

/// Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
#[must_use]
pub fn remote_media_track_on_muted(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) -> SyncReturn<()> {
    track.on_muted(unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw().cast())
    });

    SyncReturn(())
}

/// Sets callback to invoke when this [`RemoteMediaTrack`] is unmuted.
#[must_use]
pub fn remote_media_track_on_unmuted(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) -> SyncReturn<()> {
    track.on_unmuted(unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw().cast())
    });

    SyncReturn(())
}

/// Sets callback to invoke when this [`RemoteMediaTrack`] is stopped.
#[must_use]
pub fn remote_media_track_on_stopped(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) -> SyncReturn<()> {
    track.on_stopped(unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw().cast())
    });

    SyncReturn(())
}

/// Sets callback to invoke whenever this [`RemoteMediaTrack`]'s general
/// [`MediaDirection`] is changed.
#[must_use]
pub fn remote_media_track_on_media_direction_changed(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) -> SyncReturn<()> {
    track.on_media_direction_changed(unsafe {
        platform::Function::<MediaDirection>::new(
            f.try_unwrap().unwrap().into_raw().cast(),
        )
    });

    SyncReturn(())
}

/// Indicate whether this [`RemoteMediaTrack`] is muted.
#[must_use]
pub fn remote_media_track_muted(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<bool> {
    SyncReturn(track.muted())
}

/// Returns this [`RemoteMediaTrack`]'s kind (audio/video).
#[must_use]
pub fn remote_media_track_kind(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<MediaKind> {
    SyncReturn(track.kind())
}

/// Returns this [`RemoteMediaTrack`]'s media source kind.
#[must_use]
pub fn remote_media_track_media_source_kind(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<MediaSourceKind> {
    SyncReturn(track.media_source_kind())
}

/// Returns the current general [`MediaDirection`] of this [`RemoteMediaTrack`].
#[must_use]
pub fn remote_media_track_media_direction(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<MediaDirection> {
    SyncReturn(track.media_direction())
}

// -------------------------------------------------------------------

impl ForeignClass for RoomCloseReason {}

/// Returns the [`RoomCloseReason`] from the address
/// [`ForeignClass`].
#[must_use]
pub fn room_close_reason_from_ptr(ptr: usize) -> SyncReturn<RoomCloseReason> {
    SyncReturn(unsafe {
        RoomCloseReason::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
    })
}

// -------------------------------------------------------------------

impl RefUnwindSafe for RoomHandle {}
impl UnwindSafe for RoomHandle {}

/// Connects to a media server and joins the [`Room`] with the provided
/// authorization `token`.
///
/// Authorization token has a fixed format:
/// `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
/// (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_join(
    room_handle: RustOpaque<RoomHandle>,
    token: String,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);
    let result = async move {
        room_handle.join(token).await?;
        Ok::<_, Traced<RoomJoinError>>(())
    }
    .into_dart_future()
    .into_dart_opaque();

    SyncReturn(result)
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
/// [`Room`]: crate::room::Room
/// [`PeerConnection`]: crate::peer::PeerConnection
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
#[must_use]
pub fn room_handle_set_local_media_settings(
    room_handle: RustOpaque<RoomHandle>,
    settings: ApiMediaStreamSettings,
    stop_first: bool,
    rollback_on_fail: bool,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let result = async move {
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
    .into_dart_opaque();

    SyncReturn(result)
}

/// Mutes outbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_mute_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    SyncReturn(
        async move {
            room_handle.mute_audio().await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    )
}

/// Unmutes outbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_unmute_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    SyncReturn(
        async move {
            room_handle.unmute_audio().await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    )
}

/// Enables outbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_enable_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    SyncReturn(
        async move {
            room_handle.enable_audio().await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    )
}

/// Disables outbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_disable_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    SyncReturn(
        async move {
            room_handle.disable_audio().await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    )
}

/// Mutes outbound video in this [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// [`Room`]: crate::room::Room
///
/// # Errors
///
/// If `source_kind` is not a [`MediaSourceKind`] index.
pub fn room_handle_mute_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    Ok(SyncReturn(
        async move {
            room_handle.mute_video(source_kind).await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    ))
}

/// Unmutes outbound video in this [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// [`Room`]: crate::room::Room
///
/// # Errors
///
/// If `source_kind` is not a [`MediaSourceKind`] index.
pub fn room_handle_unmute_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    Ok(SyncReturn(
        async move {
            room_handle.unmute_video(source_kind).await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    ))
}

/// Enables outbound video.
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
pub fn room_handle_enable_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    Ok(SyncReturn(
        async move {
            room_handle.enable_video(source_kind).await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    ))
}

/// Disables outbound video.
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
pub fn room_handle_disable_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    Ok(SyncReturn(
        async move {
            room_handle.disable_video(source_kind).await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    ))
}

/// Enables inbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_enable_remote_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    SyncReturn(
        async move {
            room_handle.enable_remote_audio().await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    )
}

/// Disables inbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_disable_remote_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    SyncReturn(
        async move {
            room_handle.disable_remote_audio().await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    )
}

/// Enables inbound video in this [`Room`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`Room`]: crate::room::Room
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
pub fn room_handle_enable_remote_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    Ok(SyncReturn(
        async move {
            room_handle.enable_remote_video(source_kind).await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    ))
}

/// Disables inbound video in this [`Room`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`Room`]: crate::room::Room
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
pub fn room_handle_disable_remote_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<MediaSourceKind>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    Ok(SyncReturn(
        async move {
            room_handle.disable_remote_video(source_kind).await?;

            Ok::<_, Traced<room::ChangeMediaStateError>>(())
        }
        .into_dart_future()
        .into_dart_opaque(),
    ))
}

/// Sets callback, invoked when a new [`Connection`] with some remote `Peer`
/// is established.
///
/// [`Connection`]: crate::connection::Connection
///
/// # Errors
///
/// If [`RoomHandle::on_new_connection`] returns error.
pub fn room_handle_on_new_connection(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    Ok(SyncReturn(
        room_handle
            .on_new_connection(unsafe {
                platform::Function::new(
                    cb.try_unwrap().unwrap().into_raw().cast(),
                )
            })
            .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?,
    ))
}

/// Sets callback, invoked on this [`Room`] close, providing a
/// [`RoomCloseReason`].
///
/// [`Room`]: crate::room::Room
/// [`RoomCloseReason`]: crate::room::RoomCloseReason
///
/// # Errors
///
/// If [`RoomHandle::on_close`] returns error.
pub fn room_handle_on_close(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_close(unsafe {
            platform::Function::new(cb.try_unwrap().unwrap().into_raw().cast())
        })
        .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?;

    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a new [`LocalMediaTrack`] is added to this
/// [`Room`].
///
/// This might happen in such cases:
/// 1. Media server initiates a media request.
/// 2. `enable_audio`/`enable_video` is called.
/// 3. [`MediaStreamSettings`] updated via `set_local_media_settings`.
///
/// [`Room`]: crate::room::Room
/// [`MediaStreamSettings`]: crate::media::MediaStreamSettings
/// [`LocalMediaTrack`]: crate::media::track::local::LocalMediaTrack
///
/// # Errors
///
/// If [`RoomHandle::on_local_track`] returns error.
pub fn room_handle_on_local_track(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_local_track(unsafe {
            platform::Function::new(cb.try_unwrap().unwrap().into_raw().cast())
        })
        .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?;

    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a connection with server is lost.
///
/// # Errors
///
/// If [`RoomHandle::on_connection_loss`] returns error.
pub fn room_handle_on_connection_loss(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_connection_loss(unsafe {
            platform::Function::new(cb.try_unwrap().unwrap().into_raw().cast())
        })
        .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?;

    Ok(SyncReturn(()))
}

/// Sets callback, invoked on local media acquisition failures.
///
/// # Errors
///
/// If [`RoomHandle::on_failed_local_media`] returns error.
pub fn room_handle_on_failed_local_media(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_failed_local_media(unsafe {
            platform::Function::new(cb.try_unwrap().unwrap().into_raw().cast())
        })
        .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?;

    Ok(SyncReturn(()))
}
