//! Wrapper around [MediaStreamTrack][1].
//!
//! [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack

use async_once::AsyncOnce;
use derive_more::AsRef;
use futures::future;
use lazy_static::lazy_static;
use once_cell::unsync::OnceCell;
use std::{cell::RefCell, future::Future, rc::Rc};
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;

use crate::{
    media::{
        track::MediaStreamTrackState, FacingMode, MediaKind, MediaSourceKind,
    },
    platform::wasm::{get_property_by_name, utils::EventListener},
};

/// Provides calculation and sending of audio level.
#[derive(Debug)]
pub struct AudioLevelProvider {
    /// Js `AudioLevelProcessor`.
    node: web_sys::AudioWorkletNode,
    /// Js `AudioLevelProcessor` on message cb.
    cb: Option<Closure<dyn FnMut(web_sys::MessageEvent)>>,
}

impl AudioLevelProvider {
    /// Creates a new [`AudioLevelProvider`].
    ///
    /// # Panics
    ///
    /// Panic if `web_sys` function return error.
    #[allow(clippy::unwrap_used)]
    pub async fn new(track: &web_sys::MediaStreamTrack) -> Self {
        let stream = web_sys::MediaStream::new().unwrap();
        stream.add_track(track);

        let context = web_sys::AudioContext::new().unwrap();
        let source = context.create_media_stream_source(&stream).unwrap();

        _ = MODULE_REGISTRATION.get().await;

        let node = web_sys::AudioWorkletNode::new(
            &context,
            "audio-level-processor",
        )
        .unwrap();

        drop(source.connect_with_audio_node(&node));
        Self { node, cb: None }
    }

    /// Sets a callback to invoke when receive audio level message from
    /// `AudioLevelProcessor`.
    pub fn set_audio_level_cb<F>(&mut self, f: Option<F>)
    where
        F: 'static + FnMut(f64),
    {
        if let Ok(port) = self.node.port() {
            if let Some(mut f) = f {
                let closure: Closure<dyn FnMut(web_sys::MessageEvent)> =
                    Closure::new(move |v: web_sys::MessageEvent| {
                        f(v.data().as_f64().unwrap_or_default());
                    });

                port.set_onmessage(Some(closure.as_ref().unchecked_ref()));
                self.cb = Some(closure);
            } else {
                port.set_onmessage(None);
                self.cb = None;
            }
        }
    }
}

impl Drop for AudioLevelProvider {
    fn drop(&mut self) {
        if let Ok(port) = self.node.port() {
            port.set_onmessage(None);
        }
    }
}

/// Wrapper around [MediaStreamTrack][1] received from a
/// [getUserMedia()][2]/[getDisplayMedia()][3] request.
///
/// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
/// [2]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [3]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
#[derive(AsRef, Debug)]
pub struct MediaStreamTrack {
    /// Underlying [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[as_ref(forward)]
    sys_track: Rc<web_sys::MediaStreamTrack>,

    /// Kind of the underlying [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    kind: MediaKind,

    /// Media source kind of this [MediaStreamTrack][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    source_kind: Option<MediaSourceKind>,

    /// Listener for an [ended][1] event.
    ///
    /// [1]: https://tinyurl.com/w3-streams#event-mediastreamtrack-ended
    on_ended: RefCell<
        Option<EventListener<web_sys::MediaStreamTrack, web_sys::Event>>,
    >,

    /// Listener audio level change.
    on_audio_level: OnceCell<RefCell<AudioLevelProvider>>,
}

lazy_static! {
    /// Url to the audio level calculation module.
    static ref AUDIO_LEVEL_PROCESSOR_URL: String = {
        let generate_processor = js_sys::Function::new_no_args(
            "
            function generateProcessor()
            {
                return (`
                    class AudioLevelProcessor extends AudioWorkletProcessor
                    {
                        process(inputs, outputs)
                        {
                            if (inputs.length > 0) {
                                const samples = inputs[0][0];
                                let sum = 0.0;
                                for (let i = 0; i < samples.length; ++i)
                                {
                                    sum += samples[i] * samples[i];
                                }
                                let rms = Math.sqrt(sum / samples.length);
                                this.port.postMessage(rms);
                            }
                            return true;
                        }
                    }
                    registerProcessor('audio-level-processor',
                    AudioLevelProcessor);
                    `);
                }
                return URL.createObjectURL(
                    new Blob([generateProcessor()],
                {type: \"application/javascript\"})
            );
            ",
    );

    generate_processor
    .call0(&wasm_bindgen::JsValue::from_str(""))
    .map(|js| js.as_string().unwrap_or_default())
    .unwrap_or_default()
};

/// Registration `AudioLevelProcessor`.
static ref MODULE_REGISTRATION: AsyncOnce<wasm_bindgen::JsValue> =
    #[allow(clippy::unwrap_used)]
    AsyncOnce::new(
    async {
        let context = web_sys::AudioContext::new().unwrap();

        JsFuture::from(
            context
                .audio_worklet()
                .unwrap()
                .add_module(&AUDIO_LEVEL_PROCESSOR_URL)
                .unwrap(),
        )
        .await.unwrap()
    });
}

impl MediaStreamTrack {
    /// Creates a new [`MediaStreamTrack`].
    #[must_use]
    pub fn new<T>(sys_track: T, source_kind: Option<MediaSourceKind>) -> Self
    where
        web_sys::MediaStreamTrack: From<T>,
    {
        let sys_track = web_sys::MediaStreamTrack::from(sys_track);
        let kind = match sys_track.kind().as_ref() {
            "audio" => MediaKind::Audio,
            "video" => MediaKind::Video,
            _ => unreachable!(),
        };
        Self {
            sys_track: Rc::new(sys_track),
            source_kind,
            kind,
            on_ended: RefCell::new(None),
            on_audio_level: OnceCell::new(),
        }
    }

    /// Returns [`id`] of the underlying [MediaStreamTrack][2].
    ///
    /// [`id`]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-id
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn id(&self) -> String {
        self.sys_track.id()
    }

    /// Returns this [`MediaStreamTrack`]'s kind (audio/video).
    #[must_use]
    pub const fn kind(&self) -> MediaKind {
        self.kind
    }

    /// Returns [MediaStreamTrackState][1] of the underlying
    /// [MediaStreamTrack][2].
    ///
    /// # Panics
    ///
    /// If [`readyState`][3] property of underlying [MediaStreamTrack][2] is
    /// neither `live` nor `ended`.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrackstate
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    /// [3]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-readystate
    #[allow(clippy::unused_async)] // for platform code uniformity
    pub async fn ready_state(&self) -> MediaStreamTrackState {
        let state = self.sys_track.ready_state();
        match state {
            web_sys::MediaStreamTrackState::Live => MediaStreamTrackState::Live,
            web_sys::MediaStreamTrackState::Ended => {
                MediaStreamTrackState::Ended
            }
            web_sys::MediaStreamTrackState::__Nonexhaustive => {
                unreachable!("Unknown `MediaStreamTrackState`: {state:?}")
            }
        }
    }

    /// Returns a [`deviceId`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// # Panics
    ///
    /// If the underlying [MediaStreamTrack][2] doesn't have [`deviceId`][1].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-deviceid
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn device_id(&self) -> String {
        #[allow(clippy::unwrap_used)]
        get_property_by_name(&self.sys_track.get_settings(), "deviceId", |v| {
            v.as_string()
        })
        .unwrap()
    }

    /// Return a [`facingMode`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-facingmode
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn facing_mode(&self) -> Option<FacingMode> {
        let facing_mode = get_property_by_name(
            &self.sys_track.get_settings(),
            "facingMode",
            |v| v.as_string(),
        );
        facing_mode.and_then(|fm| match fm.as_ref() {
            "user" => Some(FacingMode::User),
            "environment" => Some(FacingMode::Environment),
            "left" => Some(FacingMode::Left),
            "right" => Some(FacingMode::Right),
            _ => {
                log::error!("Unknown FacingMode: {fm}");
                None
            }
        })
    }

    /// Returns a [`height`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediatracksettings-height
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn height(&self) -> Option<u32> {
        #[allow(
            clippy::as_conversions,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        get_property_by_name(&self.sys_track.get_settings(), "height", |h| {
            h.as_f64().map(|v| v as u32)
        })
    }

    /// Return a [`width`][1] of the underlying [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediatracksettings-width
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn width(&self) -> Option<u32> {
        #[allow(
            clippy::as_conversions,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        get_property_by_name(&self.sys_track.get_settings(), "width", |w| {
            w.as_f64().map(|v| v as u32)
        })
    }

    /// Changes an [`enabled`][1] attribute in the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    pub fn set_enabled(&self, enabled: bool) {
        self.sys_track.set_enabled(enabled);
    }

    /// Changes a [`readyState`][1] attribute in the underlying
    /// [MediaStreamTrack][2] to [`ended`][3].
    ///
    /// [1]: https://tinyurl.com/w3-streams#dom-mediastreamtrack-readystate
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    /// [3]: https://tinyurl.com/w3-streams#idl-def-MediaStreamTrackState.ended
    pub fn stop(&self) -> impl Future<Output = ()> + 'static {
        self.sys_track.stop();
        // For platform code uniformity.
        future::ready(())
    }

    /// Returns an [`enabled`][1] attribute of the underlying
    /// [MediaStreamTrack][2].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-enabled
    /// [2]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    #[must_use]
    pub fn enabled(&self) -> bool {
        self.sys_track.enabled()
    }

    /// Detects whether a video track captured from display searching
    /// [specific fields][1] in its settings.
    ///
    /// Only works in Chrome browser at the moment.
    ///
    /// [1]: https://w3.org/TR/screen-capture/#extensions-to-mediatracksettings
    #[must_use]
    pub fn guess_is_from_display(&self) -> bool {
        self.source_kind == Some(MediaSourceKind::Display)
    }

    /// Forks this [`MediaStreamTrack`].
    ///
    /// Creates a new [`MediaStreamTrack`] from this [`MediaStreamTrack`] using
    /// a [`clone()`][1] method. It won't clone current [`MediaStreamTrack`]'s
    /// callbacks.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-clone
    pub fn fork(&self) -> impl Future<Output = Self> + 'static {
        future::ready(Self {
            sys_track: Rc::new(web_sys::MediaStreamTrack::clone(
                &self.sys_track,
            )),
            kind: self.kind,
            source_kind: self.source_kind,
            on_ended: RefCell::new(None),
            on_audio_level: OnceCell::new(),
        })
    }

    /// Sets handler for the [`ended`][1] event on underlying
    /// [`web_sys::MediaStreamTrack`].
    ///
    /// # Panics
    ///
    /// If binding to the [`ended`][1] event fails. Not supposed to ever happen.
    ///
    /// [1]: https://tinyurl.com/w3-streams#event-mediastreamtrack-ended
    pub fn on_ended<F>(&self, f: Option<F>)
    where
        F: 'static + FnOnce(),
    {
        let mut on_ended = self.on_ended.borrow_mut();
        drop(match f {
            None => on_ended.take(),
            Some(f) => {
                on_ended.replace(
                    // PANIC: Unwrapping is OK here, because this function
                    //        shouldn't error ever.
                    #[allow(clippy::unwrap_used)]
                    EventListener::new_once(
                        Rc::clone(&self.sys_track),
                        "ended",
                        move |_| {
                            f();
                        },
                    )
                    .unwrap(),
                )
            }
        });
    }

    /// Sets a callback to invoke when this [`MediaStreamTrack`]
    /// receive audio level.
    #[allow(clippy::unwrap_used, clippy::missing_panics_doc)]
    pub async fn on_audio_level<F>(&self, f: Option<F>)
    where
        F: 'static + FnMut(f64),
    {
        if self.kind == MediaKind::Audio {
            if let Some(provider) = self.on_audio_level.get() {
                provider.borrow_mut().set_audio_level_cb(f);
            } else {
                drop(self.on_audio_level.set(RefCell::new(
                    AudioLevelProvider::new(&self.sys_track).await,
                )));
                self.on_audio_level
                    .get()
                    .unwrap()
                    .borrow_mut()
                    .set_audio_level_cb(f);
            }
        }
    }
}
