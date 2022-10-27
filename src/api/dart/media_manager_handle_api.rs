pub use dart_sys::Dart_Handle;
use flutter_rust_bridge::{Opaque, SyncReturn};
use tracerr::Traced;

use crate::{
    api::utils::DartError,
    media::{
        EnumerateDevicesError, EnumerateDisplaysError, InitLocalTracksError,
        InvalidOutputAudioDeviceIdError, MicVolumeError,
    },
    platform,
};

pub use super::{
    media_stream_settings::MediaStreamSettings,
    utils::{IntoDartFuture, MyDartFuture, PtrArray},
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
    manager: Opaque<MediaManagerHandle>,
    caps: Opaque<MediaStreamSettings>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    SyncReturn(Opaque::new({
        let manager = MediaManagerHandle::clone(&manager);
        let caps = MediaStreamSettings::clone(&caps);
        async move {
            Ok::<PtrArray<_>, Traced<InitLocalTracksError>>(PtrArray::new(
                manager.init_local_tracks(caps).await?,
            ))
        }
        .into_my_dart_future()
    }))
}

/// Returns a list of [`MediaDeviceInfo`] objects representing available media
/// input and devices, such as microphones, cameras, and so forth.
#[rustfmt::skip]
pub fn media_manager_handle_enumerate_devices(
    manager: Opaque<MediaManagerHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    SyncReturn(Opaque::new({
        let manager = MediaManagerHandle::clone(&manager);
        async move { Ok::<PtrArray<_>, Traced<EnumerateDevicesError>>(PtrArray::new(manager.enumerate_devices().await?)) }
            .into_my_dart_future()
    }
    ))
}

/// Returns a list of [`MediaDisplayInfo`] objects representing available
/// sources that can be used for screen capturing.

pub fn media_manager_handle_enumerate_displays(
    manager: Opaque<MediaManagerHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    SyncReturn(Opaque::new({
        let manager = MediaManagerHandle::clone(&manager);

        async move {
            Ok::<PtrArray<_>, Traced<EnumerateDisplaysError>>(PtrArray::new(
                manager.enumerate_displays().await?,
            ))
        }
        .into_my_dart_future()
    }))
}

/// Switches the current output audio device to the device with the provided
/// `device_id`.

pub fn media_manager_handle_set_output_audio_id(
    manager: Opaque<MediaManagerHandle>,
    device_id: String,
) -> SyncReturn<Opaque<MyDartFuture>> {
    SyncReturn(Opaque::new({
        let manager = MediaManagerHandle::clone(&manager);

        async move {
            manager
                .set_output_audio_id(device_id)
                .await
                .map_err(tracerr::map_from_and_wrap!())?;
            Ok::<_, Traced<InvalidOutputAudioDeviceIdError>>(())
        }
        .into_my_dart_future()
    }))
}

/// Sets the microphone volume level in percents.

pub fn media_manager_handle_set_microphone_volume(
    manager: Opaque<MediaManagerHandle>,
    level: i64,
) -> SyncReturn<Opaque<MyDartFuture>> {
    SyncReturn(Opaque::new({
        let manager = MediaManagerHandle::clone(&manager);

        async move {
            manager
                .set_microphone_volume(level)
                .await
                .map_err(tracerr::map_from_and_wrap!())?;
            Ok::<_, Traced<MicVolumeError>>(())
        }
        .into_my_dart_future()
    }))
}

/// Indicates whether it's possible to access microphone volume settings.

pub fn media_manager_handle_microphone_volume_is_available(
    manager: Opaque<MediaManagerHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    SyncReturn(Opaque::new({
        let manager = MediaManagerHandle::clone(&manager);

        async move { manager.microphone_volume_is_available().await }
            .into_my_dart_future()
    }))
}

/// Returns the current microphone volume level in percents.
pub fn media_manager_handle_microphone_volume(
    manager: Opaque<MediaManagerHandle>,
) -> SyncReturn<Opaque<MyDartFuture>> {
    SyncReturn(Opaque::new({
        let manager = MediaManagerHandle::clone(&manager);

        async move {
            let res = manager.microphone_volume().await;
            let res: Result<_, Traced<MicVolumeError>> =
                res.map_err(tracerr::map_from_and_wrap!());
            res
        }
        .into_my_dart_future()
    }))
}

/// Subscribes onto the [`MediaManagerHandle`]'s `devicechange` event.

// todo(rogurotus): all methods with opaque dart_handle must be inone file.

// pub fn media_manager_handle_on_device_change(
//     manager: Opaque<MediaManagerHandle>,
//     cb: Opaque<Dart_Handle>,
// ) -> anyhow::Result<SyncReturn<()>> {
//     let manager = MediaManagerHandle::clone(&manager);
//     manager
//         .on_device_change(unsafe {
//             platform::Function::new(Dart_Handle::clone(&cb))
//         })
//         .map_err(|err| anyhow::anyhow!("{}", err))?;
//     Ok(SyncReturn(()))
// }

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
        cause: Opaque<Dart_Handle>,
    ) -> DartResult {
        let cause = platform::Error::from_handle(cause);
        let err = tracerr::new!(InitLocalTracksError::GetUserMediaFailed(
            platform::GetUserMediaError::Audio(cause).into()
        ));
        DartError::from(err).into()
    }

    pub fn returns_future_with_local_media_init_exception(
        cause: Opaque<Dart_Handle>,
    ) -> DartFuture<Result<(), Traced<InitLocalTracksError>>> {
        let cause = platform::Error::from_handle(cause);
        let err = tracerr::new!(InitLocalTracksError::GetDisplayMediaFailed(
            cause.into()
        ));

        future::err(err).into_my_dart_future()
    }

    pub fn returns_enumerate_devices_exception(
        cause: Opaque<Dart_Handle>,
    ) -> DartResult {
        let cause = platform::Error::from_handle(cause);
        DartError::from(tracerr::new!(EnumerateDevicesError::from(cause)))
            .into()
    }

    pub fn returns_future_enumerate_devices_exception(
        cause: Opaque<Dart_Handle>,
    ) -> DartFuture<Result<(), Traced<EnumerateDevicesError>>> {
        let cause = platform::Error::from_handle(cause);
        let err = tracerr::new!(EnumerateDevicesError::from(cause));

        future::err(err).into_my_dart_future()
    }
}
