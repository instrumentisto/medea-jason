//! Tests for the [`TransitableStateController`].

use futures::StreamExt;
use js_sys::Array;
use medea_client_api_proto::EncodingParameters;
use medea_jason::{
    peer::{media_exchange_state, MediaExchangeStateController},
    platform::get_property_by_name,
};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::wasm_bindgen_test;
use web_sys::{
    RtcPeerConnection, RtcRtpEncodingParameters, RtcRtpTransceiverDirection,
    RtcRtpTransceiverInit,
};

use crate::timeout;

/// Tests that [`TransitableStateController`] will freeze transition timeout for
/// the new transitions.
#[wasm_bindgen_test]
async fn controller_inheritance_delay_freeze() {
    let controller = MediaExchangeStateController::new(
        media_exchange_state::Stable::Enabled,
    );
    controller.stop_transition_timeout();
    controller.transition_to(media_exchange_state::Stable::Disabled);

    timeout(600, controller.subscribe_stable().next())
        .await
        .unwrap_err();
}

/// Tests that [`TransitableStateController`] will unfreeze frozen on start
/// transition timeout.
#[wasm_bindgen_test]
async fn unfreezes_inheritance_delay_freeze() {
    let controller = MediaExchangeStateController::new(
        media_exchange_state::Stable::Enabled,
    );
    controller.stop_transition_timeout();
    controller.transition_to(media_exchange_state::Stable::Disabled);
    controller.reset_transition_timeout();

    let rollbacked_state = timeout(600, controller.subscribe_stable().next())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(rollbacked_state, media_exchange_state::Stable::Enabled);
}

#[wasm_bindgen_test]
async fn aaaaaaaaaaaaaaaaaaaaaaaaaaaaa() {
    let pc = RtcPeerConnection::new().unwrap();

    let mut init = RtcRtpTransceiverInit::new();
    _ = init.direction(RtcRtpTransceiverDirection::Sendrecv);

    let send_encodings = ::js_sys::Array::new();

    for encoding in [
        EncodingParameters {
            rid: "h".to_string(),
            active: true,
            max_bitrate: Some(900000),
            scale_resolution_down_by: None,
        },
        EncodingParameters {
            rid: "l".to_string(),
            active: true,
            max_bitrate: Some(300000),
            scale_resolution_down_by: Some(2),
        },
    ] {
        let mut params = RtcRtpEncodingParameters::new();
        _ = params.rid(&encoding.rid);
        _ = params.active(encoding.active);
        if let Some(max_bitrate) = encoding.max_bitrate {
            _ = params.max_bitrate(max_bitrate);
        }
        if let Some(scale_resolution_down_by) =
            encoding.scale_resolution_down_by
        {
            _ = params
                .scale_resolution_down_by(scale_resolution_down_by.into());
        }

        _ = send_encodings.push(&params);
    }
    _ = init.send_encodings(&send_encodings);

    let trans = pc.add_transceiver_with_str_and_init("video", &init);

    let mut params = trans.sender().get_parameters();
    let encs = get_property_by_name(&params, "encodings", |v| {
        v.is_array().then_some(Array::from(&v))
    }).unwrap();

    let mut bits_before = Vec::new();

    for enc in encs.iter() {
        let mut enc_params = RtcRtpEncodingParameters::from(enc);
        bits_before.push(
            get_property_by_name(&enc_params, "maxBitrate", |v| {
                v.as_f64().map(|v| v as u32)
            })
            .unwrap(),
        );
        enc_params.max_bitrate(500000);
    }

    JsFuture::from(trans.sender().set_parameters_with_parameters(&params))
        .await
        .unwrap();

    let params = trans.sender().get_parameters();
    let encs = get_property_by_name(&params, "encodings", |v| {
        v.is_array().then_some(Array::from(&v))
    }).unwrap();

    let mut bits_after = Vec::new();

    for enc in encs.iter() {
        let enc_params = RtcRtpEncodingParameters::from(enc);
        bits_after.push(
            get_property_by_name(&enc_params, "maxBitrate", |v| {
                v.as_f64().map(|v| v as u32)
            })
            .unwrap(),
        );
    }

    panic!("before: {bits_before:?}\nafter: {bits_after:?}");
}
