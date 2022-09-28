use std::ptr;

use medea_client_api_proto::stats::{Float, MediaSourceStats};

use crate::{api::DartValueArg, platform::utils::NonNullDartValueArgExt};

#[allow(missing_copy_implementations)]
#[derive(Debug)]
#[repr(C)]
pub struct RTCMediaSourceFfiStats {
    /// Value of the [MediaStreamTrack][1]'s ID attribute.
    ///
    /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
    track_identifier: ptr::NonNull<DartValueArg<Option<String>>>,

    /// Fields which should be in the [`RtcStat`] based on `kind`.
    kind: ptr::NonNull<RTCMediaSourceFfiStatsMediaType>,
}

// Type-erased value that can be transferred via Ffi boundaries to/from Dart.
#[allow(missing_copy_implementations, dead_code)] // not trivially copyable
#[derive(Debug)]
#[repr(u8)]
/// [`RtcStat`] fields of [`RtcStatsType::MediaSource`] type based on its
/// `kind`.
enum RTCMediaSourceFfiStatsMediaType {
    /// [`RtcStat`] fields of [`RtcStatsType::MediaSource`]
    /// type based on video kind.
    Video(ptr::NonNull<RTCVideoSourceFfiStats>),
    /// [`RtcStat`] fields of [`RtcStatsType::MediaSource`]
    /// type based on audio kind.
    Audio(ptr::NonNull<RTCAudioSourceFfiStats>),
}

#[allow(clippy::fallible_impl_from)]
impl From<RTCMediaSourceFfiStatsMediaType>
    for medea_client_api_proto::stats::MediaKind
{
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn from(stats: RTCMediaSourceFfiStatsMediaType) -> Self {
        match stats {
            RTCMediaSourceFfiStatsMediaType::Video(stats) => {
                let stats = unsafe { stats.unbox() };
                Self::Video {
                    width: Option::try_from(unsafe { stats.width.unbox() })
                        .unwrap(),
                    height: Option::try_from(unsafe { stats.height.unbox() })
                        .unwrap(),
                    frames_per_second: Option::<f64>::try_from(unsafe {
                        stats.frames_per_second.unbox()
                    })
                    .unwrap()
                    .map(|s| s as u32),
                }
            }
            RTCMediaSourceFfiStatsMediaType::Audio(stats) => {
                let stats = unsafe { stats.unbox() };
                Self::Audio {
                    audio_level: Option::try_from(unsafe {
                        stats.audio_level.unbox()
                    })
                    .unwrap()
                    .map(Float),
                    total_audio_energy: Option::try_from(unsafe {
                        stats.total_audio_energy.unbox()
                    })
                    .unwrap()
                    .map(Float),
                    total_samples_duration: Option::try_from(unsafe {
                        stats.total_samples_duration.unbox()
                    })
                    .unwrap()
                    .map(Float),
                }
            }
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<RTCMediaSourceFfiStats> for MediaSourceStats {
    fn from(stats: RTCMediaSourceFfiStats) -> Self {
        Self {
            track_identifier: Option::try_from(unsafe {
                stats.track_identifier.unbox()
            })
            .unwrap(),
            kind: medea_client_api_proto::stats::MediaKind::from(
                Option::try_from(unsafe { stats.kind.unbox() })
                    .unwrap()
                    .unwrap(),
            ),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
#[allow(missing_copy_implementations)]
    /// [`RtcStat`] fields of [`RtcStatsType::MediaSource`]
    /// type based on video kind.
struct RTCVideoSourceFfiStats {
    /// Width (in pixels) of the last frame originating from the source.
    /// Before a frame has been produced this attribute is missing.
    width: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Height (in pixels) of the last frame originating from the source.
    /// Before a frame has been produced this attribute is missing.
    height: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// The total number of frames originating from this source.
    frames: ptr::NonNull<DartValueArg<Option<u32>>>,

    /// Number of frames originating from the source, measured during the
    /// last second. For the first second of this object's lifetime this
    /// attribute is missing.
    frames_per_second: ptr::NonNull<DartValueArg<Option<f64>>>,
}

#[derive(Debug)]
#[repr(C)]
#[allow(missing_copy_implementations)]
    /// [`RtcStat`] fields of [`RtcStatsType::MediaSource`]
    /// type based on audio kind.
struct RTCAudioSourceFfiStats {
    /// Audio level of the media source.
    audio_level: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Audio energy of the media source.
    total_audio_energy: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Audio duration of the media source.
    total_samples_duration: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Only exists when the MediaStreamTrack is sourced
    /// from a microphone where echo cancellation is applied.
    echo_return_loss: ptr::NonNull<DartValueArg<Option<f64>>>,

    /// Only exists when the [`MediaStreamTrack`]
    /// is sourced from a microphone where
    /// echo cancellation is applied.
    echo_return_loss_enhancement: ptr::NonNull<DartValueArg<Option<f64>>>,
}
