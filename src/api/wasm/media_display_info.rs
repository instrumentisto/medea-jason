//! Empty representation of a display source info.
use derive_more::From;
use wasm_bindgen::prelude::*;

use crate::platform;

/// Representation of a display source info
#[wasm_bindgen]
#[derive(Debug, From)]
pub struct MediaDisplayInfo(platform::MediaDisplayInfo);

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl MediaDisplayInfo {}

#[allow(clippy::empty_drop)]
impl Drop for MediaDisplayInfo {
    fn drop(&mut self) {}
}
