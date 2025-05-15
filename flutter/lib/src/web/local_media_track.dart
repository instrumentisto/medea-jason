// ignore_for_file: implementation_imports

import 'dart:js_interop';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;
import 'package:medea_flutter_webrtc/src/platform/web/media_stream_track.dart';

import '../interface/enums.dart';
import '../interface/media_track.dart';
import '../util/move_semantic.dart';
import 'exceptions.dart';
import 'jason_wasm.dart' as wasm;

class WebLocalMediaTrack implements LocalMediaTrack {
  late wasm.LocalMediaTrack obj;

  WebLocalMediaTrack(this.obj);

  @override
  MediaKind kind() {
    return fallibleFunction(() => MediaKind.values[obj.kind().toInt()]);
  }

  @override
  MediaSourceKind mediaSourceKind() {
    return fallibleFunction(
      () => MediaSourceKind.values[obj.media_source_kind().toInt()],
    );
  }

  @override
  webrtc.MediaStreamTrack getTrack() {
    return fallibleFunction(() => WebMediaStreamTrack(obj.get_track()));
  }

  @override
  void onEnded(OnEndedCallback f) {
    obj.get_track().onended = f.toJS;
  }

  @moveSemantics
  @override
  Future<void> free() async {
    obj.free();
  }

  @override
  Future<MediaStreamTrackState> state() async {
    final index = await fallibleFuture(obj.state().toDart) as JSNumber;
    return MediaStreamTrackState.values[index.toDartInt];
  }

  @override
  bool isOnAudioLevelAvailable() {
    return obj.is_on_audio_level_available();
  }

  @override
  void onAudioLevelChanged(OnAudioLevelChangedCallback f) {
    void fn(JSAny? l) => f((l as JSNumber).toDartInt);
    fallibleFunction(() => obj.on_audio_level_changed(fn.toJS));
  }

  @override
  bool isAudioProcessingAvailable() {
    return fallibleFunction(() => obj.is_audio_processing_available());
  }

  @override
  Future<void> setAutoGainControlEnabled(bool enabled) async {
    await fallibleFuture(obj.set_auto_gain_control_enabled(enabled).toDart);
  }

  @override
  Future<void> setEchoCancellationEnabled(bool enabled) async {
    await fallibleFuture(obj.set_echo_cancellation_enabled(enabled).toDart);
  }

  @override
  Future<void> setNoiseSuppressionEnabled(bool enabled) async {
    await fallibleFuture(obj.set_noise_suppression_enabled(enabled).toDart);
  }

  @override
  Future<bool> isAutoGainControlEnabled() async {
    return (await fallibleFuture(
      obj.is_auto_gain_control_enabled().toDart,
    )).toDart;
  }

  @override
  Future<bool> isEchoCancellationEnabled() async {
    return (await fallibleFuture(
      obj.is_echo_cancellation_enabled().toDart,
    )).toDart;
  }

  @override
  Future<bool> isNoiseSuppressionEnabled() async {
    return (await fallibleFuture(
      obj.is_noise_suppression_enabled().toDart,
    )).toDart;
  }

  @override
  Future<void> setNoiseSuppressionLevel(NoiseSuppressionLevel enabled) async {
    throw UnsupportedError('setNoiseSuppressionLevel in not supported on web');
  }

  @override
  Future<NoiseSuppressionLevel> getNoiseSuppressionLevel() async {
    throw UnsupportedError('getNoiseSuppressionLevel in not supported on web');
  }

  @override
  Future<void> setHighPassFilterEnabled(bool enabled) async {
    throw UnsupportedError('setHighPassFilterEnabled in not supported on web');
  }

  @override
  Future<bool> isHighPassFilterEnabled() async {
    throw UnsupportedError('isHighPassFilterEnabled in not supported on web');
  }
}
