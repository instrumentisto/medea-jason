`medea-demo` changelog
======================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.1.0] · 2021-04-09
[0.1.0]: https://github.com/instrumentisto/medea/tree/medea-demo-0.1.0/jason/demo

### Added

- UI/UX:
    - Multiple room members ([#38]);
    - Multiple rooms ([#147]);
    - Audio/video device selection ([#38]);
    - Nickname specifying ([#38]);
    - Muting audio/video tracks ([#156]);
    - Disabling audio/video send/recv tracks ([#40], [#127], [#155]);
    - Connection state indication ([#75]);
    - Call quality indication ([#132]);
    - Force relaying via [TURN] server ([#79]);
    - Screen sharing ([#144]).
- Deployment:
    - [Docker] image ([#38]);
    - [Helm] chart ([#41]).

[#38]: https://github.com/instrumentisto/medea/pull/38
[#40]: https://github.com/instrumentisto/medea/pull/40
[#41]: https://github.com/instrumentisto/medea/pull/41
[#75]: https://github.com/instrumentisto/medea/pull/75
[#79]: https://github.com/instrumentisto/medea/pull/79
[#127]: https://github.com/instrumentisto/medea/pull/127
[#132]: https://github.com/instrumentisto/medea/pull/132
[#144]: https://github.com/instrumentisto/medea/pull/144
[#147]: https://github.com/instrumentisto/medea/pull/147
[#155]: https://github.com/instrumentisto/medea/pull/155
[#156]: https://github.com/instrumentisto/medea/pull/156





[Docker]: https://docker.io
[Helm]: https://helm.sh
[Semantic Versioning 2.0.0]: https://semver.org
[TURN]: https://webrtc.org/getting-started/turn-server
