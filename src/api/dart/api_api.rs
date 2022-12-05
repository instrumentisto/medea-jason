pub use super::utils::{ApiWrapVec, IntoDartFuture, MyDartFuture};
use super::ForeignClass;
pub use crate::media::MediaStreamSettings;
use crate::platform::utils::dart_api::Dart_NewPersistentHandle_DL_Trampolined;
use flutter_rust_bridge::{DartOpaque, RustOpaque, SyncReturn};
use std::ptr;

use flutter_rust_bridge::DartSafe;
impl<T: DartSafe> ForeignClass for RustOpaque<T> {}

use std::panic::{RefUnwindSafe, UnwindSafe};

impl RefUnwindSafe for Jason {}
impl UnwindSafe for Jason {}

impl RefUnwindSafe for RoomHandle {}
impl UnwindSafe for RoomHandle {}

impl RefUnwindSafe for ConnectionHandle {}
impl UnwindSafe for ConnectionHandle {}

impl RefUnwindSafe for RemoteMediaTrack {}
impl UnwindSafe for RemoteMediaTrack {}

impl RefUnwindSafe for ReconnectHandle {}
impl UnwindSafe for ReconnectHandle {}

impl RefUnwindSafe for MediaManagerHandle {}
impl UnwindSafe for MediaManagerHandle {}

/// todo
pub fn touch(_track: RustOpaque<AudioTrackConstraints>) {}

// -------------------------------------------------------------------

// -------------------------------------------------------------------

pub fn connection_handle_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<ConnectionHandle>> {
    SyncReturn(unsafe {
        RustOpaque::new(ConnectionHandle::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    })
}

// -------------------------------------------------------------------

pub fn vec_local_tracks_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<ApiWrapVec<LocalMediaTrack>>> {
    SyncReturn(unsafe {
        RustOpaque::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
    })
}

pub fn vec_local_tracks_pop(
    vec: RustOpaque<ApiWrapVec<LocalMediaTrack>>,
) -> SyncReturn<Option<RustOpaque<LocalMediaTrack>>> {
    SyncReturn(vec.borrow_mut().pop().map(|v| RustOpaque::new(v)))
}

pub fn vec_media_display_info_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<ApiWrapVec<MediaDisplayInfo>>> {
    SyncReturn(unsafe {
        RustOpaque::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
    })
}

pub fn vec_media_display_info_pop(
    vec: RustOpaque<ApiWrapVec<MediaDisplayInfo>>,
) -> SyncReturn<Option<RustOpaque<MediaDisplayInfo>>> {
    SyncReturn(vec.borrow_mut().pop().map(|v| RustOpaque::new(v)))
}

pub fn vec_media_device_info_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<ApiWrapVec<MediaDeviceInfo>>> {
    SyncReturn(unsafe {
        RustOpaque::from_ptr(ptr::NonNull::new(ptr as _).unwrap())
    })
}

pub fn vec_media_device_info_pop(
    vec: RustOpaque<ApiWrapVec<MediaDeviceInfo>>,
) -> SyncReturn<Option<RustOpaque<MediaDeviceInfo>>> {
    SyncReturn(vec.borrow_mut().pop().map(|v| RustOpaque::new(v)))
}

// -------------------------------------------------------------------

pub use crate::media::AudioTrackConstraints;
/// Creates new [`AudioTrackConstraints`] with none constraints configured.
pub fn audio_track_constraints_new(
) -> SyncReturn<RustOpaque<AudioTrackConstraints>> {
    SyncReturn(RustOpaque::new(AudioTrackConstraints::new()))
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
pub fn audio_track_constraints_device_id(
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

#[cfg(feature = "mockable")]
pub use self::mock::ConnectionHandle;
#[cfg(not(feature = "mockable"))]
pub use crate::connection::ConnectionHandle;

impl ForeignClass for ConnectionHandle {}

/// Sets callback, invoked when this `Connection` will close.
pub fn connection_handle_on_close(
    connection: RustOpaque<ConnectionHandle>,
    f: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    connection
        .on_close(unsafe {
            platform::Function::new(f.try_unwrap().unwrap().into_raw())
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a new [`remote::Track`] is added to this
/// [`Connection`].
///
/// [`remote::Track`]: crate::media::track::remote::Track
/// [`Connection`]: crate::connection::Connection
pub fn connection_handle_on_remote_track_added(
    connection: RustOpaque<ConnectionHandle>,
    f: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    connection
        .on_remote_track_added(unsafe {
            platform::Function::new(f.try_unwrap().unwrap().into_raw())
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a connection quality score is updated by
/// a server.
pub fn connection_handle_on_quality_score_update(
    connection: RustOpaque<ConnectionHandle>,
    f: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    connection
        .on_quality_score_update(unsafe {
            platform::Function::new(f.try_unwrap().unwrap().into_raw())
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Returns remote `Member` ID.
pub fn connection_handle_get_remote_member_id(
    connection: RustOpaque<ConnectionHandle>,
) -> anyhow::Result<SyncReturn<String>> {
    Ok(SyncReturn(
        connection
            .get_remote_member_id()
            .map_err(|err| anyhow::anyhow!("{}", err))?,
    ))
}

/// Enables inbound audio in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
pub fn connection_handle_enable_remote_audio(
    connection: RustOpaque<ConnectionHandle>,
) -> SyncReturn<DartOpaque> {
    let fut = connection.enable_remote_audio();

    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<(), Traced<crate::connection::ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Disables inbound audio in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
pub fn connection_handle_disable_remote_audio(
    connection: RustOpaque<ConnectionHandle>,
) -> SyncReturn<DartOpaque> {
    let fut = connection.disable_remote_audio();

    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<(), Traced<crate::connection::ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Enables inbound video in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
pub fn connection_handle_enable_remote_video(
    connection: RustOpaque<ConnectionHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<DartOpaque> {
    let fut = connection.enable_remote_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<(), Traced<crate::connection::ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Disables inbound video in this [`ConnectionHandle`].
///
/// [`ConnectionHandle`]: crate::connection::ConnectionHandle
pub fn connection_handle_disable_remote_video(
    connection: RustOpaque<ConnectionHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<DartOpaque> {
    let fut = connection.disable_remote_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<(), Traced<crate::connection::ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(
        clippy::unused_self,
        clippy::missing_errors_doc,
        clippy::needless_pass_by_value,
        missing_copy_implementations
    )]

    use std::future::Future;

    use futures::future;
    use tracerr::Traced;

    use crate::{
        api::RemoteMediaTrack,
        connection::{
            ChangeMediaStateError, ConnectionHandle as CoreConnectionHandle,
            HandleDetachedError,
        },
        media::MediaSourceKind,
        platform,
    };

    /// Alias for a [`Result`] related to [`MediaState`] update functions.
    type ChangeMediaStateResult = Result<(), Traced<ChangeMediaStateError>>;

    #[derive(Debug)]
    pub struct ConnectionHandle(pub u8);

    impl From<CoreConnectionHandle> for ConnectionHandle {
        fn from(_: CoreConnectionHandle) -> Self {
            Self(0)
        }
    }

    impl ConnectionHandle {
        pub fn get_remote_member_id(
            &self,
        ) -> Result<String, Traced<HandleDetachedError>> {
            Err(tracerr::new!(HandleDetachedError))
        }

        pub fn on_close(
            &self,
            f: platform::Function<()>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            f.call0();
            Ok(())
        }

        pub fn on_remote_track_added(
            &self,
            f: platform::Function<RemoteMediaTrack>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            f.call1(RemoteMediaTrack(0));
            Ok(())
        }

        pub fn on_quality_score_update(
            &self,
            f: platform::Function<u8>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            f.call1(4);
            Ok(())
        }

        pub fn enable_remote_audio(
            &self,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }

        pub fn disable_remote_audio(
            &self,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }

        pub fn enable_remote_video(
            &self,
            _: Option<MediaSourceKind>,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::err(tracerr::new!(ChangeMediaStateError::Detached))
        }

        pub fn disable_remote_video(
            &self,
            _: Option<MediaSourceKind>,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }
    }
}

// -------------------------------------------------------------------

pub use crate::media::DeviceVideoTrackConstraints;
use crate::{
    api::Error,
    media::{FacingMode, InitLocalTracksError, MediaSourceKind},
    platform,
    room::ChangeMediaStateError,
};

/// Creates new [`DeviceVideoTrackConstraints`] with none constraints
/// configured.
pub fn device_video_track_constraints_new(
) -> SyncReturn<RustOpaque<DeviceVideoTrackConstraints>> {
    SyncReturn(RustOpaque::new(DeviceVideoTrackConstraints::new()))
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
pub fn device_video_track_constraints_device_id(
    constraints: RustOpaque<DeviceVideoTrackConstraints>,
    device_id: String,
) -> SyncReturn<RustOpaque<DeviceVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.device_id(device_id);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an exact [facingMode][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
pub fn device_video_track_constraints_exact_facing_mode(
    constraints: RustOpaque<DeviceVideoTrackConstraints>,
    facing_mode: u8,
) -> SyncReturn<RustOpaque<DeviceVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints
        .exact_facing_mode(FacingMode::try_from(facing_mode as i64).unwrap());
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an ideal [facingMode][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
pub fn device_video_track_constraints_ideal_facing_mode(
    constraints: RustOpaque<DeviceVideoTrackConstraints>,
    facing_mode: u8,
) -> SyncReturn<RustOpaque<DeviceVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints
        .ideal_facing_mode(FacingMode::try_from(facing_mode as i64).unwrap());
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an exact [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
pub fn device_video_track_constraints_exact_height(
    constraints: RustOpaque<DeviceVideoTrackConstraints>,
    exact_height: u32,
) -> SyncReturn<RustOpaque<DeviceVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.exact_height(exact_height);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an ideal [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
pub fn device_video_track_constraints_ideal_height(
    constraints: RustOpaque<DeviceVideoTrackConstraints>,
    ideal_height: u32,
) -> SyncReturn<RustOpaque<DeviceVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.ideal_height(ideal_height);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an exact [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
pub fn device_video_track_constraints_exact_width(
    constraints: RustOpaque<DeviceVideoTrackConstraints>,
    exact_width: u32,
) -> SyncReturn<RustOpaque<DeviceVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.exact_width(exact_width);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an ideal [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
pub fn device_video_track_constraints_ideal_width(
    constraints: RustOpaque<DeviceVideoTrackConstraints>,
    ideal_width: u32,
) -> SyncReturn<RustOpaque<DeviceVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.ideal_width(ideal_width);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets a range of a [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
pub fn device_video_track_constraints_height_in_range(
    constraints: RustOpaque<DeviceVideoTrackConstraints>,
    min: u32,
    max: u32,
) -> SyncReturn<RustOpaque<DeviceVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.height_in_range(min, max);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets a range of a [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
pub fn device_video_track_constraints_width_in_range(
    constraints: RustOpaque<DeviceVideoTrackConstraints>,
    min: u32,
    max: u32,
) -> SyncReturn<RustOpaque<DeviceVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.width_in_range(min, max);
    SyncReturn(RustOpaque::new(constraints))
}

// -------------------------------------------------------------------

pub use crate::media::DisplayVideoTrackConstraints;

/// Creates new [`DisplayVideoTrackConstraints`] with none constraints
/// configured.
pub fn display_video_track_constraints_new(
) -> SyncReturn<RustOpaque<DisplayVideoTrackConstraints>> {
    SyncReturn(RustOpaque::new(DisplayVideoTrackConstraints::new()))
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
pub fn display_video_track_constraints_device_id(
    constraints: RustOpaque<DisplayVideoTrackConstraints>,
    device_id: String,
) -> SyncReturn<RustOpaque<DisplayVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.device_id(device_id);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an exact [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
pub fn display_video_track_constraints_exact_height(
    constraints: RustOpaque<DisplayVideoTrackConstraints>,
    exact_height: u32,
) -> SyncReturn<RustOpaque<DisplayVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.exact_height(exact_height);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an ideal [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
pub fn display_video_track_constraints_ideal_height(
    constraints: RustOpaque<DisplayVideoTrackConstraints>,
    ideal_height: u32,
) -> SyncReturn<RustOpaque<DisplayVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.ideal_height(ideal_height);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an exact [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
pub fn display_video_track_constraints_exact_width(
    constraints: RustOpaque<DisplayVideoTrackConstraints>,
    exact_width: u32,
) -> SyncReturn<RustOpaque<DisplayVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.exact_width(exact_width);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an ideal [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
pub fn display_video_track_constraints_ideal_width(
    constraints: RustOpaque<DisplayVideoTrackConstraints>,
    ideal_width: u32,
) -> SyncReturn<RustOpaque<DisplayVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.ideal_width(ideal_width);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an ideal [frameRate][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
pub fn display_video_track_constraints_ideal_frame_rate(
    constraints: RustOpaque<DisplayVideoTrackConstraints>,
    ideal_frame_rate: u32,
) -> SyncReturn<RustOpaque<DisplayVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.ideal_frame_rate(ideal_frame_rate);
    SyncReturn(RustOpaque::new(constraints))
}

/// Sets an exact [frameRate][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
pub fn display_video_track_constraints_exact_frame_rate(
    constraints: RustOpaque<DisplayVideoTrackConstraints>,
    exact_frame_rate: u32,
) -> SyncReturn<RustOpaque<DisplayVideoTrackConstraints>> {
    let mut constraints = constraints.try_unwrap().unwrap();
    constraints.exact_frame_rate(exact_frame_rate);
    SyncReturn(RustOpaque::new(constraints))
}

// -------------------------------------------------------------------

#[cfg(feature = "mockable")]
pub use self::mock::Jason;
#[cfg(not(feature = "mockable"))]
pub use crate::jason::Jason;

/// Instantiates a new [`Jason`] interface to interact with this library.
pub fn jason_new() -> SyncReturn<RustOpaque<Jason>> {
    SyncReturn(RustOpaque::new(Jason::new()))
}

/// Creates a new [`Room`] and returns its [`RoomHandle`].
///
/// [`Room`]: crate::room::Room
pub fn jason_init_room(
    jason: RustOpaque<Jason>,
) -> SyncReturn<RustOpaque<RoomHandle>> {
    SyncReturn(RustOpaque::new(jason.init_room()))
}

/// Returns a [`MediaManagerHandle`].
pub fn jason_media_manager(
    jason: RustOpaque<Jason>,
) -> SyncReturn<RustOpaque<MediaManagerHandle>> {
    SyncReturn(RustOpaque::new(jason.media_manager()))
}

/// Closes the provided [`RoomHandle`].
pub fn jason_close_room(
    jason: RustOpaque<Jason>,
    room_to_delete: RustOpaque<RoomHandle>,
) -> SyncReturn<()> {
    jason.close_room(RoomHandle::clone(&room_to_delete));
    SyncReturn(())
}

/// Closes the provided [`RoomHandle`].
pub fn jason_dispose(jason: RustOpaque<Jason>) -> SyncReturn<()> {
    let jason = jason.try_unwrap().unwrap();
    jason.dispose();
    SyncReturn(())
}

// -------------------------------------------------------------------

#[cfg(feature = "mockable")]
pub use self::mock::LocalMediaTrack;
#[cfg(not(feature = "mockable"))]
pub use crate::media::track::local::LocalMediaTrack;

impl ForeignClass for LocalMediaTrack {}

pub fn local_media_track_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<LocalMediaTrack>> {
    SyncReturn(unsafe {
        RustOpaque::new(LocalMediaTrack::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    })
}

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
/// [`LocalMediaTrack`].
///
/// [`MediaStreamTrack`]: crate::platform::MediaStreamTrack
pub fn local_media_track_get_track(
    track: RustOpaque<LocalMediaTrack>,
) -> SyncReturn<DartOpaque> {
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(track.get_track().handle())
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
/// audio track, or a [`MediaKind::Video`] if it represents a video track.
///
/// [`MediaKind::Audio`]: crate::media::MediaKind::Audio
/// [`MediaKind::Video`]: crate::media::MediaKind::Video
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
pub fn local_media_track_media_source_kind(
    track: RustOpaque<LocalMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.media_source_kind() as u8)
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(clippy::unused_self, missing_copy_implementations)]

    use crate::{
        media::{
            track::local::LocalMediaTrack as CoreLocalMediaTrack, MediaKind,
            MediaSourceKind,
        },
        platform,
    };

    #[derive(Debug)]
    pub struct LocalMediaTrack(pub u8);

    impl From<CoreLocalMediaTrack> for LocalMediaTrack {
        fn from(_: CoreLocalMediaTrack) -> Self {
            Self(0)
        }
    }

    impl LocalMediaTrack {
        #[must_use]
        pub fn kind(&self) -> MediaKind {
            MediaKind::Video
        }

        #[must_use]
        pub fn media_source_kind(&self) -> MediaSourceKind {
            MediaSourceKind::Display
        }

        #[must_use]
        pub fn get_track(&self) -> platform::MediaStreamTrack {
            unreachable!()
        }
    }
}

// -------------------------------------------------------------------

#[cfg(feature = "mockable")]
pub use self::mock::MediaDeviceInfo;
#[cfg(not(feature = "mockable"))]
pub use crate::platform::MediaDeviceInfo;

/// Returns unique identifier of the represented device.
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
pub fn media_device_info_kind(
    media_device: RustOpaque<MediaDeviceInfo>,
) -> SyncReturn<u8> {
    SyncReturn(media_device.kind() as u8)
}

/// Returns label describing the represented device (for example "External USB
/// Webcam").
///
/// If the device has no associated label, then returns an empty string.
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
pub fn media_device_info_group_id(
    media_device: RustOpaque<MediaDeviceInfo>,
) -> SyncReturn<Option<String>> {
    SyncReturn(media_device.group_id())
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(missing_copy_implementations, clippy::unused_self)]

    use crate::media::MediaDeviceKind;

    #[derive(Debug)]
    pub struct MediaDeviceInfo(pub u8);

    impl MediaDeviceInfo {
        #[must_use]
        pub fn device_id(&self) -> String {
            String::from("MediaDeviceInfo.device_id")
        }

        #[must_use]
        pub fn kind(&self) -> MediaDeviceKind {
            MediaDeviceKind::AudioInput
        }

        #[must_use]
        pub fn label(&self) -> String {
            String::from("MediaDeviceInfo.label")
        }

        #[must_use]
        pub fn group_id(&self) -> Option<String> {
            Some(String::from("MediaDeviceInfo.group_id"))
        }
    }
}

// -------------------------------------------------------------------

#[cfg(feature = "mockable")]
pub use self::mock::MediaDisplayInfo;
#[cfg(not(feature = "mockable"))]
pub use crate::platform::MediaDisplayInfo;

/// Returns a unique identifier of the represented display.
pub fn media_display_info_device_id(
    media_display: RustOpaque<MediaDisplayInfo>,
) -> SyncReturn<String> {
    SyncReturn(media_display.device_id())
}

/// Returns a title describing the represented display.
pub fn media_display_info_title(
    media_display: RustOpaque<MediaDisplayInfo>,
) -> SyncReturn<Option<String>> {
    SyncReturn(media_display.title())
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(missing_copy_implementations, clippy::unused_self)]

    #[derive(Debug)]
    pub struct MediaDisplayInfo(pub u8);

    impl MediaDisplayInfo {
        #[must_use]
        pub fn device_id(&self) -> String {
            String::from("device_id")
        }

        #[must_use]
        pub fn title(&self) -> Option<String> {
            Some(String::from("title"))
        }
    }
}

// -------------------------------------------------------------------

use crate::media::{
    EnumerateDevicesError, EnumerateDisplaysError,
    InvalidOutputAudioDeviceIdError, MicVolumeError,
};

#[cfg(feature = "mockable")]
pub use self::mock::MediaManagerHandle;
#[cfg(not(feature = "mockable"))]
pub use crate::media::MediaManagerHandle;

/// Returns [`LocalMediaTrack`]s objects, built from the provided
/// [`MediaStreamSettings`].
///
/// [`LocalMediaTrack`]: crate::media::track::local::LocalMediaTrack
pub fn media_manager_handle_init_local_tracks(
    manager: RustOpaque<MediaManagerHandle>,
    caps: RustOpaque<MediaStreamSettings>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let caps = MediaStreamSettings::clone(&caps);
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                Ok::<RustOpaque<_>, Traced<InitLocalTracksError>>(
                    RustOpaque::new(ApiWrapVec::new(
                        manager.init_local_tracks(caps).await?,
                    )),
                )
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Returns a list of [`MediaDeviceInfo`] objects representing available media
/// input and devices, such as microphones, cameras, and so forth.
pub fn media_manager_handle_enumerate_devices(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                Ok::<RustOpaque<_>, Traced<EnumerateDevicesError>>(
                    RustOpaque::new(ApiWrapVec::new(
                        manager.enumerate_devices().await?,
                    )),
                )
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Returns a list of [`MediaDisplayInfo`] objects representing available
/// sources that can be used for screen capturing.

pub fn media_manager_handle_enumerate_displays(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                Ok::<RustOpaque<_>, Traced<EnumerateDisplaysError>>(
                    RustOpaque::new(manager.enumerate_displays().await?),
                )
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Switches the current output audio device to the device with the provided
/// `device_id`.

pub fn media_manager_handle_set_output_audio_id(
    manager: RustOpaque<MediaManagerHandle>,
    device_id: String,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                manager
                    .set_output_audio_id(device_id)
                    .await
                    .map_err(tracerr::map_from_and_wrap!())?;
                Ok::<_, Traced<InvalidOutputAudioDeviceIdError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Sets the microphone volume level in percents.

pub fn media_manager_handle_set_microphone_volume(
    manager: RustOpaque<MediaManagerHandle>,
    level: i64,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                manager
                    .set_microphone_volume(level)
                    .await
                    .map_err(tracerr::map_from_and_wrap!())?;
                Ok::<_, Traced<MicVolumeError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Indicates whether it's possible to access microphone volume settings.

pub fn media_manager_handle_microphone_volume_is_available(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move { manager.microphone_volume_is_available().await }
                .into_my_dart_future()
                .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Returns the current microphone volume level in percents.
pub fn media_manager_handle_microphone_volume(
    manager: RustOpaque<MediaManagerHandle>,
) -> SyncReturn<DartOpaque> {
    let manager = MediaManagerHandle::clone(&manager);

    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                let res = manager.microphone_volume().await;
                let res: Result<_, Traced<MicVolumeError>> =
                    res.map_err(tracerr::map_from_and_wrap!());
                res
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Subscribes onto the [`MediaManagerHandle`]'s `devicechange` event.

pub fn media_manager_handle_on_device_change(
    manager: RustOpaque<MediaManagerHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    let manager = MediaManagerHandle::clone(&manager);
    manager
        .on_device_change(unsafe {
            platform::Function::new(cb.try_unwrap().unwrap().into_raw())
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(
        clippy::needless_pass_by_value,
        clippy::unused_async,
        clippy::unused_self,
        missing_copy_implementations
    )]

    use dart_sys::Dart_Handle;
    use futures::future;
    use tracerr::Traced;

    use crate::{
        api::{
            dart::{
                utils::{DartFuture, DartResult, IntoDartFuture},
                DartError,
            },
            LocalMediaTrack, MediaDeviceInfo, MediaDisplayInfo,
            MediaStreamSettings,
        },
        media::{
            EnumerateDevicesError, EnumerateDisplaysError, HandleDetachedError,
            InitLocalTracksError, InvalidOutputAudioDeviceIdError,
            MicVolumeError,
        },
        platform,
    };

    #[derive(Clone, Debug)]
    pub struct MediaManagerHandle(pub u8);

    #[allow(clippy::missing_errors_doc, clippy::unused_async)]
    impl MediaManagerHandle {
        pub async fn enumerate_devices(
            &self,
        ) -> Result<Vec<MediaDeviceInfo>, Traced<EnumerateDevicesError>>
        {
            Ok(vec![
                MediaDeviceInfo(0),
                MediaDeviceInfo(0),
                MediaDeviceInfo(0),
            ])
        }

        pub async fn enumerate_displays(
            &self,
        ) -> Result<Vec<MediaDisplayInfo>, Traced<EnumerateDisplaysError>>
        {
            Ok(vec![MediaDisplayInfo(0)])
        }

        pub async fn init_local_tracks(
            &self,
            _caps: MediaStreamSettings,
        ) -> Result<Vec<LocalMediaTrack>, Traced<InitLocalTracksError>>
        {
            Ok(vec![
                LocalMediaTrack(0),
                LocalMediaTrack(0),
                LocalMediaTrack(0),
            ])
        }

        pub async fn set_output_audio_id(
            &self,
            _device_id: String,
        ) -> Result<(), Traced<InvalidOutputAudioDeviceIdError>> {
            Ok(())
        }

        pub async fn set_microphone_volume(
            &self,
            _: i64,
        ) -> Result<(), Traced<MicVolumeError>> {
            Ok(())
        }

        pub async fn microphone_volume_is_available(
            &self,
        ) -> Result<bool, Traced<HandleDetachedError>> {
            Ok(true)
        }

        pub async fn microphone_volume(
            &self,
        ) -> Result<i64, Traced<MicVolumeError>> {
            Ok(50)
        }

        pub fn on_device_change(
            &self,
            cb: platform::Function<()>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            cb.call0();
            Ok(())
        }
    }

    pub fn returns_local_media_init_exception(
        cause: RustOpaque<MediaManagerHandleDH>,
    ) -> DartResult {
        let cause = platform::Error::from_handle(cause);
        let err = tracerr::new!(InitLocalTracksError::GetUserMediaFailed(
            platform::GetUserMediaError::Audio(cause).into()
        ));
        DartError::from(err).into()
    }

    pub fn returns_future_with_local_media_init_exception(
        cause: RustOpaque<MediaManagerHandleDH>,
    ) -> DartFuture<Result<(), Traced<InitLocalTracksError>>> {
        let cause = platform::Error::from_handle(cause);
        let err = tracerr::new!(InitLocalTracksError::GetDisplayMediaFailed(
            cause.into()
        ));

        future::err(err).into_my_dart_future()
    }

    pub fn returns_enumerate_devices_exception(
        cause: RustOpaque<MediaManagerHandleDH>,
    ) -> DartResult {
        let cause = platform::Error::from_handle(cause);
        DartError::from(tracerr::new!(EnumerateDevicesError::from(cause)))
            .into()
    }

    pub fn returns_future_enumerate_devices_exception(
        cause: RustOpaque<MediaManagerHandleDH>,
    ) -> DartFuture<Result<(), Traced<EnumerateDevicesError>>> {
        let cause = platform::Error::from_handle(cause);
        let err = tracerr::new!(EnumerateDevicesError::from(cause));

        future::err(err).into_my_dart_future()
    }
}

// -------------------------------------------------------------------

/// Creates new [`MediaStreamSettings`] with none constraints configured.
pub fn media_stream_settings_new() -> SyncReturn<RustOpaque<MediaStreamSettings>>
{
    SyncReturn(RustOpaque::new(MediaStreamSettings::new()))
}

/// Specifies a nature and settings of an audio [`MediaStreamTrack`].
///
/// [`MediaStreamTrack`]: crate::platform::MediaStreamTrack
pub fn media_stream_settings_audio(
    media_stream_settings: RustOpaque<MediaStreamSettings>,
    constraints: RustOpaque<AudioTrackConstraints>,
) -> SyncReturn<RustOpaque<MediaStreamSettings>> {
    let mut media_stream_settings = media_stream_settings.try_unwrap().unwrap();
    media_stream_settings.audio(AudioTrackConstraints::clone(&constraints));
    SyncReturn(RustOpaque::new(media_stream_settings))
}

/// Set constraints for obtaining a local video sourced from a media device.
pub fn media_stream_settings_device_video(
    media_stream_settings: RustOpaque<MediaStreamSettings>,
    constraints: RustOpaque<DeviceVideoTrackConstraints>,
) -> SyncReturn<RustOpaque<MediaStreamSettings>> {
    let mut media_stream_settings = media_stream_settings.try_unwrap().unwrap();
    media_stream_settings
        .device_video(DeviceVideoTrackConstraints::clone(&constraints));
    SyncReturn(RustOpaque::new(media_stream_settings))
}

/// Set constraints for capturing a local video from user's display.
pub fn media_stream_settings_display_video(
    media_stream_settings: RustOpaque<MediaStreamSettings>,
    constraints: RustOpaque<DisplayVideoTrackConstraints>,
) -> SyncReturn<RustOpaque<MediaStreamSettings>> {
    let mut media_stream_settings = media_stream_settings.try_unwrap().unwrap();
    media_stream_settings
        .display_video(DisplayVideoTrackConstraints::clone(&constraints));
    SyncReturn(RustOpaque::new(media_stream_settings))
}

// -------------------------------------------------------------------

#[cfg(feature = "mockable")]
pub use self::mock::ReconnectHandle;
use crate::api::dart::utils::ArgumentError;
#[cfg(not(feature = "mockable"))]
pub use crate::rpc::ReconnectHandle;

impl ForeignClass for ReconnectHandle {}

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
pub fn reconnect_handle_reconnect_with_delay(
    reconnect_handle: RustOpaque<ReconnectHandle>,
    delay_ms: i64,
) -> SyncReturn<DartOpaque> {
    let reconnect_handle = ReconnectHandle::clone(&reconnect_handle);
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                #[allow(clippy::map_err_ignore)]
                let delay_ms = u32::try_from(delay_ms).map_err(|_| {
                    ArgumentError::new(delay_ms, "delayMs", "Expected u32")
                })?;

                reconnect_handle.reconnect_with_delay(delay_ms).await?;
                Ok::<_, Error>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
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
/// reconnect_handle might cause a busy loop, so it's not recommended.
///
/// Max elapsed time can be limited with an optional `max_elapsed_time_ms`
/// argument.
///
/// If the [`Room`] is already reconnecting then new reconnection attempt won't
/// be performed. Instead, it will wait for the first reconnection attempt
/// result and use it here.
///
/// [`Room`]: crate::room::Room
pub fn reconnect_handle_reconnect_with_backoff(
    reconnect_handle: RustOpaque<ReconnectHandle>,
    starting_delay: i64,
    multiplier: f64,
    max_delay: u32,
    max_elapsed_time_ms: Option<u32>,
) -> SyncReturn<DartOpaque> {
    let reconnect_handle = ReconnectHandle::clone(&reconnect_handle);
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
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
                Ok::<_, Error>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(
        clippy::missing_errors_doc,
        clippy::unused_async,
        missing_copy_implementations
    )]

    use dart_sys::Dart_Handle;
    use futures::future;
    use tracerr::{Trace, Traced};

    use crate::{
        api::{
            dart::utils::{
                DartError, DartFuture, DartResult, IntoDartFuture as _,
            },
            err::{RpcClientException, RpcClientExceptionKind},
        },
        platform,
        rpc::{ReconnectError, ReconnectHandle as CoreReconnectHandle},
    };

    #[derive(Clone, Debug)]
    pub struct ReconnectHandle(pub u8);

    impl From<CoreReconnectHandle> for ReconnectHandle {
        fn from(_: CoreReconnectHandle) -> Self {
            Self(0)
        }
    }

    impl ReconnectHandle {
        pub async fn reconnect_with_delay(
            &self,
            _delay_ms: u32,
        ) -> Result<(), Traced<ReconnectError>> {
            Ok(())
        }

        pub async fn reconnect_with_backoff(
            &self,
            _starting_delay_ms: u32,
            _multiplier: f64,
            _max_delay: u32,
            _max_elapsed_time_ms: Option<u32>,
        ) -> Result<(), Traced<ReconnectError>> {
            Ok(())
        }
    }

    pub fn returns_rpc_client_exception(cause: Dart_Handle) -> DartResult {
        let err = RpcClientException::new(
            RpcClientExceptionKind::ConnectionLost,
            "RpcClientException::ConnectionLost",
            Some(platform::Error::from_handle(cause)),
            Trace::new(vec![tracerr::new_frame!()]),
        );

        DartError::from(err).into()
    }

    pub fn returns_future_rpc_client_exception(
        cause: Dart_Handle,
    ) -> SyncReturn<DartOpaque> {
        let err = RpcClientException::new(
            RpcClientExceptionKind::SessionFinished,
            "RpcClientException::SessionFinished",
            Some(platform::Error::from_handle(cause)),
            Trace::new(vec![tracerr::new_frame!()]),
        );

        let persistent_handle = unsafe {
            Dart_NewPersistentHandle_DL_Trampolined(
                future::err(err.into()).into_my_dart_future().into_raw(),
            )
        };
        SyncReturn(DartOpaque::new(persistent_handle, 0))
    }
}

// -------------------------------------------------------------------

#[cfg(feature = "mockable")]
pub use self::mock::RemoteMediaTrack;
#[cfg(not(feature = "mockable"))]
pub use crate::media::track::remote::Track as RemoteMediaTrack;

impl ForeignClass for RemoteMediaTrack {}

pub fn remote_media_track_from_ptr(
    ptr: usize,
) -> SyncReturn<RustOpaque<RemoteMediaTrack>> {
    SyncReturn(unsafe {
        RustOpaque::new(RemoteMediaTrack::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        ))
    })
}

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of track
/// [`RemoteMediaTrack`].
///
/// [`MediaStreamTrack`]: platform::MediaStreamTrack
pub fn remote_media_track_get_track(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<DartOpaque> {
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(track.get_track().handle())
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Sets callback to invoke when track [`RemoteMediaTrack`] is muted.
pub fn remote_media_track_on_muted(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) -> SyncReturn<()> {
    track.on_muted(unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw())
    });
    SyncReturn(())
}

/// Sets callback to invoke when track [`RemoteMediaTrack`] is unmuted.
pub fn remote_media_track_on_unmuted(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) -> SyncReturn<()> {
    track.on_unmuted(unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw())
    });
    SyncReturn(())
}

/// Sets callback to invoke when track [`RemoteMediaTrack`] is stopped.
pub fn remote_media_track_on_stopped(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) -> SyncReturn<()> {
    track.on_stopped(unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw())
    });
    SyncReturn(())
}

/// Sets callback to invoke whenever track [`RemoteMediaTrack`]'s general
/// [`MediaDirection`] is changed.
pub fn remote_media_track_on_media_direction_changed(
    track: RustOpaque<RemoteMediaTrack>,
    f: DartOpaque,
) -> SyncReturn<()> {
    track.on_media_direction_changed(unsafe {
        platform::Function::new(f.try_unwrap().unwrap().into_raw())
    });
    SyncReturn(())
}

/// Indicate whether track [`RemoteMediaTrack`] is muted.
pub fn remote_media_track_muted(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<bool> {
    SyncReturn(track.muted())
}

/// Returns track [`RemoteMediaTrack`]'s kind (audio/video).
pub fn remote_media_track_kind(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.kind() as u8)
}

/// Returns track [`RemoteMediaTrack`]'s media source kind.
pub fn remote_media_track_media_source_kind(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.media_source_kind() as u8)
}

/// Returns the current general [`MediaDirection`] of track
/// [`RemoteMediaTrack`].
pub fn remote_media_track_media_direction(
    track: RustOpaque<RemoteMediaTrack>,
) -> SyncReturn<u8> {
    SyncReturn(track.media_direction() as u8)
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(
        clippy::unused_self,
        clippy::needless_pass_by_value,
        missing_copy_implementations
    )]

    use crate::{
        api,
        media::{
            track::remote::Track as CoreRemoteMediaTrack, MediaDirection,
            MediaKind, MediaSourceKind,
        },
        platform,
    };

    #[derive(Clone, Debug)]
    pub struct RemoteMediaTrack(pub u8);

    impl From<CoreRemoteMediaTrack> for RemoteMediaTrack {
        fn from(_: CoreRemoteMediaTrack) -> Self {
            Self(0)
        }
    }

    impl RemoteMediaTrack {
        #[must_use]
        pub fn enabled(&self) -> bool {
            true
        }

        #[must_use]
        pub fn kind(&self) -> MediaKind {
            MediaKind::Video
        }

        #[must_use]
        pub fn media_source_kind(&self) -> MediaSourceKind {
            MediaSourceKind::Device
        }

        #[must_use]
        pub fn muted(&self) -> bool {
            false
        }

        pub fn on_enabled(&self, cb: platform::Function<()>) {
            cb.call0();
        }

        pub fn on_disabled(&self, cb: platform::Function<()>) {
            cb.call0();
        }

        pub fn on_muted(&self, cb: platform::Function<()>) {
            cb.call0();
        }

        pub fn on_unmuted(&self, cb: platform::Function<()>) {
            cb.call0();
        }

        pub fn on_stopped(&self, cb: platform::Function<()>) {
            cb.call0();
        }

        #[allow(unused_qualifications)]
        pub fn on_media_direction_changed(
            &self,
            cb: platform::Function<api::MediaDirection>,
        ) {
            cb.call1(api::MediaDirection::SendRecv);
        }

        #[must_use]
        pub fn media_direction(&self) -> MediaDirection {
            MediaDirection::SendRecv
        }

        #[must_use]
        pub fn get_track(&self) -> platform::MediaStreamTrack {
            unreachable!()
        }
    }
}

// -------------------------------------------------------------------

pub use crate::room::RoomCloseReason;
impl ForeignClass for RoomCloseReason {}

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
pub fn room_close_reason_reason(
    room_close_reason: RustOpaque<RoomCloseReason>,
) -> SyncReturn<String> {
    SyncReturn(room_close_reason.reason())
}

/// Indicates whether a [`Room`] was closed by server.
///
/// [`Room`]: crate::room::Room
pub fn room_close_reason_is_closed_by_server(
    room_close_reason: RustOpaque<RoomCloseReason>,
) -> SyncReturn<bool> {
    SyncReturn(room_close_reason.is_closed_by_server())
}

/// Indicates whether a [`Room`]'s close reason is considered as an error.
///
/// [`Room`]: crate::room::Room
pub fn room_close_reason_is_err(
    room_close_reason: RustOpaque<RoomCloseReason>,
) -> SyncReturn<bool> {
    SyncReturn(room_close_reason.is_err())
}

// -------------------------------------------------------------------

#[cfg(feature = "mockable")]
pub use self::mock::RoomHandle;
#[cfg(not(feature = "mockable"))]
pub use crate::room::RoomHandle;
use crate::room::{ConstraintsUpdateError, RoomJoinError};

/// Connects to a media server and joins the [`Room`] with the provided
/// authorization `token`.
///
/// Authorization token has a fixed format:
/// `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
/// (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
///
/// [`Room`]: crate::room::Room
pub fn room_handle_join(
    room_handle: RustOpaque<RoomHandle>,
    token: String,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                room_handle.join(token).await?;
                Ok::<_, Traced<RoomJoinError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };

    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Updates room_handle [`Room`]'s [`MediaStreamSettings`]. room_handle affects
/// all the [`PeerConnection`]s in room_handle [`Room`]. If
/// [`MediaStreamSettings`] are configured for some [`Room`], then room_handle
/// [`Room`] can only send media tracks that correspond to these settings.
/// [`MediaStreamSettings`] update will change media tracks in all sending
/// peers, so that might cause a new [getUserMedia()][1] request to happen.
///
/// Media obtaining/injection errors are additionally fired to
/// `on_failed_local_media` callback.
///
/// If `stop_first` set to `true` then affected local `Tracks` will be
/// dropped before new [`MediaStreamSettings`] are applied. room_handle is
/// usually required when changing video source device due to hardware
/// limitations, e.g. having an active track sourced from device `A` may hinder
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

pub fn room_handle_set_local_media_settings(
    room_handle: RustOpaque<RoomHandle>,
    settings: RustOpaque<MediaStreamSettings>,
    stop_first: bool,
    rollback_on_fail: bool,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);
    let settings = MediaStreamSettings::clone(&settings);

    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
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
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Mutes outbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_mute_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.mute_audio();
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Unmutes outbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_unmute_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);
    let fut = room_handle.unmute_audio();
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Enables outbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_enable_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_audio();
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Disables outbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_disable_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_audio();
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Mutes outbound video in room_handle [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// [`Room`]: crate::room::Room

pub fn room_handle_mute_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.mute_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Unmutes outbound video in room_handle [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// [`Room`]: crate::room::Room

pub fn room_handle_unmute_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.unmute_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Enables outbound video.
///
/// Affects only video with specific [`MediaSourceKind`] if specified.

pub fn room_handle_enable_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Disables outbound video.
///
/// Affects only video with specific [`MediaSourceKind`] if specified.

pub fn room_handle_disable_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Enables inbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_enable_remote_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_remote_audio();
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Disables inbound audio in room_handle [`Room`].
///
/// [`Room`]: crate::room::Room

pub fn room_handle_disable_remote_audio(
    room_handle: RustOpaque<RoomHandle>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_remote_audio();
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Enables inbound video in room_handle [`Room`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`Room`]: crate::room::Room

pub fn room_handle_enable_remote_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.enable_remote_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Disables inbound video in room_handle [`Room`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// [`Room`]: crate::room::Room

pub fn room_handle_disable_remote_video(
    room_handle: RustOpaque<RoomHandle>,
    source_kind: Option<u8>,
) -> SyncReturn<DartOpaque> {
    let room_handle = RoomHandle::clone(&room_handle);

    let fut = room_handle.disable_remote_video(
        source_kind.map(|v| MediaSourceKind::try_from(v as i64).unwrap()),
    );
    let persistent_handle = unsafe {
        Dart_NewPersistentHandle_DL_Trampolined(
            async move {
                fut.await?;
                Ok::<_, Traced<ChangeMediaStateError>>(())
            }
            .into_my_dart_future()
            .into_raw(),
        )
    };
    SyncReturn(DartOpaque::new(persistent_handle, 0))
}

/// Sets callback, invoked when a new [`Connection`] with some remote `Peer`
/// is established.
///
/// [`Connection`]: crate::connection::Connection

pub fn room_handle_on_new_connection(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    Ok(SyncReturn(
        room_handle
            .on_new_connection(unsafe {
                platform::Function::new(cb.try_unwrap().unwrap().into_raw())
            })
            .map_err(|err| anyhow::anyhow!("{}", err))?,
    ))
}

/// Sets callback, invoked on room_handle [`Room`] close, providing a
/// [`RoomCloseReason`].
///
/// [`Room`]: crate::room::Room
/// [`RoomCloseReason`]: crate::room::RoomCloseReason

pub fn room_handle_on_close(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_close(unsafe {
            platform::Function::new(cb.try_unwrap().unwrap().into_raw())
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a new [`LocalMediaTrack`] is added to
/// room_handle [`Room`].
///
/// room_handle might happen in such cases:
/// 1. Media server initiates a media request.
/// 2. `enable_audio`/`enable_video` is called.
/// 3. [`MediaStreamSettings`] updated via `set_local_media_settings`.
///
/// [`Room`]: crate::room::Room
/// [`MediaStreamSettings`]: crate::media::MediaStreamSettings
/// [`LocalMediaTrack`]: crate::media::track::local::LocalMediaTrack

pub fn room_handle_on_local_track(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_local_track(unsafe {
            platform::Function::new(cb.try_unwrap().unwrap().into_raw())
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Sets callback, invoked when a connection with server is lost.

pub fn room_handle_on_connection_loss(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_connection_loss(unsafe {
            platform::Function::new(cb.try_unwrap().unwrap().into_raw())
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

/// Sets callback, invoked on local media acquisition failures.
pub fn room_handle_on_failed_local_media(
    room_handle: RustOpaque<RoomHandle>,
    cb: DartOpaque,
) -> anyhow::Result<SyncReturn<()>> {
    room_handle
        .on_failed_local_media(unsafe {
            platform::Function::new(cb.try_unwrap().unwrap().into_raw())
        })
        .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(SyncReturn(()))
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(
        clippy::needless_pass_by_value,
        clippy::unused_async,
        clippy::unused_self,
        missing_copy_implementations
    )]

    use std::future::Future;

    use futures::future;
    use tracerr::Traced;

    use crate::{
        api::{
            dart::utils::DartError, ConnectionHandle, LocalMediaTrack,
            MediaStreamSettings, ReconnectHandle,
        },
        media::MediaSourceKind,
        peer::{LocalMediaError, TracksRequestError, UpdateLocalStreamError},
        platform,
        room::{
            ChangeMediaStateError, ConstraintsUpdateError, HandleDetachedError,
            RoomCloseReason, RoomJoinError,
        },
        rpc::{ClientDisconnect, CloseReason, ConnectionInfo},
    };

    /// Alias for a [`Result`] related to [`MediaState`] update functions.
    type ChangeMediaStateResult = Result<(), Traced<ChangeMediaStateError>>;

    #[derive(Clone, Debug)]
    pub struct RoomHandle(pub u8);

    #[allow(clippy::missing_errors_doc, clippy::unused_async)]
    impl RoomHandle {
        pub fn on_new_connection(
            &self,
            cb: platform::Function<ConnectionHandle>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            cb.call1(ConnectionHandle(0));
            Ok(())
        }

        pub fn on_close(
            &self,
            cb: platform::Function<RoomCloseReason>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            cb.call1(RoomCloseReason::new(CloseReason::ByClient {
                is_err: true,
                reason: ClientDisconnect::RpcClientUnexpectedlyDropped,
            }));
            Ok(())
        }

        pub fn on_local_track(
            &self,
            cb: platform::Function<LocalMediaTrack>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            cb.call1(LocalMediaTrack(0));
            Ok(())
        }

        pub fn on_connection_loss(
            &self,
            cb: platform::Function<ReconnectHandle>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            cb.call1(ReconnectHandle(0));
            Ok(())
        }

        pub async fn join(
            &self,
            token: String,
        ) -> Result<(), Traced<RoomJoinError>> {
            token
                .parse::<ConnectionInfo>()
                .map_err(tracerr::map_from_and_wrap!())
                .map(drop)
        }

        pub fn on_failed_local_media(
            &self,
            cb: platform::Function<DartError>,
        ) -> Result<(), Traced<HandleDetachedError>> {
            cb.call1(
                tracerr::new!(LocalMediaError::UpdateLocalStreamError(
                    UpdateLocalStreamError::InvalidLocalTracks(
                        TracksRequestError::NoTracks,
                    ),
                ))
                .into(),
            );
            Ok(())
        }

        pub async fn set_local_media_settings(
            &self,
            _settings: MediaStreamSettings,
            _stop_first: bool,
            _rollback_on_fail: bool,
        ) -> Result<(), ConstraintsUpdateError> {
            Ok(())
        }

        pub fn mute_audio(
            &self,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }

        pub fn unmute_audio(
            &self,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }

        pub fn enable_audio(
            &self,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }

        pub fn disable_audio(
            &self,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }

        pub fn mute_video(
            &self,
            source_kind: Option<MediaSourceKind>,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            assert_eq!(source_kind, None);
            future::ok(())
        }

        pub fn unmute_video(
            &self,
            source_kind: Option<MediaSourceKind>,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            assert_eq!(source_kind, Some(MediaSourceKind::Display));
            future::ok(())
        }

        pub fn enable_video(
            &self,
            source_kind: Option<MediaSourceKind>,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            assert_eq!(source_kind, Some(MediaSourceKind::Device));
            future::ok(())
        }

        pub fn disable_video(
            &self,
            source_kind: Option<MediaSourceKind>,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            assert_eq!(source_kind, Some(MediaSourceKind::Display));
            future::ok(())
        }

        pub fn enable_remote_audio(
            &self,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }

        pub fn disable_remote_audio(
            &self,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }

        pub fn enable_remote_video(
            &self,
            _: Option<MediaSourceKind>,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::err(tracerr::new!(ChangeMediaStateError::Detached))
        }

        pub fn disable_remote_video(
            &self,
            _: Option<MediaSourceKind>,
        ) -> impl Future<Output = ChangeMediaStateResult> + 'static {
            future::ok(())
        }
    }
}

// -------------------------------------------------------------------
