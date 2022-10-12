use std::{os::raw::c_char, ptr};

use dart_sys::Dart_Handle;
use tracerr::Traced;

use crate::{
    api::{
        dart_string_into_rust,
        utils::{DartError, DartResult},
    },
    media::{
        EnumerateDevicesError, EnumerateDisplaysError, HandleDetachedError,
        InitLocalTracksError, InvalidOutputAudioDeviceIdError, MicVolumeError,
    },
    platform,
};

use super::{
    media_stream_settings::MediaStreamSettings,
    propagate_panic,
    utils::{DartFuture, IntoDartFuture, PtrArray},
    ForeignClass, LocalMediaTrack, MediaDeviceInfo, MediaDisplayInfo,
};

#[cfg(feature = "mockable")]
pub use self::mock::MediaManagerHandle;
#[cfg(not(feature = "mockable"))]
pub use crate::media::MediaManagerHandle;

impl ForeignClass for MediaManagerHandle {}

/// Returns [`LocalMediaTrack`]s objects, built from the provided
/// [`MediaStreamSettings`].
///
/// [`LocalMediaTrack`]: crate::media::track::local::LocalMediaTrack
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__init_local_tracks(
    this: ptr::NonNull<MediaManagerHandle>,
    caps: ptr::NonNull<MediaStreamSettings>,
) -> DartFuture<Result<PtrArray<LocalMediaTrack>, Traced<InitLocalTracksError>>>
{
    propagate_panic(move || {
        let this = this.as_ref().clone();
        let caps = caps.as_ref().clone();

        async move { Ok(PtrArray::new(this.init_local_tracks(caps).await?)) }
            .into_dart_future()
    })
}

/// Returns a list of [`MediaDeviceInfo`] objects representing available media
/// input and devices, such as microphones, cameras, and so forth.
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__enumerate_devices(
    this: ptr::NonNull<MediaManagerHandle>,
) -> DartFuture<
    Result<PtrArray<MediaDeviceInfo>, Traced<EnumerateDevicesError>>,
> {
    propagate_panic(move || {
        let this = this.as_ref().clone();

        async move { Ok(PtrArray::new(this.enumerate_devices().await?)) }
            .into_dart_future()
    })
}

/// Returns a list of [`MediaDisplayInfo`] objects representing available
/// sources that can be used for screen capturing.
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__enumerate_displays(
    this: ptr::NonNull<MediaManagerHandle>,
) -> DartFuture<
    Result<PtrArray<MediaDisplayInfo>, Traced<EnumerateDisplaysError>>,
> {
    propagate_panic(move || {
        let this = this.as_ref().clone();

        async move { Ok(PtrArray::new(this.enumerate_displays().await?)) }
            .into_dart_future()
    })
}

/// Switches the current output audio device to the device with the provided
/// `device_id`.
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__set_output_audio_id(
    this: ptr::NonNull<MediaManagerHandle>,
    device_id: ptr::NonNull<c_char>,
) -> DartFuture<Result<(), Traced<InvalidOutputAudioDeviceIdError>>> {
    propagate_panic(move || {
        let this = this.as_ref().clone();
        let device_id = dart_string_into_rust(device_id);

        async move {
            this.set_output_audio_id(device_id)
                .await
                .map_err(tracerr::map_from_and_wrap!())?;
            Ok(())
        }
        .into_dart_future()
    })
}

/// Sets the microphone volume level in percents.
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__set_microphone_volume(
    this: ptr::NonNull<MediaManagerHandle>,
    level: i64,
) -> DartFuture<Result<(), Traced<MicVolumeError>>> {
    propagate_panic(move || {
        let this = this.as_ref().clone();

        async move {
            this.set_microphone_volume(level)
                .await
                .map_err(tracerr::map_from_and_wrap!())?;
            Ok(())
        }
        .into_dart_future()
    })
}

/// Indicates whether it's possible to access microphone volume settings.
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__microphone_volume_is_available(
    this: ptr::NonNull<MediaManagerHandle>,
) -> DartFuture<Result<bool, Traced<HandleDetachedError>>> {
    propagate_panic(move || {
        let this = this.as_ref().clone();

        async move { this.microphone_volume_is_available().await }
            .into_dart_future()
    })
}

/// Returns the current microphone volume level in percents.
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__microphone_volume(
    this: ptr::NonNull<MediaManagerHandle>,
) -> DartFuture<Result<i64, Traced<MicVolumeError>>> {
    propagate_panic(move || {
        let this = this.as_ref().clone();

        async move {
            this.microphone_volume()
                .await
                .map_err(tracerr::map_from_and_wrap!())
        }
        .into_dart_future()
    })
}

/// Subscribes onto the [`MediaManagerHandle`]'s `devicechange` event.
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__on_device_change(
    this: ptr::NonNull<MediaManagerHandle>,
    cb: Dart_Handle,
) -> DartResult {
    propagate_panic(move || {
        let this = this.as_ref();
        this.on_device_change(platform::Function::new(cb))
            .map_err(DartError::from)
            .into()
    })
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__free(
    this: ptr::NonNull<MediaManagerHandle>,
) {
    propagate_panic(move || {
        drop(MediaManagerHandle::from_ptr(this));
    });
}

#[cfg(feature = "mockable")]
#[allow(clippy::unused_async)]
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

    #[allow(clippy::missing_errors_doc)]
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

    #[no_mangle]
    pub unsafe extern "C" fn returns_local_media_init_exception(
        cause: Dart_Handle,
    ) -> DartResult {
        let cause = platform::Error::from_handle(cause);
        let err = tracerr::new!(InitLocalTracksError::GetUserMediaFailed(
            platform::GetUserMediaError::Audio(cause).into()
        ));
        DartError::from(err).into()
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_future_with_local_media_init_exception(
        cause: Dart_Handle,
    ) -> DartFuture<Result<(), Traced<InitLocalTracksError>>> {
        let cause = platform::Error::from_handle(cause);
        let err = tracerr::new!(InitLocalTracksError::GetDisplayMediaFailed(
            cause.into()
        ));

        future::err(err).into_dart_future()
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_enumerate_devices_exception(
        cause: Dart_Handle,
    ) -> DartResult {
        let cause = platform::Error::from_handle(cause);
        DartError::from(tracerr::new!(EnumerateDevicesError::from(cause)))
            .into()
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_future_enumerate_devices_exception(
        cause: Dart_Handle,
    ) -> DartFuture<Result<(), Traced<EnumerateDevicesError>>> {
        let cause = platform::Error::from_handle(cause);
        let err = tracerr::new!(EnumerateDevicesError::from(cause));

        future::err(err).into_dart_future()
    }
}
