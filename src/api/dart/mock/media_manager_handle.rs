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
        InitLocalTracksError, InvalidOutputAudioDeviceIdError, MicVolumeError,
    },
    platform,
};

#[derive(Clone, Debug)]
pub struct MediaManagerHandle(pub u8);

#[allow(clippy::missing_errors_doc, clippy::unused_async)]
impl MediaManagerHandle {
    pub async fn enumerate_devices(
        &self,
    ) -> Result<Vec<MediaDeviceInfo>, Traced<EnumerateDevicesError>> {
        Ok(vec![
            MediaDeviceInfo(0),
            MediaDeviceInfo(0),
            MediaDeviceInfo(0),
        ])
    }

    pub async fn enumerate_displays(
        &self,
    ) -> Result<Vec<MediaDisplayInfo>, Traced<EnumerateDisplaysError>> {
        Ok(vec![MediaDisplayInfo(0)])
    }

    pub async fn init_local_tracks(
        &self,
        _caps: MediaStreamSettings,
    ) -> Result<Vec<LocalMediaTrack>, Traced<InitLocalTracksError>> {
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
    DartError::from(tracerr::new!(EnumerateDevicesError::from(cause))).into()
}

#[no_mangle]
pub unsafe extern "C" fn returns_future_enumerate_devices_exception(
    cause: Dart_Handle,
) -> DartFuture<Result<(), Traced<EnumerateDevicesError>>> {
    let cause = platform::Error::from_handle(cause);
    let err = tracerr::new!(EnumerateDevicesError::from(cause));

    future::err(err).into_dart_future()
}
