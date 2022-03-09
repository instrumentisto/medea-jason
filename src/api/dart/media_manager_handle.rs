use std::{os::raw::c_char, ptr};

use tracerr::Traced;

use crate::{
    api::c_str_into_string,
    media::{
        EnumerateDevicesError, InitLocalTracksError,
        InvalidOutputAudioDeviceIdError,
    },
};

use super::{
    media_stream_settings::MediaStreamSettings,
    utils::{DartFuture, IntoDartFuture, PtrArray},
    ForeignClass, LocalMediaTrack, MediaDeviceInfo,
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
    let this = this.as_ref().clone();
    let caps = caps.as_ref().clone();

    async move { Ok(PtrArray::new(this.init_local_tracks(caps).await?)) }
        .into_dart_future()
}

/// Returns a list of [`MediaDeviceInfo`] objects representing available media
/// input and devices, such as microphones, cameras, and so forth.
///
/// [`MediaDeviceInfo`]: super::media_device_info::MediaDeviceInfo
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__enumerate_devices(
    this: ptr::NonNull<MediaManagerHandle>,
) -> DartFuture<
    Result<PtrArray<MediaDeviceInfo>, Traced<EnumerateDevicesError>>,
> {
    let this = this.as_ref().clone();

    async move { Ok(PtrArray::new(this.enumerate_devices().await?)) }
        .into_dart_future()
}

/// Switches output audio device to the device with a provided `device_id`.
#[no_mangle]
pub unsafe extern "C" fn MediaManagerHandle__set_output_audio_id(
    this: ptr::NonNull<MediaManagerHandle>,
    device_id: ptr::NonNull<c_char>,
) -> DartFuture<Result<(), Traced<InvalidOutputAudioDeviceIdError>>> {
    let this = this.as_ref().clone();
    let device_id = c_str_into_string(device_id);

    async move {
        this.set_output_audio_id(device_id)
            .await
            .map_err(tracerr::map_from_and_wrap!())?;
        Ok(())
    }
    .into_dart_future()
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
    drop(MediaManagerHandle::from_ptr(this));
}

#[cfg(feature = "mockable")]
mod mock {
    use dart_sys::Dart_Handle;
    use futures::future;
    use tracerr::Traced;

    use crate::{
        api::{
            dart::{
                utils::{DartFuture, DartResult, IntoDartFuture},
                DartError,
            },
            LocalMediaTrack, MediaDeviceInfo, MediaStreamSettings,
        },
        media::{
            EnumerateDevicesError, InitLocalTracksError,
            InvalidOutputAudioDeviceIdError,
        },
        platform,
    };

    #[derive(Clone)]
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
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_local_media_init_exception(
        cause: Dart_Handle,
    ) -> DartResult {
        let cause = platform::Error::from(cause);
        let err = tracerr::new!(InitLocalTracksError::GetUserMediaFailed(
            cause.into()
        ));
        DartError::from(err).into()
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_future_with_local_media_init_exception(
        cause: Dart_Handle,
    ) -> DartFuture<Result<(), Traced<InitLocalTracksError>>> {
        let cause = platform::Error::from(cause);
        let err = tracerr::new!(InitLocalTracksError::GetDisplayMediaFailed(
            cause.into()
        ));

        future::err(err).into_dart_future()
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_enumerate_devices_exception(
        cause: Dart_Handle,
    ) -> DartResult {
        let cause = platform::Error::from(cause);
        DartError::from(tracerr::new!(EnumerateDevicesError::from(cause)))
            .into()
    }

    #[no_mangle]
    pub unsafe extern "C" fn returns_future_enumerate_devices_exception(
        cause: Dart_Handle,
    ) -> DartFuture<Result<(), Traced<EnumerateDevicesError>>> {
        let cause = platform::Error::from(cause);
        let err = tracerr::new!(EnumerateDevicesError::from(cause));

        future::err(err).into_dart_future()
    }
}
