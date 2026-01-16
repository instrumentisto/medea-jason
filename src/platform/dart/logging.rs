//! Bindings for [`medea-flutter-webrtc`][0] logging management.
//!
//! [0]: https://github.com/instrumentisto/medea-flutter-webrtc

use medea_macro::dart_bridge;

use crate::platform::utils::dart_future::FutureFromDart;

#[dart_bridge("flutter/lib/src/native/platform/logging.g.dart")]
mod logging {
    use dart_sys::Dart_Handle;

    use crate::platform::Error;

    /// Sets logging severity for the [`medea-flutter-webrtc`][0].
    ///
    /// [0]: https://github.com/instrumentisto/medea-flutter-webrtc
    extern "C" {
        pub fn set_webrtc_log_level(level: i64) -> Result<Dart_Handle, Error>;
    }
}

/// Log levels supported by [`medea-flutter-webrtc`][0].
///
/// [0]: https://github.com/instrumentisto/medea-flutter-webrtc
enum WebRtcLogLevel {
    Verbose,
    Info,
    Warning,
    Error,
}

impl From<log::LevelFilter> for WebRtcLogLevel {
    fn from(value: log::LevelFilter) -> Self {
        match value {
            log::LevelFilter::Off | log::LevelFilter::Error => Self::Error,
            log::LevelFilter::Warn => Self::Warning,
            log::LevelFilter::Info => Self::Info,
            log::LevelFilter::Debug | log::LevelFilter::Trace => Self::Verbose,
        }
    }
}

/// Sets logging severity for the [`medea-flutter-webrtc`][0].
///
/// [0]: https://github.com/instrumentisto/medea-flutter-webrtc
pub async fn set_log_level(level: log::LevelFilter) {
    let fut = unsafe {
        logging::set_webrtc_log_level(WebRtcLogLevel::from(level) as i64)
    }
    .unwrap();

    unsafe { FutureFromDart::execute::<()>(fut) }
        .await
        .map_err(tracerr::wrap!())
        .unwrap();
}
