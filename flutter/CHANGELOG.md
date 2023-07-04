`medea_jason` Flutter plugin changelog
======================================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.4.0] · 2023-07-04
[0.4.0]: /../../tree/medea-jason-0.4.0/flutter

### BC Breaks

- Changed values in `FacingMode` ([#119]):
    - from `User` to `user`;
    - from `Environment` to `environment`;
    - from `Left` to `left`;
    - from `Right` to `right`.
- Changed values in `MediaDeviceKind` ([#119]):
    - from `AudioInput` to `audioInput`;
    - from `VideoInput` to `videoInput`;
    - from `AudioOutput` to `audioOutput`.
- Changed values in `MediaDirection` ([#119]):
    - from `SendRecv` to `sendRecv`;
    - from `SendOnly` to `sendOnly`;
    - from `RecvOnly` to `recvOnly`;
    - from `Inactive` to `inactive`.
- Changed values in `MediaKind` ([#119]):
    - from `Audio` to `audio`;
    - from `Video` to `video`.
- Changed values in `MediaSourceKind` ([#119]):
    - from `Device` to `device`;
    - from `Display` to `display`.

[#119]: /../../pull/119




## [0.3.0] · 2023-06-09
[0.3.0]: /../../tree/medea-jason-0.3.0/flutter

### Added

- Bindings to [`medea-jason`] Rust crate.
- Errors ([#4], [#9], [#28], [#31], [#52]).
- Basic API objects ([#5], [#12], [#45]):
    - `Jason`;
    - `RoomHandle`;
    - `ConnectionHandle`;
    - `MediaManagerHandle`;
    - `ReconnectHandle`;
    - `MediaStreamSettings`, `AudioTrackConstraints`, `DeviceVideoTrackConstraints`, `DisplayVideoTrackConstraints` ([#79]);
    - `MediaDeviceDetails`, `MediaDisplayDetails` ([#96]);
    - `LocalMediaTrack`, `RemoteMediaTrack` ([#42], [#46], [#101], [#109]);
    - `enumerate_displays` ([#81]);
    - `microphoneVolumeIsAvailable`, `setMicrophoneVolume`, `microphoneVolume` ([#49]);
    - `setOutputAudioId` ([#29]);
    - `on_device_change` ([#30]).

[#4]: https://github.com/instrumentisto/medea-jason/pull/4
[#5]: https://github.com/instrumentisto/medea-jason/pull/5
[#9]: https://github.com/instrumentisto/medea-jason/pull/9
[#12]: https://github.com/instrumentisto/medea-jason/pull/12
[#28]: https://github.com/instrumentisto/medea-jason/pull/28
[#29]: https://github.com/instrumentisto/medea-jason/pull/29
[#30]: https://github.com/instrumentisto/medea-jason/pull/30
[#31]: https://github.com/instrumentisto/medea-jason/pull/31
[#42]: https://github.com/instrumentisto/medea-jason/pull/42
[#45]: https://github.com/instrumentisto/medea-jason/pull/45
[#46]: https://github.com/instrumentisto/medea-jason/pull/46
[#49]: https://github.com/instrumentisto/medea-jason/pull/49
[#52]: https://github.com/instrumentisto/medea-jason/pull/52
[#79]: https://github.com/instrumentisto/medea-jason/pull/79
[#81]: https://github.com/instrumentisto/medea-jason/pull/81
[#96]: https://github.com/instrumentisto/medea-jason/pull/96
[#101]: https://github.com/instrumentisto/medea-jason/pull/101
[#109]: https://github.com/instrumentisto/medea-jason/pull/109




[`medea-jason`]: https://docs.rs/medea-jason
[Semantic Versioning 2.0.0]: https://semver.org
