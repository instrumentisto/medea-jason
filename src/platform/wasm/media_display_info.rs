//todo
//! [MediaDeviceInfo][1] related objects.
//!
//! [1]: https://w3.org/TR/mediacapture-streams#device-info

use derive_more::From;
use web_sys as sys;

#[derive(Debug, From)]
pub struct MediaDisplayInfo(sys::MediaDisplayInfo);

impl MediaDisplayInfo {
    /// Returns a unique identifier for the represented display.
    #[must_use]
    pub fn device_id(&self) -> String {
        self.0.device_id()
    }

    #[must_use]
    pub fn title(&self) -> String {
        self.0.title()
    }
}
