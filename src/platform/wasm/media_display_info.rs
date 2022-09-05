//todo
//! [MediaDeviceInfo][1] related objects.
//!
//! [1]: https://w3.org/TR/mediacapture-streams#device-info

use derive_more::From;

#[derive(Debug, From)]
pub struct MediaDisplayInfo {
    device_id: String, // index in list
    title: Option<String>, // label
}

impl MediaDisplayInfo {
    pub fn new(index: usize, title: Option<String>) -> Self {
        Self {
            device_id: index.to_string(),
            title
        }
    }
    /// Returns a unique identifier for the represented display.
    #[must_use]
    pub fn device_id(&self) -> String {
        self.device_id.clone()
    }

    #[must_use]
    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }
}
