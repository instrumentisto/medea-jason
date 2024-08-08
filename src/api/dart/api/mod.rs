//! External [`Jason`] API exposing functions that can be called via FFI and
//! designed to be integrated into a [Flutter] plugin.
//!
//! [Flutter]: https://flutter.dev

#![allow(
clippy::as_conversions,
clippy::doc_markdown, // TODO: From generated code in #[frb].
clippy::missing_panics_doc,
clippy::needless_pass_by_value,
clippy::undocumented_unsafe_blocks,
clippy::unwrap_used,
non_snake_case,
)]

#[allow(
    clippy::absolute_paths,
    clippy::as_conversions,
    clippy::default_trait_access,
    clippy::let_underscore_untyped,
    clippy::missing_docs_in_private_items,
    clippy::multiple_unsafe_ops_per_block,
    clippy::ptr_as_ptr,
    clippy::undocumented_unsafe_blocks,
    clippy::empty_structs_with_brackets,
    clippy::use_self,
    clippy::wildcard_imports,
    let_underscore_drop,
    unused_qualifications,
    unit_bindings
)]
#[rustfmt::skip]
mod api_bridge_generated;

pub mod connection_handle;
pub mod jason;
pub mod local_media_track;
pub mod media_manager;
pub mod reconnect_handle;
pub mod remote_media_track;
pub mod room;
pub mod room_close_reason;

use std::ptr;

use flutter_rust_bridge::{frb, DartOpaque};

use crate::{
    api::ForeignClass,
    media::{
        self,
        constraints::{ConstrainBoolean, ConstrainU32},
        MediaDeviceKind,
    },
    platform::{self},
};

pub use dart_sys::Dart_Handle;

pub use self::{
    connection_handle::ConnectionHandle, jason::JasonHandle,
    local_media_track::LocalMediaTrack, media_manager::MediaManagerHandle,
    reconnect_handle::ReconnectHandle, remote_media_track::RemoteMediaTrack,
    room::RoomHandle, room_close_reason::RoomCloseReason,
};

/// Representation of a [MediaDeviceInfo][0] ONLY for input devices.
///
/// [0]: https://w3.org/TR/mediacapture-streams#device-info
#[derive(Debug)]
pub struct ApiMediaDeviceDetails {
    /// [`MediaDeviceKind`] of this [`ApiMediaDeviceDetails`].
    ///
    /// [`MediaDeviceKind`]: MediaDeviceKind
    pub kind: MediaDeviceKind,

    /// Unique identifier of the device represented by this
    /// [`ApiMediaDeviceDetails`].
    pub device_id: String,

    /// Label describing the device represented by this
    /// [`ApiMediaDeviceDetails`] (for example, "External USB Webcam").
    pub label: String,

    /// Group identifier of the device represented by this
    /// [`ApiMediaDeviceDetails`].
    ///
    /// Two devices have the same group identifier if they belong to the same
    /// physical device. For example, the audio input and output devices
    /// representing the speaker and microphone of the same headset have the
    /// same [groupId][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
    pub group_id: Option<String>,

    /// Indicates whether the last attempt to use the provided device failed.
    pub is_failed: bool,
}

/// Representation of a display source.
#[derive(Debug)]
pub struct ApiMediaDisplayDetails {
    /// Unique identifier of the display represented by this
    /// [`ApiMediaDisplayDetails`].
    pub device_id: String,

    /// Title describing the represented display.
    pub title: Option<String>,
}

/// Constraints applicable to audio tracks.
#[derive(Debug)]
#[frb]
pub struct ApiAudioConstraints {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub device_id: Option<String>,

    /// Automatically manages changes in the volume of its source media to
    /// maintain a steady overall volume level.
    #[frb(non_final)]
    pub auto_gain_control: Option<ConstrainBoolean>,
}

impl From<ApiAudioConstraints> for media::AudioTrackConstraints {
    fn from(value: ApiAudioConstraints) -> Self {
        let mut res = Self::new();
        if let Some(id) = value.device_id {
            res.device_id(id);
        }
        if let Some(auto_gain_control) = value.auto_gain_control {
            match auto_gain_control {
                ConstrainBoolean::Exact(e) => res.exact_auto_gain_control(e),
                ConstrainBoolean::Ideal(i) => res.ideal_auto_gain_control(i),
            }
        }
        res
    }
}

/// [facingMode] constraint.
///
/// Can set exact (must be the parameter's value) and ideal (should be used if
/// possible) constrain.
///
/// [facingMode]: https://tinyurl.com/w3-streams#def-constraint-facingMode
#[derive(Copy, Clone, Debug)]
pub enum ApiConstrainFacingMode {
    /// Exact value required for this property.
    Exact(media::FacingMode),

    /// Ideal (target) value for this property.
    Ideal(media::FacingMode),
}

/// Constraints applicable to video tracks that are sourced from some media
/// device.
#[derive(Debug)]
#[frb]
pub struct ApiDeviceVideoTrackConstraints {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub device_id: Option<String>,

    /// Describes the directions that the camera can face, as seen from the
    /// user's perspective.
    #[frb(non_final)]
    pub facing_mode: Option<ApiConstrainFacingMode>,

    /// Height of the video in pixels.
    #[frb(non_final)]
    pub height: Option<ConstrainU32>,

    /// Width of the video in pixels.
    #[frb(non_final)]
    pub width: Option<ConstrainU32>,
}

impl From<ApiDeviceVideoTrackConstraints>
    for media::DeviceVideoTrackConstraints
{
    fn from(value: ApiDeviceVideoTrackConstraints) -> Self {
        let mut res = Self::new();
        if let Some(id) = value.device_id {
            res.device_id(id);
        }
        if let Some(mode) = value.facing_mode {
            match mode {
                ApiConstrainFacingMode::Exact(e) => res.exact_facing_mode(e),
                ApiConstrainFacingMode::Ideal(i) => res.ideal_facing_mode(i),
            }
        }

        if let Some(height) = value.height {
            match height {
                ConstrainU32::Exact(e) => res.exact_height(e),
                ConstrainU32::Ideal(i) => res.ideal_height(i),
                ConstrainU32::Range(min, max) => res.height_in_range(min, max),
            }
        }

        if let Some(width) = value.width {
            match width {
                ConstrainU32::Exact(e) => res.exact_width(e),
                ConstrainU32::Ideal(i) => res.ideal_width(i),
                ConstrainU32::Range(min, max) => res.width_in_range(min, max),
            }
        }
        res
    }
}

/// Constraints applicable to video tracks sourced from a screen capturing.
#[derive(Debug)]
#[frb]
pub struct ApiDisplayVideoTrackConstraints {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub device_id: Option<String>,

    /// [Height][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    #[frb(non_final)]
    pub height: Option<ConstrainU32>,

    /// [Width][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    #[frb(non_final)]
    pub width: Option<ConstrainU32>,

    /// [Frame rate][1] of the video.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
    #[frb(non_final)]
    pub frame_rate: Option<ConstrainU32>,
}

impl From<ApiDisplayVideoTrackConstraints>
    for media::DisplayVideoTrackConstraints
{
    fn from(value: ApiDisplayVideoTrackConstraints) -> Self {
        let mut res = Self::new();
        if let Some(id) = value.device_id {
            res.device_id(id);
        }

        if let Some(height) = value.height {
            match height {
                ConstrainU32::Exact(e) => res.exact_height(e),
                ConstrainU32::Ideal(i) => res.ideal_height(i),
                ConstrainU32::Range(..) => unreachable!(),
            }
        }

        if let Some(width) = value.width {
            match width {
                ConstrainU32::Exact(e) => res.exact_width(e),
                ConstrainU32::Ideal(i) => res.ideal_width(i),
                ConstrainU32::Range(..) => unreachable!(),
            }
        }

        if let Some(frame_rate) = value.frame_rate {
            match frame_rate {
                ConstrainU32::Exact(e) => res.exact_frame_rate(e),
                ConstrainU32::Ideal(i) => res.ideal_frame_rate(i),
                ConstrainU32::Range(..) => unreachable!(),
            }
        }
        res
    }
}

/// [MediaStreamConstraints][1] wrapper.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
#[derive(Debug)]
#[frb]
pub struct ApiMediaStreamSettings {
    /// [MediaStreamConstraints][1] for the audio media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    #[frb(non_final)]
    pub audio: Option<ApiAudioConstraints>,

    /// [MediaStreamConstraints][1] for the device video media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    #[frb(non_final)]
    pub device_video: Option<ApiDeviceVideoTrackConstraints>,

    /// [MediaStreamConstraints][1] for the display video media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    #[frb(non_final)]
    pub display_video: Option<ApiDisplayVideoTrackConstraints>,
}

impl From<ApiMediaStreamSettings> for media::MediaStreamSettings {
    fn from(value: ApiMediaStreamSettings) -> Self {
        let mut res = Self::new();
        if let Some(audio) = value.audio {
            res.audio(audio.into());
        }
        if let Some(device) = value.device_video {
            res.device_video(device.into());
        }
        if let Some(display) = value.display_video {
            res.display_video(display.into());
        }
        res
    }
}

impl<T> ForeignClass for Vec<T> {}

/// Returns the [`Vec<ApiMediaDeviceDetails>`] from the [`ForeignClass`]
/// address.
#[frb(sync, type_64bit_int)]
#[must_use]
pub fn vec_media_device_details_from_raw(
    ptr: usize,
) -> Vec<ApiMediaDeviceDetails> {
    unsafe {
        Vec::<ApiMediaDeviceDetails>::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        )
    }
}

/// Returns the [`Vec<RustOpaque<ApiMediaDisplayDetails>>`] from the
/// [`ForeignClass`] address.
#[frb(sync, type_64bit_int)]
#[must_use]
pub fn vec_media_display_details_from_raw(
    ptr: usize,
) -> Vec<ApiMediaDisplayDetails> {
    unsafe {
        Vec::<ApiMediaDisplayDetails>::from_ptr(
            ptr::NonNull::new(ptr as _).unwrap(),
        )
    }
}

impl ForeignClass for RoomCloseReason {}

/// Logs Dart exception.
#[frb(sync)]
#[must_use]
pub fn log_dart_exception(message: String, stack_trace: String) {
    log::error!("{message}\n{stack_trace}");
}

/// Sets the provided [`Dart_Handle`] as a callback for the Rust panic hook.
#[frb(sync)]
#[must_use]
pub fn on_panic(cb: DartOpaque) {
    platform::set_panic_callback(platform::Function::new(cb));
}