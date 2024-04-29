use js_sys::{Array, JsString, Reflect};
use wasm_bindgen::JsValue;
use web_sys::{
    Event, RtcBundlePolicy, RtcConfiguration, RtcIceCandidateInit,
    RtcIceConnectionState, RtcIceTransportPolicy, RtcOfferOptions,
    RtcPeerConnection as SysRtcPeerConnection, RtcPeerConnectionIceErrorEvent,
    RtcPeerConnectionIceEvent, RtcRtpEncodingParameters, RtcRtpSender,
    RtcRtpTransceiver, RtcRtpTransceiverInit, RtcSdpType,
    RtcSessionDescription, RtcSessionDescriptionInit, RtcTrackEvent,
};

use crate::{
    media::MediaKind,
    platform::{
        codec_capability::CodecCapabilityError, Error, TransceiverDirection,
    },
};

#[derive(Clone, Debug)]
pub struct CodecCapability(JsValue);

impl CodecCapability {
    pub async fn get_sender_codec_capabilities(
        kind: MediaKind,
    ) -> Result<Vec<Self>, Error> {
        let codecs = RtcRtpSender::get_capabilities("video")
            .and_then(|capabs| {
                Reflect::get(&capabs, &JsString::from("codecs")).ok()
            })
            .unwrap();
        Ok(Array::from(&codecs)
            .iter()
            .map(|codec| Self(codec))
            .collect())
    }

    pub fn mime_type(&self) -> Result<String, Error> {
        Ok(Reflect::get(&self.0, &JsString::from("mimeType"))
            .ok()
            .and_then(|a| Some(a.as_string()?))
            .unwrap())
    }

    pub fn handle(&self) -> &JsValue {
        &self.0
    }
}
