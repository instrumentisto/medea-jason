use flutter_rust_bridge::frb;

use crate::media::{
    constraints::{ConstrainString, ConstrainU32},
    FacingMode, MediaDeviceKind,
};

#[derive(Debug)]
/// Representation of a [`ApiMediaDeviceInfo`][0] ONLY for input devices.
///
/// [0]: https://w3.org/TR/mediacapture-streams#device-info
pub struct ApiMediaDeviceInfo {
    /// [`MediaDeviceKind`] of this [`ApiMediaDeviceInfo`].
    pub(crate) kind: MediaDeviceKind,

    /// Unique identifier of the device represented by this
    /// [`ApiMediaDeviceInfo`].
    pub(crate) device_id: String,

    /// Label describing the device represented by this
    /// [`ApiMediaDeviceInfo`] (for example, "External USB Webcam").
    pub(crate) label: String,

    /// Group identifier of the device represented by this
    /// [`ApiMediaDeviceInfo`]
    ///
    /// Two devices have the same group identifier if they belong to the same
    /// physical device. For example, the audio input and output devices
    /// representing the speaker and microphone of the same headset have the
    /// same [groupId][1].
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
    pub(crate) group_id: Option<String>,
}

/// Representation of a Option [ConstrainULong][1].
///
/// Underlying value must fit in a `[0, 4294967295]` range.
///
/// [1]: https://tinyurl.com/w3-streams#dom-constrainulong
#[derive(Copy, Clone, Debug)]
pub enum ApiOptionConstrainU32 {
    Some(ConstrainU32),

    // TODO Option<Enum>
    // https://github.com/fzyzcjy/flutter_rust_bridge/issues/828
    /// Lack of value.
    None,
}

impl TryFrom<ApiOptionConstrainU32> for ConstrainU32 {
    type Error = ApiOptionConstrainU32;

    fn try_from(value: ApiOptionConstrainU32) -> Result<Self, Self::Error> {
        match value {
            ApiOptionConstrainU32::Some(v) => Ok(v),
            ApiOptionConstrainU32::None => Err(ApiOptionConstrainU32::None),
        }
    }
}

/// Representation of a display source.
#[derive(Debug)]
pub struct ApiMediaDisplayInfo {
    /// Unique identifier of the display represented by this
    /// [`ApiMediaDisplayInfo`].
    pub(crate) device_id: String,

    /// Title describing the represented display.
    pub(crate) title: Option<String>,
}

#[derive(Debug)]
#[frb]
pub struct ApiAudioTrackConstrs {
    #[frb(non_final)]
    /// Identifier of the device generating the content for the media track.
    pub(crate) device_id: Option<String>,
}

impl From<ApiAudioTrackConstrs>
    for crate::media::constraints::AudioTrackConstraints
{
    fn from(value: ApiAudioTrackConstrs) -> Self {
        let mut res = Self::new();
        if let Some(id) = value.device_id {
            res.device_id(id);
        }
        res
    }
}

/// Representation of the [`ConstrainString<FacingMode>`]
/// [ConstrainDOMString][1].
///
/// Can set exact (must be the parameter's value) and ideal (should be used if
/// possible) constrain.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
#[derive(Copy, Clone, Debug)]
pub enum ApiConstrainFacingMode {
    /// Exact value required for this property.
    Exact(FacingMode),

    /// Ideal (target) value for this property.
    Ideal(FacingMode),
}

/// Representation of the [`Option<ConstrainString<FacingMode>>`]
#[derive(Copy, Clone, Debug)]
pub enum ApiOptionConstrainFacingMode {
    Some(ApiConstrainFacingMode),

    // TODO Option<Enum>
    // https://github.com/fzyzcjy/flutter_rust_bridge/issues/828
    None,
}

impl TryFrom<ApiOptionConstrainFacingMode> for ConstrainString<FacingMode> {
    type Error = ApiOptionConstrainFacingMode;

    fn try_from(
        value: ApiOptionConstrainFacingMode,
    ) -> Result<Self, Self::Error> {
        match value {
            ApiOptionConstrainFacingMode::Some(m) => Ok(match m {
                ApiConstrainFacingMode::Exact(e) => Self::Exact(e),
                ApiConstrainFacingMode::Ideal(i) => Self::Ideal(i),
            }),
            ApiOptionConstrainFacingMode::None => {
                Err(ApiOptionConstrainFacingMode::None)
            }
        }
    }
}

/// Constraints applicable to video tracks that are sourced from some media
/// device.
#[derive(Debug)]
#[frb]
pub struct ApiDeviceVideoTrackConstrs {
    #[frb(non_final)]
    /// Identifier of the device generating the content for the media track.
    pub(crate) device_id: Option<String>,

    /// Describes the directions that the camera can face, as seen from the
    /// user's perspective.
    #[frb(non_final)]
    // TODO Delete Box https://github.com/fzyzcjy/flutter_rust_bridge/pull/901
    pub(crate) facing_mode: Box<ApiOptionConstrainFacingMode>,

    /// Height of the video in pixels.
    #[frb(non_final)]
    // TODO Delete Box https://github.com/fzyzcjy/flutter_rust_bridge/pull/901
    pub(crate) height: Box<ApiOptionConstrainU32>,

    /// Width of the video in pixels.
    #[frb(non_final)]
    // TODO Delete Box https://github.com/fzyzcjy/flutter_rust_bridge/pull/901
    pub(crate) width: Box<ApiOptionConstrainU32>,
}

impl From<ApiDeviceVideoTrackConstrs>
    for crate::media::constraints::DeviceVideoTrackConstraints
{
    fn from(value: ApiDeviceVideoTrackConstrs) -> Self {
        let mut res = Self::new();
        if let Some(id) = value.device_id {
            res.device_id(id);
        }
        if let Ok(mode) = (*value.facing_mode).try_into() {
            match mode {
                ConstrainString::Exact(e) => res.exact_facing_mode(e),
                ConstrainString::Ideal(i) => res.ideal_facing_mode(i),
            }
        }

        if let Ok(height) = (*value.height).try_into() {
            match height {
                ConstrainU32::Exact(e) => res.exact_height(e),
                ConstrainU32::Ideal(i) => res.ideal_height(i),
                ConstrainU32::Range(min, max) => res.height_in_range(min, max),
            }
        }

        if let Ok(width) = (*value.width).try_into() {
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
pub struct ApiDisplayVideoTrackConstrs {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub(crate) device_id: Option<String>,

    /// [Height][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    #[frb(non_final)]
    // TODO Delete Box https://github.com/fzyzcjy/flutter_rust_bridge/pull/901
    pub(crate) height: Box<ApiOptionConstrainU32>,

    /// [Width][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    #[frb(non_final)]
    // TODO Delete Box https://github.com/fzyzcjy/flutter_rust_bridge/pull/901
    pub(crate) width: Box<ApiOptionConstrainU32>,

    /// [Frame rate][1] of the video.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
    #[frb(non_final)]
    // TODO Delete Box https://github.com/fzyzcjy/flutter_rust_bridge/pull/901
    pub(crate) frame_rate: Box<ApiOptionConstrainU32>,
}

impl From<ApiDisplayVideoTrackConstrs>
    for crate::media::constraints::DisplayVideoTrackConstraints
{
    fn from(value: ApiDisplayVideoTrackConstrs) -> Self {
        let mut res = Self::new();
        if let Some(id) = value.device_id {
            res.device_id(id);
        }

        if let Ok(height) = (*value.height).try_into() {
            match height {
                ConstrainU32::Exact(e) => res.exact_height(e),
                ConstrainU32::Ideal(i) => res.ideal_height(i),
                ConstrainU32::Range(..) => unreachable!(),
            }
        }

        if let Ok(width) = (*value.width).try_into() {
            match width {
                ConstrainU32::Exact(e) => res.exact_width(e),
                ConstrainU32::Ideal(i) => res.ideal_width(i),
                ConstrainU32::Range(..) => unreachable!(),
            }
        }

        if let Ok(frame_rate) = (*value.frame_rate).try_into() {
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
    // TODO Delete Box https://github.com/fzyzcjy/flutter_rust_bridge/pull/901
    pub(crate) audio: Box<ApiAudioTrackConstrs>,

    /// [MediaStreamConstraints][1] for the device video media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    #[frb(non_final)]
    pub(crate) device_video: Option<ApiDeviceVideoTrackConstrs>,

    /// [MediaStreamConstraints][1] for the display video media type.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
    #[frb(non_final)]
    pub(crate) display_video: Option<ApiDisplayVideoTrackConstrs>,
}

impl From<ApiMediaStreamSettings> for crate::media::MediaStreamSettings {
    fn from(value: ApiMediaStreamSettings) -> Self {
        let mut res = Self::new();
        res.audio((*value.audio).into());
        if let Some(device) = value.device_video {
            res.device_video(device.into());
        }
        if let Some(display) = value.display_video {
            res.display_video(display.into());
        }
        res
    }
}
