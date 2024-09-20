`medea_jason` Flutter plugin changelog
======================================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.7.0] · 2024-??-?? (unreleased)
[0.7.0]: /../../tree/medea-jason-0.7.0/flutter

See also [`medea-jason` crate 0.7.0 changes](/../../tree/medea-jason-0.7.0/CHANGELOG.md).

### Upgraded

- Dependencies:
    - [`flutter_rust_bridge`] to 2.4.0 version. ([#188])

[#188]: /../../pull/188




## [0.6.1] · 2024-09-11
[0.6.1]: /../../tree/medea-jason-0.6.1/flutter

See also [`medea-jason` crate 0.6.1 changes](/../../tree/medea-jason-0.6.1/CHANGELOG.md).

### Upgraded

- Dependencies:
    - [`medea_flutter_webrtc`] to 0.11.1 version. ([#186])

[#186]: /../../pull/186




## [0.6.0] · 2024-08-27
[0.6.0]: /../../tree/medea-jason-0.6.0/flutter

See also [`medea-jason` crate 0.6.0 changes](/../../tree/medea-jason-0.6.0/CHANGELOG.md).

### BC Breaks

- `Jason` constructor is now private and static async factory `Jason.init()` should be used instead. ([#182])

### Upgraded

- [Flutter] to 3.24 version. ([#181])
- Dependencies:
    - [`flutter_rust_bridge`] to 2.2.0 version. ([#182])

[#181]: /../../pull/181
[#182]: /../../pull/182




## [0.5.0] · 2024-08-05
[0.5.0]: /../../tree/medea-jason-0.5.0/flutter

See also [`medea-jason` crate 0.5.0 changes](/../../tree/medea-jason-0.5.0/CHANGELOG.md).

### Added

- More information in `toString()` on custom exceptions ([#140]).

### Changed

- Migrated from [`dart:html`] to [`package:web`] package ([#178]).

[#140]: /../../pull/140
[#178]: /../../pull/178
[`dart:html`]: https://dart.dev/libraries/dart-html
[`package:web`]: https://pub.dev/packages/web




## [0.4.0] · 2023-07-11
[0.4.0]: /../../tree/medea-jason-0.4.0/flutter

### BC Breaks

- Renamed all enum variants in `сamelCase` style ([#119]).

### Fixed

- Initial mute state on incoming tracks in [SFU] mode ([#119]).
- Incorrect `ConnectionHandle`s creation and disposal in [SFU] mode ([#119]).

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




[`flutter_rust_bridge`]: https://pub.dev/packages/flutter_rust_bridge
[`medea_flutter_webrtc`]: https://pub.dev/packages/medea_flutter_webrtc
[`medea-jason`]: https://docs.rs/medea-jason
[Flutter]: https://flutter.dev
[Semantic Versioning 2.0.0]: https://semver.org
[SFU]: https://webrtcglossary.com/sfu
