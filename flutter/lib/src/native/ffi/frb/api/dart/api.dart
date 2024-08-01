// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.1.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;

import '../../frb_generated.dart';
import '../../media.dart';
import '../../media/constraints.dart';
import '../../media/track.dart';
import '../../media/track/remote.dart';
import '../../room.dart';

part 'api.freezed.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `fmt`, `from`, `from`, `from`, `from`

/// Returns the [`ConnectionHandle`] from the [`ForeignClass`] address.
ConnectionHandle connectionHandleFromPtr({required BigInt ptr}) =>
    RustLib.instance.api.crateApiDartApiConnectionHandleFromPtr(ptr: ptr);

/// Sets a callback to be invoked once the provided `connection` is closed.
///
/// # Errors
///
/// If [`ConnectionHandle::on_close()`] errors.
void connectionHandleOnClose(
        {required ConnectionHandle connection, required Object f}) =>
    RustLib.instance.api
        .crateApiDartApiConnectionHandleOnClose(connection: connection, f: f);

/// Sets a callback to be invoked once a new [`remote::Track`] is added to the
/// provided `connection`.
///
/// # Errors
///
/// If [`ConnectionHandle::on_remote_track_added()`] errors.
///
/// [`remote::Track`]: media::track::remote::Track
void connectionHandleOnRemoteTrackAdded(
        {required ConnectionHandle connection, required Object f}) =>
    RustLib.instance.api.crateApiDartApiConnectionHandleOnRemoteTrackAdded(
        connection: connection, f: f);

/// Sets a callback to be invoked when a quality score of the provided
/// `connection` is updated by a server.
///
/// # Errors
///
/// If [`ConnectionHandle::on_quality_score_update()`] errors.
void connectionHandleOnQualityScoreUpdate(
        {required ConnectionHandle connection, required Object f}) =>
    RustLib.instance.api.crateApiDartApiConnectionHandleOnQualityScoreUpdate(
        connection: connection, f: f);

/// Returns remote `Member` ID of the provided `connection`.
///
/// # Errors
///
/// If [`ConnectionHandle::get_remote_member_id()`] errors.
String connectionHandleGetRemoteMemberId(
        {required ConnectionHandle connection}) =>
    RustLib.instance.api.crateApiDartApiConnectionHandleGetRemoteMemberId(
        connection: connection);

/// Enables inbound audio in the provided `connection`.
Object connectionHandleEnableRemoteAudio(
        {required ConnectionHandle connection}) =>
    RustLib.instance.api.crateApiDartApiConnectionHandleEnableRemoteAudio(
        connection: connection);

/// Disables inbound audio in the provided `connection`.
Object connectionHandleDisableRemoteAudio(
        {required ConnectionHandle connection}) =>
    RustLib.instance.api.crateApiDartApiConnectionHandleDisableRemoteAudio(
        connection: connection);

/// Enables inbound video in the provided `connection`.
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
Object connectionHandleEnableRemoteVideo(
        {required ConnectionHandle connection, MediaSourceKind? sourceKind}) =>
    RustLib.instance.api.crateApiDartApiConnectionHandleEnableRemoteVideo(
        connection: connection, sourceKind: sourceKind);

/// Disables inbound video in the provided `connection`.
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
Object connectionHandleDisableRemoteVideo(
        {required ConnectionHandle connection, MediaSourceKind? sourceKind}) =>
    RustLib.instance.api.crateApiDartApiConnectionHandleDisableRemoteVideo(
        connection: connection, sourceKind: sourceKind);

/// Sets the provided [`Dart_Handle`] as a callback for the Rust panic hook.
void onPanic({required Object cb}) =>
    RustLib.instance.api.crateApiDartApiOnPanic(cb: cb);

/// Instantiates a new [`Jason`] interface to interact with this library.
Jason jasonNew() => RustLib.instance.api.crateApiDartApiJasonNew();

/// Creates a new [`Room`] and returns its [`RoomHandle`].
///
/// [`Room`]: room::Room
RoomHandle jasonInitRoom({required Jason jason}) =>
    RustLib.instance.api.crateApiDartApiJasonInitRoom(jason: jason);

/// Returns a [`MediaManagerHandle`].
MediaManagerHandle jasonMediaManager({required Jason jason}) =>
    RustLib.instance.api.crateApiDartApiJasonMediaManager(jason: jason);

/// Closes the provided [`RoomHandle`].
void jasonCloseRoom({required Jason jason, required RoomHandle roomToDelete}) =>
    RustLib.instance.api.crateApiDartApiJasonCloseRoom(
        jason: jason, roomToDelete: roomToDelete);

/// Closes the provided [`RoomHandle`].
void jasonDispose({required Jason jason}) =>
    RustLib.instance.api.crateApiDartApiJasonDispose(jason: jason);

/// Returns the [`LocalMediaTrack`] from the [`ForeignClass`] address.
LocalMediaTrack localMediaTrackFromPtr({required BigInt ptr}) =>
    RustLib.instance.api.crateApiDartApiLocalMediaTrackFromPtr(ptr: ptr);

/// Returns the [`Vec<RustOpaque<LocalMediaTrack>>`] from the [`ForeignClass`]
/// address.
List<LocalMediaTrack> vecLocalTracksFromPtr({required BigInt ptr}) =>
    RustLib.instance.api.crateApiDartApiVecLocalTracksFromPtr(ptr: ptr);

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of the
/// provided [`LocalMediaTrack`].
///
/// [`MediaStreamTrack`]: platform::MediaStreamTrack
Object localMediaTrackGetTrack({required LocalMediaTrack track}) =>
    RustLib.instance.api.crateApiDartApiLocalMediaTrackGetTrack(track: track);

/// Returns a [`MediaKind::Audio`] if the provided [`LocalMediaTrack`]
/// represents an audio track, or a [`MediaKind::Video`] if it represents a
/// video track.
MediaKind localMediaTrackKind({required LocalMediaTrack track}) =>
    RustLib.instance.api.crateApiDartApiLocalMediaTrackKind(track: track);

/// Sets callback to invoke when this [`LocalMediaTrack`] is ended.
void localMediaTrackOnEnded(
        {required LocalMediaTrack track, required Object f}) =>
    RustLib.instance.api
        .crateApiDartApiLocalMediaTrackOnEnded(track: track, f: f);

/// Returns a [`media::MediaStreamTrackState::Live`] if this [`LocalMediaTrack`]
/// is active, or a [`media::MediaStreamTrackState::Ended`] if it has ended.
Object localMediaTrackState({required LocalMediaTrack track}) =>
    RustLib.instance.api.crateApiDartApiLocalMediaTrackState(track: track);

/// Indicates whether an `OnAudioLevelChangedCallback` is supported for this
/// [`LocalMediaTrack`].
bool isOnAudioLevelAvailable({required LocalMediaTrack track}) =>
    RustLib.instance.api.crateApiDartApiIsOnAudioLevelAvailable(track: track);

/// Sets the provided `OnAudioLevelChangedCallback` for this
/// [`LocalMediaTrack`].
///
/// It's called for live [`LocalMediaTrack`]s when their audio level changes.
void onAudioLevelChanged({required LocalMediaTrack track, required Object f}) =>
    RustLib.instance.api.crateApiDartApiOnAudioLevelChanged(track: track, f: f);

/// Returns a [`MediaSourceKind::Device`] if the provided [`LocalMediaTrack`] is
/// sourced from some device (webcam/microphone), or a
/// [`MediaSourceKind::Display`] if it's captured via
/// [MediaDevices.getDisplayMedia()][1].
///
/// [1]: https://w3.org/TR/screen-capture#dom-mediadevices-getdisplaymedia
MediaSourceKind localMediaTrackMediaSourceKind(
        {required LocalMediaTrack track}) =>
    RustLib.instance.api
        .crateApiDartApiLocalMediaTrackMediaSourceKind(track: track);

/// Frees the data behind the provided opaque local track.
Object localMediaTrackFree({required LocalMediaTrack track}) =>
    RustLib.instance.api.crateApiDartApiLocalMediaTrackFree(track: track);

/// Returns the [`Vec<ApiMediaDeviceDetails>`] from the [`ForeignClass`]
/// address.
List<ApiMediaDeviceDetails> vecMediaDeviceDetailsFromPtr(
        {required BigInt ptr}) =>
    RustLib.instance.api.crateApiDartApiVecMediaDeviceDetailsFromPtr(ptr: ptr);

/// Returns the [`Vec<RustOpaque<ApiMediaDisplayDetails>>`] from the
/// [`ForeignClass`] address.
List<ApiMediaDisplayDetails> vecMediaDisplayDetailsFromPtr(
        {required BigInt ptr}) =>
    RustLib.instance.api.crateApiDartApiVecMediaDisplayDetailsFromPtr(ptr: ptr);

/// Returns [`LocalMediaTrack`]s objects, built from the provided
/// [`ApiMediaStreamSettings`].
Object mediaManagerHandleInitLocalTracks(
        {required MediaManagerHandle manager,
        required ApiMediaStreamSettings caps}) =>
    RustLib.instance.api.crateApiDartApiMediaManagerHandleInitLocalTracks(
        manager: manager, caps: caps);

/// Returns a list of [`ApiMediaDeviceDetails`] objects representing available
/// media input and devices, such as microphones, cameras, and so forth.
Object mediaManagerHandleEnumerateDevices(
        {required MediaManagerHandle manager}) =>
    RustLib.instance.api
        .crateApiDartApiMediaManagerHandleEnumerateDevices(manager: manager);

/// Returns a list of [`ApiMediaDisplayDetails`] objects representing available
/// sources that can be used for screen capturing.
Object mediaManagerHandleEnumerateDisplays(
        {required MediaManagerHandle manager}) =>
    RustLib.instance.api
        .crateApiDartApiMediaManagerHandleEnumerateDisplays(manager: manager);

/// Switches the current output audio device to the device with the provided
/// `device_id`.
Object mediaManagerHandleSetOutputAudioId(
        {required MediaManagerHandle manager, required String deviceId}) =>
    RustLib.instance.api.crateApiDartApiMediaManagerHandleSetOutputAudioId(
        manager: manager, deviceId: deviceId);

/// Sets the microphone volume level in percents.
Object mediaManagerHandleSetMicrophoneVolume(
        {required MediaManagerHandle manager, required PlatformInt64 level}) =>
    RustLib.instance.api.crateApiDartApiMediaManagerHandleSetMicrophoneVolume(
        manager: manager, level: level);

/// Indicates whether it's possible to access microphone volume settings.
Object mediaManagerHandleMicrophoneVolumeIsAvailable(
        {required MediaManagerHandle manager}) =>
    RustLib.instance.api
        .crateApiDartApiMediaManagerHandleMicrophoneVolumeIsAvailable(
            manager: manager);

/// Returns the current microphone volume level in percents.
Object mediaManagerHandleMicrophoneVolume(
        {required MediaManagerHandle manager}) =>
    RustLib.instance.api
        .crateApiDartApiMediaManagerHandleMicrophoneVolume(manager: manager);

/// Subscribes onto the [`MediaManagerHandle`]'s `devicechange` event.
/// Sets an ideal [frameRate][1] constraint.
///
/// # Errors
///
/// If [`MediaManagerHandle::on_device_change()`] errors.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
void mediaManagerHandleOnDeviceChange(
        {required MediaManagerHandle manager, required Object cb}) =>
    RustLib.instance.api.crateApiDartApiMediaManagerHandleOnDeviceChange(
        manager: manager, cb: cb);

/// Returns the [`ReconnectHandle`] from the [`ForeignClass`] address.
ReconnectHandle reconnectHandleFromPtr({required BigInt ptr}) =>
    RustLib.instance.api.crateApiDartApiReconnectHandleFromPtr(ptr: ptr);

/// Tries to reconnect a [`Room`] after the provided delay in milliseconds.
///
/// If the [`Room`] is already reconnecting then new reconnection attempt won't
/// be performed. Instead, it will wait for the first reconnection attempt
/// result and use it here.
///
/// [`Room`]: room::Room
Object reconnectHandleReconnectWithDelay(
        {required ReconnectHandle reconnectHandle, required int delayMs}) =>
    RustLib.instance.api.crateApiDartApiReconnectHandleReconnectWithDelay(
        reconnectHandle: reconnectHandle, delayMs: delayMs);

/// Tries to reconnect a [`Room`] in a loop with a growing backoff delay.
///
/// The first attempt will be performed immediately, and the second attempt will
/// be performed after `starting_delay_ms`.
///
/// Delay between reconnection attempts won't be greater than
/// `max_delay_ms`.
///
/// After each reconnection attempt, delay between reconnections will be
/// multiplied by the given `multiplier` until it reaches `max_delay_ms`.
///
/// If `multiplier` is a negative number then it will be considered as `0.0`.
/// This might cause a busy loop, so it's not recommended.
///
/// Max elapsed time can be limited with an optional `max_elapsed_time_ms`
/// argument.
///
/// If the [`Room`] is already reconnecting then new reconnection attempt won't
/// be performed. Instead, it will wait for the first reconnection attempt
/// result and use it here.
///
/// [`Room`]: room::Room
Object reconnectHandleReconnectWithBackoff(
        {required ReconnectHandle reconnectHandle,
        required int startingDelay,
        required double multiplier,
        required int maxDelay,
        int? maxElapsedTimeMs}) =>
    RustLib.instance.api.crateApiDartApiReconnectHandleReconnectWithBackoff(
        reconnectHandle: reconnectHandle,
        startingDelay: startingDelay,
        multiplier: multiplier,
        maxDelay: maxDelay,
        maxElapsedTimeMs: maxElapsedTimeMs);

/// Returns the [`RemoteMediaTrack`] from the [`ForeignClass`] address.
RemoteMediaTrack remoteMediaTrackFromPtr({required BigInt ptr}) =>
    RustLib.instance.api.crateApiDartApiRemoteMediaTrackFromPtr(ptr: ptr);

/// Returns a [`Dart_Handle`] to the underlying [`MediaStreamTrack`] of this
/// [`RemoteMediaTrack`].
///
/// [`MediaStreamTrack`]: platform::MediaStreamTrack
Object remoteMediaTrackGetTrack({required RemoteMediaTrack track}) =>
    RustLib.instance.api.crateApiDartApiRemoteMediaTrackGetTrack(track: track);

/// Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
void remoteMediaTrackOnMuted(
        {required RemoteMediaTrack track, required Object f}) =>
    RustLib.instance.api
        .crateApiDartApiRemoteMediaTrackOnMuted(track: track, f: f);

/// Sets callback to invoke when this [`RemoteMediaTrack`] is unmuted.
void remoteMediaTrackOnUnmuted(
        {required RemoteMediaTrack track, required Object f}) =>
    RustLib.instance.api
        .crateApiDartApiRemoteMediaTrackOnUnmuted(track: track, f: f);

/// Sets callback to invoke when this [`RemoteMediaTrack`] is stopped.
void remoteMediaTrackOnStopped(
        {required RemoteMediaTrack track, required Object f}) =>
    RustLib.instance.api
        .crateApiDartApiRemoteMediaTrackOnStopped(track: track, f: f);

/// Sets callback to invoke whenever this [`RemoteMediaTrack`]'s general
/// [`MediaDirection`] is changed.
void remoteMediaTrackOnMediaDirectionChanged(
        {required RemoteMediaTrack track, required Object f}) =>
    RustLib.instance.api.crateApiDartApiRemoteMediaTrackOnMediaDirectionChanged(
        track: track, f: f);

/// Indicate whether this [`RemoteMediaTrack`] is muted.
bool remoteMediaTrackMuted({required RemoteMediaTrack track}) =>
    RustLib.instance.api.crateApiDartApiRemoteMediaTrackMuted(track: track);

/// Returns this [`RemoteMediaTrack`]'s kind (audio/video).
MediaKind remoteMediaTrackKind({required RemoteMediaTrack track}) =>
    RustLib.instance.api.crateApiDartApiRemoteMediaTrackKind(track: track);

/// Returns this [`RemoteMediaTrack`]'s media source kind.
MediaSourceKind remoteMediaTrackMediaSourceKind(
        {required RemoteMediaTrack track}) =>
    RustLib.instance.api
        .crateApiDartApiRemoteMediaTrackMediaSourceKind(track: track);

/// Returns the current general [`MediaDirection`] of this [`RemoteMediaTrack`].
MediaDirection remoteMediaTrackMediaDirection(
        {required RemoteMediaTrack track}) =>
    RustLib.instance.api
        .crateApiDartApiRemoteMediaTrackMediaDirection(track: track);

/// Returns the [`RoomCloseReason`] from the [`ForeignClass`] address.
RoomCloseReason roomCloseReasonFromPtr({required BigInt ptr}) =>
    RustLib.instance.api.crateApiDartApiRoomCloseReasonFromPtr(ptr: ptr);

/// Connects to a media server and joins the [`Room`] with the provided
/// authorization `token`.
///
/// Authorization token has a fixed format:
/// `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
/// (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
///
/// [`Room`]: room::Room
Object roomHandleJoin(
        {required RoomHandle roomHandle, required String token}) =>
    RustLib.instance.api
        .crateApiDartApiRoomHandleJoin(roomHandle: roomHandle, token: token);

/// Updates this [`Room`]'s [`ApiMediaStreamSettings`]. This affects all the
/// [`PeerConnection`]s in this [`Room`]. If [`ApiMediaStreamSettings`] are
/// configured for some [`Room`], then this [`Room`] can only send media tracks
/// that correspond to these settings. [`ApiMediaStreamSettings`] update will
/// change media tracks in all sending peers, so that might cause a new
/// [getUserMedia()][1] request to happen.
///
/// Media obtaining/injection errors are additionally fired to
/// `on_failed_local_media` callback.
///
/// If `stop_first` set to `true` then affected local `Tracks` will be
/// dropped before new [`ApiMediaStreamSettings`] are applied. This is usually
/// required when changing video source device due to hardware limitations,
/// e.g. having an active track sourced from device `A` may hinder
/// [getUserMedia()][1] requests to device `B`.
///
/// `rollback_on_fail` option configures [`ApiMediaStreamSettings`] update
/// request to automatically rollback to previous settings if new settings
/// cannot be applied.
///
/// If recovering from fail state isn't possible then affected media types will
/// be disabled.
///
/// [`Room`]: room::Room
/// [`PeerConnection`]: crate::peer::PeerConnection
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
Object roomHandleSetLocalMediaSettings(
        {required RoomHandle roomHandle,
        required ApiMediaStreamSettings settings,
        required bool stopFirst,
        required bool rollbackOnFail}) =>
    RustLib.instance.api.crateApiDartApiRoomHandleSetLocalMediaSettings(
        roomHandle: roomHandle,
        settings: settings,
        stopFirst: stopFirst,
        rollbackOnFail: rollbackOnFail);

/// Mutes outbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
Object roomHandleMuteAudio({required RoomHandle roomHandle}) =>
    RustLib.instance.api
        .crateApiDartApiRoomHandleMuteAudio(roomHandle: roomHandle);

/// Unmutes outbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
Object roomHandleUnmuteAudio({required RoomHandle roomHandle}) =>
    RustLib.instance.api
        .crateApiDartApiRoomHandleUnmuteAudio(roomHandle: roomHandle);

/// Enables outbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
Object roomHandleEnableAudio({required RoomHandle roomHandle}) =>
    RustLib.instance.api
        .crateApiDartApiRoomHandleEnableAudio(roomHandle: roomHandle);

/// Disables outbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
Object roomHandleDisableAudio({required RoomHandle roomHandle}) =>
    RustLib.instance.api
        .crateApiDartApiRoomHandleDisableAudio(roomHandle: roomHandle);

/// Mutes outbound video in the provided [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// # Errors
///
/// If `source_kind` is not a [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
Object roomHandleMuteVideo(
        {required RoomHandle roomHandle, MediaSourceKind? sourceKind}) =>
    RustLib.instance.api.crateApiDartApiRoomHandleMuteVideo(
        roomHandle: roomHandle, sourceKind: sourceKind);

/// Unmutes outbound video in the provided [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// # Errors
///
/// If `source_kind` is not a [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
Object roomHandleUnmuteVideo(
        {required RoomHandle roomHandle, MediaSourceKind? sourceKind}) =>
    RustLib.instance.api.crateApiDartApiRoomHandleUnmuteVideo(
        roomHandle: roomHandle, sourceKind: sourceKind);

/// Enables outbound video in the provided [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
Object roomHandleEnableVideo(
        {required RoomHandle roomHandle, MediaSourceKind? sourceKind}) =>
    RustLib.instance.api.crateApiDartApiRoomHandleEnableVideo(
        roomHandle: roomHandle, sourceKind: sourceKind);

/// Disables outbound video in the provided [`Room`].
///
/// Affects only video with specific [`MediaSourceKind`] if specified.
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
Object roomHandleDisableVideo(
        {required RoomHandle roomHandle, MediaSourceKind? sourceKind}) =>
    RustLib.instance.api.crateApiDartApiRoomHandleDisableVideo(
        roomHandle: roomHandle, sourceKind: sourceKind);

/// Enables inbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
Object roomHandleEnableRemoteAudio({required RoomHandle roomHandle}) =>
    RustLib.instance.api
        .crateApiDartApiRoomHandleEnableRemoteAudio(roomHandle: roomHandle);

/// Disables inbound audio in the provided [`Room`].
///
/// [`Room`]: room::Room
Object roomHandleDisableRemoteAudio({required RoomHandle roomHandle}) =>
    RustLib.instance.api
        .crateApiDartApiRoomHandleDisableRemoteAudio(roomHandle: roomHandle);

/// Enables inbound video in the provided [`Room`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
Object roomHandleEnableRemoteVideo(
        {required RoomHandle roomHandle, MediaSourceKind? sourceKind}) =>
    RustLib.instance.api.crateApiDartApiRoomHandleEnableRemoteVideo(
        roomHandle: roomHandle, sourceKind: sourceKind);

/// Disables inbound video in the provided [`Room`].
///
/// Affects only video with the specific [`MediaSourceKind`], if specified.
///
/// # Errors
///
/// If `source_kind` is not [`MediaSourceKind`] index.
///
/// [`Room`]: room::Room
Object roomHandleDisableRemoteVideo(
        {required RoomHandle roomHandle, MediaSourceKind? sourceKind}) =>
    RustLib.instance.api.crateApiDartApiRoomHandleDisableRemoteVideo(
        roomHandle: roomHandle, sourceKind: sourceKind);

/// Sets a callback to be invoked once a new [`Connection`] with some remote
/// `Peer` is established.
///
/// # Errors
///
/// If [`RoomHandle::on_new_connection()`] errors.
///
/// [`Connection`]: connection::Connection
void roomHandleOnNewConnection(
        {required RoomHandle roomHandle, required Object cb}) =>
    RustLib.instance.api.crateApiDartApiRoomHandleOnNewConnection(
        roomHandle: roomHandle, cb: cb);

/// Sets a callback to be invoked once the provided [`Room`] is closed,
/// providing a [`RoomCloseReason`].
///
/// # Errors
///
/// If [`RoomHandle::on_close()`] errors.
///
/// [`Room`]: room::Room
void roomHandleOnClose({required RoomHandle roomHandle, required Object cb}) =>
    RustLib.instance.api
        .crateApiDartApiRoomHandleOnClose(roomHandle: roomHandle, cb: cb);

/// Sets a callback to be invoked when a new [`LocalMediaTrack`] is added to
/// the provided [`Room`].
///
/// This might happen in such cases:
/// 1. Media server initiates a media request.
/// 2. `enable_audio`/`enable_video` is called.
/// 3. [`MediaStreamSettings`] updated via `set_local_media_settings`.
///
/// # Errors
///
/// If [`RoomHandle::on_local_track()`] errors.
///
/// [`MediaStreamSettings`]: media::MediaStreamSettings
/// [`Room`]: room::Room
void roomHandleOnLocalTrack(
        {required RoomHandle roomHandle, required Object cb}) =>
    RustLib.instance.api
        .crateApiDartApiRoomHandleOnLocalTrack(roomHandle: roomHandle, cb: cb);

/// Sets a callback to be invoked once a connection with server is lost.
///
/// # Errors
///
/// If [`RoomHandle::on_connection_loss()`] errors.
void roomHandleOnConnectionLoss(
        {required RoomHandle roomHandle, required Object cb}) =>
    RustLib.instance.api.crateApiDartApiRoomHandleOnConnectionLoss(
        roomHandle: roomHandle, cb: cb);

/// Sets a callback to be invoked on local media acquisition failures.
///
/// # Errors
///
/// If [`RoomHandle::on_failed_local_media()`] errors.
void roomHandleOnFailedLocalMedia(
        {required RoomHandle roomHandle, required Object cb}) =>
    RustLib.instance.api.crateApiDartApiRoomHandleOnFailedLocalMedia(
        roomHandle: roomHandle, cb: cb);

/// Logs Dart exception.
void logDartException({required String message, required String stackTrace}) =>
    RustLib.instance.api.crateApiDartApiLogDartException(
        message: message, stackTrace: stackTrace);

// Rust type: RustOpaqueMoi<ConnectionHandle>
abstract class ConnectionHandle implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<Jason>
abstract class Jason implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<LocalMediaTrack>
abstract class LocalMediaTrack implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<MediaManagerHandle>
abstract class MediaManagerHandle implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<ReconnectHandle>
abstract class ReconnectHandle implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<RemoteMediaTrack>
abstract class RemoteMediaTrack implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<RoomHandle>
abstract class RoomHandle implements RustOpaqueInterface {}

/// Constraints applicable to audio tracks.
class ApiAudioConstraints {
  /// Identifier of the device generating the content for the media track.
  String? deviceId;

  /// Automatically manages changes in the volume of its source media to
  /// maintain a steady overall volume level.
  ConstrainBoolean? autoGainControl;

  ApiAudioConstraints({
    this.deviceId,
    this.autoGainControl,
  });

  @override
  int get hashCode => deviceId.hashCode ^ autoGainControl.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApiAudioConstraints &&
          runtimeType == other.runtimeType &&
          deviceId == other.deviceId &&
          autoGainControl == other.autoGainControl;
}

@freezed
sealed class ApiConstrainFacingMode with _$ApiConstrainFacingMode {
  const ApiConstrainFacingMode._();

  /// Exact value required for this property.
  const factory ApiConstrainFacingMode.exact(
    FacingMode field0,
  ) = ApiConstrainFacingMode_Exact;

  /// Ideal (target) value for this property.
  const factory ApiConstrainFacingMode.ideal(
    FacingMode field0,
  ) = ApiConstrainFacingMode_Ideal;
}

/// Constraints applicable to video tracks that are sourced from some media
/// device.
class ApiDeviceVideoTrackConstraints {
  /// Identifier of the device generating the content for the media track.
  String? deviceId;

  /// Describes the directions that the camera can face, as seen from the
  /// user's perspective.
  ApiConstrainFacingMode? facingMode;

  /// Height of the video in pixels.
  ConstrainU32? height;

  /// Width of the video in pixels.
  ConstrainU32? width;

  ApiDeviceVideoTrackConstraints({
    this.deviceId,
    this.facingMode,
    this.height,
    this.width,
  });

  @override
  int get hashCode =>
      deviceId.hashCode ^
      facingMode.hashCode ^
      height.hashCode ^
      width.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApiDeviceVideoTrackConstraints &&
          runtimeType == other.runtimeType &&
          deviceId == other.deviceId &&
          facingMode == other.facingMode &&
          height == other.height &&
          width == other.width;
}

/// Constraints applicable to video tracks sourced from a screen capturing.
class ApiDisplayVideoTrackConstraints {
  /// Identifier of the device generating the content for the media track.
  String? deviceId;

  /// [Height][1] of the video in pixels.
  ///
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
  ConstrainU32? height;

  /// [Width][1] of the video in pixels.
  ///
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
  ConstrainU32? width;

  /// [Frame rate][1] of the video.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
  ConstrainU32? frameRate;

  ApiDisplayVideoTrackConstraints({
    this.deviceId,
    this.height,
    this.width,
    this.frameRate,
  });

  @override
  int get hashCode =>
      deviceId.hashCode ^ height.hashCode ^ width.hashCode ^ frameRate.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApiDisplayVideoTrackConstraints &&
          runtimeType == other.runtimeType &&
          deviceId == other.deviceId &&
          height == other.height &&
          width == other.width &&
          frameRate == other.frameRate;
}

/// Representation of a [MediaDeviceInfo][0] ONLY for input devices.
///
/// [0]: https://w3.org/TR/mediacapture-streams#device-info
class ApiMediaDeviceDetails {
  /// [`MediaDeviceKind`] of this [`ApiMediaDeviceDetails`].
  ///
  /// [`MediaDeviceKind`]: MediaDeviceKind
  final MediaDeviceKind kind;

  /// Unique identifier of the device represented by this
  /// [`ApiMediaDeviceDetails`].
  final String deviceId;

  /// Label describing the device represented by this
  /// [`ApiMediaDeviceDetails`] (for example, "External USB Webcam").
  final String label;

  /// Group identifier of the device represented by this
  /// [`ApiMediaDeviceDetails`].
  ///
  /// Two devices have the same group identifier if they belong to the same
  /// physical device. For example, the audio input and output devices
  /// representing the speaker and microphone of the same headset have the
  /// same [groupId][1].
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
  final String? groupId;

  /// Indicates whether the last attempt to use the provided device failed.
  final bool isFailed;

  const ApiMediaDeviceDetails({
    required this.kind,
    required this.deviceId,
    required this.label,
    this.groupId,
    required this.isFailed,
  });

  @override
  int get hashCode =>
      kind.hashCode ^
      deviceId.hashCode ^
      label.hashCode ^
      groupId.hashCode ^
      isFailed.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApiMediaDeviceDetails &&
          runtimeType == other.runtimeType &&
          kind == other.kind &&
          deviceId == other.deviceId &&
          label == other.label &&
          groupId == other.groupId &&
          isFailed == other.isFailed;
}

/// Representation of a display source.
class ApiMediaDisplayDetails {
  /// Unique identifier of the display represented by this
  /// [`ApiMediaDisplayDetails`].
  final String deviceId;

  /// Title describing the represented display.
  final String? title;

  const ApiMediaDisplayDetails({
    required this.deviceId,
    this.title,
  });

  @override
  int get hashCode => deviceId.hashCode ^ title.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApiMediaDisplayDetails &&
          runtimeType == other.runtimeType &&
          deviceId == other.deviceId &&
          title == other.title;
}

/// [MediaStreamConstraints][1] wrapper.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
class ApiMediaStreamSettings {
  /// [MediaStreamConstraints][1] for the audio media type.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
  ApiAudioConstraints? audio;

  /// [MediaStreamConstraints][1] for the device video media type.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
  ApiDeviceVideoTrackConstraints? deviceVideo;

  /// [MediaStreamConstraints][1] for the display video media type.
  ///
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
  ApiDisplayVideoTrackConstraints? displayVideo;

  ApiMediaStreamSettings({
    this.audio,
    this.deviceVideo,
    this.displayVideo,
  });

  @override
  int get hashCode =>
      audio.hashCode ^ deviceVideo.hashCode ^ displayVideo.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ApiMediaStreamSettings &&
          runtimeType == other.runtimeType &&
          audio == other.audio &&
          deviceVideo == other.deviceVideo &&
          displayVideo == other.displayVideo;
}
