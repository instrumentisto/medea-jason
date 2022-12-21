use super::{utils::new_dart_opaque, ForeignClass, MediaDirection};

pub use super::utils::ApiWrap;

use crate::{
    api::{ArgumentError, Error as DartError},
    media::FacingMode,
    platform::utils::dart_future::{dart_enum_try_into, IntoDartFuture},
    room::ChangeMediaStateError,
};

use flutter_rust_bridge::{DartOpaque, RustOpaque, SyncReturn};
use std::{
    panic::{RefUnwindSafe, UnwindSafe},
    ptr,
};

use flutter_rust_bridge::DartSafe;
impl<T: DartSafe> ForeignClass for RustOpaque<T> {}

// -------------------------------------------------------------------

pub use crate::media::AudioTrackConstraints;

/// Creates new [`AudioTrackConstraints`] with none constr configured.
#[must_use]
pub fn audio_track_constr_new() -> SyncReturn<RustOpaque<AudioTrackConstraints>>
{
    SyncReturn(RustOpaque::new(AudioTrackConstraints::new()))
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
#[must_use]
pub fn audio_track_constr_device_id(
    track: RustOpaque<AudioTrackConstraints>,
    device_id: String,
) -> SyncReturn<RustOpaque<AudioTrackConstraints>> {
    let mut track = track.try_unwrap().unwrap();
    track.device_id(device_id);
    SyncReturn(RustOpaque::new(track))
}

// -------------------------------------------------------------------

pub use dart_sys::Dart_Handle;

use tracerr::Traced;

pub use crate::connection::ConnectionHandle;

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
    connection
        .on_close(unsafe {
            platform::Function::new(f.try_unwrap().unwrap().into_raw())
        })
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
    connection
        .on_remote_track_added(unsafe {
            platform::Function::new(f.try_unwrap().unwrap().into_raw())
        })
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
    connection
        .on_quality_score_update(unsafe {
            platform::Function::new(f.try_unwrap().unwrap().into_raw())
        })
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
    let fut = connection.enable_remote_audio();
    SyncReturn(unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<(), Traced<crate::connection::ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    })
}

/// Disables inbound audio in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[must_use]
pub fn connection_handle_disable_remote_audio(
    connection: RustOpaque<ConnectionHandle>,
) -> SyncReturn<DartOpaque> {
    let fut = connection.disable_remote_audio();

    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<(), Traced<crate::connection::ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Enables inbound video in this [`ConnectionHandle`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[must_use]
pub fn connection_handle_enable_remote_video(
    connection: RustOpaque<ConnectionHandle>,
    source_kind: Option<i64>,
) -> SyncReturn<DartOpaque> {
    let fut = connection.enable_remote_video(
        source_kind.map(|v| MediaSourceKind::try_from(v).unwrap()),
    );
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<(), Traced<crate::connection::ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Disables inbound video in this [`ConnectionHandle`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
#[must_use]
pub fn connection_handle_disable_remote_video(
    connection: RustOpaque<ConnectionHandle>,
    source_kind: Option<i64>,
) -> SyncReturn<DartOpaque> {
    let fut = connection.disable_remote_video(
        source_kind.map(|v| MediaSourceKind::try_from(v).unwrap()),
    );
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<(), Traced<crate::connection::ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

// -------------------------------------------------------------------

pub use crate::media::DeviceVideoTrackConstraints;
use crate::{
    api::Error,
    media::{InitLocalTracksError, MediaSourceKind},
    platform,
};

/// Creates new [`DeviceVideoTrackConstraints`] with none constr
/// configured.
#[must_use]
pub fn device_video_track_constr_new(
) -> SyncReturn<RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>> {
    SyncReturn(RustOpaque::new(unsafe {
        ApiWrap::new(DeviceVideoTrackConstraints::new())
    }))
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
#[must_use]
pub fn device_video_track_constr_device_id(
    constr: RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>,
    device_id: String,
) -> SyncReturn<()> {
    let mut constr = constr.borrow_mut();
    constr.device_id(device_id);
    SyncReturn(())
}

/// Sets an exact [facingMode][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
///
/// # Errors
///
/// If `facing_mode` is not [`FacingMode`] index.
#[must_use]
pub fn device_video_track_constr_exact_facing_mode(
    constr: RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>,
    facing_mode: FacingMode,
) -> SyncReturn<()> {
    let mut constr = constr.borrow_mut();
    constr.exact_facing_mode(facing_mode);
    SyncReturn(())
}

/// Sets an ideal [facingMode][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
///
/// # Errors
///
/// If `facing_mode` is not [`FacingMode`] index.
#[must_use]
pub fn device_video_track_constr_ideal_facing_mode(
    constr: RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>,
    facing_mode: FacingMode,
) -> SyncReturn<()> {
    let mut constr = constr.borrow_mut();
    constr.ideal_facing_mode(facing_mode);
    SyncReturn(())
}

/// Sets an exact [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
///
/// # Errors
///
/// If `exact_height` is not [u32].
pub fn device_video_track_constr_exact_height(
    constr: RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>,
    exact_height: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    let Ok(exact_height) = u32::try_from(exact_height) else {
        anyhow::bail!("{:?}",
            DartError::from(
                ArgumentError::new(
                    exact_height,
                    "exact_height",
                    "Expected u32")
                )
            );
    };
    constr.exact_height(exact_height);
    Ok(SyncReturn(()))
}

/// Sets an ideal [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
///
/// # Errors
///
/// If `ideal_height` is not [u32].
pub fn device_video_track_constr_ideal_height(
    constr: RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>,
    ideal_height: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    let Ok(ideal_height) = u32::try_from(ideal_height) else {
        anyhow::bail!("{:?}",
            DartError::from(
                ArgumentError::new(
                    ideal_height,
                    "ideal_height",
                    "Expected u32")
                )
            );
    };
    constr.ideal_height(ideal_height);
    Ok(SyncReturn(()))
}

/// Sets an exact [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
///
/// # Errors
///
/// If `exact_width` is not [u32].
pub fn device_video_track_constr_exact_width(
    constr: RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>,
    exact_width: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    let Ok(exact_width) = u32::try_from(exact_width) else {
        anyhow::bail!("{:?}",
            DartError::from(
                ArgumentError::new(
                    exact_width,
                    "exact_width",
                    "Expected u32")
                )
            );
    };
    constr.exact_width(exact_width);
    Ok(SyncReturn(()))
}

/// Sets an ideal [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
///
/// # Errors
///
/// If `ideal_width` is not [u32].
pub fn device_video_track_constr_ideal_width(
    constr: RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>,
    ideal_width: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    let Ok(ideal_width) = u32::try_from(ideal_width) else {
        anyhow::bail!("{:?}",
            DartError::from(
                ArgumentError::new(
                    ideal_width,
                    "ideal_width",
                    "Expected u32")
                )
            );
    };
    constr.ideal_width(ideal_width);
    Ok(SyncReturn(()))
}

/// Sets a range of a [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
///
/// # Errors
///
/// If `min` or `max` is not [u32].
pub fn device_video_track_constr_height_in_range(
    constr: RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>,
    min: i64,
    max: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    match (u32::try_from(min), u32::try_from(max)) {
        (Ok(min), Ok(max)) => {
            constr.height_in_range(min, max);
        }
        (Err(_), _) => {
            anyhow::bail!(
                "{:?}",
                DartError::from(ArgumentError::new(min, "min", "Expected u32"))
            )
        }
        (_, Err(_)) => {
            anyhow::bail!(
                "{:?}",
                DartError::from(ArgumentError::new(max, "max", "Expected u32"))
            )
        }
    }
    Ok(SyncReturn(()))
}

/// Sets a range of a [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
///
/// # Errors
///
/// If `min` or `max` is not [u32].
pub fn device_video_track_constr_width_in_range(
    constr: RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>,
    min: i64,
    max: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    match (u32::try_from(min), u32::try_from(max)) {
        (Ok(min), Ok(max)) => {
            constr.width_in_range(min, max);
        }
        (Err(_), _) => {
            anyhow::bail!(
                "{:?}",
                DartError::from(ArgumentError::new(min, "min", "Expected u32"))
            )
        }
        (_, Err(_)) => {
            anyhow::bail!(
                "{:?}",
                DartError::from(ArgumentError::new(max, "max", "Expected u32"))
            )
        }
    }
    Ok(SyncReturn(()))
}

// -------------------------------------------------------------------

pub use crate::media::DisplayVideoTrackConstraints;

/// Creates new [`DisplayVideoTrackConstraints`] with none constr
/// configured.
#[must_use]
pub fn display_video_track_constr_new(
) -> SyncReturn<RustOpaque<ApiWrap<DisplayVideoTrackConstraints>>> {
    SyncReturn(RustOpaque::new(unsafe {
        ApiWrap::new(DisplayVideoTrackConstraints::new())
    }))
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
#[must_use]
pub fn display_video_track_constr_device_id(
    constr: RustOpaque<ApiWrap<DisplayVideoTrackConstraints>>,
    device_id: String,
) -> SyncReturn<()> {
    let mut constr = constr.borrow_mut();
    constr.device_id(device_id);
    SyncReturn(())
}

/// Sets an exact [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
///
/// # Errors
///
/// If `exact_height` is not [u32].
pub fn display_video_track_constr_exact_height(
    constr: RustOpaque<ApiWrap<DisplayVideoTrackConstraints>>,
    exact_height: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    let Ok(exact_height) = u32::try_from(exact_height) else {
        anyhow::bail!("{:?}",
            DartError::from(
                ArgumentError::new(
                    exact_height,
                    "exact_height",
                    "Expected u32")
                )
            );
    };
    constr.ideal_width(exact_height);
    Ok(SyncReturn(()))
}

/// Sets an ideal [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
///
/// # Errors
///
/// If `ideal_height` is not [u32].
pub fn display_video_track_constr_ideal_height(
    constr: RustOpaque<ApiWrap<DisplayVideoTrackConstraints>>,
    ideal_height: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    let Ok(ideal_height) = u32::try_from(ideal_height) else {
        anyhow::bail!("{:?}",
            DartError::from(
                ArgumentError::new(
                    ideal_height,
                    "ideal_height",
                    "Expected u32")
                )
            );
    };
    constr.ideal_height(ideal_height);
    Ok(SyncReturn(()))
}

/// Sets an exact [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
///
/// # Errors
///
/// If `exact_width` is not [u32].
pub fn display_video_track_constr_exact_width(
    constr: RustOpaque<ApiWrap<DisplayVideoTrackConstraints>>,
    exact_width: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    let Ok(exact_width) = u32::try_from(exact_width) else {
        anyhow::bail!("{:?}",
            DartError::from(
                ArgumentError::new(
                    exact_width,
                    "exact_width",
                    "Expected u32")
                )
            );
    };
    constr.exact_width(exact_width);
    Ok(SyncReturn(()))
}

/// Sets an ideal [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
///
/// # Errors
///
/// If `ideal_width` is not [u32].
pub fn display_video_track_constr_ideal_width(
    constr: RustOpaque<ApiWrap<DisplayVideoTrackConstraints>>,
    ideal_width: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    let Ok(ideal_width) = u32::try_from(ideal_width) else {
        anyhow::bail!("{:?}",
            DartError::from(
                ArgumentError::new(
                    ideal_width,
                    "ideal_width",
                    "Expected u32")
                )
            );
    };
    constr.ideal_width(ideal_width);
    Ok(SyncReturn(()))
}

/// Sets an ideal [frameRate][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
///
/// # Errors
///
/// If `ideal_frame_rate` is not [u32].
pub fn display_video_track_constr_ideal_frame_rate(
    constr: RustOpaque<ApiWrap<DisplayVideoTrackConstraints>>,
    ideal_frame_rate: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    let Ok(ideal_frame_rate) = u32::try_from(ideal_frame_rate) else {
        anyhow::bail!("{:?}",
            DartError::from(
                ArgumentError::new(
                    ideal_frame_rate,
                    "ideal_frame_rate",
                    "Expected u32")
                )
            );
    };
    constr.ideal_frame_rate(ideal_frame_rate);
    Ok(SyncReturn(()))
}

/// Sets an exact [frameRate][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
///
/// # Errors
///
/// If `exact_frame_rate` is not [u32].
pub fn display_video_track_constr_exact_frame_rate(
    constr: RustOpaque<ApiWrap<DisplayVideoTrackConstraints>>,
    exact_frame_rate: i64,
) -> anyhow::Result<SyncReturn<()>> {
    let mut constr = constr.borrow_mut();
    let Ok(exact_frame_rate) = u32::try_from(exact_frame_rate) else {
        anyhow::bail!("{:?}",
            DartError::from(
                ArgumentError::new(
                    exact_frame_rate,
                    "exact_frame_rate",
                    "Expected u32")
                )
            );
    };
    constr.exact_frame_rate(exact_frame_rate);
    Ok(SyncReturn(()))
}

// -------------------------------------------------------------------

pub use crate::jason::Jason;

impl RefUnwindSafe for Jason {}
impl UnwindSafe for Jason {}

/// Sets the provided [`Dart_Handle`] as a callback for the Rust panic hook.
#[must_use]
pub fn on_panic(cb: DartOpaque) -> SyncReturn<()> {
    platform::set_panic_callback(unsafe {
        platform::Function::new(cb.try_unwrap().unwrap().into_raw())
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

pub use crate::media::track::local::LocalMediaTrack;

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

/// Returns the [`ApiWrap<Vec<LocalMediaTrack>>`] from the address
/// [`ForeignClass`].
#[must_use]
pub fn vec_local_tracks_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<ApiWrap<Vec<LocalMediaTrack>>>> {
    SyncReturn(unsafe {
        RustOpaque::new(ApiWrap::from_ptr(ptr::NonNull::new(ptr as _).unwrap()))
    })
}

/// Removes the last element from a
/// [`ApiWrap<Vec<LocalMediaTrack>>`] and returns it,
/// or [`None`] if it is empty.
#[must_use]
pub fn vec_local_tracks_pop(
    vec: RustOpaque<ApiWrap<Vec<LocalMediaTrack>>>,
) -> SyncReturn<Option<RustOpaque<LocalMediaTrack>>> {
    SyncReturn(vec.borrow_mut().pop().map(RustOpaque::new))
}

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
/// [`LocalMediaTrack`].
///
/// [`MediaStreamTrack`]: crate::platform::MediaStreamTrack
#[must_use]
pub fn local_media_track_get_track(
    track: RustOpaque<LocalMediaTrack>,
) -> SyncReturn<DartOpaque> {
    let dart_opaque = unsafe { new_dart_opaque(track.get_track().handle()) };
    SyncReturn(dart_opaque)
}

/// Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
/// audio track, or a [`MediaKind::Video`] if it represents a video track.
///
/// [`MediaKind::Audio`]: crate::media::MediaKind::Audio
/// [`MediaKind::Video`]: crate::media::MediaKind::Video
#[must_use]
pub fn local_media_track_kind(
    track: RustOpaque<LocalMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.kind() as u8)
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
) -> SyncReturn<u8> {
    SyncReturn(track.media_source_kind() as u8)
}

// -------------------------------------------------------------------

pub use crate::platform::MediaDeviceInfo;

impl ForeignClass for MediaDeviceInfo {}

/// Returns the [`ApiWrap<Vec<MediaDeviceInfo>>`] from the address
/// [`ForeignClass`].
#[must_use]
pub fn vec_media_device_info_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<ApiWrap<Vec<MediaDeviceInfo>>>> {
    SyncReturn(unsafe {
        RustOpaque::new(ApiWrap::from_ptr(ptr::NonNull::new(ptr as _).unwrap()))
    })
}

/// Removes the last element from a
/// [`ApiWrap<Vec<MediaDeviceInfo>>`] and returns it,
/// or [`None`] if it is empty.
#[must_use]
pub fn vec_media_device_info_pop(
    vec: RustOpaque<ApiWrap<Vec<MediaDeviceInfo>>>,
) -> SyncReturn<Option<RustOpaque<MediaDeviceInfo>>> {
    SyncReturn(vec.borrow_mut().pop().map(RustOpaque::new))
}

/// Returns unique identifier of the represented device.
#[must_use]
pub fn media_device_info_device_id(
    media_device: RustOpaque<MediaDeviceInfo>,
) -> SyncReturn<String> {
    SyncReturn(media_device.device_id())
}

/// Returns kind of the represented device.
///
/// This representation of [MediaDeviceInfo][1] ONLY for input device.
///
/// [1]: https://w3.org/TR/mediacapture-streams/#device-info
#[must_use]
pub fn media_device_info_kind(
    media_device: RustOpaque<MediaDeviceInfo>,
) -> SyncReturn<u8> {
    SyncReturn(media_device.kind() as u8)
}

/// Returns label describing the represented device (for example "External USB
/// Webcam").
///
/// If the device has no associated label, then returns an empty string.
#[must_use]
pub fn media_device_info_label(
    media_device: RustOpaque<MediaDeviceInfo>,
) -> SyncReturn<String> {
    SyncReturn(media_device.label())
}

/// Returns group identifier of the represented device.
///
/// Two devices have the same group identifier if they belong to the same
/// physical device. For example, the audio input and output devices
/// representing the speaker and microphone of the same headset have the
/// same [groupId][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediadeviceinfo-groupid
#[must_use]
pub fn media_device_info_group_id(
    media_device: RustOpaque<MediaDeviceInfo>,
) -> SyncReturn<Option<String>> {
    SyncReturn(media_device.group_id())
}

// -------------------------------------------------------------------

pub use crate::platform::MediaDisplayInfo;

/// Returns the [`ApiWrap<Vec<MediaDisplayInfo>>`] from the address
/// [`ForeignClass`].
#[must_use]
pub fn vec_media_display_info_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<ApiWrap<Vec<MediaDisplayInfo>>>> {
    SyncReturn(RustOpaque::new(unsafe {
        ApiWrap::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
    }))
}

/// Removes the last element from a
/// [`ApiWrap<Vec<MediaDisplayInfo>>`] and returns it,
/// or [`None`] if it is empty.
#[must_use]
pub fn vec_media_display_info_pop(
    vec: RustOpaque<ApiWrap<Vec<MediaDisplayInfo>>>,
) -> SyncReturn<Option<RustOpaque<MediaDisplayInfo>>> {
    SyncReturn(vec.borrow_mut().pop().map(RustOpaque::new))
}

/// Returns a unique identifier of the represented display.
#[must_use]
pub fn media_display_info_device_id(
    media_display: RustOpaque<MediaDisplayInfo>,
) -> SyncReturn<String> {
    SyncReturn(media_display.device_id())
}

/// Returns a title describing the represented display.
#[must_use]
pub fn media_display_info_title(
    media_display: RustOpaque<MediaDisplayInfo>,
) -> SyncReturn<Option<String>> {
    SyncReturn(media_display.title())
}

// -------------------------------------------------------------------

use crate::media::{
    EnumerateDevicesError, EnumerateDisplaysError,
    InvalidOutputAudioDeviceIdError, MicVolumeError,
};

pub use crate::media::MediaManagerHandle;

impl RefUnwindSafe for MediaManagerHandle {}
impl UnwindSafe for MediaManagerHandle {}

/// Returns [`LocalMediaTrack`]s objects, built from the provided
/// [`MediaStreamSettings`].
///
/// [`LocalMediaTrack`]: crate::media::track::local::LocalMediaTrack
#[must_use]
pub fn media_manager_handle_init_local_tracks(
    manager: RustOpaque<MediaManagerHandle>,
    caps: RustOpaque<MediaStreamSettings>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let caps = MediaStreamSettings::clone(&caps);
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                Ok::<ApiWrap<_>, Traced<InitLocalTracksError>>(ApiWrap::new(
                    manager.init_local_tracks(caps).await?,
                ))
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Returns a list of [`MediaDeviceInfo`] objects representing available media
/// input and devices, such as microphones, cameras, and so forth.
#[must_use]
pub fn media_manager_handle_enumerate_devices(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                Ok::<ApiWrap<_>, Traced<EnumerateDevicesError>>(ApiWrap::new(
                    manager.enumerate_devices().await?,
                ))
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Returns a list of [`MediaDisplayInfo`] objects representing available
/// sources that can be used for screen capturing.
#[must_use]
pub fn media_manager_handle_enumerate_displays(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                Ok::<ApiWrap<_>, Traced<EnumerateDisplaysError>>(ApiWrap::new(
                    manager.enumerate_displays().await?,
                ))
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Switches the current output audio device to the device with the provided
/// `device_id`.
#[must_use]
pub fn media_manager_handle_set_output_audio_id(
    manager: RustOpaque<MediaManagerHandle>,
    device_id: String,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                manager
                    .set_output_audio_id(device_id)
                    .await
                    .map_err(tracerr::map_from_and_wrap!())?;
                Ok::<_, Traced<InvalidOutputAudioDeviceIdError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Sets the microphone volume level in percents.
#[must_use]
pub fn media_manager_handle_set_microphone_volume(
    manager: RustOpaque<MediaManagerHandle>,
    level: i64,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                manager
                    .set_microphone_volume(level)
                    .await
                    .map_err(tracerr::map_from_and_wrap!())?;
                Ok::<_, Traced<MicVolumeError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Indicates whether it's possible to access microphone volume settings.
#[must_use]
pub fn media_manager_handle_microphone_volume_is_available(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move { manager.microphone_volume_is_available().await }
                .into_dart_future()
                .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Returns the current microphone volume level in percents.
#[must_use]
pub fn media_manager_handle_microphone_volume(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);

    let dart_opaque = unsafe {
        new_dart_opaque(
            async move { manager.microphone_volume().await }
                .into_dart_future()
                .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
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
            platform::Function::new(cb.try_unwrap().unwrap().into_raw())
        })
        .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?;
    Ok(SyncReturn(()))
}

// -------------------------------------------------------------------

pub use crate::media::MediaStreamSettings;
impl ForeignClass for MediaStreamSettings {}

/// Creates new [`MediaStreamSettings`] with none constr configured.
#[rustfmt::skip]
#[must_use]
pub fn media_stream_settings_new() -> SyncReturn<RustOpaque<MediaStreamSettings>>
{
    SyncReturn(RustOpaque::new(MediaStreamSettings::new()))
}

/// Specifies a nature and settings of an audio [`MediaStreamTrack`].
///
/// [`MediaStreamTrack`]: crate::platform::MediaStreamTrack
#[must_use]
pub fn media_stream_settings_audio(
    media_stream_settings: RustOpaque<MediaStreamSettings>,
    constr: RustOpaque<AudioTrackConstraints>,
) -> SyncReturn<RustOpaque<MediaStreamSettings>> {
    let mut media_stream_settings = media_stream_settings.try_unwrap().unwrap();
    media_stream_settings.audio(AudioTrackConstraints::clone(&constr));
    SyncReturn(RustOpaque::new(media_stream_settings))
}

/// Set constr for obtaining a local video sourced from a media device.
#[must_use]
pub fn media_stream_settings_device_video(
    media_stream_settings: RustOpaque<MediaStreamSettings>,
    constr: RustOpaque<ApiWrap<DeviceVideoTrackConstraints>>,
) -> SyncReturn<RustOpaque<MediaStreamSettings>> {
    let mut media_stream_settings = media_stream_settings.try_unwrap().unwrap();
    let constr = constr.try_unwrap().unwrap().into_inner();
    media_stream_settings.device_video(constr);
    SyncReturn(RustOpaque::new(media_stream_settings))
}

/// Set constr for capturing a local video from user's display.
#[must_use]
pub fn media_stream_settings_display_video(
    media_stream_settings: RustOpaque<MediaStreamSettings>,
    constr: RustOpaque<ApiWrap<DisplayVideoTrackConstraints>>,
) -> SyncReturn<RustOpaque<MediaStreamSettings>> {
    let mut media_stream_settings = media_stream_settings.try_unwrap().unwrap();
    let constr = constr.try_unwrap().unwrap().into_inner();
    media_stream_settings.display_video(constr);
    SyncReturn(RustOpaque::new(media_stream_settings))
}

// -------------------------------------------------------------------

pub use crate::rpc::ReconnectHandle;

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
    delay_ms: i64,
) -> SyncReturn<DartOpaque> {
    let reconnect_handle = ReconnectHandle::clone(&reconnect_handle);
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                #[allow(clippy::map_err_ignore)]
                let delay_ms = u32::try_from(delay_ms).map_err(|_| {
                    ArgumentError::new(delay_ms, "delayMs", "Expected u32")
                })?;

                reconnect_handle.reconnect_with_delay(delay_ms).await?;
                Ok::<_, Error>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
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
    starting_delay: i64,
    multiplier: f64,
    max_delay: i64,
    max_elapsed_time_ms: Option<i64>,
) -> SyncReturn<DartOpaque> {
    let reconnect_handle = ReconnectHandle::clone(&reconnect_handle);
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                #[allow(clippy::map_err_ignore)]
                let starting_delay =
                    u32::try_from(starting_delay).map_err(|_| {
                        ArgumentError::new(
                            starting_delay,
                            "startingDelayMs",
                            "Expected u32",
                        )
                    })?;
                #[allow(clippy::map_err_ignore)]
                let max_delay = u32::try_from(max_delay).map_err(|_| {
                    ArgumentError::new(max_delay, "maxDelay", "Expected u32")
                })?;
                let max_elapsed_time_ms = max_elapsed_time_ms
                    .map(|v| {
                        #[allow(clippy::map_err_ignore)]
                        u32::try_from(v).map_err(|_| {
                            ArgumentError::new(
                                v,
                                "maxElapsedTimeMs",
                                "Expected u32",
                            )
                        })
                    })
                    .transpose()?;

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
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

// -------------------------------------------------------------------

pub use crate::media::track::remote::Track as RemoteMediaTrack;

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
    let dart_opaque = unsafe { new_dart_opaque(track.get_track().handle()) };
    SyncReturn(dart_opaque)
}

/// Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
#[must_use]
pub fn remote_media_track_on_muted(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) -> SyncReturn<()> {
    track.on_muted(unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw())
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
        platform::Function::new(f.try_unwrap().unwrap().into_raw())
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
        platform::Function::new(f.try_unwrap().unwrap().into_raw())
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
            f.try_unwrap().unwrap().into_raw(),
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
) -> SyncReturn<u8> {
    SyncReturn(track.kind() as u8)
}

/// Returns this [`RemoteMediaTrack`]'s media source kind.
#[must_use]
pub fn remote_media_track_media_source_kind(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.media_source_kind() as u8)
}

/// Returns the current general [`MediaDirection`] of this [`RemoteMediaTrack`].
#[must_use]
pub fn remote_media_track_media_direction(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.media_direction() as u8)
}

// -------------------------------------------------------------------

pub use crate::room::RoomCloseReason;
impl ForeignClass for RoomCloseReason {}

/// Returns the [`RoomCloseReason`] from the address
/// [`ForeignClass`].
#[must_use]
pub fn room_close_reason_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<RoomCloseReason>> {
    SyncReturn(unsafe {
        RustOpaque::new(RoomCloseReason::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    })
}

/// Returns a close reason of a [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_close_reason_reason(
    room_close_reason: RustOpaque<RoomCloseReason>,
) -> SyncReturn<String> {
    SyncReturn(room_close_reason.reason())
}

/// Indicates whether a [`Room`] was closed by server.
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_close_reason_is_closed_by_server(
    room_close_reason: RustOpaque<RoomCloseReason>,
) -> SyncReturn<bool> {
    SyncReturn(room_close_reason.is_closed_by_server())
}

/// Indicates whether a [`Room`]'s close reason is considered as an error.
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_close_reason_is_err(
    room_close_reason: RustOpaque<RoomCloseReason>,
) -> SyncReturn<bool> {
    SyncReturn(room_close_reason.is_err())
}

// -------------------------------------------------------------------

pub use crate::room::RoomHandle;
use crate::room::{ConstraintsUpdateError, RoomJoinError};

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
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                room_handle.join(token).await?;
                Ok::<_, Traced<RoomJoinError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };

    SyncReturn(dart_opaque)
}

/// Updates this [`Room`]'s [`MediaStreamSettings`]. This affects all the
/// [`PeerConnection`]s in this [`Room`]. If [`MediaStreamSettings`] are
/// configured for some [`Room`], then this [`Room`] can only send media tracks
/// that correspond to these settings. [`MediaStreamSettings`] update will
/// change media tracks in all sending peers, so that might cause a new
/// [getUserMedia()][1] request to happen.
///
/// Media obtaining/injection errors are additionally fired to
/// `on_failed_local_media` callback.
///
/// If `stop_first` set to `true` then affected local `Tracks` will be
/// dropped before new [`MediaStreamSettings`] are applied. This is usually
/// required when changing video source device due to hardware limitations,
/// e.g. having an active track sourced from device `A` may hinder
/// [getUserMedia()][1] requests to device `B`.
///
/// `rollback_on_fail` option configures [`MediaStreamSettings`] update request
/// to automatically rollback to previous settings if new settings cannot be
/// applied.
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
    settings: RustOpaque<MediaStreamSettings>,
    stop_first: bool,
    rollback_on_fail: bool,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);
    let settings = MediaStreamSettings::clone(&settings);

    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                room_handle
                    .set_local_media_settings(
                        settings,
                        stop_first,
                        rollback_on_fail,
                    )
                    .await?;
                Ok::<_, ConstraintsUpdateError>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Mutes outbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_mute_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.mute_audio();
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Unmutes outbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_unmute_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);
    let fut = room_handle.unmute_audio();
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Enables outbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_enable_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_audio();
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Disables outbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_disable_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_audio();
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
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
    source_kind: Option<i64>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);
    let fut = room_handle.mute_video(dart_enum_try_into!(
        source_kind,
        "kind",
        "Invalid value"
    ));
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    Ok(SyncReturn(dart_opaque))
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
    source_kind: Option<i64>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.unmute_video(dart_enum_try_into!(
        source_kind,
        "kind",
        "Invalid value"
    ));
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    Ok(SyncReturn(dart_opaque))
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
    source_kind: Option<i64>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_video(dart_enum_try_into!(
        source_kind,
        "kind",
        "Invalid value"
    ));
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    Ok(SyncReturn(dart_opaque))
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
    source_kind: Option<i64>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_video(dart_enum_try_into!(
        source_kind,
        "kind",
        "Invalid value"
    ));
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    Ok(SyncReturn(dart_opaque))
}

/// Enables inbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_enable_remote_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_remote_audio();
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
}

/// Disables inbound audio in this [`Room`].
///
/// [`Room`]: crate::room::Room
#[must_use]
pub fn room_handle_disable_remote_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_remote_audio();
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(dart_opaque)
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
    source_kind: Option<i64>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_remote_video(dart_enum_try_into!(
        source_kind,
        "kind",
        "Invalid value"
    ));
    let dart_opaque = unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    };
    Ok(SyncReturn(dart_opaque))
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
    source_kind: Option<i64>,
) -> anyhow::Result<SyncReturn<DartOpaque>> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_remote_video(dart_enum_try_into!(
        source_kind,
        "kind",
        "Invalid value"
    ));
    Ok(SyncReturn(unsafe {
        new_dart_opaque(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_dart_future()
            .into_raw(),
        )
    }))
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
                platform::Function::new(cb.try_unwrap().unwrap().into_raw())
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
            platform::Function::new(cb.try_unwrap().unwrap().into_raw())
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
            platform::Function::new(cb.try_unwrap().unwrap().into_raw())
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
            platform::Function::new(cb.try_unwrap().unwrap().into_raw())
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
            platform::Function::new(cb.try_unwrap().unwrap().into_raw())
        })
        .map_err(|err| anyhow::anyhow!("{:?}", DartError::from(err)))?;
    Ok(SyncReturn(()))
}

// -------------------------------------------------------------------
