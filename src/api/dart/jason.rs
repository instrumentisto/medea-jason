use std::ptr;

use super::{
    catch_panic, media_manager_handle::MediaManagerHandle,
    room_handle::RoomHandle, ForeignClass,
};

#[cfg(feature = "mockable")]
pub use self::mock::Jason;
#[cfg(not(feature = "mockable"))]
pub use crate::jason::Jason;

impl ForeignClass for Jason {}

/// Instantiates a new [`Jason`] interface to interact with this library.
#[no_mangle]
pub extern "C" fn Jason__new() -> ptr::NonNull<Jason> {
    catch_panic(|| Jason::new().into_ptr())
}

/// Creates a new [`Room`] and returns its [`RoomHandle`].
///
/// [`Room`]: crate::room::Room
#[no_mangle]
pub unsafe extern "C" fn Jason__init_room(
    this: ptr::NonNull<Jason>,
) -> ptr::NonNull<RoomHandle> {
    catch_panic(move || this.as_ref().init_room().into_ptr())
}

/// Returns a [`MediaManagerHandle`].
#[no_mangle]
pub unsafe extern "C" fn Jason__media_manager(
    this: ptr::NonNull<Jason>,
) -> ptr::NonNull<MediaManagerHandle> {
    catch_panic(move || this.as_ref().media_manager().into_ptr())
}

/// Closes the provided [`RoomHandle`].
#[no_mangle]
pub unsafe extern "C" fn Jason__close_room(
    this: ptr::NonNull<Jason>,
    room_to_delete: ptr::NonNull<RoomHandle>,
) {
    catch_panic(move || {
        this.as_ref()
            .close_room(RoomHandle::from_ptr(room_to_delete));
    });
}

/// Frees the data behind the provided pointer.
///
/// # Safety
///
/// Should be called when object is no longer needed. Calling this more than
/// once for the same pointer is equivalent to double free.
#[no_mangle]
pub unsafe extern "C" fn Jason__free(this: ptr::NonNull<Jason>) {
    catch_panic(move || {
        let jason = Jason::from_ptr(this);
        jason.dispose();
    });
}

#[cfg(feature = "mockable")]
mod mock {
    use crate::api::{MediaManagerHandle, RoomHandle};

    pub struct Jason(u8);

    impl Jason {
        pub fn new() -> Self {
            crate::platform::init_logger();
            Self(0)
        }

        pub fn init_room(&self) -> RoomHandle {
            RoomHandle(0)
        }

        pub fn media_manager(&self) -> MediaManagerHandle {
            MediaManagerHandle(0)
        }

        pub fn close_room(&self, _: RoomHandle) {}

        pub fn dispose(self) {}
    }
}
