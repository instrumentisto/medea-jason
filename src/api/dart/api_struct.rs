use flutter_rust_bridge::frb;

use crate::media::{constraints::ConstrainU32, FacingMode, MediaDeviceKind};

/// Representation of a [`ApiMediaDeviceInfo`][0] ONLY for input devices.
///
/// [0]: https://w3.org/TR/mediacapture-streams#device-info
#[derive(Debug)]
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

    /// Indicates whether the last attempt to use the provided device
    /// failed.
    pub(crate) is_failed: bool,
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
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
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

/// Constraints applicable to video tracks that are sourced from some media
/// device.
#[frb]
#[derive(Debug)]
pub struct ApiDeviceVideoTrackConstrs {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub(crate) device_id: Option<String>,

    /// Describes the directions that the camera can face, as seen from the
    /// user's perspective.
    #[frb(non_final)]
    pub(crate) facing_mode: Option<ApiConstrainFacingMode>,

    /// Height of the video in pixels.
    #[frb(non_final)]
    pub(crate) height: Option<ConstrainU32>,

    /// Width of the video in pixels.
    #[frb(non_final)]
    pub(crate) width: Option<ConstrainU32>,
}

impl From<ApiDeviceVideoTrackConstrs>
    for crate::media::constraints::DeviceVideoTrackConstraints
{
    fn from(value: ApiDeviceVideoTrackConstrs) -> Self {
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
pub struct ApiDisplayVideoTrackConstrs {
    /// Identifier of the device generating the content for the media track.
    #[frb(non_final)]
    pub(crate) device_id: Option<String>,

    /// [Height][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
    #[frb(non_final)]
    pub(crate) height: Option<ConstrainU32>,

    /// [Width][1] of the video in pixels.
    ///
    /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
    #[frb(non_final)]
    pub(crate) width: Option<ConstrainU32>,

    /// [Frame rate][1] of the video.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
    #[frb(non_final)]
    pub(crate) frame_rate: Option<ConstrainU32>,
}

impl From<ApiDisplayVideoTrackConstrs>
    for crate::media::constraints::DisplayVideoTrackConstraints
{
    fn from(value: ApiDisplayVideoTrackConstrs) -> Self {
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
    pub(crate) audio: Option<ApiAudioTrackConstrs>,

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
