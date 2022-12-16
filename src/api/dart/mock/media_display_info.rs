#![allow(missing_copy_implementations, clippy::unused_self)]

#[derive(Debug)]
pub struct MediaDisplayInfo(pub u8);

impl MediaDisplayInfo {
    #[must_use]
    pub fn device_id(&self) -> String {
        String::from("device_id")
    }

    #[must_use]
    pub fn title(&self) -> Option<String> {
        Some(String::from("title"))
    }
}
