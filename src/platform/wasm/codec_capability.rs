use js_sys::{Array, JsString, Reflect};
use wasm_bindgen::JsValue;
use web_sys::RtcRtpSender;

use crate::{
    media::MediaKind, platform::codec_capability::CodecCapabilityError as Error,
};

#[derive(Clone, Debug)]
pub struct CodecCapability(JsValue);

impl CodecCapability {
    pub async fn get_sender_codec_capabilities(
        kind: MediaKind,
    ) -> Result<Vec<Self>, Error> {
        let codecs = RtcRtpSender::get_capabilities(&kind.to_string())
            .and_then(|capabs| {
                Reflect::get(&capabs, &JsString::from("codecs")).ok()
            })
            .ok_or_else(|| Error::FailedToGetCapabilities)?;

        Ok(Array::from(&codecs)
            .iter()
            .map(|codec| Self(codec))
            .collect())
    }

    pub fn mime_type(&self) -> Result<String, Error> {
        Reflect::get(&self.0, &JsString::from("mimeType"))
            .ok()
            .and_then(|a| Some(a.as_string()?))
            .ok_or_else(|| Error::FailedToGetMimeType)
    }

    pub fn handle(&self) -> &JsValue {
        &self.0
    }
}
