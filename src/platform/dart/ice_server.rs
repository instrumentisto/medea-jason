//! Collection of [RTCIceServer][1]s.
//!
//! [1]: https://w3.org/TR/webrtc/#rtciceserver-dictionary

use std::{os::raw::c_char, ptr};

use dart_sys::Dart_Handle;
use medea_client_api_proto::IceServer;

use crate::{
    api::{string_into_c_str, DartValueArg},
    platform::utils::handle::DartHandle,
};

/// Pointer to an extern function returning a [`Dart_Handle`] to a newly created
/// empty list of `IceServer`s.
type NewFunction = extern "C" fn() -> Dart_Handle;

/// Stores pointer to the [`NewFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut NEW_FUNCTION: Option<NewFunction> = None;

/// Registers the provided [`NewFunction`] as [`NEW_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_IceServers__new(f: NewFunction) {
    NEW_FUNCTION = Some(f);
}

/// Pointer to an extern function which adds an `IceServer` to the provided
/// `list`.
type AddFunction = extern "C" fn(
    list: Dart_Handle,
    url: ptr::NonNull<c_char>,
    username: DartValueArg<String>,
    credentials: DartValueArg<String>,
);

/// Stores pointer to the [`AddFunction`] extern function.
///
/// Must be initialized by Dart during FFI initialization phase.
static mut ADD_FUNCTION: Option<AddFunction> = None;

/// Registers the provided [`AddFunction`] as [`ADD_FUNCTION`].
///
/// # Safety
///
/// Must ONLY be called by Dart during FFI initialization.
#[no_mangle]
pub unsafe extern "C" fn register_IceServers__add(f: AddFunction) {
    ADD_FUNCTION = Some(f);
}

/// Collection of [RTCIceServer][1]s.
///
/// [1]: https://w3.org/TR/webrtc/#rtciceserver-dictionary
#[derive(Debug)]
pub struct RtcIceServers(DartHandle);

impl RtcIceServers {
    /// Returns [`Dart_Handle`] of these [`RtcIceServers`].
    #[must_use]
    pub fn get_handle(&self) -> Dart_Handle {
        self.0.get()
    }
}

impl<I> From<I> for RtcIceServers
where
    I: IntoIterator<Item = IceServer>,
{
    fn from(servers: I) -> Self {
        let ice_servers = DartHandle::new(unsafe { NEW_FUNCTION.unwrap()() });
        for srv in servers {
            for url in srv.urls {
                unsafe {
                    ADD_FUNCTION.unwrap()(
                        ice_servers.get(),
                        string_into_c_str(url),
                        srv.username.clone().into(),
                        srv.credential.clone().into(),
                    );
                }
            }
        }
        Self(ice_servers)
    }
}
