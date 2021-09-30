/* tslint:disable */
/* eslint-disable */
/**
* Describes directions that a camera can face, as seen from a user's
* perspective. Representation of a [VideoFacingModeEnum][1].
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-videofacingmodeenum
*/
export enum FacingMode {
/**
* Facing towards a user (a self-view camera).
*/
  User,
/**
* Facing away from a user (viewing the environment).
*/
  Environment,
/**
* Facing to the left of a user.
*/
  Left,
/**
* Facing to the right of a user.
*/
  Right,
}
/**
* Media source type.
*/
export enum MediaSourceKind {
/**
* Media is sourced from some media device (webcam or microphone).
*/
  Device,
/**
* Media is obtained via screen capturing.
*/
  Display,
}
/**
* [MediaStreamTrack.kind][1] representation.
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack-kind
*/
export enum MediaKind {
/**
* Audio track.
*/
  Audio,
/**
* Video track.
*/
  Video,
}
/**
* Constraints applicable to audio tracks.
*/
export class AudioTrackConstraints {
  free(): void;
/**
* Creates new [`AudioTrackConstraints`] with none constraints configured.
*/
  constructor();
/**
* Sets an exact [deviceId][1] constraint.
*
* [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
* @param {string} device_id
*/
  device_id(device_id: string): void;
}
/**
* Connection with a specific remote `Member`, that is used on JS side.
*
* Like all the handles it contains a weak reference to the object that is
* managed by Rust, so its methods will fail if a weak reference could not be
* upgraded.
*/
export class ConnectionHandle {
  free(): void;
/**
* Sets callback, invoked when this [`Connection`] is closed.
*
* [`Connection`]: connection::Connection
* @param {Function} cb
*/
  on_close(cb: Function): void;
/**
* Returns ID of the remote `Member`.
* @returns {string}
*/
  get_remote_member_id(): string;
/**
* Sets callback, invoked when a new [`RemoteMediaTrack`] is added to this
* [`Connection`].
*
* [`Connection`]: connection::Connection
* [`RemoteMediaTrack`]: crate::api::RemoteMediaTrack
* @param {Function} cb
*/
  on_remote_track_added(cb: Function): void;
/**
* Sets callback, invoked when connection quality score is updated by a
* server.
* @param {Function} cb
*/
  on_quality_score_update(cb: Function): void;
}
/**
* Exception returned from [`RoomHandle::set_local_media_settings()`][1].
*
* [1]: crate::api::RoomHandle::set_local_media_settings
*/
export class ConstraintsUpdateException {
  free(): void;
/**
* Returns name of this [`ConstraintsUpdateException`].
* @returns {string}
*/
  name(): string;
/**
* Returns an [`Error`] if this [`ConstraintsUpdateException`] represents
* a `RecoveredException` or a `RecoverFailedException`.
*
* Returns `undefined` otherwise.
* @returns {JasonError | undefined}
*/
  recover_reason(): JasonError | undefined;
/**
* Returns [`js_sys::Array`] with an [`Error`]s if this
* [`ConstraintsUpdateException`] represents a `RecoverFailedException`.
* @returns {any}
*/
  recover_fail_reasons(): any;
/**
* Returns [`Error`] if this [`ConstraintsUpdateException`] represents
* an `ErroredException`.
*
* Returns `undefined` otherwise.
* @returns {JasonError | undefined}
*/
  error(): JasonError | undefined;
}
/**
* Constraints applicable to video tracks that are sourced from some media
* device.
*/
export class DeviceVideoTrackConstraints {
  free(): void;
/**
* Creates new [`DeviceVideoTrackConstraints`] with none constraints
* configured.
*/
  constructor();
/**
* Sets an exact [deviceId][1] constraint.
*
* [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
* @param {string} device_id
*/
  device_id(device_id: string): void;
/**
* Sets an exact [facingMode][1] constraint.
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
* @param {number} facing_mode
*/
  exact_facing_mode(facing_mode: number): void;
/**
* Sets an ideal [facingMode][1] constraint.
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
* @param {number} facing_mode
*/
  ideal_facing_mode(facing_mode: number): void;
/**
* Sets an exact [`height`][1] constraint.
*
* [1]: https://tinyurl.com/w3-streams#def-constraint-height
* @param {number} height
*/
  exact_height(height: number): void;
/**
* Sets an ideal [`height`][1] constraint.
*
* [1]: https://tinyurl.com/w3-streams#def-constraint-height
* @param {number} height
*/
  ideal_height(height: number): void;
/**
* Sets a range of a [`height`][1] constraint.
*
* [1]: https://tinyurl.com/w3-streams#def-constraint-height
* @param {number} min
* @param {number} max
*/
  height_in_range(min: number, max: number): void;
/**
* Sets an exact [`width`][1] constraint.
*
* [1]: https://tinyurl.com/w3-streams#def-constraint-width
* @param {number} width
*/
  exact_width(width: number): void;
/**
* Sets an ideal [`width`][1] constraint.
*
* [1]: https://tinyurl.com/w3-streams#def-constraint-width
* @param {number} width
*/
  ideal_width(width: number): void;
/**
* Sets a range of a [`width`][1] constraint.
*
* [1]: https://tinyurl.com/w3-streams#def-constraint-width
* @param {number} min
* @param {number} max
*/
  width_in_range(min: number, max: number): void;
}
/**
* Constraints applicable to video tracks sourced from a screen capturing.
*/
export class DisplayVideoTrackConstraints {
  free(): void;
/**
* Creates new [`DisplayVideoTrackConstraints`] with none constraints
* configured.
*/
  constructor();
}
/**
* Representation of a [MediaDeviceInfo][1].
*
* [1]: https://w3.org/TR/mediacapture-streams#device-info
*/
export class InputDeviceInfo {
  free(): void;
/**
* Returns a unique identifier for the represented device.
* @returns {string}
*/
  device_id(): string;
/**
* Returns a kind of the represented device.
*
* This representation of [MediaDeviceInfo][1] is for input device ONLY.
*
* [1]: https://w3.org/TR/mediacapture-streams#device-info
* @returns {number}
*/
  kind(): number;
/**
* Returns label describing the represented device (for example "External
* USB Webcam").
*
* If the device has no associated label, then returns an empty string.
* @returns {string}
*/
  label(): string;
/**
* Returns a group identifier of the represented device.
*
* Two devices have the same group identifier if they belong to the same
* physical device. For example, the audio input and output devices
* representing the speaker and microphone of the same headset have the
* same [groupId][1].
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediadeviceinfo-groupid
* @returns {string}
*/
  group_id(): string;
}
/**
* General JS side library interface.
*
* Responsible for managing shared transports, local media and room
* initialization.
*/
export class Jason {
  free(): void;
/**
* Instantiates a new [`Jason`] interface to interact with this library.
*/
  constructor();
/**
* Creates a new `Room` and returns its [`RoomHandle`].
* @returns {RoomHandle}
*/
  init_room(): RoomHandle;
/**
* Returns a [`MediaManagerHandle`].
* @returns {MediaManagerHandle}
*/
  media_manager(): MediaManagerHandle;
/**
* Closes the provided [`RoomHandle`].
* @param {RoomHandle} room_to_delete
*/
  close_room(room_to_delete: RoomHandle): void;
/**
* Drops [`Jason`] API object, so all the related objects (rooms,
* connections, streams etc.) respectively. All objects related to this
* [`Jason`] API object will be detached (you will still hold them, but
* unable to use).
*/
  dispose(): void;
}
/**
* Representation of an app error exported to JS side.
*
* Contains JS side error if it's the cause, and a trace information.
*/
export class JasonError {
  free(): void;
/**
* Returns a name of this error.
* @returns {string}
*/
  name(): string;
/**
* Returns a message of this error.
* @returns {string}
*/
  message(): string;
/**
* Returns a trace information of this error.
* @returns {string}
*/
  trace(): string;
/**
* Returns a JS side error if it's the cause.
* @returns {Error | undefined}
*/
  source(): Error | undefined;
}
/**
* Wrapper around a local [MediaStreamTrack][1].
*
* Backed by a strong reference to the actual track implementing auto stop on
* dropping. Can be manually dropped with a `free()` call.
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
*/
export class LocalMediaTrack {
  free(): void;
/**
* Returns the underlying [MediaStreamTrack][1].
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamtrack
* @returns {MediaStreamTrack}
*/
  get_track(): MediaStreamTrack;
/**
* Returns a [`MediaKind::Audio`] if this [`LocalMediaTrack`] represents an
* audio track, or a [`MediaKind::Video`] if it represents a video track.
* @returns {number}
*/
  kind(): number;
/**
* Returns a [`MediaSourceKind::Device`] if this [`LocalMediaTrack`] is
* sourced from some device (webcam/microphone), or a
* [`MediaSourceKind::Display`] if it's captured via
* [MediaDevices.getDisplayMedia()][1].
*
* [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
* @returns {number}
*/
  media_source_kind(): number;
}
/**
* [`MediaManagerHandle`] is a weak reference to a [`MediaManager`].
*
* [`MediaManager`] performs all the media acquisition requests
* ([getUserMedia()][1]/[getDisplayMedia()][2]) and stores all the received
* tracks for further re-usage.
*
* [`MediaManager`] stores weak references to [`LocalMediaTrack`]s, so if there
* are no strong references to some track, then this track is stopped and
* removed from [`MediaManager`].
*
* Like all the handles it contains a weak reference to the object that is
* managed by Rust, so its methods will fail if a weak reference could not be
* upgraded.
*
* [`MediaManager`]: media::MediaManager
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediadevices-getusermedia
* [2]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
*/
export class MediaManagerHandle {
  free(): void;
/**
* Returns a list of [`InputDeviceInfo`] objects representing available
* media input and output devices, such as microphones, cameras, and so
* forth.
* @returns {Promise<any>}
*/
  enumerate_devices(): Promise<any>;
/**
* Returns [`LocalMediaTrack`]s objects, built from the provided
* [`MediaStreamSettings`].
* @param {MediaStreamSettings} caps
* @returns {Promise<any>}
*/
  init_local_tracks(caps: MediaStreamSettings): Promise<any>;
}
/**
* [MediaStreamConstraints][1] wrapper.
*
* [1]: https://w3.org/TR/mediacapture-streams#dom-mediastreamconstraints
*/
export class MediaStreamSettings {
  free(): void;
/**
* Creates new [`MediaStreamSettings`] with none constraints configured.
*/
  constructor();
/**
* Specifies the nature and settings of an audio [MediaStreamTrack][1].
*
* [1]: https://w3.org/TR/mediacapture-streams#mediastreamtrack
* @param {AudioTrackConstraints} constraints
*/
  audio(constraints: AudioTrackConstraints): void;
/**
* Set constraints that will be used to obtain a local video sourced from
* a media device.
* @param {DeviceVideoTrackConstraints} constraints
*/
  device_video(constraints: DeviceVideoTrackConstraints): void;
/**
* Set constraints that will be used to capture a local video from a user's
* display.
* @param {DisplayVideoTrackConstraints} constraints
*/
  display_video(constraints: DisplayVideoTrackConstraints): void;
}
/**
* Handle that JS side can reconnect to a media server with when a connection
* is lost.
*
* This handle is passed into a [`RoomHandle.on_connection_loss`] callback.
*
* Like all the handles it contains a weak reference to the object that is
* managed by Rust, so its methods will fail if a weak reference could not be
* upgraded.
*
* [`RoomHandle.on_connection_loss`]: crate::api::RoomHandle.on_connection_loss
*/
export class ReconnectHandle {
  free(): void;
/**
* Tries to reconnect after the provided delay in milliseconds.
*
* If [`RpcSession`] is already reconnecting then a new reconnection
* attempt won't be performed. Instead, it will wait for the first
* reconnection attempt result and use it.
*
* [`RpcSession`]: rpc::RpcSession
* @param {number} delay_ms
* @returns {Promise<any>}
*/
  reconnect_with_delay(delay_ms: number): Promise<any>;
/**
* Tries to reconnect a [`RpcSession`] in a loop with a growing backoff
* delay.
*
* The first attempt will be performed immediately, and the second attempt
* will be performed after `starting_delay_ms`.
*
* Delay between reconnection attempts won't be greater than
* `max_delay_ms`.
*
* After each reconnection attempt, delay between reconnections will be
* multiplied by the given `multiplier` until it reaches `max_delay_ms`.
*
*
* If `multiplier` is a negative number then it will be considered as
* `0.0`. This might cause a busy loop, so it's not recommended.
*
* Max elapsed time can be limited with an optional `max_elapsed_time_ms`
* argument.
*
* If [`RpcSession`] is already reconnecting then new reconnection attempt
* won't be performed. Instead, it will wait for the first reconnection
* attempt result and use it here.
*
* [`RpcSession`]: rpc::RpcSession
* @param {number} starting_delay_ms
* @param {number} multiplier
* @param {number} max_delay
* @param {number | undefined} max_elapsed_time_ms
* @returns {Promise<any>}
*/
  reconnect_with_backoff(starting_delay_ms: number, multiplier: number, max_delay: number, max_elapsed_time_ms?: number): Promise<any>;
}
/**
* Wrapper around a received remote [MediaStreamTrack][1].
*
* [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
*/
export class RemoteMediaTrack {
  free(): void;
/**
* Returns the underlying [MediaStreamTrack][1].
*
* [1]: https://w3.org/TR/mediacapture-streams/#dom-mediastreamtrack
* @returns {MediaStreamTrack}
*/
  get_track(): MediaStreamTrack;
/**
* Indicates whether this [`RemoteMediaTrack`] is enabled.
* @returns {boolean}
*/
  enabled(): boolean;
/**
* Indicates whether this [`RemoteMediaTrack`] is muted.
* @returns {boolean}
*/
  muted(): boolean;
/**
* Sets callback, invoked when this [`RemoteMediaTrack`] is enabled.
* @param {Function} cb
*/
  on_enabled(cb: Function): void;
/**
* Sets callback, invoked when this [`RemoteMediaTrack`] is disabled.
* @param {Function} cb
*/
  on_disabled(cb: Function): void;
/**
* Sets callback to invoke when this [`RemoteMediaTrack`] is muted.
* @param {Function} cb
*/
  on_muted(cb: Function): void;
/**
* Sets callback to invoke when this [`RemoteMediaTrack`] is unmuted.
* @param {Function} cb
*/
  on_unmuted(cb: Function): void;
/**
* Sets callback to invoke when this [`RemoteMediaTrack`] is stopped.
* @param {Function} cb
*/
  on_stopped(cb: Function): void;
/**
* Returns a [`MediaKind::Audio`] if this [`RemoteMediaTrack`] represents
* an audio track, or a [`MediaKind::Video`] if it represents a video
* track.
* @returns {number}
*/
  kind(): number;
/**
* Returns a [`MediaSourceKind::Device`] if this [`RemoteMediaTrack`] is
* sourced from some device (webcam/microphone), or a
* [`MediaSourceKind::Display`] if it's captured via
* [MediaDevices.getDisplayMedia()][1].
*
* [1]: https://w3.org/TR/screen-capture/#dom-mediadevices-getdisplaymedia
* @returns {number}
*/
  media_source_kind(): number;
}
/**
* Reason of why a [`Room`] is closed.
*
* This struct is passed to a [`RoomHandle::on_close`] JS side callback.
*
* [`Room`]: room::Room
* [`RoomHandle::on_close`]: crate::api::RoomHandle::on_close
*/
export class RoomCloseReason {
  free(): void;
/**
* Returns the [`Room`]'s close reason.
*
* [`Room`]: room::Room
* @returns {string}
*/
  reason(): string;
/**
* Indicates whether the [`Room`] was closed by server.
*
* [`Room`]: room::Room
* @returns {boolean}
*/
  is_closed_by_server(): boolean;
/**
* Indicates whether the [`Room`] close reason is considered as an error.
*
* [`Room`]: room::Room
* @returns {boolean}
*/
  is_err(): boolean;
}
/**
* JS side handle to a [`Room`] where all the media happens.
*
* Like all handles it contains a weak reference to the object that is managed
* by Rust, so its methods will fail if a weak reference could not be upgraded.
*
* [`Room`]: room::Room
*/
export class RoomHandle {
  free(): void;
/**
* Connects to a media server and joins a [`Room`] with the provided
* authorization `token`.
*
* Authorization token has a fixed format:
* `{{ Host URL }}/{{ Room ID }}/{{ Member ID }}?token={{ Auth Token }}`
* (e.g. `wss://medea.com/MyConf1/Alice?token=777`).
*
* Establishes connection with media server (if it doesn't exist already).
*
* Effectively returns `Result<(), JasonError>`.
*
* # Errors
*
* - When `on_failed_local_media` callback is not set.
* - When `on_connection_loss` callback is not set.
* - When unable to connect to a media server.
*
* [`Room`]: room::Room
* @param {string} token
* @returns {Promise<any>}
*/
  join(token: string): Promise<any>;
/**
* Sets callback, invoked when a new [`Connection`] with some remote
* `Member` is established.
*
* [`Connection`]: crate::connection::Connection
* @param {Function} cb
*/
  on_new_connection(cb: Function): void;
/**
* Sets `on_close` callback, invoked when this [`Room`] is closed,
* providing a [`RoomCloseReason`].
*
* [`Room`]: room::Room
* [`RoomCloseReason`]: room::RoomCloseReason
* @param {Function} cb
*/
  on_close(cb: Function): void;
/**
* Sets callback, invoked when a new [`LocalMediaTrack`] is added to this
* [`Room`].
*
* This might happen in such cases:
* 1. Media server initiates a media request.
* 2. `enable_audio`/`enable_video` is called.
* 3. [`MediaStreamSettings`] is updated via `set_local_media_settings`.
*
* [`Room`]: room::Room
* [`LocalMediaTrack`]: crate::api::LocalMediaTrack
* @param {Function} cb
*/
  on_local_track(cb: Function): void;
/**
* Sets `on_failed_local_media` callback, invoked on local media
* acquisition failures.
* @param {Function} cb
*/
  on_failed_local_media(cb: Function): void;
/**
* Sets `on_connection_loss` callback, invoked when a connection with a
* server is lost.
* @param {Function} cb
*/
  on_connection_loss(cb: Function): void;
/**
* Updates this [`Room`]s [`MediaStreamSettings`]. This affects all
* [`PeerConnection`]s in this [`Room`]. If [`MediaStreamSettings`] is
* configured for some [`Room`], then this [`Room`] can only send media
* tracks that correspond to this settings. [`MediaStreamSettings`]
* update will change media tracks in all sending peers, so that might
* cause new [getUserMedia()][1] request.
*
* Media obtaining/injection errors are additionally fired to
* `on_failed_local_media` callback.
*
* If `stop_first` set to `true` then affected [`LocalMediaTrack`]s will be
* dropped before new [`MediaStreamSettings`] is applied. This is usually
* required when changing video source device due to hardware limitations,
* e.g. having an active track sourced from device `A` may hinder
* [getUserMedia()][1] requests to device `B`.
*
* `rollback_on_fail` option configures [`MediaStreamSettings`] update
* request to automatically rollback to previous settings if new settings
* cannot be applied.
*
* If recovering from fail state isn't possible then affected media types
* will be disabled.
*
* [`Room`]: room::Room
* [`PeerConnection`]: crate::peer::PeerConnection
* [`LocalMediaTrack`]: crate::api::LocalMediaTrack
* [1]: https://tinyurl.com/w3-streams#dom-mediadevices-getusermedia
* @param {MediaStreamSettings} settings
* @param {boolean} stop_first
* @param {boolean} rollback_on_fail
* @returns {Promise<any>}
*/
  set_local_media_settings(settings: MediaStreamSettings, stop_first: boolean, rollback_on_fail: boolean): Promise<any>;
/**
* Mutes outbound audio in this [`Room`].
*
* # Errors
*
* With `name = 'MediaConnections'` if [`RoomHandle::unmute_audio()`] was
* called while muting or a media server didn't approve this state
* transition.
*
* [`Room`]: room::Room
* @returns {Promise<any>}
*/
  mute_audio(): Promise<any>;
/**
* Unmutes outbound audio in this [`Room`].
*
* # Errors
*
* With `name = 'MediaConnections'` if [`RoomHandle::mute_audio()`] was
* called while unmuting or a media server didn't approve this state
* transition.
*
* [`Room`]: room::Room
* @returns {Promise<any>}
*/
  unmute_audio(): Promise<any>;
/**
* Mutes outbound video in this [`Room`].
*
* # Errors
*
* With `name = 'MediaConnections'` if [`RoomHandle::unmute_video()`] was
* called while muting or a media server didn't approve this state
* transition.
*
* [`Room`]: room::Room
* @param {number | undefined} source_kind
* @returns {Promise<any>}
*/
  mute_video(source_kind?: number): Promise<any>;
/**
* Unmutes outbound video in this [`Room`].
*
* # Errors
*
* With `name = 'MediaConnections'` if [`RoomHandle::mute_video()`] was
* called while unmuting or a media server didn't approve this state
* transition.
*
* [`Room`]: room::Room
* @param {number | undefined} source_kind
* @returns {Promise<any>}
*/
  unmute_video(source_kind?: number): Promise<any>;
/**
* Disables outbound audio in this [`Room`].
*
* # Errors
*
* With `name = 'MediaConnections'` if the target sender is configured as
* `required` by a media server or [`RoomHandle::enable_audio()`] was
* called while disabling or a media server didn't approve this state
* transition.
*
* [`Room`]: room::Room
* @returns {Promise<any>}
*/
  disable_audio(): Promise<any>;
/**
* Enables outbound audio in this [`Room`].
*
* # Errors
*
* With `name = 'MediaConnections'` if [`RoomHandle::disable_audio()`] was
* called while enabling or a media server didn't approve this state
* transition.
*
* With `name = 'MediaManagerError'` if media acquisition request to User
* Agent failed.
*
* [`Room`]: room::Room
* @returns {Promise<any>}
*/
  enable_audio(): Promise<any>;
/**
* Disables outbound video.
*
* Affects only video with a specific [`MediaSourceKind`] if specified.
*
* # Errors
*
* With `name = 'MediaConnections'` if the target sender is configured as
* `required` by a media server or [`RoomHandle::enable_video()`] was
* called while disabling or a media server didn't approve this state
* transition.
* @param {number | undefined} source_kind
* @returns {Promise<any>}
*/
  disable_video(source_kind?: number): Promise<any>;
/**
* Enables outbound video.
*
* Affects only video with a specific [`MediaSourceKind`] if specified.
*
* # Errors
*
* With `name = 'MediaConnections'` if [`RoomHandle::disable_video()`] was
* called while enabling or a media server didn't approve this state
* transition.
*
* With `name = 'MediaManagerError'` if media acquisition request to User
* Agent failed.
* @param {number | undefined} source_kind
* @returns {Promise<any>}
*/
  enable_video(source_kind?: number): Promise<any>;
/**
* Disables inbound audio in this [`Room`].
*
* # Errors
*
* With `name = 'MediaConnections'` if
* [`RoomHandle::enable_remote_audio()`] was called while disabling or a
* media server didn't approve this state transition.
*
* [`Room`]: room::Room
* @returns {Promise<any>}
*/
  disable_remote_audio(): Promise<any>;
/**
* Disables inbound video in this [`Room`].
*
* # Errors
*
* With `name = 'MediaConnections'` if
* [`RoomHandle::enable_remote_video()`] was called while disabling or
* a media server didn't approve this state transition.
*
* [`Room`]: room::Room
* @returns {Promise<any>}
*/
  disable_remote_video(): Promise<any>;
/**
* Enables inbound audio in this [`Room`].
*
* # Errors
*
* With `name = 'MediaConnections'` if
* [`RoomHandle::disable_remote_audio()`] was called while enabling or a
* media server didn't approve this state transition.
*
* [`Room`]: room::Room
* @returns {Promise<any>}
*/
  enable_remote_audio(): Promise<any>;
/**
* Enables inbound video in this [`Room`].
*
* # Errors
*
* With `name = 'MediaConnections'` if
* [`RoomHandle::disable_remote_video()`] was called while enabling or a
* media server didn't approve this state transition.
*
* [`Room`]: room::Room
* @returns {Promise<any>}
*/
  enable_remote_video(): Promise<any>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly roomhandle_join: (a: number, b: number, c: number) => number;
  readonly roomhandle_on_new_connection: (a: number, b: number) => void;
  readonly roomhandle_on_close: (a: number, b: number) => void;
  readonly roomhandle_on_local_track: (a: number, b: number) => void;
  readonly roomhandle_on_failed_local_media: (a: number, b: number) => void;
  readonly roomhandle_on_connection_loss: (a: number, b: number) => void;
  readonly roomhandle_set_local_media_settings: (a: number, b: number, c: number, d: number) => number;
  readonly roomhandle_mute_audio: (a: number) => number;
  readonly roomhandle_unmute_audio: (a: number) => number;
  readonly roomhandle_mute_video: (a: number, b: number) => number;
  readonly roomhandle_unmute_video: (a: number, b: number) => number;
  readonly roomhandle_disable_audio: (a: number) => number;
  readonly roomhandle_enable_audio: (a: number) => number;
  readonly roomhandle_disable_video: (a: number, b: number) => number;
  readonly roomhandle_enable_video: (a: number, b: number) => number;
  readonly roomhandle_disable_remote_audio: (a: number) => number;
  readonly roomhandle_disable_remote_video: (a: number) => number;
  readonly roomhandle_enable_remote_audio: (a: number) => number;
  readonly roomhandle_enable_remote_video: (a: number) => number;
  readonly __wbg_roomhandle_free: (a: number) => void;
  readonly remotemediatrack_get_track: (a: number) => number;
  readonly remotemediatrack_enabled: (a: number) => number;
  readonly remotemediatrack_muted: (a: number) => number;
  readonly remotemediatrack_on_enabled: (a: number, b: number) => void;
  readonly remotemediatrack_on_disabled: (a: number, b: number) => void;
  readonly remotemediatrack_on_muted: (a: number, b: number) => void;
  readonly remotemediatrack_on_unmuted: (a: number, b: number) => void;
  readonly remotemediatrack_on_stopped: (a: number, b: number) => void;
  readonly remotemediatrack_kind: (a: number) => number;
  readonly remotemediatrack_media_source_kind: (a: number) => number;
  readonly __wbg_remotemediatrack_free: (a: number) => void;
  readonly jason_new: () => number;
  readonly jason_init_room: (a: number) => number;
  readonly jason_media_manager: (a: number) => number;
  readonly jason_close_room: (a: number, b: number) => void;
  readonly jason_dispose: (a: number) => void;
  readonly __wbg_jason_free: (a: number) => void;
  readonly constraintsupdateexception_name: (a: number, b: number) => void;
  readonly constraintsupdateexception_recover_reason: (a: number) => number;
  readonly constraintsupdateexception_recover_fail_reasons: (a: number) => number;
  readonly constraintsupdateexception_error: (a: number) => number;
  readonly __wbg_constraintsupdateexception_free: (a: number) => void;
  readonly roomclosereason_reason: (a: number, b: number) => void;
  readonly roomclosereason_is_closed_by_server: (a: number) => number;
  readonly roomclosereason_is_err: (a: number) => number;
  readonly __wbg_roomclosereason_free: (a: number) => void;
  readonly inputdeviceinfo_device_id: (a: number, b: number) => void;
  readonly inputdeviceinfo_kind: (a: number) => number;
  readonly inputdeviceinfo_label: (a: number, b: number) => void;
  readonly inputdeviceinfo_group_id: (a: number, b: number) => void;
  readonly __wbg_inputdeviceinfo_free: (a: number) => void;
  readonly jasonerror_name: (a: number, b: number) => void;
  readonly jasonerror_message: (a: number, b: number) => void;
  readonly jasonerror_trace: (a: number, b: number) => void;
  readonly jasonerror_source: (a: number) => number;
  readonly __wbg_jasonerror_free: (a: number) => void;
  readonly reconnecthandle_reconnect_with_delay: (a: number, b: number) => number;
  readonly reconnecthandle_reconnect_with_backoff: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly __wbg_reconnecthandle_free: (a: number) => void;
  readonly connectionhandle_on_close: (a: number, b: number) => void;
  readonly connectionhandle_get_remote_member_id: (a: number, b: number) => void;
  readonly connectionhandle_on_remote_track_added: (a: number, b: number) => void;
  readonly connectionhandle_on_quality_score_update: (a: number, b: number) => void;
  readonly __wbg_connectionhandle_free: (a: number) => void;
  readonly localmediatrack_get_track: (a: number) => number;
  readonly localmediatrack_kind: (a: number) => number;
  readonly localmediatrack_media_source_kind: (a: number) => number;
  readonly __wbg_localmediatrack_free: (a: number) => void;
  readonly displayvideotrackconstraints_new: () => number;
  readonly __wbg_displayvideotrackconstraints_free: (a: number) => void;
  readonly devicevideotrackconstraints_new: () => number;
  readonly devicevideotrackconstraints_device_id: (a: number, b: number, c: number) => void;
  readonly devicevideotrackconstraints_exact_facing_mode: (a: number, b: number) => void;
  readonly devicevideotrackconstraints_ideal_facing_mode: (a: number, b: number) => void;
  readonly devicevideotrackconstraints_exact_height: (a: number, b: number) => void;
  readonly devicevideotrackconstraints_ideal_height: (a: number, b: number) => void;
  readonly devicevideotrackconstraints_height_in_range: (a: number, b: number, c: number) => void;
  readonly devicevideotrackconstraints_exact_width: (a: number, b: number) => void;
  readonly devicevideotrackconstraints_ideal_width: (a: number, b: number) => void;
  readonly devicevideotrackconstraints_width_in_range: (a: number, b: number, c: number) => void;
  readonly __wbg_devicevideotrackconstraints_free: (a: number) => void;
  readonly audiotrackconstraints_new: () => number;
  readonly audiotrackconstraints_device_id: (a: number, b: number, c: number) => void;
  readonly __wbg_audiotrackconstraints_free: (a: number) => void;
  readonly mediastreamsettings_new: () => number;
  readonly mediastreamsettings_audio: (a: number, b: number) => void;
  readonly mediastreamsettings_device_video: (a: number, b: number) => void;
  readonly mediastreamsettings_display_video: (a: number, b: number) => void;
  readonly __wbg_mediastreamsettings_free: (a: number) => void;
  readonly mediamanagerhandle_enumerate_devices: (a: number) => number;
  readonly mediamanagerhandle_init_local_tracks: (a: number, b: number) => number;
  readonly __wbg_mediamanagerhandle_free: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd2d1a685c9a5698d: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hdfcc78f4159794ea: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9fb39ee6878cea01: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h020162ed1977a7df: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4a4feeabe5ee9a95: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5fa8a471e85ae9d2: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__hb74475a1c8278b47: (a: number, b: number, c: number, d: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
