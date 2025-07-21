`medea-client-api-proto` changelog
==================================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].

## master

### Changed

- Added `source_kind` to `AudioSettings`. ([#225])

[#225]: https://github.com/instrumentisto/medea-jason/pull/225




## [0.9.0] · 2025-03-29
[0.9.0]: https://github.com/instrumentisto/medea-jason/tree/medea-client-api-proto-0.9.0/proto/client-api

### BC Breaks

- Remade `Codec` enum into struct. ([#199])
- Removed `SvcSettings` struct. ([#199])
- Added `capabilities` field to `Command::JoinRoom`. ([#199])
- Added `EncodingParameters.codec` and `EncodingParameters.scalability_mode` fields. ([#199])

### Added

- `Capabilities` struct. ([#199])

[#199]: https://github.com/instrumentisto/medea-jason/pull/199




## [0.8.0] · 2025-03-10
[0.8.0]: https://github.com/instrumentisto/medea-jason/tree/medea-client-api-proto-0.8.0/proto/client-api

### Changed

- Bumped up [MSRV] to 1.85 because of migration to [2024 edition][080-1]. ([6f760c83])

### Upgraded

- Dependencies:
    - [`derive-more`] to 2.0 version. ([3ed7d2bf])

[3ed7d2bf]: https://github.com/instrumentisto/medea-jason/commit/3ed7d2bf59ed1237d9e55c4b65ea5cad833306fa
[6f760c83]: https://github.com/instrumentisto/medea-jason/commit/6f760c836f9c5293b5fefae8a0cb4ee2bd5cfda2
[080-1]: https://doc.rust-lang.org/edition-guide/rust-2024/index.html




## [0.7.0] · 2024-10-29
[0.7.0]: https://github.com/instrumentisto/medea-jason/tree/medea-client-api-proto-0.7.0/proto/client-api

### BC Breaks

- Made interior of `Credential` private. ([#189])
- Made `IceServer::credential` to `IcePassword` instead of `String`. ([#189], [#190])

### Added

- `IcePassword` type. ([#190])

[#189]: https://github.com/instrumentisto/medea-jason/pull/189
[#190]: https://github.com/instrumentisto/medea-jason/pull/190




## [0.6.1] · 2024-08-27
[0.6.1]: https://github.com/instrumentisto/medea-jason/tree/medea-client-api-proto-0.6.1/proto/client-api

### Upgraded

- Dependencies:
    - [`derive-more`] to 1.0 version. ([#181])

[#181]: https://github.com/instrumentisto/medea-jason/pull/181




## [0.6.0] · 2024-08-05
[0.6.0]: https://github.com/instrumentisto/medea-jason/tree/medea-client-api-proto-0.6.0/proto/client-api

### BC Breaks

- Added `PeerMetrics::PeerConnectionError` variant ([#151]).
- Added `TrackPatchEvent::encoding_parameters` field ([#164]).
- Added `VideoSettings::svc_settings` and `VideoSettings::encoding_parameters` fields ([#164]).
- Fields of `RtcInboundRtpStreamMediaType`, `RtcOutboundRtpStreamMediaType` and `MediaKind` renamed in `camelCase` ([#173]).
- Converted `frames_per_second` fields to `Float` instead of `u32`/`u64` ([#173]).

### Added

- `PeerConnectionError` type ([#151]).
- `IceCandidateError` type ([#151]).
- `Codec`, `ScalabilityMode`, `SvcSettings` and `EncodingParameters` types ([#164]).

[#151]: https://github.com/instrumentisto/medea-jason/pull/151
[#164]: https://github.com/instrumentisto/medea-jason/pull/164
[#173]: https://github.com/instrumentisto/medea-jason/pull/173




## [0.5.0] · 2023-07-11
[0.5.0]: https://github.com/instrumentisto/medea-jason/tree/medea-client-api-proto-0.5.0/proto/client-api

### BC Breaks

- Added `Track.muted` field ([#119]).

[#119]: https://github.com/instrumentisto/medea-jason/pull/119




## [0.4.0] · 2023-06-09
[0.4.0]: https://github.com/instrumentisto/medea-jason/tree/medea-client-api-proto-0.4.0/proto/client-api

### BC Breaks

- Cargo features: ([#16])
    - Renamed `jason` to `client`.
    - Renamed `medea` to `server`.
    - Made only `client` enabled by default.
- Replaced `From<SystemTime> for HighResTimeStamp` implementation with `TryFrom` ([#16]).
- Made `RtcOutboundRtpStreamStats::bytes_sent` and `RtcOutboundRtpStreamStats::packets_sent` optional ([#26]).
- Replaced `enabled_individual` and `enabled_individual` fields with `MediaDirection` in `TrackPatchEvent`, `state::Receiver` and `state::Sender` ([#46]).
- Added `media_direction` to `Track` ([#107]).
- Added `receivers` to `TrackPatchEvent` ([#107]).
- Added `connection_mode` to `Event::PeerCreated`, `state::Sender`, `state::Receiver` and `state::Peer` ([#113], [#116]).

### Added

- `MediaDirection` type ([#46]).
- `ConnectionMode` type ([#113]).

### Updated

- Switch to [2021 Rust edition][012-1] ([#16]).

[#16]: https://github.com/instrumentisto/medea-jason/pull/16
[#26]: https://github.com/instrumentisto/medea-jason/pull/26
[#46]: https://github.com/instrumentisto/medea-jason/pull/46
[#107]: https://github.com/instrumentisto/medea-jason/pull/107
[#113]: https://github.com/instrumentisto/medea-jason/pull/113
[#116]: https://github.com/instrumentisto/medea-jason/pull/116
[012-1]: https://doc.rust-lang.org/edition-guide/rust-2021/index.html




## [0.3.0] · 2021-04-09
[0.3.0]: https://github.com/instrumentisto/medea/tree/medea-client-api-proto-0.3.0/proto/client-api

[Diff](https://github.com/instrumentisto/medea/compare/medea-client-api-proto-0.2.0...medea-client-api-proto-0.3.0) | [Milestone](https://github.com/instrumentisto/medea/milestone/2)

### BC Breaks

- `TracksApplied` event renamed as `PeerUpdated` ([#139]).

### Added

- `PeerUpdate::Removed` variant to `PeerUpdated` event ([#109]).

[#109]: https://github.com/instrumentisto/medea/pull/109
[#139]: https://github.com/instrumentisto/medea/pull/139




## [0.2.0] · 2021-02-01
[0.2.0]: https://github.com/instrumentisto/medea/tree/medea-client-api-proto-0.2.0/proto/client-api

[Diff](https://github.com/instrumentisto/medea/compare/medea-client-api-proto-0.1.0...medea-client-api-proto-0.2.0) | [Milestone](https://github.com/instrumentisto/medea/milestone/2) | [Roadmap](https://github.com/instrumentisto/medea/issues/27)

### BC Breaks

- RPC messages ([#75]):
    - Server messages:
        - `Pong` is now `Ping`.
    - Client messages:
        - `Ping` is now `Pong`.
    - Change `sender` and `receivers` in `Track`'s `Direction` to contain remote `MemberId` instead of `PeerId` ([#124]);
    - Use 32-bit integer types instead of 64-bit ([#115]).

### Added

- `TrackId` and `PeerId` types ([#28]);
- `MemberId` type ([#124]);
- `Incrementable` trait ([#28]);
- `CloseReason` and `CloseDescription` types ([#58]);
- `AddPeerConnectionMetrics` client command with `IceConnectionState` and `PeerConnectionState` metrics ([#71], [#87]);
- `RpcSettings` server message ([#75]);
- `force_relay` field to `PeerCreated` event ([#79]);
- `UpdateTracks` command ([#81]);
- `StatsUpdate` metric into `AddPeerConnectionMetrics` command ([#90]);
- `RTCPeerConnection` stats ([#90]):
    - `RtcCodecStats`;
    - `RtcInboundRtpStreamStats`;
    - `RtcOutboundRtpStreamStats`;
    - `RtcRemoteInboundRtpStreamStats`;
    - `RtcRemoteOutboundRtpStreamStats`;
    - `MediaSourceStats`;
    - `RtpContributingSourceStats`;
    - `RtcPeerConnectionStats`;
    - `DataChannelStats`;
    - `MediaStreamStats`;
    - `TrackStats`;
    - `RtcRtpTransceiverStats`;
    - `SenderStatsKind`;
    - `ReceiverStatsKind`;
    - `RtcTransportStats`;
    - `RtcSctpTransportStats`;
    - `RtcIceCandidatePairStats`;
    - `RtcIceCandidateStats`;
    - `RtcCertificateStats`;
    - `RtcIceServerStats`.
- `Cancelled` state to the `KnownIceCandidatePairState` ([#102]);
- `required` field to `AudioSettings` and `VideoSettings` ([#106], [#155]);
- `TracksApplied` event with `TrackUpdate::Updated` and `TrackUpdate::Added` variants ([#81], [#105]);
- `ConnectionQualityUpdated` event ([#132]);
- `TrackPatchCommand` ([#127]):
    - `enabled` ([#127], [#155]);
    - `muted` ([#156]).
- `TrackPatchEvent` ([#127]):
    - `enabled_individual` ([#127], [#155]);
    - `enabled_general` ([#127], [#155]);
    - `muted` ([#156]).
- `IceRestart` variant to `TrackUpdate` ([#138]);
- `source_kind` field to `VideoSettings` type ([#145]);
- `RoomId` and `Credential` types ([#148]);
- `JoinRoom` and `LeaveRoom` client messages ([#147]);
- `RoomJoined` and `RoomLeft` server messages ([#147]);
- `StateSynchronized` server message ([#167]);
- `SynchronizeMe` client message ([#167]);
- States for the client and server synchronization ([#167]):
    - `Room`;
    - `Peer`;
    - `Sender`;
    - `Receiver`.

[#28]: https://github.com/instrumentisto/medea/pull/28
[#58]: https://github.com/instrumentisto/medea/pull/58
[#71]: https://github.com/instrumentisto/medea/pull/71
[#75]: https://github.com/instrumentisto/medea/pull/75
[#79]: https://github.com/instrumentisto/medea/pull/79
[#81]: https://github.com/instrumentisto/medea/pull/81
[#87]: https://github.com/instrumentisto/medea/pull/87
[#90]: https://github.com/instrumentisto/medea/pull/90
[#102]: https://github.com/instrumentisto/medea/pull/102
[#105]: https://github.com/instrumentisto/medea/pull/105
[#106]: https://github.com/instrumentisto/medea/pull/106
[#115]: https://github.com/instrumentisto/medea/pull/115
[#132]: https://github.com/instrumentisto/medea/pull/132
[#127]: https://github.com/instrumentisto/medea/pull/127
[#138]: https://github.com/instrumentisto/medea/pull/138
[#145]: https://github.com/instrumentisto/medea/pull/145
[#147]: https://github.com/instrumentisto/medea/pull/147
[#148]: https://github.com/instrumentisto/medea/pull/148
[#155]: https://github.com/instrumentisto/medea/pull/155
[#156]: https://github.com/instrumentisto/medea/pull/156
[#167]: https://github.com/instrumentisto/medea/pull/167




## [0.1.0] · 2019-08-21
[0.1.0]: https://github.com/instrumentisto/medea/tree/medea-client-api-proto-0.1.0/proto/client-api

[Milestone](https://github.com/instrumentisto/medea/milestone/1) | [Roadmap](https://github.com/instrumentisto/medea/issues/8)

### Added

- RPC messages ([#16](https://github.com/instrumentisto/medea/pull/16)):
    - Server messages:
        - `Pong`;
        - `Event`.
    - Client messages:
        - `Ping`;
        - `Command`.
    - Client commands:
        - `MakeSdpOffer`;
        - `MakeSdpAnswer`;
        - `SetIceCandidate`.
    - Server events:
        - `PeerCreated`;
        - `SdpAnswerMade`;
        - `IceCandidateDiscovered`;
        - `PeersRemoved`.




[`derive-more`]: https://docs.rs/derive_more
[MSRV]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field
[Semantic Versioning 2.0.0]: https://semver.org
