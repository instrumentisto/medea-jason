`medea-jason` changelog
=======================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## master

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.10.0...master)

### Changed

- `ConnectionHandle.onQualityScoreUpdate()` callback now receives `0` quality score if peer is disconnected. ([#212])

[#212]: https://github.com/instrumentisto/medea-jason/pull/212




## [0.10.0] · 2025-07-05
[0.10.0]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.10.0

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.9.1...medea-jason-0.10.0)

### Added

- Options to configure audio processing when creating local audio track: ([#206])
    - `AudioTrackConstraints.exactEchoCancellation`, `AudioTrackConstraints.idealEchoCancellation` to configure echo cancellation.
    - `AudioTrackConstraints.exactNoiseSuppression`, `AudioTrackConstraints.idealNoiseSuppression`, `AudioTrackConstraints.noiseSuppressionLevel` to configure noise suppression.
    - `AudioTrackConstraints.exactHighPassFilter`, `AudioTrackConstraints.idealHighPassFilter` to configure high-pass filter.
- Ability to inspect and configure audio processing for a audio `LocalMediaTrack` in runtime: ([#206])
    - `LocalMediaTrack.isAudioProcessingAvailable()` indicating if audio processing is available.
    - `LocalMediaTrack.setNoiseSuppressionEnabled()`, `LocalMediaTrack.isNoiseSuppressionEnabled()` to inspect and toggle noise suppression (supported on web and desktop platforms).
    - `LocalMediaTrack.setEchoCancellationEnabled()`, `LocalMediaTrack.isEchoCancellationEnabled()` to inspect and toggle echo cancellation (supported on web and desktop platforms).
    - `LocalMediaTrack.setAutoGainControlEnabled()`, `LocalMediaTrack.isAutoGainControlEnabled()` to inspect and toggle auto gain control (supported on web and desktop platforms).
    - `LocalMediaTrack.setNoiseSuppressionLevel()`, `LocalMediaTrack.getNoiseSuppressionLevel()` to inspect and configure noise suppression level (only supported on desktop platforms).
    - `LocalMediaTrack.setHighPassFilterEnabled()`, `LocalMediaTrack.isHighPassFilterEnabledO()` to inspect and toggle high-pass filter (only supported on desktop platforms).

### Changed

- `ConnectionHandle.onQualityScoreUpdate()` callback now receives `0` quality score if peer is disconnected. ([#212])

[#206]: https://github.com/instrumentisto/medea-jason/pull/206




## [0.9.1] · 2025-04-11
[0.9.1]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.9.1

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.9.0...medea-jason-0.9.1)

See [`medea_jason` pub package 0.9.1 changes](https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.9.1/flutter/CHANGELOG.md).




## [0.9.0] · 2025-03-29
[0.9.0]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.9.0

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.8.0...medea-jason-0.9.0)

### BC Breaks

- Minimal supported version of [`medea-client-api-proto`] is `0.9.0`. ([#199])

### Added

- Support of changing `RTCRtpSendParameters.encodings` via `Event::PeerUpdated`. ([#199])
- Providing user agent media capabilities via `Command::JoinRoom`. ([#199])
- `LocalMediaTrack.is_on_audio_level_available()` and `LocalMediaTrack.on_audio_level_changed()` support on Web. ([#202], [#97], [#167])

### Fixed

- Redundant async runtime created by [`flutter_rust_bridge`]. ([#203])

### Upgraded

- Dependencies:
    - Upgraded [`flutter_rust_bridge`] crate to 2.9.0 version. ([#203])

[#199]: https://github.com/instrumentisto/medea-jason/pull/199
[#202]: https://github.com/instrumentisto/medea-jason/pull/202
[#203]: https://github.com/instrumentisto/medea-jason/pull/203




## [0.8.0] · 2025-03-10
[0.8.0]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.8.0

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.7.1...medea-jason-0.8.0)

### Changed

- Bumped up [MSRV] to 1.85 because of migration to [2024 edition][080-1]. ([6f760c83])

### Upgraded

- Dependencies:
    - [`derive-more`] to 2.0 version. ([3ed7d2bf])
    - [`flutter_rust_bridge`] to 2.8.0 version. ([#200])

### Fixed

- Segfault when closing [Flutter] application on macOS. ([#201])

[#200]: https://github.com/instrumentisto/medea-jason/pull/200
[#201]: https://github.com/instrumentisto/medea-jason/pull/200
[3ed7d2bf]: https://github.com/instrumentisto/medea-jason/commit/3ed7d2bf59ed1237d9e55c4b65ea5cad833306fa
[6f760c83]: https://github.com/instrumentisto/medea-jason/commit/6f760c836f9c5293b5fefae8a0cb4ee2bd5cfda2
[080-1]: https://doc.rust-lang.org/edition-guide/rust-2024/index.html




## [0.7.1] · 2024-12-30
[0.7.1]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.7.1

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.7.0...medea-jason-0.7.1)

### Fixed

- Futures executor being called from non-main thread on Dart platforms. ([#197])

### Upgraded

- Dependencies:
    - [`flutter_rust_bridge`] to 2.7.0 version. ([#195])

[#195]: https://github.com/instrumentisto/medea-jason/pull/195
[#197]: https://github.com/instrumentisto/medea-jason/pull/197




## [0.7.0] · 2024-10-29
[0.7.0]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.7.0

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.6.1...medea-jason-0.7.0)

### BC Breaks

- [Cargo features]:
    - Replaced `wee_alloc` with `talc`. ([#187])

### Changed

- Dependencies:
    - Replaced [`wee_alloc`] allocator with [`talc`] for web. ([#187])

### Upgraded

- Dependencies:
    - [`flutter_rust_bridge`] to 2.4.0 version. ([#188])

[#187]: https://github.com/instrumentisto/medea-jason/pull/187
[#188]: https://github.com/instrumentisto/medea-jason/pull/188




## [0.6.1] · 2024-09-11
[0.6.1]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.6.1

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.6.0...medea-jason-0.6.1)

### Changed

- Bumped up [MSRV] to 1.81. ([fe29ee20])

[fe29ee20]: https://github.com/instrumentisto/medea-jason/commit/fe29ee20b7dd210145a2004ebe8389140aec10a3




## [0.6.0] · 2024-08-27
[0.6.0]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.6.0

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.5.0...medea-jason-0.6.0)

### Fixed

- [VP9] being forced to use `profile-id=2` in [SFU] mode. ([#180])

### Upgraded

- Dependencies:
    - [`derive-more`] to 1.0 version. ([#181])
    - [`flutter_rust_bridge`] to 2.2.0 version. ([#182])

[#180]: https://github.com/instrumentisto/medea-jason/pull/180
[#181]: https://github.com/instrumentisto/medea-jason/pull/181
[#182]: https://github.com/instrumentisto/medea-jason/pull/182




## [0.5.0] · 2024-08-05
[0.5.0]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.5.0

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.4.0...medea-jason-0.5.0)

### BC Breaks

- Minimal supported version of [`medea-client-api-proto`] is `0.6.0` ([#151]).
- Library API:
    - Removed `with_rpc_client()` constructor and added `WebSocketRpcClient` as argument to `new()` constructor in `Jason` ([#175]).

### Added

- Logging:
    - Exceptions thrown from Dart callbacks called by Rust ([#138]).
- Monitoring:
    - `IceCandidateError` metric sending to server ([#151]);
    - `transport_id`, `local_candidate_id` and `remote_candidate_id` to `RtcIceCandidatePairStats` ([#172]).
- Library API:
    - `ideal_auto_gain_control()` and `exact_auto_gain_control()` methods to `AudioTrackConstraints` ([#166]);
    - `is_on_audio_level_available()` and `on_audio_level_changed()` methods to `LocalMediaTrack` ([#167]).

### Fixed

- Screen sharing in Firefox ([#135]).
- State synchronization during initial negotiation in P2P mesh mode ([#162]).
- `ConnectionHandle.on_remote_track_added` callback might be called twice for the same track ([#162]).
- `RemoteMediaTrack.on_media_direction_changed` callback might not be called on direction update ([#162]).
- Segfault on Dart isolate shutdown ([#163]).
- Exception in Dart code might be ignored by Rust caller ([#176]).

[#135]: https://github.com/instrumentisto/medea-jason/pull/135
[#138]: https://github.com/instrumentisto/medea-jason/pull/138
[#151]: https://github.com/instrumentisto/medea-jason/pull/151
[#162]: https://github.com/instrumentisto/medea-jason/pull/162
[#163]: https://github.com/instrumentisto/medea-jason/pull/163
[#166]: https://github.com/instrumentisto/medea-jason/pull/166
[#167]: https://github.com/instrumentisto/medea-jason/pull/167
[#172]: https://github.com/instrumentisto/medea-jason/pull/172
[#175]: https://github.com/instrumentisto/medea-jason/pull/175
[#176]: https://github.com/instrumentisto/medea-jason/pull/176




## [0.4.0] · 2023-07-11
[0.4.0]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.4.0

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-jason-0.3.0...medea-jason-0.4.0)

### BC Breaks

- Minimal supported version of [`medea-client-api-proto`] is `0.5.0` ([#119]).

### Fixed

- Initial mute state on incoming tracks in [SFU] mode ([#119]).
- Incorrect `ConnectionHandle`s creation and disposal in [SFU] mode ([#119]).

[#119]: https://github.com/instrumentisto/medea-jason/pull/119




## [0.3.0] · 2023-06-09
[0.3.0]: https://github.com/instrumentisto/medea-jason/tree/medea-jason-0.3.0

[Diff](https://github.com/instrumentisto/medea-jason/compare/a2ce6b92...medea-jason-0.3.0)

### BC Breaks

- Library API:
    - `ReconnectHandle.reconnect_with_backoff()` now performs first reconnect attempt immediately ([instrumentisto/medea#206]).
    - Removed `JasonError` and changed thrown exceptions kind ([#4]):
        - `ConnectionHandle`:
            - `on_close` - `StateError`;
            - `get_remote_member_id` - `StateError`;
            - `on_remote_track_added` - `StateError`;
            - `on_quality_score_update` - `StateError`.
        - `MediaManager`:
            - `enumerate_devices` - `EnumerateDevicesException`.
            - `init_local_tracks` - `LocalMediaInitException`.
        - `RoomHandle`:
            - `join`:
                - `StateError`;
                - `FormatException`;
                - `RpcClientException`;
                - `InternalException`.
            - `on_new_connection` - `StateError`;
            - `on_close` - `StateError`;
            - `on_local_track` - `StateError`;
            - `on_failed_local_media` - `StateError`;
            - `on_connection_loss` - `StateError`;
            - `set_local_media_settings` - `MediaSettingsUpdateException`;
            - `mute_audio`, `unmute_audio`, `mute_video`, `unmute_video`, `disable_audio`, `enable_audio`,
              `disable_video`, `enable_video`, `disable_remote_video`, `enable_remote_video`, `disable_remote_audio`,
              `enable_remote_audio`:
                - `StateError`;
                - `MediaStateTransitionException`;
                - `InternalException`;
                - `LocalMediaInitException`.
    - Renamed `InputDeviceInfo` object to `MediaDeviceDetails` ([#29], [#106]).
    - `RemoteMediaTrack`:
        - Replaced `on_enabled` and `on_disabled` callbacks with `on_media_direction_changed` callback ([#46]);
        - Replaced `enabled` method with `media_direction` method ([#46]).

### Added

- Library API:
    - Optional argument to `ReconnectHandle.reconnect_with_backoff()` function that limits max elapsed time ([#206]).
    - Exceptions ([#4], [#31]):
        - `StateError`;
        - `LocalMediaInitException`;
        - `EnumerateDevicesException`;
        - `RpcClientException`;
        - `InternalException`;
        - `FormatException`;
        - `MediaStateTransitionException`;
        - `MediaSettingsUpdateException`.
    - `MediaManagerHandle.set_output_audio_id()` method switching output audio device on Dart platform ([#29]);
    - `MediaManagerHandle.on_device_change()` callback firing whenever `MediaManagerHandle.enumerate_devices()` list changes ([#30]);
    - `ConnectionHandle` methods ([#43], [#59]):
        - `enable_remote_video`;
        - `disable_remote_video`;
        - `enable_remote_audio`;
        - `disable_remote_audio`.
    - `MediaDirection` type ([#46]).
    - `MediaManagerHandle` methods for microphone volume on Dart platform ([#49]):
        - `microphone_volume_is_available`;
        - `microphone_volume`;
        - `set_microphone_volume`.
    - `LocalMediaInitExceptionKind` variants ([#52]):
        - `GetUserMediaAudioFailed`;
        - `GetUserMediaVideoFailed`.
    - `MediaManager.enumerate_displays()` ([#81]);
    - `LocalMediaTrack` ([#109]):
        - `on_ended`;
        - `state`.
    - Exposing all APIs via FFI to Dart ([#8], [#9], [#10], [#12], [#14], [#26], [#28]).

### Fixed

- Library API:
    - Unconverted into Dart exception error in `RoomHandle.onFailedLocalMedia()` ([#57]).

### Updated

- Switch to [2021 Rust edition][012-1] ([#16]).

[instrumentisto/medea#206]: https://github.com/instrumentisto/medea/pull/206
[#4]: https://github.com/instrumentisto/medea-jason/pull/4
[#8]: https://github.com/instrumentisto/medea-jason/pull/8
[#9]: https://github.com/instrumentisto/medea-jason/pull/9
[#10]: https://github.com/instrumentisto/medea-jason/pull/10
[#12]: https://github.com/instrumentisto/medea-jason/pull/12
[#14]: https://github.com/instrumentisto/medea-jason/pull/14
[#16]: https://github.com/instrumentisto/medea-jason/pull/16
[#26]: https://github.com/instrumentisto/medea-jason/pull/26
[#28]: https://github.com/instrumentisto/medea-jason/pull/28
[#29]: https://github.com/instrumentisto/medea-jason/pull/29
[#30]: https://github.com/instrumentisto/medea-jason/pull/30
[#31]: https://github.com/instrumentisto/medea-jason/pull/31
[#43]: https://github.com/instrumentisto/medea-jason/pull/43
[#46]: https://github.com/instrumentisto/medea-jason/pull/46
[#49]: https://github.com/instrumentisto/medea-jason/pull/49
[#52]: https://github.com/instrumentisto/medea-jason/pull/52
[#57]: https://github.com/instrumentisto/medea-jason/pull/57
[#59]: https://github.com/instrumentisto/medea-jason/pull/59
[#81]: https://github.com/instrumentisto/medea-jason/pull/81
[#106]: https://github.com/instrumentisto/medea-jason/pull/106
[#106]: https://github.com/instrumentisto/medea-jason/pull/109




## [0.2.0] · 2021-04-09
[0.2.0]: https://github.com/instrumentisto/medea/tree/medea-jason-0.2.0/jason

[Diff](https://github.com/instrumentisto/medea/compare/medea-jason-0.1.0...medea-jason-0.2.0) | [Milestone](https://github.com/instrumentisto/medea/milestone/2) | [Roadmap](https://github.com/instrumentisto/medea/issues/27)

### BC Breaks

- Library API:
    - Remove `MediaStreamHandle` ([#143]);
    - Expose `on_local_track` callback in `Room` instead of `Jason` ([#54], [#143]);
    - Replace `on_local_stream` callback with `on_local_track` ([#143]);
    - Room initialization ([#46]):
        - Remove `Jason.join_room()`.
- Transport and messaging:
    - Reverse `ping`/`pong` mechanism: expect `Ping`s from server and answer with `Pong`s ([#75]).

### Added

- Media management:
    - Library API:
        - Disable/Enable local video/audio ([#40], [#81], [#97], [#144], [#155]):
            - `Room.disable_audio()`;
            - `Room.enable_audio()`;
            - `Room.disable_video()`;
            - `Room.enable_video()`.
        - `InputDeviceInfo` class obtainable via `MediaManager.enumerate_devices()` ([#46]);
        - `MediaManager` class obtainable via `Jason.media_manager()` ([#46]):
            - `MediaManager.enumerate_devices()`;
            - `MediaManager.init_local_tracks()` ([#46], [#143]).
        - Local media stream constraints:
            - `MediaStreamSettings`, `AudioTrackConstraints` classes ([#46], [#97]);
            - `DeviceVideoTrackConstraints`, `DisplayVideoTrackConstraints` classes ([#78]);
            - `DeviceVideoTrackConstraints.ideal_facing_mode` and `DeviceVideoTrackConstraints.exact_facing_mode` functions ([#137]);
            - `DeviceVideoTrackConstraints` width and height configuration ([#158]):
                - `DeviceVideoTrackConstraints.ideal_width`;
                - `DeviceVideoTrackConstraints.exact_width`;
                - `DeviceVideoTrackConstraints.width_in_range`;
                - `DeviceVideoTrackConstraints.ideal_height`;
                - `DeviceVideoTrackConstraints.exact_height`;
                - `DeviceVideoTrackConstraints.height_in_range`.
            - `FacingMode` enum ([#137]).
        - `MediaKind` enum that provides `LocalMediaTrack`/`RemoteMediaTrack` and `InputDeviceInfo` kind ([#146]);
        - `MediaSourceKind` enum that provides `MediaTrack` media source kind (`Device` or `Display`) ([#146], [#156]);
        - Room management:
            - `Jason.init_room()` ([#46]);
            - `Room.join()` ([#46]);
            - `Jason.close_room()` ([#147]).
        - Ability to configure local media stream used by `Room` via `Room.set_local_media_settings()` ([#54], [#97], [#145], [#160]):
            - `Room.set_local_media_settings()` can be configured to stop used tracks before trying to acquire new tracks ([#160]);
            - `Room.set_local_media_settings()` can be configured to rollback to previous settings if fail to set new settings ([#160]).
        - `Room.on_failed_local_media` callback ([#54], [#143]);
        - `Room.on_close` callback for WebSocket close initiated by server ([#55]);
        - `RemoteMediaTrack.on_enabled` and `RemoteMediaTrack.on_disabled` callbacks being called when `RemoteMediaTrack` is enabled or disabled ([#123], [#143], [#156]);
        - `RemoteMediaTrack.on_stopped` callback that is called when `RemoteMediaTrack` is stopped ([#109]);
        - `RemoteMediaTrack.on_muted` and `RemoteMediaTrack.on_unmuted` callbacks being called when `RemoteMediaTrack` is muted or unmuted ([#191]);
        - `RemoteMediaTrack.muted()` method indicating whether this `RemoteMediaTrack` is muted ([#191]);
        - `ConnectionHandle.on_remote_track_added` callback being called when new receiver `RemoteMediaTrack` is added ([#123], [#143], [#156]);
        - Enabling/disabling remote video/audio ([#127], [#155]):
            - `Room.disable_remote_audio`;
            - `Room.enable_remote_audio`;
            - `Room.disable_remote_video`;
            - `Room.enable_remote_video`.
        - Muting/unmuting audio/video send ([#156]):
            - `Room.mute_audio`;
            - `Room.unmute_audio`;
            - `Room.mute_video`;
            - `Room.unmute_video`.
        - `RemoteMediaTrack`/`LocalMediaTrack` `media_source_kind` function ([#145], [#146], [#156]);
        - `RemoteMediaTrack` class ([#156]);
        - `LocalMediaTrack` class ([#156]).
    - Optional tracks support ([#106]);
    - Simultaneous device and display video tracks publishing and receiving ([#144]);
    - `RtcIceTransportPolicy` configuration ([#79]).
- Room management:
    - Library API:
        - `Room.on_connection_loss` callback that JS side can start Jason reconnection on connection loss with ([#75]);
        - `Room.on_close` callback for WebSocket close initiated by server ([#55]);
        - `ConnectionHandle.on_close` callback ([#120]);
        - `ConnectionHandle.get_remote_member_id` method ([#124]);
        - `ConnectionHandle.on_quality_score_update` callback for quality score updates received from server ([#132]).
- RPC messaging:
    - Cleanup Jason state on normal (`code = 1000`) WebSocket close ([#55]);
    - `RpcClient` and `RpcTransport` reconnection ([#75]);
    - State synchronization on a RPC reconnection ([#167]).
- Signalling:
    - Emitting of RPC commands:
        - `AddPeerConnectionMetrics` with `IceConnectionState` and `PeerConnectionState` ([#71], [#87]);
        - `AddPeerConnectionStats` with `RtcStats` ([#90]);
        - Enabling/disabling audio/video send/receive via `UpdateTracks` command ([#81], [#155]);
        - Muting/unmuting audio/video send via `UpdateTracks` ([#156]).
    - Handling of RPC events:
        - `PeerUpdated` with `PeerUpdate::Added`, `PeerUpdate::Updated`, `PeerUpdate::IceRestart` and `PeerUpdate::Removed` ([#105], [#138], [#139], [#109]);
        - `ConnectionQualityUpdated` ([#132]).
- Error handling:
    - Library API:
        - `JasonError` as library error with trace information and underlying JS error if it is the cause ([#55])

### Fixed

- Signalling:
    - Skipped `IceCandidate`s received before receiving remote SDP ([#50]).

[#40]: https://github.com/instrumentisto/medea/pull/40
[#46]: https://github.com/instrumentisto/medea/pull/46
[#50]: https://github.com/instrumentisto/medea/pull/59
[#54]: https://github.com/instrumentisto/medea/pull/54
[#55]: https://github.com/instrumentisto/medea/pull/55
[#71]: https://github.com/instrumentisto/medea/pull/71
[#75]: https://github.com/instrumentisto/medea/pull/75
[#78]: https://github.com/instrumentisto/medea/pull/78
[#79]: https://github.com/instrumentisto/medea/pull/79
[#81]: https://github.com/instrumentisto/medea/pull/81
[#87]: https://github.com/instrumentisto/medea/pull/87
[#90]: https://github.com/instrumentisto/medea/pull/90
[#97]: https://github.com/instrumentisto/medea/pull/97
[#105]: https://github.com/instrumentisto/medea/pull/105
[#106]: https://github.com/instrumentisto/medea/pull/106
[#109]: https://github.com/instrumentisto/medea/pull/109
[#120]: https://github.com/instrumentisto/medea/pull/120
[#123]: https://github.com/instrumentisto/medea/pull/123
[#124]: https://github.com/instrumentisto/medea/pull/124
[#127]: https://github.com/instrumentisto/medea/pull/127
[#132]: https://github.com/instrumentisto/medea/pull/132
[#137]: https://github.com/instrumentisto/medea/pull/137
[#138]: https://github.com/instrumentisto/medea/pull/138
[#139]: https://github.com/instrumentisto/medea/pull/139
[#143]: https://github.com/instrumentisto/medea/pull/143
[#144]: https://github.com/instrumentisto/medea/pull/144
[#145]: https://github.com/instrumentisto/medea/pull/145
[#146]: https://github.com/instrumentisto/medea/pull/146
[#147]: https://github.com/instrumentisto/medea/pull/147
[#155]: https://github.com/instrumentisto/medea/pull/155
[#156]: https://github.com/instrumentisto/medea/pull/156
[#158]: https://github.com/instrumentisto/medea/pull/158
[#160]: https://github.com/instrumentisto/medea/pull/160
[#167]: https://github.com/instrumentisto/medea/pull/167
[#191]: https://github.com/instrumentisto/medea/pull/191




## [0.1.0] · 2019-08-21
[0.1.0]: https://github.com/instrumentisto/medea/tree/medea-jason-0.1.0/jason

[Milestone](https://github.com/instrumentisto/medea/milestone/1) | [Roadmap](https://github.com/instrumentisto/medea/issues/8)

### Added

- Transport and messaging ([#18]):
    - Library API:
        - `new Jason()`;
        - `Jason.join_room()`;
        - `Jason.dispose()`.
    - RPC transport and heartbeat.
- Ability to use ICE servers provided by server ([#20]).
- Signalling ([#22]):
    - Library API:
       - `RoomHandle.on_new_connection` callback.
    - Handling of RPC events:
        - `PeerCreated`;
        - `SdpAnswerMade`;
        - `IceCandidateDiscovered`;
        - `PeersRemoved`.
    - Emitting of RPC commands:
        - `MakeSdpOffer`;
        - `MakeSdpAnswer`;
        - `SetIceCandidate`.
- Media management ([#22]):
    - Library API:
        - `MediaStreamHandle.get_media_stream()`;
        - `ConnectionHandle.on_remote_stream` callback;
        - `Jason.on_local_stream` callback.

[#18]: https://github.com/instrumentisto/medea/pull/18
[#20]: https://github.com/instrumentisto/medea/pull/20
[#22]: https://github.com/instrumentisto/medea/pull/22




[Cargo features]: https://doc.rust-lang.org/cargo/reference/features.html
[Flutter]: https://flutter.dev
[MSRV]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field
[SFU]: https://webrtcglossary.com/sfu
[Semantic Versioning 2.0.0]: https://semver.org
[VP9]: https://bloggeek.me/webrtcglossary/vp9
[`derive_more`]: https://docs.rs/derive_more
[`flutter_rust_bridge`]: https://docs.rs/flutter_rust_bridge
[`medea-client-api-proto`]: https://docs.rs/medea-client-api-proto
[`talc`]: https://docs.rs/talc
[`wee_alloc`]: https://docs.rs/wee_alloc
