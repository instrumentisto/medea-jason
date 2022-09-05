//! Representation of a [MediaDeviceInfo][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams#device-info
//todo
use derive_more::From;
use wasm_bindgen::prelude::*;

use crate::{api::MediaDeviceKind, platform};

/// Representation of a [MediaDeviceInfo][1].
///
/// [1]: https://w3.org/TR/mediacapture-streams#device-info
#[wasm_bindgen]
#[derive(Debug, From)]
pub struct MediaDeviceInfo(platform::MediaDeviceInfo);

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl MediaDeviceInfo {
    /// Returns a unique identifier for the represented device.
    #[must_use]
    pub fn device_id(&self) -> String {
        // self.0.device_id()
        todo!()
    }

    /// Returns label describing the represented device (for example "External
    /// USB Webcam").
    ///
    /// If the device has no associated label, then returns an empty string.
    #[must_use]
    pub fn title(&self) -> Option<String> {
        todo!()
    }
}
