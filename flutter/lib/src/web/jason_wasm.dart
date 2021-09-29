@JS()
library pkg;

import 'package:js/js.dart';
import 'package:js/js_util.dart' show promiseToFuture;
import 'dart:html' as html;

/// tslint:disable
/// eslint-disable
/// Describes directions that a camera can face, as seen from a user's
/// perspective. Representation of a [VideoFacingModeEnum][1].
/// [1]: https://w3.org/TR/mediacapture-streams#dom-videofacingmodeenum
@JS()
class FacingMode {
  external static num get

      /// Facing towards a user (a self-view camera).
      User;
  external static num get

      /// Facing away from a user (viewing the environment).
      Environment;
  external static num get

      /// Facing to the left of a user.
      Left;
  external static num get

      /// Facing to the right of a user.
      Right;
}

/// Media source type.
@JS()
class MediaSourceKind {
  external static num get

      /// Media is sourced from some media device (webcam or microphone).
      Device;
  external static num get

      /// Media is obtained via screen capturing.
      Display;
}

/// [MediaStreamTrack.kind][1] representation.
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
@JS()
class MediaKind {
  external static num get

      /// Audio track.
      Audio;
  external static num get

      /// Video track.
      Video;
}

/// Constraints applicable to audio tracks.
@JS()
class AudioTrackConstraints {
  external void free();

  /// Creates new [`AudioTrackConstraints`] with none constraints configured.
  external factory AudioTrackConstraints();

  /// Sets an exact [deviceId][1] constraint.
  /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
  external void device_id(String device_id);
}

/// Connection with a specific remote `Member`, that is used on JS side.
/// Like all the handles it contains a weak reference to the object that is
/// managed by Rust, so its methods will fail if a weak reference could not be
/// upgraded.
@JS()
class ConnectionHandle {
  external void free();

  /// Sets callback, invoked when this [`Connection`] is closed.
  /// [`Connection`]: connection::Connection
  external void on_close(Function cb);

  /// Returns ID of the remote `Member`.
  external String get_remote_member_id();

  /// Sets callback, invoked when a new [`RemoteMediaTrack`] is added to this
  /// [`Connection`].
  /// [`Connection`]: connection::Connection
  /// [`RemoteMediaTrack`]: crate::api::RemoteMediaTrack
  external void on_remote_track_added(Function cb);

  /// Sets callback, invoked when connection quality score is updated by a
  /// server.
  external void on_quality_score_update(Function cb);
}

/// Exception returned from [`RoomHandle::set_local_media_settings()`][1].
/// [1]: crate::api::RoomHandle::set_local_media_settings
@JS()
class ConstraintsUpdateException {
  external void free();

  /// Returns name of this [`ConstraintsUpdateException`].
  external String name();

  /// Returns an [`Error`] if this [`ConstraintsUpdateException`] represents
  /// a `RecoveredException` or a `RecoverFailedException`.
  /// Returns `undefined` otherwise.
  external dynamic /*JasonError|dynamic*/ recover_reason();

  /// Returns [`js_sys::Array`] with an [`Error`]s if this
  /// [`ConstraintsUpdateException`] represents a `RecoverFailedException`.
  external dynamic recover_fail_reasons();

  /// Returns [`Error`] if this [`ConstraintsUpdateException`] represents
  /// an `ErroredException`.
  /// Returns `undefined` otherwise.
  external dynamic /*JasonError|dynamic*/ error();
}

/// Constraints applicable to video tracks that are sourced from some media
/// device.
@JS()
class DeviceVideoTrackConstraints {
  external void free();

  /// Creates new [`DeviceVideoTrackConstraints`] with none constraints
  /// configured.
  external factory DeviceVideoTrackConstraints();

  /// Sets an exact [deviceId][1] constraint.
  /// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
  external void device_id(String device_id);

  /// Sets an exact [facingMode][1] constraint.
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
  external void exact_facing_mode(num facing_mode);

  /// Sets an ideal [facingMode][1] constraint.
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
  external void ideal_facing_mode(num facing_mode);

  /// Sets an exact [`height`][1] constraint.
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
  external void exact_height(num height);

  /// Sets an ideal [`height`][1] constraint.
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
  external void ideal_height(num height);

  /// Sets a range of a [`height`][1] constraint.
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-height
  external void height_in_range(num min, num max);

  /// Sets an exact [`width`][1] constraint.
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
  external void exact_width(num width);

  /// Sets an ideal [`width`][1] constraint.
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
  external void ideal_width(num width);

  /// Sets a range of a [`width`][1] constraint.
  /// [1]: https://tinyurl.com/w3-streams#def-constraint-width
  external void width_in_range(num min, num max);
}

/// Constraints applicable to video tracks sourced from a screen capturing.
@JS()
class DisplayVideoTrackConstraints {
  external void free();

  /// Creates new [`DisplayVideoTrackConstraints`] with none constraints
  /// configured.
  external factory DisplayVideoTrackConstraints();
}

/// Representation of a [MediaDeviceInfo][1].
/// [1]: https://w3.org/TR/mediacapture-streams#device-info
@JS()
class InputDeviceInfo {
  external void free();

  /// Returns a unique identifier for the represented device.
  external String device_id();

  /// Returns a kind of the represented device.
  /// This representation of [MediaDeviceInfo][1] is for input device ONLY.
  /// [1]: https://w3.org/TR/mediacapture-streams#device-info
  external num kind();

  /// Returns label describing the represented device (for example "External
  /// USB Webcam").
  /// If the device has no associated label, then returns an empty string.
  external String label();

  /// Returns a group identifier of the represented device.
  /// Two devices have the same group identifier if they belong to the same
  /// physical device. For example, the audio input and output devices
  /// representing the speaker and microphone of the same headset have the
  /// same [groupId][1].
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
  external String group_id();
}

/// General JS side library interface.
/// Responsible for managing shared transports, local media and room
/// initialization.
@JS()
class Jason {
  external void free();

  /// Instantiates a new [`Jason`] interface to interact with this library.
  external factory Jason();

  /// Creates a new `Room` and returns its [`RoomHandle`].
  external RoomHandle init_room();

  /// Returns a [`MediaManagerHandle`].
  external MediaManagerHandle media_manager();

  /// Closes the provided [`RoomHandle`].
  external void close_room(RoomHandle room_to_delete);

  /// Drops [`Jason`] API object, so all the related objects (rooms,
  /// connections, streams etc.) respectively. All objects related to this
  /// [`Jason`] API object will be detached (you will still hold them, but
  /// unable to use).
  external void dispose();
}

/// Representation of an app error exported to JS side.
/// Contains JS side error if it's the cause, and a trace information.
@JS()
class JasonError {
  external void free();

  /// Returns a name of this error.
  external String name();

  /// Returns a message of this error.
  external String message();

  /// Returns a trace information of this error.
  external String trace();

  /// Returns a JS side error if it's the cause.
  external dynamic /*Error|dynamic*/ source();
}

/// Wrapper around a local [MediaStreamTrack][1].
/// Backed by a strong reference to the actual track implementing auto stop on
/// dropping. Can be manually dropped with a `free()` call.
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
@JS()
class LocalMediaTrack {
  external void free();

  /// Returns the underlying [MediaStreamTrack][1].
  /// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
  external html.MediaStreamTrack get_track();

  /// Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
  /// audio track, or a [`MediaKind::Video`] if it represents a video track.
  external num kind();

  /// Returns a [`MediaSourceKind::Device`] if this [`LocalMediaTrack`] is
  /// sourced from some device (webcam/microphone), or a
  /// [`MediaSourceKind::Display`] if it's captured via
  /// [MediaDevices.getDisplayMedia()][1].
  /// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
  external num media_source_kind();
}

/// [`MediaManagerHandle`] is a weak reference to a [`MediaManager`].
/// [`MediaManager`] performs all the media acquisition requests
/// ([getUserMedia()][1]/[getDisplayMedia()][2]) and stores all the received
/// tracks for further re-usage.
/// [`MediaManager`] stores weak references to [`LocalMediaTrack`]s, so if there
/// are no strong references to some track, then this track is stopped and
/// removed from [`MediaManager`].
/// Like all the handles it contains a weak reference to the object that is
/// managed by Rust, so its methods will fail if a weak reference could not be
/// upgraded.
/// [`MediaManager`]: media::MediaManager
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
/// [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
@JS()
class MediaManagerHandle {
  external void free();
}

@JS("MediaManagerHandle")
abstract class _MediaManagerHandle {
  /// Returns a list of [`InputDeviceInfo`] objects representing available
  /// media input and output devices, such as microphones, cameras, and so
  /// forth.
  external Promise<List<dynamic>> enumerate_devices();

  /// Returns [`LocalMediaTrack`]s objects, built from the provided
  /// [`MediaStreamSettings`].
  external Promise<List<dynamic>> init_local_tracks(MediaStreamSettings caps);
}

extension MediaManagerHandleExtensions on MediaManagerHandle {
  Future<List<dynamic>> enumerate_devices() {
    final tt = this as _MediaManagerHandle;
    return promiseToFuture(tt.enumerate_devices());
  }

  Future<List<dynamic>> init_local_tracks(MediaStreamSettings caps) {
    final tt = this as _MediaManagerHandle;
    return promiseToFuture(tt.init_local_tracks(caps));
  }
}

/// [MediaStreamConstraints][1] wrapper.
/// [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
@JS()
class MediaStreamSettings {
  external void free();

  /// Creates new [`MediaStreamSettings`] with none constraints configured.
  external factory MediaStreamSettings();

  /// Specifies the nature and settings of an audio [MediaStreamTrack][1].
  /// [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
  external void audio(AudioTrackConstraints constraints);

  /// Set constraints that will be used to obtain a local video sourced from
  /// a media device.
  external void device_video(DeviceVideoTrackConstraints constraints);

  /// Set constraints that will be used to capture a local video from a user's
  /// display.
  external void display_video(DisplayVideoTrackConstraints constraints);
}

/// Handle that JS side can reconnect to a media server with when a connection
/// is lost.
/// This handle is passed into a [`RoomHandle.on_connection_loss`] callback.
/// Like all the handles it contains a weak reference to the object that is
/// managed by Rust, so its methods will fail if a weak reference could not be
/// upgraded.
/// [`RoomHandle.on_connection_loss`]: crate::api::RoomHandle.on_connection_loss
@JS()
class ReconnectHandle {
  external void free();
}

@JS("ReconnectHandle")
abstract class _ReconnectHandle {
  /// Tries to reconnect after the provided delay in milliseconds.
  /// If [`RpcSession`] is already reconnecting then a new reconnection
  /// attempt won't be performed. Instead, it will wait for the first
  /// reconnection attempt result and use it.
  /// [`RpcSession`]: rpc::RpcSession
  external Promise<dynamic> reconnect_with_delay(num delay_ms);

  /// Tries to reconnect a [`RpcSession`] in a loop with a growing backoff
  /// delay.
  /// The first attempt will be performed immediately, and the second attempt
  /// will be performed after `starting_delay_ms`.
  /// Delay between reconnection attempts won't be greater than
  /// `max_delay_ms`.
  /// After each reconnection attempt, delay between reconnections will be
  /// multiplied by the given `multiplier` until it reaches `max_delay_ms`.
  /// If `multiplier` is a negative number then it will be considered as
  /// `0.0`. This might cause a busy loop, so it's not recommended.
  /// Max elapsed time can be limited with an optional `max_elapsed_time_ms`
  /// argument.
  /// If [`RpcSession`] is already reconnecting then new reconnection attempt
  /// won't be performed. Instead, it will wait for the first reconnection
  /// attempt result and use it here.
  /// [`RpcSession`]: rpc::RpcSession
  external Promise<dynamic> reconnect_with_backoff(num starting_delay_ms,
      num multiplier, num max_delay, num? max_elapsed_time_ms);
}

extension ReconnectHandleExtensions on ReconnectHandle {
  Future<dynamic> reconnect_with_delay(num delay_ms) {
    final tt = this as _ReconnectHandle;
    return promiseToFuture(tt.reconnect_with_delay(delay_ms));
  }

  Future<dynamic> reconnect_with_backoff(num starting_delay_ms, num multiplier,
      num max_delay, num? max_elapsed_time_ms) {
    final tt = this as _ReconnectHandle;
    return promiseToFuture(tt.reconnect_with_backoff(
        starting_delay_ms, multiplier, max_delay, max_elapsed_time_ms));
  }
}

/// Wrapper around a received remote [MediaStreamTrack][1].
/// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
@JS()
class RemoteMediaTrack {
  external void free();

  /// Returns the underlying [MediaStreamTrack][1].
  /// [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
  external html.MediaStreamTrack get_track();

  /// Indicates whether this [`RemoteMediaTrack`] is enabled.
  external bool enabled();

  /// Indicates whether this [`RemoteMediaTrack`] is muted.
  external bool muted();

  /// Sets callback, invoked when this [`RemoteMediaTrack`] is enabled.
  external void on_enabled(Function cb);

  /// Sets callback, invoked when this [`RemoteMediaTrack`] is disabled.
  external void on_disabled(Function cb);

  /// Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
  external void on_muted(Function cb);

  /// Sets callback to invoke when this [`RemoteMediaTrack`] is unmuted.
  external void on_unmuted(Function cb);

  /// Sets callback to invoke when this [`RemoteMediaTrack`] is stopped.
  external void on_stopped(Function cb);

  /// Returns a [`MediaKind::Audio`] if this [`RemoteMediaTrack`] represents
  /// an audio track, or a [`MediaKind::Video`] if it represents a video
  /// track.
  external num kind();

  /// Returns a [`MediaSourceKind::Device`] if this [`RemoteMediaTrack`] is
  /// sourced from some device (webcam/microphone), or a
  /// [`MediaSourceKind::Display`] if it's captured via
  /// [MediaDevices.getDisplayMedia()][1].
  /// [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
  external num media_source_kind();
}

/// Reason of why a [`Room`] is closed.
/// This struct is passed to a [`RoomHandle::on_close`] JS side callback.
/// [`Room`]: room::Room
/// [`RoomHandle::on_close`]: crate::api::RoomHandle::on_close
@JS()
class RoomCloseReason {
  external void free();

  /// Returns the [`Room`]'s close reason.
  /// [`Room`]: room::Room
  external String reason();

  /// Indicates whether the [`Room`] was closed by server.
  /// [`Room`]: room::Room
  external bool is_closed_by_server();

  /// Indicates whether the [`Room`] close reason is considered as an error.
  /// [`Room`]: room::Room
  external bool is_err();
}

/// JS side handle to a [`Room`] where all the media happens.
/// Like all handles it contains a weak reference to the object that is managed
/// by Rust, so its methods will fail if a weak reference could not be upgraded.
/// [`Room`]: room::Room
@JS()
class RoomHandle {
  external void free();

  /// Sets callback, invoked when a new [`Connection`] with some remote
  /// `Member` is established.
  /// [`Connection`]: crate::connection::Connection
  external void on_new_connection(Function cb);

  /// Sets `on_close` callback, invoked when this [`Room`] is closed,
  /// providing a [`RoomCloseReason`].
  /// [`Room`]: room::Room
  /// [`RoomCloseReason`]: room::RoomCloseReason
  external void on_close(Function cb);

  /// Sets callback, invoked when a new [`LocalMediaTrack`] is added to this
  /// [`Room`].
  /// This might happen in such cases:
  /// 1. Media server initiates a media request.
  /// 2. `enable_audio`/`enable_video` is called.
  /// 3. [`MediaStreamSettings`] is updated via `set_local_media_settings`.
  /// [`Room`]: room::Room
  /// [`LocalMediaTrack`]: crate::api::LocalMediaTrack
  external void on_local_track(Function cb);

  /// Sets `on_failed_local_media` callback, invoked on local media
  /// acquisition failures.
  external void on_failed_local_media(Function cb);

  /// Sets `on_connection_loss` callback, invoked when a connection with a
  /// server is lost.
  external void on_connection_loss(Function cb);
}

@JS("RoomHandle")
abstract class _RoomHandle {
  external Promise<dynamic> join(String token);

  /// Updates this [`Room`]s [`MediaStreamSettings`]. This affects all
  /// [`PeerConnection`]s in this [`Room`]. If [`MediaStreamSettings`] is
  /// configured for some [`Room`], then this [`Room`] can only send media
  /// tracks that correspond to this settings. [`MediaStreamSettings`]
  /// update will change media tracks in all sending peers, so that might
  /// cause new [getUserMedia()][1] request.
  /// Media obtaining/injection errors are additionally fired to
  /// `on_failed_local_media` callback.
  /// If `stop_first` set to `true` then affected [`LocalMediaTrack`]s will be
  /// dropped before new [`MediaStreamSettings`] is applied. This is usually
  /// required when changing video source device due to hardware limitations,
  /// e.g. having an active track sourced from device `A` may hinder
  /// [getUserMedia()][1] requests to device `B`.
  /// `rollback_on_fail` option configures [`MediaStreamSettings`] update
  /// request to automatically rollback to previous settings if new settings
  /// cannot be applied.
  /// If recovering from fail state isn't possible then affected media types
  /// will be disabled.
  /// [`Room`]: room::Room
  /// [`PeerConnection`]: crate::peer::PeerConnection
  /// [`LocalMediaTrack`]: crate::api::LocalMediaTrack
  /// [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
  external Promise<dynamic> set_local_media_settings(
      MediaStreamSettings settings, bool stop_first, bool rollback_on_fail);

  /// Mutes outbound audio in this [`Room`].
  /// # Errors
  /// With `name = 'MediaConnections'` if [`RoomHandle::unmute_audio()`] was
  /// called while muting or a media server didn't approve this state
  /// transition.
  /// [`Room`]: room::Room
  external Promise<dynamic> mute_audio();

  /// Unmutes outbound audio in this [`Room`].
  /// # Errors
  /// With `name = 'MediaConnections'` if [`RoomHandle::mute_audio()`] was
  /// called while unmuting or a media server didn't approve this state
  /// transition.
  /// [`Room`]: room::Room
  external Promise<dynamic> unmute_audio();

  /// Mutes outbound video in this [`Room`].
  /// # Errors
  /// With `name = 'MediaConnections'` if [`RoomHandle::unmute_video()`] was
  /// called while muting or a media server didn't approve this state
  /// transition.
  /// [`Room`]: room::Room
  external Promise<dynamic> mute_video(num? source_kind);

  /// Unmutes outbound video in this [`Room`].
  /// # Errors
  /// With `name = 'MediaConnections'` if [`RoomHandle::mute_video()`] was
  /// called while unmuting or a media server didn't approve this state
  /// transition.
  /// [`Room`]: room::Room
  external Promise<dynamic> unmute_video(num? source_kind);

  /// Disables outbound audio in this [`Room`].
  /// # Errors
  /// With `name = 'MediaConnections'` if the target sender is configured as
  /// `required` by a media server or [`RoomHandle::enable_audio()`] was
  /// called while disabling or a media server didn't approve this state
  /// transition.
  /// [`Room`]: room::Room
  external Promise<dynamic> disable_audio();

  /// Enables outbound audio in this [`Room`].
  /// # Errors
  /// With `name = 'MediaConnections'` if [`RoomHandle::disable_audio()`] was
  /// called while enabling or a media server didn't approve this state
  /// transition.
  /// With `name = 'MediaManagerError'` if media acquisition request to User
  /// Agent failed.
  /// [`Room`]: room::Room
  external Promise<dynamic> enable_audio();

  /// Disables outbound video.
  /// Affects only video with a specific [`MediaSourceKind`] if specified.
  /// # Errors
  /// With `name = 'MediaConnections'` if the target sender is configured as
  /// `required` by a media server or [`RoomHandle::enable_video()`] was
  /// called while disabling or a media server didn't approve this state
  /// transition.
  external Promise<dynamic> disable_video(num? source_kind);

  /// Enables outbound video.
  /// Affects only video with a specific [`MediaSourceKind`] if specified.
  /// # Errors
  /// With `name = 'MediaConnections'` if [`RoomHandle::disable_video()`] was
  /// called while enabling or a media server didn't approve this state
  /// transition.
  /// With `name = 'MediaManagerError'` if media acquisition request to User
  /// Agent failed.
  external Promise<dynamic> enable_video(num? source_kind);

  /// Disables inbound audio in this [`Room`].
  /// # Errors
  /// With `name = 'MediaConnections'` if
  /// [`RoomHandle::enable_remote_audio()`] was called while disabling or a
  /// media server didn't approve this state transition.
  /// [`Room`]: room::Room
  external Promise<dynamic> disable_remote_audio();

  /// Disables inbound video in this [`Room`].
  /// # Errors
  /// With `name = 'MediaConnections'` if
  /// [`RoomHandle::enable_remote_video()`] was called while disabling or
  /// a media server didn't approve this state transition.
  /// [`Room`]: room::Room
  external Promise<dynamic> disable_remote_video();

  /// Enables inbound audio in this [`Room`].
  /// # Errors
  /// With `name = 'MediaConnections'` if
  /// [`RoomHandle::disable_remote_audio()`] was called while enabling or a
  /// media server didn't approve this state transition.
  /// [`Room`]: room::Room
  external Promise<dynamic> enable_remote_audio();

  /// Enables inbound video in this [`Room`].
  /// # Errors
  /// With `name = 'MediaConnections'` if
  /// [`RoomHandle::disable_remote_video()`] was called while enabling or a
  /// media server didn't approve this state transition.
  /// [`Room`]: room::Room
  external Promise<dynamic> enable_remote_video();
}

extension RoomHandleExtensions on RoomHandle {
  Future<dynamic> join(String token) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.join(token));
  }

  Future<dynamic> set_local_media_settings(
      MediaStreamSettings settings, bool stop_first, bool rollback_on_fail) {
    final tt = this as _RoomHandle;
    return promiseToFuture(
        tt.set_local_media_settings(settings, stop_first, rollback_on_fail));
  }

  Future<dynamic> mute_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.mute_audio());
  }

  Future<dynamic> unmute_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.unmute_audio());
  }

  Future<dynamic> mute_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.mute_video(source_kind));
  }

  Future<dynamic> unmute_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.unmute_video(source_kind));
  }

  Future<dynamic> disable_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.disable_audio());
  }

  Future<dynamic> enable_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.enable_audio());
  }

  Future<dynamic> disable_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.disable_video(source_kind));
  }

  Future<dynamic> enable_video(num? source_kind) {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.enable_video(source_kind));
  }

  Future<dynamic> disable_remote_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.disable_remote_audio());
  }

  Future<dynamic> disable_remote_video() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.disable_remote_video());
  }

  Future<dynamic> enable_remote_audio() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.enable_remote_audio());
  }

  Future<dynamic> enable_remote_video() {
    final tt = this as _RoomHandle;
    return promiseToFuture(tt.enable_remote_video());
  }
}

@JS()
abstract class Promise<T> {
  external factory Promise(
      void executor(void resolve(T result), Function reject));
  external Promise then(void onFulfilled(T result), [Function onRejected]);
}
