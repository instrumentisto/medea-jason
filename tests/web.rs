#![cfg(target_arch = "wasm32")]
#![forbid(non_ascii_idents)]

use js_sys::{Object, Reflect};

/// Analog for [`assert_eq`] but for [`js_callback`] macro.
/// Simply use it as [`assert_eq`]. For use cases and reasons
/// just read [`js_callback`]'s docs.
///
/// __Use it only in [`js_callback`]'s closures.__
macro_rules! cb_assert_eq {
    ($a:expr, $b:expr $(,)?) => {
        if $a != $b {
            return Err(format!("{:?} != {:?}", $a, $b));
        }
    };
}

/// Macro which generates [`wasm_bindgen::closure::Closure`] with provided
/// [`FnOnce`] and [`futures::channel::oneshot::Receiver`] which will receive
/// result of assertions from provided [`FnOnce`]. In provided [`FnOnce`] you
/// may use [`cb_assert_eq`] macro in same way as a vanilla [`assert_eq`].
/// Result of this assertions will be returned with
/// [`futures::channel::oneshot::Receiver`]. You may simply use
/// [`wait_and_check_test_result`] which will `panic` if some assertion failed.
///
/// # Use cases
///
/// This macro is useful in tests which should check that JS callback provided
/// in some function was called and some assertions was passed. We can't use
/// habitual `assert_eq` because panics from [`wasm_bindgen::closure::Closure`]
/// will not fail `wasm_bindgen_test`'s tests.
///
/// # Example
///
/// ```ignore
/// let room: Room = get_room();
/// let mut room_handle: RoomHandle = room.new_handle();
///
/// // Create 'Closure' which we can provide as JS closure and
/// // 'Future' which will be resolved with assertions result.
/// let (cb, test_result) = js_callback!(|closed: JsValue| {
///     // You can write here any Rust code which you need.
///     // The only difference is that within this macro
///     // you can use 'cb_assert_eq!'.
///     let closed_reason = get_reason(&closed);
///     cb_assert_eq!(closed_reason, "RoomUnexpectedlyDropped");
///
///     cb_assert_eq!(get_is_err(&closed), false);
///     cb_assert_eq!(get_is_closed_by_server(&closed), false);
/// });
/// room_handle.on_close(cb.into()).unwrap();
///
/// room.close(CloseReason::ByClient {
///     reason: ClientDisconnect::RoomUnexpectedlyDropped,
///     is_err: false,
/// });
///
/// // Wait for closure execution, get assertions result and check it.
/// // 'wait_and_check_test_result' will panic if assertion errored.
/// wait_and_check_test_result(test_result).await;
/// ```
macro_rules! js_callback {
    (|$($arg_name:ident: $arg_type:ty),*| $body:block) => {{
        let (test_tx, test_rx) = futures::channel::oneshot::channel();
        let closure = wasm_bindgen::closure::Closure::once_into_js(
            move |$($arg_name: $arg_type),*| {
                let test_fn = || {
                    $body;
                    Ok(())
                };
                test_tx.send((test_fn)()).unwrap();
            }
        );

        (closure, test_rx)
    }}
}

mod media;
mod peer;
mod room;
mod rpc;
mod utils;

use futures::{channel::oneshot, future::Either};
use js_sys::Promise;
use medea_client_api_proto::{
    AudioSettings, Direction, MediaSourceKind, MediaType, MemberId, Track,
    TrackId, VideoSettings,
};
use medea_jason::{
    api,
    media::{
        AudioTrackConstraints, DeviceVideoTrackConstraints,
        LocalTracksConstraints, MediaDirection, MediaKind, MediaManager,
        MediaStreamSettings, track::remote,
    },
    peer::media_exchange_state,
    rpc::ApiUrl,
};
use url::Url;
use wasm_bindgen::{convert::RefFromWasmAbi, prelude::*};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(module = "/tests/mock_navigator.js")]
extern "C" {
    pub type MockNavigator;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MockNavigator;

    #[wasm_bindgen(method, setter = errorGetUserMedia)]
    fn error_get_user_media(this: &MockNavigator, err: JsValue);

    #[wasm_bindgen(method, setter = errorGetDisplayMedia)]
    fn error_get_display_media(this: &MockNavigator, err: JsValue);

    #[wasm_bindgen(method, setter = errorEnumerateDevices)]
    fn error_enumerate_devices(this: &MockNavigator, err: JsValue);

    #[wasm_bindgen(method, getter = getUserMediaRequestsCount)]
    fn get_user_media_requests_count(this: &MockNavigator) -> i32;

    #[wasm_bindgen(method, getter = getDisplayMediaRequestsCount)]
    fn get_display_media_requests_count(this: &MockNavigator) -> i32;

    #[wasm_bindgen(method, setter = setUserMediaReturns)]
    fn setUserMediaReturns(this: &MockNavigator, stream: web_sys::MediaStream);

    #[wasm_bindgen(method, setter = setDisplayMediaReturns)]
    fn setDisplayMediaReturns(
        this: &MockNavigator,
        stream: web_sys::MediaStream,
    );

    #[wasm_bindgen(method)]
    fn stop(this: &MockNavigator);
}

/// Performs a unchecked conversion from a [`JsValue`] into an instance of the
/// specified `FromWasmAbi<Abi = u32>` implementor.
pub fn jsval_cast<T: RefFromWasmAbi<Abi = u32>>(
    val: JsValue,
    t: &str,
) -> Result<<T as RefFromWasmAbi>::Anchor, String> {
    if !val.is_object() {
        return Err(String::from(
            "`unchecked_jsval_cast` is only applicable to objects",
        ));
    }

    let obj: Object = Object::from(val);
    let class_name = obj.constructor().name();
    if class_name != t {
        return Err(format!(
            "type check failed, expected {t}, but got {class_name}",
        ));
    }

    let ptr = Reflect::get(&obj, &JsValue::from_str("__wbg_ptr")).unwrap();
    let ptr = ptr.as_f64().unwrap() as u32;

    Ok(unsafe { T::ref_from_abi(ptr) })
}

pub fn get_test_required_tracks() -> (Track, Track) {
    get_test_tracks(true, true)
}

pub fn get_test_unrequired_tracks() -> (Track, Track) {
    get_test_tracks(false, false)
}

pub fn get_media_stream_settings(
    audio_enabled: bool,
    video_enabled: bool,
) -> MediaStreamSettings {
    let mut settings = MediaStreamSettings::default();
    settings.set_track_media_state(
        media_exchange_state::Stable::from(audio_enabled).into(),
        MediaKind::Audio,
        None,
    );
    settings.set_track_media_state(
        media_exchange_state::Stable::from(video_enabled).into(),
        MediaKind::Video,
        None,
    );

    settings
}

pub fn get_test_tracks(
    is_audio_required: bool,
    is_video_required: bool,
) -> (Track, Track) {
    (
        Track {
            id: TrackId(1),
            direction: Direction::Send {
                receivers: vec![MemberId::from("bob")],
                mid: None,
            },
            media_direction: MediaDirection::SendRecv.into(),
            muted: false,
            media_type: MediaType::Audio(AudioSettings {
                required: is_audio_required,
                source_kind: MediaSourceKind::Device,
            }),
        },
        Track {
            id: TrackId(2),
            direction: Direction::Send {
                receivers: vec![MemberId::from("bob")],
                mid: None,
            },
            media_direction: MediaDirection::SendRecv.into(),
            muted: false,
            media_type: MediaType::Video(VideoSettings {
                required: is_video_required,
                source_kind: MediaSourceKind::Device,
                encoding_parameters: Vec::new(),
            }),
        },
    )
}

/// Returns [`Track`]s with a [`Direction::Recv`].
///
/// First [`Track`] will be [`MediaType::Audio`] and second will be
/// [`MediaType::Video`].
pub fn get_test_recv_tracks() -> (Track, Track) {
    (
        Track {
            id: TrackId(0),
            direction: Direction::Recv {
                sender: "bob".into(),
                mid: Some("mid0".to_string()),
            },
            media_direction: MediaDirection::SendRecv.into(),
            muted: false,
            media_type: MediaType::Audio(AudioSettings {
                required: false,
                source_kind: MediaSourceKind::Device,
            }),
        },
        Track {
            id: TrackId(1),
            direction: Direction::Recv {
                sender: "bob".into(),
                mid: Some("mid1".to_string()),
            },
            media_direction: MediaDirection::SendRecv.into(),
            muted: false,
            media_type: MediaType::Video(VideoSettings {
                required: false,
                source_kind: MediaSourceKind::Device,
                encoding_parameters: Vec::new(),
            }),
        },
    )
}

const TEST_ROOM_URL: &str = "ws://example.com/room_id/member_id?token=token";

pub fn join_room_url() -> ApiUrl {
    Url::parse("ws://example.com/ws").unwrap().into()
}

/// Resolves after provided number of milliseconds.
pub async fn delay_for(delay_ms: i32) {
    JsFuture::from(Promise::new(&mut |yes, _| {
        window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes, delay_ms,
            )
            .unwrap();
    }))
    .await
    .unwrap();
}

fn media_stream_settings(
    is_audio_enabled: bool,
    is_video_enabled: bool,
) -> api::MediaStreamSettings {
    let mut settings = api::MediaStreamSettings::new();
    if is_audio_enabled {
        settings.device_audio(api::AudioTrackConstraints::new());
        settings.display_audio(api::AudioTrackConstraints::new());
    }
    if is_video_enabled {
        settings.device_video(api::DeviceVideoTrackConstraints::new());
        settings.display_video(api::DisplayVideoTrackConstraints::new());
    }

    settings
}

fn local_constraints(
    is_audio_enabled: bool,
    is_video_enabled: bool,
) -> LocalTracksConstraints {
    let constraints = LocalTracksConstraints::default();
    constraints.constrain(
        media_stream_settings(is_audio_enabled, is_video_enabled).into(),
    );

    constraints
}

/// Waits for [`Result`] from [`oneshot::Receiver`] with tests result.
///
/// Also, it will check result of test and will panic if some error will be
/// found.
async fn wait_and_check_test_result(
    rx: oneshot::Receiver<Result<(), String>>,
    finally: impl FnOnce(),
) {
    let result =
        futures::future::select(Box::pin(rx), Box::pin(delay_for(5000))).await;
    finally();
    match result {
        Either::Left((oneshot_fut_result, _)) => {
            let assert_result = oneshot_fut_result.expect("cancelled");
            assert_result.expect("assertion failed");
        }
        Either::Right(_) => {
            panic!("callback didn't fired");
        }
    };
}

#[expect(dead_code, reason = "preserved for future tests")]
async fn get_video_track() -> api::RemoteMediaTrack {
    let manager = MediaManager::default();
    let mut settings = MediaStreamSettings::new();
    settings.device_video(DeviceVideoTrackConstraints::new());
    let mut tracks = manager.get_tracks(settings).await.unwrap();
    let track = tracks.pop().unwrap().0.as_ref().as_ref().fork().await;
    remote::Track::new(
        track,
        MediaSourceKind::Device,
        false,
        MediaDirection::SendRecv.into(),
    )
    .into()
}

async fn get_audio_track() -> api::RemoteMediaTrack {
    let manager = MediaManager::default();
    let mut settings = MediaStreamSettings::new();
    settings.device_audio(AudioTrackConstraints::new());
    let mut tracks = manager.get_tracks(settings).await.unwrap();
    let track = tracks.pop().unwrap().0.as_ref().as_ref().fork().await;
    remote::Track::new(
        track,
        MediaSourceKind::Device,
        false,
        MediaDirection::SendRecv.into(),
    )
    .into()
}

/// Awaits provided [`LocalBoxFuture`] for `timeout` milliseconds. If within
/// provided `timeout` time this [`LocalBoxFuture`] won't be resolved, then
/// `Err(String)` will be returned, otherwise a result of the provided
/// [`LocalBoxFuture`] will be returned.
async fn timeout<T>(timeout_ms: i32, future: T) -> Result<T::Output, String>
where
    T: Future,
{
    match futures::future::select(
        Box::pin(future),
        Box::pin(delay_for(timeout_ms)),
    )
    .await
    {
        Either::Left((res, _)) => Ok(res),
        Either::Right((_, _)) => Err("Future timed out.".to_string()),
    }
}

/// Async [`std::thread::yield_now`].
pub async fn yield_now() {
    delay_for(0).await;
}

/// Returns [`Window`] object.
///
/// # Panics
///
/// When global [`Window`] object is inaccessible.
pub fn window() -> web_sys::Window {
    // Cannot use `lazy_static` since `window` is `!Sync`.
    // Safe to unwrap.
    web_sys::window().unwrap()
}

// TODO: Might be extended to proc macro at some point.
#[wasm_bindgen(inline_js = "\
    export function is_firefox() { \
        return navigator.userAgent.toLowerCase().indexOf('firefox') > -1; \
    }\
")]
extern "C" {
    fn is_firefox() -> bool;
}

#[wasm_bindgen(inline_js = "\
    export function is_chromium() { \
        return navigator.vendor == 'Google Inc.'; \
    }\
")]
extern "C" {
    fn is_chromium() -> bool;
}
