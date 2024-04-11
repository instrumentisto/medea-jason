//! [SendEncodingParameters] wrapper.
//!
//! [SendEncodingParameters]: https://w3.org/TR/webrtc#dom-rtcrtptransceiver

use dart_sys::_Dart_Handle;
use medea_macro::dart_bridge;

use crate::{
    media::MediaKind,
    platform::dart::utils::handle::DartHandle,
};

use super::utils::{
    dart_future::FutureFromDart, dart_string_into_rust, list::DartList,
};

#[dart_bridge("flutter/lib/src/native/platform/codec_capability.g.dart")]
mod codec_capability {
    use std::{ffi::c_char, ptr};

    use dart_sys::Dart_Handle;

    extern "C" {
        pub fn get_sender_codec_capabilities(kind: i64) -> Dart_Handle;

        pub fn mime_type(codec_capability: Dart_Handle)
            -> ptr::NonNull<c_char>;
    }
}

/// Asd
#[derive(Clone, Debug)]
pub struct CodecCapability(DartHandle);

impl From<DartHandle> for CodecCapability {
    fn from(value: DartHandle) -> Self {
        Self(value)
    }
}

impl CodecCapability {
    /// Asd
    #[must_use]
    pub async fn get_sender_codec_capabilities(
        kind: MediaKind,
    ) -> Vec<CodecCapability> {
        let fut = unsafe {
            codec_capability::get_sender_codec_capabilities(kind as i64)
        };

        let res: DartHandle =
            unsafe { FutureFromDart::execute(fut) }.await.unwrap();

        let codec_capabilities = Vec::from(DartList::from(res))
            .into_iter()
            .map(|capabs: DartHandle| Self::from(capabs))
            .collect();

        codec_capabilities
    }

    /// Asd
    #[must_use]
    pub fn mime_type(&self) -> String {
        let mime_type = unsafe { codec_capability::mime_type(self.0.get()) };
        unsafe { dart_string_into_rust(mime_type) }
    }
    
    /// Returns underlying [`_Dart_Handle`].
    #[must_use]
    pub fn handle(&self) -> *mut _Dart_Handle {
        self.0.get()
    }
}
