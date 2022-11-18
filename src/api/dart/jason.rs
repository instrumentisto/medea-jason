use std::ptr;

use super::{
    media_manager_handle::MediaManagerHandle, propagate_panic,
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
    propagate_panic(|| Jason::new().into_ptr())
}

/// Creates a new [`Room`] and returns its [`RoomHandle`].
///
/// [`Room`]: crate::room::Room
#[no_mangle]
pub unsafe extern "C" fn Jason__init_room(
    this: ptr::NonNull<Jason>,
) -> ptr::NonNull<RoomHandle> {
    propagate_panic(move || this.as_ref().init_room().into_ptr())
}

/// Returns a [`MediaManagerHandle`].
#[no_mangle]
pub unsafe extern "C" fn Jason__media_manager(
    this: ptr::NonNull<Jason>,
) -> ptr::NonNull<MediaManagerHandle> {
    propagate_panic(move || this.as_ref().media_manager().into_ptr())
}

/// Closes the provided [`RoomHandle`].
#[no_mangle]
pub unsafe extern "C" fn Jason__close_room(
    this: ptr::NonNull<Jason>,
    room_to_delete: ptr::NonNull<RoomHandle>,
) {
    propagate_panic(move || {
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
    propagate_panic(move || {
        let jason = Jason::from_ptr(this);
        jason.dispose();
    });
}

#[cfg(feature = "mockable")]
mod mock {
    #![allow(
        clippy::new_without_default,
        clippy::unused_self,
        unused_tuple_struct_fields
    )]

    use crate::api::{MediaManagerHandle, RoomHandle};

    #[derive(Clone, Copy, Debug)]
    pub struct Jason(u8);

    impl Jason {
        #[must_use]
        pub fn new() -> Self {
            crate::platform::init_logger();
            Self(0)
        }

        #[must_use]
        pub const fn init_room(&self) -> RoomHandle {
            RoomHandle(0)
        }

        #[must_use]
        pub const fn media_manager(&self) -> MediaManagerHandle {
            MediaManagerHandle(0)
        }

        pub const fn close_room(&self, _: RoomHandle) {}

        pub const fn dispose(self) {}
    }
}
