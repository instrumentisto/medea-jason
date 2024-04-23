use js_sys::{Array, JsString, Reflect};
use web_sys::{
    Event, RtcBundlePolicy, RtcConfiguration, RtcIceCandidateInit,
    RtcIceConnectionState, RtcIceTransportPolicy, RtcOfferOptions,
    RtcPeerConnection as SysRtcPeerConnection, RtcPeerConnectionIceErrorEvent,
    RtcPeerConnectionIceEvent, RtcRtpEncodingParameters, RtcRtpSender,
    RtcRtpTransceiver, RtcRtpTransceiverInit, RtcSdpType,
    RtcSessionDescription, RtcSessionDescriptionInit, RtcTrackEvent,
};

use crate::platform::codec_capability::{
    CodecCapability, CodecCapabilityError,
};
use crate::platform::TransceiverDirection;

pub fn get_codec_capabilities(
) -> Result<Vec<CodecCapability>, CodecCapabilityError> {
    let codecs = RtcRtpSender::get_capabilities("video")
        .and_then(|capabs| {
            Reflect::get(&capabs, &JsString::from("codecs")).ok()
        })
        .unwrap();
    Ok(Array::from(&codecs)
        .iter()
        .map(|codec| {
            Reflect::get(&codec, &JsString::from("mimeType"))
                .ok()
                .and_then(|a| {
                    Some(CodecCapability {
                        mime_type: a.as_string()?,
                    })
                })
                .unwrap()
        })
        .collect())
}

