//! `wasm32`-platform-specific functionality.

pub mod codec_capability;
pub mod constraints;
pub mod error;
pub mod ice_server;
pub mod media_device_info;
pub mod media_devices;
pub mod media_track;
pub mod peer_connection;
pub mod rtc_stats;
pub mod send_encoding_parameters;
pub mod transceiver;
pub mod transport;
pub mod utils;

use std::{collections::HashMap, time::Duration};

use futures::Future;
use js_sys::Promise;
use medea_client_api_proto as proto;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RtcRtpCapabilities, RtcRtpCodecCapability, Window};

pub use self::{
    codec_capability::CodecCapability,
    constraints::{DisplayMediaStreamConstraints, MediaStreamConstraints},
    error::Error,
    media_device_info::MediaDeviceInfo,
    media_devices::MediaDevices,
    media_track::MediaStreamTrack,
    peer_connection::RtcPeerConnection,
    rtc_stats::RtcStats,
    transceiver::{Transceiver, TransceiverInit},
    transport::WebSocketRpcTransport,
    utils::Function,
};

/// Unimplemented on WASM targets.
pub type MediaDisplayInfo = ();

#[cfg(feature = "wee_alloc")]
/// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
/// allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

/// When the `console_error_panic_hook` feature is enabled, we can call the
/// `set_panic_hook` function at least once during initialization, and then
/// we will get better error messages if our code ever panics.
///
/// For more details see:
/// <https://github.com/rustwasm/console_error_panic_hook#readme>
#[cfg(feature = "console_error_panic_hook")]
pub use console_error_panic_hook::set_once as set_panic_hook;

/// Initialize [`wasm_logger`] as default application logger.
///
/// [`wasm_logger`]: https://docs.rs/wasm-logger
pub fn init_logger() {
    wasm_logger::init(wasm_logger::Config::default());
}

/// Runs a Rust [`Future`] on the current thread.
pub fn spawn<F>(task: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(task);
}

/// [`Future`] which resolves after the provided [`Duration`].
///
/// # Panics
///
/// If fails to interact with JS side.
///
/// [`Future`]: std::future::Future
#[expect(clippy::unwrap_used, reason = "JS interop error is unexpected")]
pub async fn delay_for(delay: Duration) {
    let delay_ms = delay.as_millis().try_into().unwrap_or(i32::MAX);
    JsFuture::from(Promise::new(&mut |yes, _| {
        _ = window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes, delay_ms,
            )
            .unwrap();
    }))
    .await
    .map(drop)
    .unwrap();
}

/// Returns [`Window`] object.
///
/// # Panics
///
/// When global [`Window`] object is inaccessible.
#[must_use]
pub fn window() -> Window {
    // Cannot use `lazy_static` since `window` is `!Sync`.
    #![expect(clippy::unwrap_used, reason = "`window` is always present")]
    web_sys::window().unwrap()
}

/// Wrapper around interval timer ID.
#[derive(Debug)]
pub struct IntervalHandle(pub i32);

impl Drop for IntervalHandle {
    /// Clears interval with provided ID.
    fn drop(&mut self) {
        window().clear_interval_with_handle(self.0);
    }
}

/// Returns [`proto::Capabilities`] of the current platform.
#[expect(clippy::unused_async, reason = "`cfg` code uniformity")]
#[must_use]
pub async fn get_capabilities() -> proto::Capabilities {
    let convert_caps =
        |caps: RtcRtpCapabilities| -> Vec<proto::CodecCapability> {
            caps.get_codecs()
                .into_iter()
                .map(RtcRtpCodecCapability::from)
                .map(|c| -> proto::CodecCapability {
                    let sdp_fmtp = c.get_sdp_fmtp_line().unwrap_or_default();

                    let sdp_fmtp_line = sdp_fmtp
                        .split(';')
                        .map(|s| s.split('='))
                        .filter_map(|mut a| match (a.next(), a.next()) {
                            (Some(k), Some(v)) => {
                                Some((k.to_owned(), v.to_owned()))
                            }
                            _ => None,
                        })
                        .collect::<HashMap<String, String>>();

                    proto::CodecCapability {
                        mime_type: c.get_mime_type(),
                        clock_rate: c.get_clock_rate(),
                        channels: c.get_channels(),
                        parameters: sdp_fmtp_line,
                    }
                })
                .collect::<Vec<_>>()
        };

    proto::Capabilities {
        audio_tx: web_sys::RtcRtpSender::get_capabilities("audio")
            .map(convert_caps)
            .unwrap_or_default(),
        audio_rx: web_sys::RtcRtpReceiver::get_capabilities("audio")
            .map(convert_caps)
            .unwrap_or_default(),
        video_tx: web_sys::RtcRtpSender::get_capabilities("video")
            .map(convert_caps)
            .unwrap_or_default(),
        video_rx: web_sys::RtcRtpReceiver::get_capabilities("video")
            .map(convert_caps)
            .unwrap_or_default(),
    }
}
