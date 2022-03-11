`medea-jason` changelog
=======================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.3.0] · 2021-??-?? · To-be-done
[0.3.0]: /../../tree/medea-jason-0.3.0

[Diff](https://github.com/instrumentisto/medea-jason/compare/a2ce6b92...medea-jason-0.3.0) | [Milestone](https://github.com/instrumentisto/medea/milestone/3) | [Roadmap](https://github.com/instrumentisto/medea/issues/182)

### BC Breaks

- Library API:
    - `ReconnectHandle.reconnect_with_backoff()` performs first reconnect attempt immediately now ([instrumentisto/medea#206]).
    - Remove `JasonError` and change thrown exceptions kind ([#4]):
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
    - Rename `InputDeviceInfo` object to `MediaDeviceInfo` ([#29]).

### Added

- Library API:
    - Optional argument to `ReconnectHandle.reconnect_with_backoff()` function that limits max elapsed time ([#206]).
    - Exceptions ([#4]):
        - `StateError`;
        - `LocalMediaInitException`;
        - `EnumerateDevicesException`;
        - `RpcClientException`;
        - `InternalException`;
        - `FormatException`;
        - `MediaStateTransitionException`;
        - `MediaSettingsUpdateException`.
    - `MediaManagerHandle.set_output_audio_id()` function that switches output audio device on Dart platform ([#29]).

### Updated

- Switch to [2021 Rust edition][012-1] ([#16]).

[instrumentisto/medea#206]: https://github.com/instrumentisto/medea/pull/206
[#4]: /../../pull/4
[#16]: /../../pull/16
[#29]: /../../pull/29




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
        - `MediaDeviceInfo` class obtainable via `MediaManager.enumerate_devices()` ([#46]);
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
        - `MediaKind` enum that provides `LocalMediaTrack`/`RemoteMediaTrack` and `MediaDeviceInfo` kind ([#146]);
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





[Semantic Versioning 2.0.0]: https://semver.org
