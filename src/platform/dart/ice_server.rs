//! Collection of [RTCIceServer][1]s.
//!
//! [1]: https://w3.org/TR/webrtc#rtciceserver-dictionary

use dart_sys::Dart_Handle;
use medea_client_api_proto::IceServer;
use medea_macro::dart_bridge;

use crate::platform::utils::{handle::DartHandle, string_into_c_str};

#[dart_bridge("flutter/lib/src/native/platform/ice_servers.g.dart")]
mod ice_servers {
    use std::{os::raw::c_char, ptr};

    use dart_sys::Dart_Handle;

    use crate::{api::DartValueArg, platform::Error};

    extern "C" {
        /// Returns a [`Dart_Handle`] to the newly created empty `List` with
        /// `IceServer`s.
        pub fn init() -> Result<Dart_Handle, Error>;

        /// Adds an `IceServer` to the provided `List`.
        pub fn add(
            list: Dart_Handle,
            url: ptr::NonNull<c_char>,
            username: DartValueArg<String>,
            credentials: DartValueArg<String>,
        ) -> Result<(), Error>;
    }
}

/// Collection of [RTCIceServer][1]s.
///
/// [1]: https://w3.org/TR/webrtc#rtciceserver-dictionary
#[derive(Debug)]
pub struct RtcIceServers(DartHandle);

impl RtcIceServers {
    /// Returns [`Dart_Handle`] of these [`RtcIceServers`].
    #[must_use]
    pub fn get_handle(&self) -> Dart_Handle {
        self.0.get()
    }
}

#[expect(clippy::fallible_impl_from, reason = "FFI error is unexpected")]
impl<I> From<I> for RtcIceServers
where
    I: IntoIterator<Item = IceServer>,
{
    fn from(servers: I) -> Self {
        let ice_servers = unsafe { ice_servers::init() }.unwrap();
        let ice_servers = unsafe { DartHandle::new(ice_servers) };
        for srv in servers {
            for url in srv.urls {
                unsafe {
                    ice_servers::add(
                        ice_servers.get(),
                        string_into_c_str(url),
                        srv.username.clone().into(),
                        srv.credential
                            .clone()
                            .map(|s| s.expose_str().to_owned())
                            .into(),
                    )
                }
                .unwrap();
            }
        }
        Self(ice_servers)
    }
}
