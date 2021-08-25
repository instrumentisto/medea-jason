`medea-control-api-proto` changelog
===================================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.3.0] · 2021-05-12
[0.3.0]: https://github.com/instrumentisto/medea/tree/medea-control-api-proto-0.3.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea/compare/medea-control-api-proto-0.2.0...medea-control-api-proto-0.3.0)

### Upgraded

- Dependencies: ([#199])
    - `prost` to `0.7`;
    - `tonic` to `0.4`.

[#199]: https://github.com/instrumentisto/medea/pull/199




## [0.2.0] · 2021-04-08
[0.2.0]: https://github.com/instrumentisto/medea/tree/medea-control-api-proto-0.2.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea/compare/medea-control-api-proto-0.1.0...medea-control-api-proto-0.2.0) | [Milestone](https://github.com/instrumentisto/medea/milestone/2)

### Added

- gRPC:
    - `ControlApi` service:
        - Methods:
            - `Apply` ([#187]).

[#187]: https://github.com/instrumentisto/medea/pull/187




## [0.1.0] · 2021-02-01
[0.1.0]: https://github.com/instrumentisto/medea/tree/medea-control-api-proto-0.1.0/proto/control-api

[Milestone](https://github.com/instrumentisto/medea/milestone/2) | [Roadmap](https://github.com/instrumentisto/medea/issues/27)

### Added

- gRPC:
    - Services:
        - `ControlApi` ([#57]);
        - `Callback` ([#63]).
    - `ControlApi` service:
        - Methods ([#57]):
            - `Create`;
            - `Get`;
            - `Delete`.
        - Elements ([#57], [#79], [#106]):
            - `Room`;
            - `Member`;
            - `WebRtcPlayEndpoint`;
            - `WebRtcPublishEndpoint`.
    - `Callback` service:
        - Callbacks ([#63]):
            - `OnJoin`;
            - `OnLeave`.

[#57]: https://github.com/instrumentisto/medea/pull/57
[#63]: https://github.com/instrumentisto/medea/pull/63
[#79]: https://github.com/instrumentisto/medea/pull/79
[#106]: https://github.com/instrumentisto/medea/pull/106





[Semantic Versioning 2.0.0]: https://semver.org
