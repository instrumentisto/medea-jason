import 'dart:ffi';

import 'package:ffi/ffi.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as medea;

import '../../util/move_semantic.dart';
import '../ffi/foreign_value.dart';
import '../jason.dart';

typedef _boxForeignStats_C = Pointer Function(RTCFfiStats);
typedef _boxForeignStats_Dart = Pointer Function(RTCFfiStats);

final _boxForeignStats_Dart _boxForeignStats =
    dl.lookupFunction<_boxForeignStats_C, _boxForeignStats_Dart>(
        'box_foreign_stats');

class RTCFfiStats extends Struct {
  RTCFfiStats._();

  /// Unique ID that is associated with the object that was inspected to
  /// produce this [RTCStats] object.
  ///
  /// [RTCStats]: https://w3.org/TR/webrtc#dom-rtcstats
  external Pointer<Utf8> id;

  /// Timestamp associated with this object.
  ///
  /// The time is relative to the UNIX epoch (Jan 1, 1970, UTC).
  ///
  /// For statistics that came from a remote source (e.g., from received RTCP
  /// packets), timestamp represents the time at which the information
  /// arrived at the local endpoint. The remote timestamp can be found in an
  /// additional field in an [`RtcStat`]-derived dictionary, if applicable.
  @Int64()
  external int timestampUs;

  /// Actual stats of this [`RtcStat`].
  ///
  /// All possible stats are described in the [`RtcStatsType`] enum.
  external Pointer<RTCStatsType> type;

  /// Allocates a new [ForeignValue] with the provided pointer to some Rust
  /// object.
  static Pointer<RTCFfiStats> fromDartStats(medea.RTCStats stats) {
    var fVal = calloc<RTCFfiStats>();

    fVal.ref.id = stats.id.toNativeUtf8();
    fVal.ref.timestampUs = stats.timestampUs;
    fVal.ref.type = RTCStatsType.fromDartStats(stats.type);

    return fVal;
  }
}

extension RTCStatsPointer on Pointer<RTCFfiStats> {
  /// Transfers [RTCFfiStats] ownership to Rust.
  ///
  /// Frees Dart side [RTCFfiStats].
  Pointer<NativeType> intoRustOwned() {
    var out = _boxForeignStats(ref);
    calloc.free(this);
    return out;
  }

  /// Releases the memory allocated on a native heap.
  @moveSemantics
  void free() {
    calloc.free(this);
  }
}

class RTCStatsType extends Struct {
  RTCStatsType._();

  /// 0 -> Unimplemented.
  @Int32()
  // ignore: unused_field
  external int _tag;

  /// Actual stats of this [`RtcStat`].
  ///
  /// All possible stats are described in the [`RtcStatsType`] enum.
  external _ForeignValueStats _payload;

  static Pointer<RTCStatsType> fromDartStats(medea.RtcStatsType? type) {
    var fVal = calloc<RTCStatsType>();
    var strType = type.runtimeType.toString();
    switch (strType) {
      case 'RtcTransportStats':
        {
          fVal.ref._tag = 1;
          fVal.ref._payload.transport =
              RTCTransportStats.fromDartStats(type as medea.RtcTransportStats);
          return fVal;
        }
      case 'RtcAudioSourceStats':
        {
          fVal.ref._tag = 2;
          fVal.ref._payload.mediaSource = RTCMediaSourceStats.fromDartStats(
              type as medea.RtcMediaSourceStats);
          return fVal;
        }
      case 'RtcVideoSourceStats':
        {
          fVal.ref._tag = 2;
          fVal.ref._payload.mediaSource = RTCMediaSourceStats.fromDartStats(
              type as medea.RtcMediaSourceStats);
          return fVal;
        }
      case 'RtcRemoteIceCandidateStats':
        {
          fVal.ref._tag = 3;
          fVal.ref._payload.iceCandidate = RTCIceCandidateStats.fromDartStats(
              type as medea.RtcIceCandidateStats);
          return fVal;
        }
      case 'RtcLocalIceCandidateStats':
        {
          fVal.ref._tag = 3;
          fVal.ref._payload.iceCandidate = RTCIceCandidateStats.fromDartStats(
              type as medea.RtcIceCandidateStats);
          return fVal;
        }
      case 'RtcOutboundRTPStreamStats':
        {
          fVal.ref._tag = 4;
          fVal.ref._payload.outboundRTPStream =
              RTCOutboundRTPStreamStats.fromDartStats(
                  type as medea.RtcOutboundRTPStreamStats);
          return fVal;
        }
      case 'RtcInboundRTPStreamStats':
        {
          fVal.ref._tag = 5;
          fVal.ref._payload.inboundRTPStream =
              RTCInboundRTPStreamStats.fromDartStats(
                  type as medea.RtcInboundRTPStreamStats);
          return fVal;
        }
      case 'RtcIceCandidatePairStats':
        {
          fVal.ref._tag = 6;
          fVal.ref._payload.iceCandidatePair =
              RTCIceCandidatePairStats.fromDartStats(
                  type as medea.RtcIceCandidatePairStats);
          return fVal;
        }
      case 'RtcRemoteInboundRtpStreamStats':
        {
          fVal.ref._tag = 7;
          fVal.ref._payload.remoteInboundRTPStream =
              RTCRemoteInboundRtpStreamStats.fromDartStats(
                  type as medea.RtcRemoteInboundRtpStreamStats);
          return fVal;
        }
      case 'RtcRemoteOutboundRtpStreamStats':
        {
          fVal.ref._tag = 8;
          fVal.ref._payload.remoteOutboundRTPStream =
              RTCRemoteOutboundRtpStreamStats.fromDartStats(
                  type as medea.RtcRemoteOutboundRtpStreamStats);
          return fVal;
        }
      default:
        {
          fVal.ref._tag = 0;
          return fVal;
        }
    }
  }
}

/// Transport statistics related to the [RTCPeerConnection] object.
///
/// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
class RTCTransportStats extends Struct {
  RTCTransportStats._();

  /// Total number of packets sent over this transport.
  external Pointer<ForeignValue> packetsSent;

  /// Total number of packets received on this transport.
  external Pointer<ForeignValue> packetsReceived;

  /// Total number of payload bytes sent on this [RTCPeerConnection], i.e.
  /// not including headers or padding.
  ///
  /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
  external Pointer<ForeignValue> bytesSent;

  /// Total number of bytes received on this [RTCPeerConnection], i.e. not
  /// including headers or padding.
  ///
  /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
  external Pointer<ForeignValue> bytesReceived;

  /// Set to the current value of the [`role` attribute][1] of the
  /// [underlying RTCDtlsTransport's `transport`][2].
  ///
  /// [1]: https://w3.org/TR/webrtc#dom-icetransport-role
  /// [2]: https://w3.org/TR/webrtc#dom-rtcdtlstransport-icetransport
  external Pointer<ForeignValue> iceRole;

  static Pointer<RTCTransportStats> fromDartStats(
      medea.RtcTransportStats stats) {
    var fVal = calloc<RTCTransportStats>();
    fVal.ref.packetsSent = ForeignValue.fromDart(stats.packetsSent);
    fVal.ref.packetsReceived = ForeignValue.fromDart(stats.packetsReceived);
    fVal.ref.bytesSent = ForeignValue.fromDart(stats.bytesSent);
    fVal.ref.bytesReceived = ForeignValue.fromDart(stats.bytesReceived);
    fVal.ref.iceRole = ForeignValue.fromDart(stats.iceRole);
    return fVal;
  }
}

/// Statistics for the media produced by a [MediaStreamTrack][1] that
/// is currently attached to an [RTCRtpSender]. This reflects
/// the media that is fed to the encoder after [getUserMedia]
/// constraints have been applied (i.e. not the raw media
/// produced by the camera).
///
/// [RTCRtpSender]: https://w3.org/TR/webrtc#rtcrtpsender-interface
/// [getUserMedia]: https://tinyurl.com/sngpyr6
/// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
class RTCMediaSourceStats extends Struct {
  RTCMediaSourceStats._();

  /// Value of the [MediaStreamTrack][1]'s ID attribute.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
  external Pointer<ForeignValue> trackIdentifier;

  /// Fields which should be in the [`RtcStat`] based on `kind`.
  external Pointer<RTCMediaSourceStatsMediaType> kind;

  static Pointer<RTCMediaSourceStats> fromDartStats(
      medea.RtcMediaSourceStats stats) {
    var fVal = calloc<RTCMediaSourceStats>();
    fVal.ref.trackIdentifier = ForeignValue.fromDart(stats.trackIdentifier);
    fVal.ref.kind = RTCMediaSourceStatsMediaType.fromDartStats(stats);
    return fVal;
  }
}

class RTCMediaSourceStatsMediaType extends Struct {
  @Int32()
  // ignore: unused_field
  external int _tag;
  external _RTCMediaSourceStatsMediaTypeFields _payload;

  static Pointer<RTCMediaSourceStatsMediaType> fromDartStats(
      medea.RtcMediaSourceStats stats) {
    var fVal = calloc<RTCMediaSourceStatsMediaType>();
    if (stats is medea.RtcVideoSourceStats) {
      fVal.ref._tag = 0;
      fVal.ref._payload.video = RTCVideoSourceStats.fromDartStats(stats);
    } else if (stats is medea.RtcAudioSourceStats) {
      fVal.ref._tag = 1;
      fVal.ref._payload.audio = RTCAudioSourceStats.fromDartStats(stats);
    }
    return fVal;
  }
}

class RTCVideoSourceStats extends Struct {
  RTCVideoSourceStats._();

  /// Width (in pixels) of the last frame originating from the source.
  /// Before a frame has been produced this attribute is missing.
  external Pointer<ForeignValue> width;

  /// Height (in pixels) of the last frame originating from the source.
  /// Before a frame has been produced this attribute is missing.
  external Pointer<ForeignValue> height;

  /// The total number of frames originating from this source.
  external Pointer<ForeignValue> frames;

  /// Number of frames originating from the source, measured during the
  /// last second. For the first second of this object's lifetime this
  /// attribute is missing.
  external Pointer<ForeignValue> framesPerSecond;

  static Pointer<RTCVideoSourceStats> fromDartStats(
      medea.RtcVideoSourceStats stats) {
    var fVal = calloc<RTCVideoSourceStats>();
    fVal.ref.width = ForeignValue.fromDart(stats.width);
    fVal.ref.height = ForeignValue.fromDart(stats.height);
    fVal.ref.frames = ForeignValue.fromDart(stats.frames);
    fVal.ref.framesPerSecond = ForeignValue.fromDart(stats.framesPerSecond);

    return fVal;
  }
}

class RTCAudioSourceStats extends Struct {
  RTCAudioSourceStats._();

  /// Audio level of the media source.
  external Pointer<ForeignValue> audioLevel;

  /// Audio energy of the media source.
  external Pointer<ForeignValue> totalAudioEnergy;

  /// Audio duration of the media source.
  external Pointer<ForeignValue> totalSamplesDuration;

  /// Only exists when the MediaStreamTrack is sourced
  /// from a microphone where echo cancellation is applied.
  external Pointer<ForeignValue> echoReturnLoss;

  /// Only exists when the [`MediaStreamTrack`]
  /// is sourced from a microphone where
  /// echo cancellation is applied.
  external Pointer<ForeignValue> echoReturnLossEnhancement;

  static Pointer<RTCAudioSourceStats> fromDartStats(
      medea.RtcAudioSourceStats stats) {
    var fVal = calloc<RTCAudioSourceStats>();
    fVal.ref.audioLevel = ForeignValue.fromDart(stats.audioLevel);
    fVal.ref.totalAudioEnergy = ForeignValue.fromDart(stats.totalAudioEnergy);
    fVal.ref.totalSamplesDuration =
        ForeignValue.fromDart(stats.totalSamplesDuration);
    fVal.ref.echoReturnLoss = ForeignValue.fromDart(stats.echoReturnLoss);
    fVal.ref.echoReturnLossEnhancement =
        ForeignValue.fromDart(stats.echoReturnLossEnhancement);
    return fVal;
  }
}

/// ICE candidate statistics related to the [RTCIceTransport]
/// objects.
///
/// A local candidate is [deleted][1] when the [RTCIceTransport] does
/// an ICE restart, and the candidate is no longer a member of
/// any non-deleted candidate pair.
///
/// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
/// [1]: https://w3.org/TR/webrtc-stats/#dfn-deleted
class RTCIceCandidateStats extends Struct {
  RTCIceCandidateStats._();

  @Int32()
  // ignore: unused_field
  external int _tag;

  external _RTCIceCandidateStats _payload;

  static Pointer<RTCIceCandidateStats> fromDartStats(
      medea.RtcIceCandidateStats stats) {
    var fVal = calloc<RTCIceCandidateStats>();
    if (stats is medea.RtcRemoteIceCandidateStats) {
      fVal.ref._tag = 0;
      fVal.ref._payload.remote = IceCandidateStats.fromDartStats(stats);
    } else if (stats is medea.RtcLocalIceCandidateStats) {
      fVal.ref._tag = 1;
      fVal.ref._payload.local = IceCandidateStats.fromDartStats(stats);
    }
    return fVal;
  }
}

class IceCandidateStats extends Struct {
  IceCandidateStats._();

  /// Unique ID that is associated to the object that was inspected to
  /// produce the [RTCTransportStats][1] associated with this candidate.
  ///
  /// [1]: https://w3.org/TR/webrtc-stats/#transportstats-dict%2A
  external Pointer<ForeignValue> transportId;

  /// Address of the candidate, allowing for IPv4 addresses, IPv6 addresses,
  /// and fully qualified domain names (FQDNs).
  external Pointer<ForeignValue> address;

  /// Port number of the candidate.
  external Pointer<ForeignValue> port;

  /// Valid values for transport is one of `udp` and `tcp`.
  @Int32()
  external int protocol;

  /// Type of the ICE candidate.
  @Int32()
  external int candidateType;

  /// Calculated as defined in [Section 15.1 of RFC 5245][1].
  ///
  /// [1]: https://tools.ietf.org/html/rfc5245#section-15.1
  external Pointer<ForeignValue> priority;

  /// For local candidates this is the URL of the ICE server from which the
  /// candidate was obtained. It is the same as the
  /// [url surfaced in the RTCPeerConnectionIceEvent][1].
  ///
  /// `None` for remote candidates.
  ///
  /// [1]: https://w3.org/TR/webrtc#rtcpeerconnectioniceevent
  external Pointer<ForeignValue> url;

  /// Protocol used by the endpoint to communicate with the TURN server.
  ///
  /// Only present for local candidates.
  external Pointer<ForeignValue> relayProtocol;

  static Pointer<IceCandidateStats> fromDartStats(
      medea.RtcIceCandidateStats stats) {
    var fVal = calloc<IceCandidateStats>();
    fVal.ref.address = ForeignValue.fromDart(stats.address);
    fVal.ref.port = ForeignValue.fromDart(stats.port);
    fVal.ref.transportId = ForeignValue.fromDart(stats.transportId);
    fVal.ref.priority = ForeignValue.fromDart(stats.priority);
    fVal.ref.url = ForeignValue.fromDart(stats.url);
    fVal.ref.relayProtocol = ForeignValue.fromDart(stats.relayProtocol);
    fVal.ref.protocol = stats.protocol.index;
    fVal.ref.candidateType = stats.candidateType.index;
    return fVal;
  }
}

/// Statistics for an outbound [RTP] stream that is currently sent with
/// [RTCPeerConnection] object.
///
/// When there are multiple [RTP] streams connected to the same sender,
/// such as when using simulcast or RTX, there will be one
/// [`RtcOutboundRtpStreamStats`] per RTP stream, with distinct values
/// of the `ssrc` attribute, and all these senders will have a
/// reference to the same "sender" object (of type
/// [RTCAudioSenderStats][1] or [RTCVideoSenderStats][2]) and
/// "track" object (of type
/// [RTCSenderAudioTrackAttachmentStats][3] or
/// [RTCSenderVideoTrackAttachmentStats][4]).
///
/// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
/// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
/// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcaudiosenderstats
/// [2]: https://w3.org/TR/webrtc-stats/#dom-rtcvideosenderstats
/// [3]: https://tinyurl.com/sefa5z4
/// [4]: https://tinyurl.com/rkuvpl4
class RTCOutboundRTPStreamStats extends Struct {
  RTCOutboundRTPStreamStats._();

  /// ID of the stats object representing the current track attachment
  /// to the sender of this stream.
  external Pointer<ForeignValue> trackId;

  /// Total number of bytes sent for this SSRC.
  external Pointer<ForeignValue> bytesSent;

  /// Total number of RTP packets sent for this SSRC.
  external Pointer<ForeignValue> packetsSent;

  /// ID of the stats object representing the track currently
  /// attached to the sender of this stream.
  external Pointer<ForeignValue> mediaSourceId;

  /// Fields which should be in the [`RtcStat`] based on `mediaType`.
  external Pointer<ForeignValue> mediaType;

  static Pointer<RTCOutboundRTPStreamStats> fromDartStats(
      medea.RtcOutboundRTPStreamStats stats) {
    var fVal = calloc<RTCOutboundRTPStreamStats>();
    fVal.ref.trackId = ForeignValue.fromDart(stats.trackId);
    fVal.ref.bytesSent = ForeignValue.fromDart(stats.bytesSent);
    fVal.ref.packetsSent = ForeignValue.fromDart(stats.packetsSent);
    fVal.ref.mediaSourceId = ForeignValue.fromDart(stats.mediaSourceId);

    fVal.ref.mediaType = ForeignValue.fromDart(
        RTCOutboundRTPStreamStatsMediaType.fromDartStats(stats.mediaType));
    return fVal;
  }
}

class RTCOutboundRTPStreamStatsAudio extends Struct {
  RTCOutboundRTPStreamStatsAudio._();

  /// Total number of samples that have been sent over this RTP stream.
  external Pointer<ForeignValue> totalSamplesSent;

  /// Whether the last RTP packet sent contained voice activity or not
  /// based on the presence of the V bit in the extension header.
  external Pointer<ForeignValue> voiceActivityFlag;

  static Pointer<RTCOutboundRTPStreamStatsAudio> fromDartStats(
      medea.RtcOutboundRTPStreamStatsAudio stats) {
    var fVal = calloc<RTCOutboundRTPStreamStatsAudio>();
    fVal.ref.totalSamplesSent = ForeignValue.fromDart(stats.totalSamplesSent);
    fVal.ref.voiceActivityFlag = ForeignValue.fromDart(stats.voiceActivityFlag);
    return fVal;
  }
}

class RTCOutboundRTPStreamStatsVideo extends Struct {
  RTCOutboundRTPStreamStatsVideo._();

  /// Width of the last encoded frame.
  ///
  /// The resolution of the encoded frame may be lower than the media
  /// source (see [RTCVideoSourceStats.width][1]).
  ///
  /// Before the first frame is encoded this attribute is missing.
  ///
  /// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats-width
  external Pointer<ForeignValue> frameWidth;

  /// Height of the last encoded frame.
  ///
  /// The resolution of the encoded frame may be lower than the media
  /// source (see [RTCVideoSourceStats.height][1]).
  ///
  /// Before the first frame is encoded this attribute is missing.
  ///
  /// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcvideosourcestats-height
  external Pointer<ForeignValue> frameHeight;

  /// Number of encoded frames during the last second.
  ///
  /// This may be lower than the media source frame rate (see
  /// [RTCVideoSourceStats.framesPerSecond][1]).
  ///
  /// [1]: https://tinyurl.com/rrmkrfk
  external Pointer<ForeignValue> framesPerSecond;

  static Pointer<RTCOutboundRTPStreamStatsVideo> fromDartStats(
      medea.RtcOutboundRTPStreamStatsVideo stats) {
    var fVal = calloc<RTCOutboundRTPStreamStatsVideo>();
    fVal.ref.frameWidth = ForeignValue.fromDart(stats.frameWidth);
    fVal.ref.frameHeight = ForeignValue.fromDart(stats.frameHeight);
    fVal.ref.framesPerSecond = ForeignValue.fromDart(stats.framesPerSecond);
    return fVal;
  }
}

/// Statistics for an inbound [RTP] stream that is currently received
/// with [RTCPeerConnection] object.
///
/// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
/// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
class RTCInboundRTPStreamStats extends Struct {
  RTCInboundRTPStreamStats._();

  /// ID of the stats object representing the receiving track.
  external Pointer<ForeignValue> remoteId;

  /// Total number of bytes received for this SSRC.
  external Pointer<ForeignValue> bytesReceived;

  /// Total number of RTP data packets received for this SSRC.
  external Pointer<ForeignValue> packetsReceived;

  /// Total number of RTP data packets for this SSRC that have been lost
  /// since the beginning of reception.
  ///
  /// This number is defined to be the number of packets expected less the
  /// number of packets actually received, where the number of packets
  /// received includes any which are late or duplicates.
  /// Thus, packets that arrive late are not counted as lost,
  /// and the loss __may be negative__
  /// if there are duplicates.
  external Pointer<ForeignValue> packetsLost;

  /// Packet jitter measured in seconds for this SSRC.
  external Pointer<ForeignValue> jitter;

  /// Total number of seconds that have been spent decoding the
  /// [`framesDecoded`] frames of this stream.
  ///
  /// The average decode time can be calculated by dividing this value
  /// with [`framesDecoded`].
  /// The time it takes to decode one frame is the time
  /// passed between feeding the decoder a frame and the decoder returning
  /// decoded data for that frame.
  ///
  /// [`framesDecoded`]: https://tinyurl.com/srfwrwt
  external Pointer<ForeignValue> totalDecodeTime;

  /// Total number of audio samples or video frames
  /// that have come out of the
  /// jitter buffer (increasing [`jitterBufferDelay`]).
  ///
  /// [`jitterBufferDelay`]: https://tinyurl.com/qvoojt5
  external Pointer<ForeignValue> jitterBufferEmittedCount;

  /// Fields which should be in the [`RtcStat`] based on `mediaType`.
  external Pointer<RTCInboundRTPStreamMediaType> mediaType;

  static Pointer<RTCInboundRTPStreamStats> fromDartStats(
      medea.RtcInboundRTPStreamStats stats) {
    var fVal = calloc<RTCInboundRTPStreamStats>();
    fVal.ref.remoteId = ForeignValue.fromDart(stats.remoteId);
    fVal.ref.bytesReceived = ForeignValue.fromDart(stats.bytesReceived);
    fVal.ref.packetsReceived = ForeignValue.fromDart(stats.packetsReceived);
    fVal.ref.packetsLost = ForeignValue.fromDart(stats.packetsLost);
    fVal.ref.jitter = ForeignValue.fromDart(stats.jitter);
    fVal.ref.totalDecodeTime = ForeignValue.fromDart(stats.totalDecodeTime);
    fVal.ref.jitterBufferEmittedCount =
        ForeignValue.fromDart(stats.jitterBufferEmittedCount);
    fVal.ref.mediaType =
        RTCInboundRTPStreamMediaType.fromDartStats(stats.mediaType!);
    return fVal;
  }
}

class RTCInboundRTPStreamMediaType extends Struct {
  RTCInboundRTPStreamMediaType._();

  @Int32()
  // ignore: unused_field
  external int _tag;

  external _RTCInboundRTPStreamMediaType _payload;

  static Pointer<RTCInboundRTPStreamMediaType> fromDartStats(
      medea.RtcInboundRTPStreamMediaType stats) {
    var fVal = calloc<RTCInboundRTPStreamMediaType>();
    if (stats is medea.RtcInboundRTPStreamVideo) {
      fVal.ref._tag = 0;
      fVal.ref._payload.video = RTCInboundRTPStreamVideo.fromDartStats(stats);
    } else if (stats is medea.RtcInboundRTPStreamAudio) {
      fVal.ref._tag = 1;
      fVal.ref._payload.audio = RTCInboundRTPStreamAudio.fromDartStats(stats);
    }
    return fVal;
  }
}

class RTCOutboundRTPStreamStatsMediaType extends Struct {
  RTCOutboundRTPStreamStatsMediaType._();

  @Int32()
  // ignore: unused_field
  external int _tag;

  external _RTCOutboundRTPStreamStatsMediaType _payload;

  static Pointer<RTCOutboundRTPStreamStatsMediaType>? fromDartStats(
      medea.RtcOutboundRTPStreamStatsMediaType? stats) {
    if (stats == null) {
      return null;
    }
    var fVal = calloc<RTCOutboundRTPStreamStatsMediaType>();
    if (stats is medea.RtcOutboundRTPStreamStatsVideo) {
      fVal.ref._tag = 0;
      fVal.ref._payload.video =
          RTCOutboundRTPStreamStatsVideo.fromDartStats(stats);
    } else if (stats is medea.RtcOutboundRTPStreamStatsAudio) {
      fVal.ref._tag = 1;
      fVal.ref._payload.audio =
          RTCOutboundRTPStreamStatsAudio.fromDartStats(stats);
    }
    return fVal;
  }
}

class RTCInboundRTPStreamAudio extends Struct {
  RTCInboundRTPStreamAudio._();

  /// Indicator whether the last RTP packet whose frame was delivered to
  /// the [RTCRtpReceiver]'s [MediaStreamTrack][1] for playout contained
  /// voice activity or not based on the presence of the V bit in the
  /// extension header, as defined in [RFC 6464].
  ///
  /// [RTCRtpReceiver]: https://w3.org/TR/webrtc#rtcrtpreceiver-interface
  /// [RFC 6464]: https://tools.ietf.org/html/rfc6464#page-3
  /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
  external Pointer<ForeignValue> voiceActivityFlag;

  /// Total number of samples that have been received on this RTP stream.
  /// This includes [`concealedSamples`].
  ///
  /// [`concealedSamples`]: https://tinyurl.com/s6c4qe4
  external Pointer<ForeignValue> totalSamplesReceived;

  /// Total number of samples that are concealed samples.
  ///
  /// A concealed sample is a sample that was replaced with synthesized
  /// samples generated locally before being played out.
  /// Examples of samples that have to be concealed are samples from lost
  /// packets (reported in [`packetsLost`]) or samples from packets that
  /// arrive too late to be played out (reported in
  /// [`packetsDiscarded`]).
  ///
  /// [`packetsLost`]: https://tinyurl.com/u2gq965
  /// [`packetsDiscarded`]: https://tinyurl.com/yx7qyox3
  external Pointer<ForeignValue> concealedSamples;

  /// Total number of concealed samples inserted that are "silent".
  ///
  /// Playing out silent samples results in silence or comfort noise.
  /// This is a subset of [`concealedSamples`].
  ///
  /// [`concealedSamples`]: https://tinyurl.com/s6c4qe4
  external Pointer<ForeignValue> silentConcealedSamples;

  /// Audio level of the receiving track.
  external Pointer<ForeignValue> audioLevel;

  /// Audio energy of the receiving track.
  external Pointer<ForeignValue> totalAudioEnergy;

  /// Audio duration of the receiving track.
  ///
  /// For audio durations of tracks attached locally, see
  /// [RTCAudioSourceStats][1] instead.
  ///
  /// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcaudiosourcestats
  external Pointer<ForeignValue> totalSamplesDuration;

  static Pointer<RTCInboundRTPStreamAudio> fromDartStats(
      medea.RtcInboundRTPStreamAudio stats) {
    var fVal = calloc<RTCInboundRTPStreamAudio>();
    fVal.ref.totalSamplesReceived =
        ForeignValue.fromDart(stats.totalSamplesReceived);
    fVal.ref.concealedSamples = ForeignValue.fromDart(stats.concealedSamples);
    fVal.ref.silentConcealedSamples =
        ForeignValue.fromDart(stats.silentConcealedSamples);
    fVal.ref.audioLevel = ForeignValue.fromDart(stats.audioLevel);
    fVal.ref.totalAudioEnergy = ForeignValue.fromDart(stats.totalAudioEnergy);
    fVal.ref.totalSamplesDuration =
        ForeignValue.fromDart(stats.totalSamplesDuration);
    fVal.ref.voiceActivityFlag = ForeignValue.fromDart(stats.voiceActivityFlag);
    return fVal;
  }
}

class RTCInboundRTPStreamVideo extends Struct {
  RTCInboundRTPStreamVideo._();

  /// Total number of frames correctly decoded for this RTP stream, i.e.
  /// frames that would be displayed if no frames are dropped.
  external Pointer<ForeignValue> framesDecoded;

  /// Total number of key frames, such as key frames in VP8 [RFC 6386] or
  /// IDR-frames in H.264 [RFC 6184], successfully decoded for this RTP
  /// media stream.
  ///
  /// This is a subset of [`framesDecoded`].
  /// [`framesDecoded`] - [`keyFramesDecoded`] gives you the number of
  /// delta frames decoded.
  ///
  /// [RFC 6386]: https://w3.org/TR/webrtc-stats/#bib-rfc6386
  /// [RFC 6184]: https://w3.org/TR/webrtc-stats/#bib-rfc6184
  /// [`framesDecoded`]: https://tinyurl.com/srfwrwt
  /// [`keyFramesDecoded`]: https://tinyurl.com/qtdmhtm
  external Pointer<ForeignValue> keyFramesDecoded;

  /// Width of the last decoded frame.
  ///
  /// Before the first frame is decoded this attribute is missing.
  external Pointer<ForeignValue> frameWidth;

  /// Height of the last decoded frame.
  ///
  /// Before the first frame is decoded this attribute is missing.
  external Pointer<ForeignValue> frameHeight;

  /// Sum of the interframe delays in seconds between consecutively
  /// decoded frames, recorded just after a frame has been decoded.
  external Pointer<ForeignValue> totalInterFrameDelay;

  /// Number of decoded frames in the last second.
  external Pointer<ForeignValue> framesPerSecond;

  /// Bit depth per pixel of the last decoded frame.
  ///
  /// Typical values are 24, 30, or 36 bits. Before the first frame is
  /// decoded this attribute is missing.
  external Pointer<ForeignValue> frameBitDepth;

  /// Total number of Full Intra Request (FIR) packets sent by this
  /// receiver.
  external Pointer<ForeignValue> firCount;

  /// Total number of Picture Loss Indication (PLI) packets sent by this
  /// receiver.
  external Pointer<ForeignValue> pliCount;

  /// Number of concealment events.
  ///
  /// This counter increases every time a concealed sample is synthesized
  /// after a non-concealed sample. That is, multiple consecutive
  /// concealed samples will increase the [`concealedSamples`] count
  /// multiple times but is a single concealment event.
  ///
  /// [`concealedSamples`]: https://tinyurl.com/s6c4qe4
  external Pointer<ForeignValue> concealmentEvents;

  /// Total number of complete frames received on this RTP stream.
  ///
  /// This metric is incremented when the complete frame is received.
  external Pointer<ForeignValue> framesReceived;

  /// Total number of Slice Loss Indication (SLI) packets sent by this
  /// receiver.
  external Pointer<ForeignValue> sliCount;

  static Pointer<RTCInboundRTPStreamVideo> fromDartStats(
      medea.RtcInboundRTPStreamVideo stats) {
    var fVal = calloc<RTCInboundRTPStreamVideo>();
    fVal.ref.framesDecoded = ForeignValue.fromDart(stats.framesDecoded);
    fVal.ref.keyFramesDecoded = ForeignValue.fromDart(stats.keyFramesDecoded);
    fVal.ref.frameWidth = ForeignValue.fromDart(stats.frameWidth);
    fVal.ref.frameHeight = ForeignValue.fromDart(stats.frameHeight);
    fVal.ref.totalInterFrameDelay =
        ForeignValue.fromDart(stats.totalInterFrameDelay);
    fVal.ref.framesPerSecond = ForeignValue.fromDart(stats.framesPerSecond);
    fVal.ref.frameBitDepth = ForeignValue.fromDart(stats.frameBitDepth);
    fVal.ref.firCount = ForeignValue.fromDart(stats.firCount);
    fVal.ref.pliCount = ForeignValue.fromDart(stats.pliCount);
    fVal.ref.concealmentEvents = ForeignValue.fromDart(stats.concealmentEvents);
    fVal.ref.framesReceived = ForeignValue.fromDart(stats.framesReceived);
    fVal.ref.sliCount = ForeignValue.fromDart(stats.sliCount);
    return fVal;
  }
}

/// ICE candidate pair statistics related to the [RTCIceTransport]
/// objects.
///
/// A candidate pair that is not the current pair for a transport is
/// [deleted][1] when the [RTCIceTransport] does an ICE restart, at the
/// time the state changes to `new`.
///
/// The candidate pair that is the current pair for a transport is
/// deleted after an ICE restart when the [RTCIceTransport]
/// switches to using a candidate pair generated from the new
/// candidates; this time doesn't correspond to any other
/// externally observable event.
///
/// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
/// [1]: https://w3.org/TR/webrtc-stats/#dfn-deleted
class RTCIceCandidatePairStats extends Struct {
  RTCIceCandidatePairStats._();

  /// State of the checklist for the local
  /// and remote candidates in a pair.
  @Int32()
  external int state;

  /// Related to updating the nominated flag described in
  /// [Section 7.1.3.2.4 of RFC 5245][1].
  ///
  /// [1]: https://tools.ietf.org/html/rfc5245#section-7.1.3.2.4
  external Pointer<ForeignValue> nominated;

  /// Total number of payload bytes sent on this candidate pair, i.e. not
  /// including headers or padding.
  external Pointer<ForeignValue> bytesSent;

  /// Total number of payload bytes received on this candidate pair, i.e.
  /// not including headers or padding.
  external Pointer<ForeignValue> bytesReceived;

  /// Sum of all round trip time measurements in seconds since
  /// the beginning of the session,
  /// based on STUN connectivity check [STUN-PATH-CHAR]
  /// responses (responsesReceived), including those that reply
  /// to requests that are sent in order to verify consent [RFC 7675].
  ///
  /// The average round trip time can be computed from
  /// [`totalRoundTripTime`][1] by dividing it
  /// by [`responsesReceived`][2].
  ///
  /// [STUN-PATH-CHAR]: https://w3.org/TR/webrtc-stats/#bib-stun-path-char
  /// [RFC 7675]: https://tools.ietf.org/html/rfc7675
  /// [1]: https://tinyurl.com/tgr543a
  /// [2]: https://tinyurl.com/r3zo2um
  external Pointer<ForeignValue> totalRoundTripTime;

  /// Latest round trip time measured in seconds, computed from both STUN
  /// connectivity checks [STUN-PATH-CHAR],
  /// including those that are sent for consent verification [RFC 7675].
  ///
  /// [STUN-PATH-CHAR]: https://w3.org/TR/webrtc-stats/#bib-stun-path-char
  /// [RFC 7675]: https://tools.ietf.org/html/rfc7675
  external Pointer<ForeignValue> currentRoundTripTime;

  /// Calculated by the underlying congestion control by combining the
  /// available bitrate for all the outgoing RTP streams using
  /// this candidate pair.
  /// The bitrate measurement does not count the size of the IP or
  /// other transport layers like TCP or UDP. It is similar to the TIAS
  /// defined in [RFC 3890], i.e. it is measured in bits per second and
  /// the bitrate is calculated over a 1 second window.
  ///
  /// Implementations that do not calculate a sender-side estimate
  /// MUST leave this undefined. Additionally, the value MUST be undefined
  /// for candidate pairs that were never used. For pairs in use,
  /// the estimate is normally
  /// no lower than the bitrate for the packets sent at
  /// [`lastPacketSentTimestamp`][1], but might be higher. For candidate
  /// pairs that are not currently in use but were used before,
  /// implementations MUST return undefined.
  ///
  /// [RFC 3890]: https://tools.ietf.org/html/rfc3890
  /// [1]: https://tinyurl.com/rfc72eh
  external Pointer<ForeignValue> availableOutgoingBitrate;

  static Pointer<RTCIceCandidatePairStats> fromDartStats(
      medea.RtcIceCandidatePairStats stats) {
    var fVal = calloc<RTCIceCandidatePairStats>();
    fVal.ref.state = stats.state.index;
    fVal.ref.nominated = ForeignValue.fromDart(stats.nominated);
    fVal.ref.bytesSent = ForeignValue.fromDart(stats.bytesSent);
    fVal.ref.bytesReceived = ForeignValue.fromDart(stats.bytesReceived);
    fVal.ref.totalRoundTripTime =
        ForeignValue.fromDart(stats.totalRoundTripTime);
    fVal.ref.currentRoundTripTime =
        ForeignValue.fromDart(stats.currentRoundTripTime);
    fVal.ref.availableOutgoingBitrate =
        ForeignValue.fromDart(stats.availableOutgoingBitrate);
    return fVal;
  }
}

/// Statistics for the remote endpoint's inbound [RTP] stream
/// corresponding to an outbound stream that is currently sent with
/// [RTCPeerConnection] object.
///
/// It is measured at the remote endpoint and reported in a RTCP
/// Receiver Report (RR) or RTCP Extended Report (XR).
///
/// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
/// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
class RTCRemoteInboundRtpStreamStats extends Struct {
  RTCRemoteInboundRtpStreamStats._();

  /// [`localId`] is used for looking up the local
  /// [RTCOutboundRtpStreamStats] object for the same SSRC.
  ///
  /// [`localId`]: https://tinyurl.com/r8uhbo9
  /// [RTCOutBoundRtpStreamStats]: https://tinyurl.com/r6f5vqg
  external Pointer<ForeignValue> localId;

  /// Packet [jitter] measured in seconds for this SSRC.
  ///
  /// [jitter]: https://en.wikipedia.org/wiki/Jitter
  external Pointer<ForeignValue> jitter;

  /// Estimated round trip time for this SSRC based on
  /// the RTCP timestamps in
  /// the RTCP Receiver Report (RR) and measured in seconds.
  /// Calculated as defined in [Section 6.4.1 of RFC 3550][1].
  /// If no RTCP Receiver Report
  /// is received with a DLSR value other than 0, the round trip time is
  /// left undefined.
  ///
  /// [1]: https://tools.ietf.org/html/rfc3550#section-6.4.1
  external Pointer<ForeignValue> roundTripTime;

  /// Fraction packet loss reported for this SSRC.
  /// Calculated as defined in
  /// [Section 6.4.1 of RFC 3550][1] and [Appendix A.3][2].
  ///
  /// [1]: https://tools.ietf.org/html/rfc3550#section-6.4.1
  /// [2]: https://tools.ietf.org/html/rfc3550#appendix-A.3
  external Pointer<ForeignValue> fractionLost;

  /// Total number of RTCP RR blocks received for this SSRC.
  external Pointer<ForeignValue> reportsReceived;

  /// Total number of RTCP RR blocks received for this SSRC that contain a
  /// valid round trip time. This counter will increment if the
  /// [`roundTripTime`] is undefined.
  ///
  /// [`roundTripTime`]: https://tinyurl.com/ssg83hq
  external Pointer<ForeignValue> roundTripTimeMeasurements;

  static Pointer<RTCRemoteInboundRtpStreamStats> fromDartStats(
      medea.RtcRemoteInboundRtpStreamStats stats) {
    var fVal = calloc<RTCRemoteInboundRtpStreamStats>();
    fVal.ref.localId = ForeignValue.fromDart(stats.localId);
    fVal.ref.jitter = ForeignValue.fromDart(stats.jitter);
    fVal.ref.roundTripTime = ForeignValue.fromDart(stats.roundTripTime);
    fVal.ref.fractionLost = ForeignValue.fromDart(stats.fractionLost);
    fVal.ref.reportsReceived = ForeignValue.fromDart(stats.reportsReceived);
    fVal.ref.roundTripTimeMeasurements =
        ForeignValue.fromDart(stats.roundTripTimeMeasurements);
    return fVal;
  }
}

/// Statistics for the remote endpoint's outbound [RTP] stream
/// corresponding to an inbound stream that is currently received with
/// [RTCPeerConnection] object.
///
/// It is measured at the remote endpoint and reported in an RTCP
/// Sender Report (SR).
///
/// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
/// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
class RTCRemoteOutboundRtpStreamStats extends Struct {
  RTCRemoteOutboundRtpStreamStats._();

  /// [`localId`] is used for looking up the local
  /// [RTCInboundRtpStreamStats][1] object for the same SSRC.
  ///
  /// [`localId`]: https://tinyurl.com/vu9tb2e
  /// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcinboundrtpstreamstats
  external Pointer<ForeignValue> localId;

  /// [`remoteTimestamp`] (as [HIGHRES-TIME]) is the remote timestamp at
  /// which these statistics were sent by the remote endpoint. This
  /// differs from timestamp, which represents the time at which the
  /// statistics were generated or received by the local endpoint. The
  /// [`remoteTimestamp`], if present, is derived from the NTP timestamp
  /// in an RTCP Sender Report (SR) block, which reflects the remote
  /// endpoint's clock. That clock may not be synchronized with the local
  /// clock.
  ///
  /// [`remoteTimestamp`]: https://tinyurl.com/rzlhs87
  /// [HIGRES-TIME]: https://w3.org/TR/webrtc-stats/#bib-highres-time
  external Pointer<ForeignValue> remoteTimestamp;

  /// Total number of RTCP SR blocks sent for this SSRC.
  external Pointer<ForeignValue> reportsSent;

  static Pointer<RTCRemoteOutboundRtpStreamStats> fromDartStats(
      medea.RtcRemoteOutboundRtpStreamStats stats) {
    var fVal = calloc<RTCRemoteOutboundRtpStreamStats>();
    fVal.ref.localId = ForeignValue.fromDart(stats.localId);
    fVal.ref.remoteTimestamp = ForeignValue.fromDart(stats.remoteTimestamp);
    fVal.ref.reportsSent = ForeignValue.fromDart(stats.reportsSent);
    return fVal;
  }
}

class _RTCInboundRTPStreamMediaType extends Union {
  external Pointer<RTCInboundRTPStreamVideo> video;
  external Pointer<RTCInboundRTPStreamAudio> audio;
}

class _RTCOutboundRTPStreamStatsMediaType extends Union {
  external Pointer<RTCOutboundRTPStreamStatsVideo> video;
  external Pointer<RTCOutboundRTPStreamStatsAudio> audio;
}

class _RTCMediaSourceStatsMediaTypeFields extends Union {
  external Pointer<RTCVideoSourceStats> video;
  external Pointer<RTCAudioSourceStats> audio;
}

class _RTCIceCandidateStats extends Union {
  external Pointer<IceCandidateStats> remote;
  external Pointer<IceCandidateStats> local;
}

/// All known types of [`RtcStat`]s.
///
/// [List of all RTCStats types on W3C][1].
///
/// [1]: https://w3.org/TR/webrtc-stats/#rtctatstype-%2A
/// [`RtcStat`]: super::RtcStat
class _ForeignValueStats extends Union {
  /// Transport statistics related to the [RTCPeerConnection] object.
  ///
  /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
  external Pointer<RTCTransportStats> transport;

  /// Statistics for the media produced by a [MediaStreamTrack][1] that
  /// is currently attached to an [RTCRtpSender]. This reflects
  /// the media that is fed to the encoder after [getUserMedia]
  /// constraints have been applied (i.e. not the raw media
  /// produced by the camera).
  ///
  /// [RTCRtpSender]: https://w3.org/TR/webrtc#rtcrtpsender-interface
  /// [getUserMedia]: https://tinyurl.com/sngpyr6
  /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
  external Pointer<RTCMediaSourceStats> mediaSource;

  /// ICE candidate statistics related to the [RTCIceTransport]
  /// objects.
  ///
  /// A local candidate is [deleted][1] when the [RTCIceTransport] does
  /// an ICE restart, and the candidate is no longer a member of
  /// any non-deleted candidate pair.
  ///
  /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
  /// [1]: https://w3.org/TR/webrtc-stats/#dfn-deleted
  external Pointer<RTCIceCandidateStats> iceCandidate;

  /// Statistics for an outbound [RTP] stream that is currently sent with
  /// [RTCPeerConnection] object.
  ///
  /// When there are multiple [RTP] streams connected to the same sender,
  /// such as when using simulcast or RTX, there will be one
  /// [`RtcOutboundRtpStreamStats`] per RTP stream, with distinct values
  /// of the `ssrc` attribute, and all these senders will have a
  /// reference to the same "sender" object (of type
  /// [RTCAudioSenderStats][1] or [RTCVideoSenderStats][2]) and
  /// "track" object (of type
  /// [RTCSenderAudioTrackAttachmentStats][3] or
  /// [RTCSenderVideoTrackAttachmentStats][4]).
  ///
  /// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
  /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
  /// [1]: https://w3.org/TR/webrtc-stats/#dom-rtcaudiosenderstats
  /// [2]: https://w3.org/TR/webrtc-stats/#dom-rtcvideosenderstats
  /// [3]: https://tinyurl.com/sefa5z4
  /// [4]: https://tinyurl.com/rkuvpl4
  external Pointer<RTCOutboundRTPStreamStats> outboundRTPStream;

  /// Statistics for an inbound [RTP] stream that is currently received
  /// with [RTCPeerConnection] object.
  ///
  /// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
  /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
  external Pointer<RTCInboundRTPStreamStats> inboundRTPStream;

  /// ICE candidate pair statistics related to the [RTCIceTransport]
  /// objects.
  ///
  /// A candidate pair that is not the current pair for a transport is
  /// [deleted][1] when the [RTCIceTransport] does an ICE restart, at the
  /// time the state changes to `new`.
  ///
  /// The candidate pair that is the current pair for a transport is
  /// deleted after an ICE restart when the [RTCIceTransport]
  /// switches to using a candidate pair generated from the new
  /// candidates; this time doesn't correspond to any other
  /// externally observable event.
  ///
  /// [RTCIceTransport]: https://w3.org/TR/webrtc#dom-rtcicetransport
  /// [1]: https://w3.org/TR/webrtc-stats/#dfn-deleted
  external Pointer<RTCIceCandidatePairStats> iceCandidatePair;

  /// Statistics for the remote endpoint's inbound [RTP] stream
  /// corresponding to an outbound stream that is currently sent with
  /// [RTCPeerConnection] object.
  ///
  /// It is measured at the remote endpoint and reported in a RTCP
  /// Receiver Report (RR) or RTCP Extended Report (XR).
  ///
  /// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
  /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
  external Pointer<RTCRemoteInboundRtpStreamStats> remoteInboundRTPStream;

  /// Statistics for the remote endpoint's outbound [RTP] stream
  /// corresponding to an inbound stream that is currently received with
  /// [RTCPeerConnection] object.
  ///
  /// It is measured at the remote endpoint and reported in an RTCP
  /// Sender Report (SR).
  ///
  /// [RTP]: https://en.wikipedia.org/wiki/Real-time_Transport_Protocol
  /// [RTCPeerConnection]: https://w3.org/TR/webrtc#dom-rtcpeerconnection
  external Pointer<RTCRemoteOutboundRtpStreamStats> remoteOutboundRTPStream;
}
