`medea-control-api-proto` changelog
===================================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.14.0] · 2025-03-10
[0.14.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.14.0/proto/control-api

### Changed

- Bumped up [MSRV] to 1.85 because of migration to [2024 edition][0140-1]. ([6f760c83])

### Upgraded

- Dependencies:
    - [`derive-more`] to 2.0 version. ([3ed7d2bf])

[3ed7d2bf]: https://github.com/instrumentisto/medea-jason/commit/3ed7d2bf59ed1237d9e55c4b65ea5cad833306fa
[6f760c83]: https://github.com/instrumentisto/medea-jason/commit/6f760c836f9c5293b5fefae8a0cb4ee2bd5cfda2
[0140-1]: https://doc.rust-lang.org/edition-guide/rust-2024/index.html




## [0.13.0] · 2024-10-29
[0.13.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.13.0/proto/control-api

### BC Breaks

- Made `member::Sid::to_string()` inherent method instead of implementing `fmt::Display`. ([#189])
- Removed `Ord` and `PartialOrd` implementations from `member::PlainCredentials`. ([#189])

### Upgraded

- Dependencies:
    - [`medea-client-api-proto`] to `0.7` ([#189]).

[#189]: https://github.com/instrumentisto/medea-jason/pull/189




## [0.12.1] · 2024-08-27
[0.12.1]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.12.1/proto/control-api

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-control-api-proto-0.12.0...medea-control-api-proto-0.12.1)

### Upgraded

- Dependencies:
    - [`derive-more`] to 1.0 version. ([#181])

[#181]: https://github.com/instrumentisto/medea-jason/pull/181




## [0.12.0] · 2024-08-05
[0.12.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.12.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-control-api-proto-0.11.0...medea-control-api-proto-0.12.0)

### Upgraded

- Dependencies:
    - [`medea-client-api-proto`] to `0.6` ([#151]).
    - [`tonic`] to `0.12` ([22560ba3]).

[#151]: https://github.com/instrumentisto/medea-jason/pull/151
[22560ba3]: https://github.com/instrumentisto/medea-jason/commit/22560ba3d0d970c907c060231a5c6363d5c1dbfa




## [0.11.0] · 2024-02-12
[0.11.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.11.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-control-api-proto-0.10.0...medea-control-api-proto-0.11.0)

### Upgraded

- Dependencies:
    - [`tonic`] to `0.11` ([b504f9dc]).

[b504f9dc]: https://github.com/instrumentisto/medea-jason/commit/b504f9dc97451135e2138afabf67935e3bc53475




## [0.10.0] · 2023-09-25
[0.10.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.10.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-control-api-proto-0.9.0...medea-control-api-proto-0.10.0)

### Upgraded

- Dependencies:
    - [`prost`] to `0.12` ([7ab40e8a]);
    - [`tonic`] to `0.10` ([7ab40e8a]).

[7ab40e8a]: https://github.com/instrumentisto/medea-jason/commit/7ab40e8a48b4add3ddee31935f11dbcd09cecece




## [0.9.0] · 2023-07-12
[0.9.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.9.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-control-api-proto-0.8.0...medea-control-api-proto-0.9.0)

### Upgraded

- Dependencies:
    - [`medea-client-api-proto`] to `0.5` ([#123]).

[#123]: https://github.com/instrumentisto/medea-jason/pull/123




## [0.8.0] · 2023-06-09
[0.8.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.8.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-control-api-proto-0.7.0...medea-control-api-proto-0.8.0)

### Added

- Abstractions:
    - `ControlApi` and `CallbackApi` ([#55]).
- gRPC:
    - `ControlApi` and `CallbackApi` implementations ([#56]).
- Direct:
    - `ControlApi` and `CallbackApi` implementations ([#65]).
- Cargo features:
    - `client` and `server` ([#56]);
    - `serde` ([#67]);
    - `client-api` ([#68]).

### Upgraded

- Dependencies:
    - [`prost`] to `0.11` ([c8332ea9]);
    - [`tonic`] to `0.9` ([6234b23f]).

[#55]: https://github.com/instrumentisto/medea-jason/pull/55
[#56]: https://github.com/instrumentisto/medea-jason/pull/56
[#67]: https://github.com/instrumentisto/medea-jason/pull/67
[#68]: https://github.com/instrumentisto/medea-jason/pull/68
[6234b23f]: https://github.com/instrumentisto/medea-jason/commit/6234b23f66e81c0ce411dfb8cdf983eda51cd2ad
[c8332ea9]: https://github.com/instrumentisto/medea-jason/commit/c8332ea9b6310958549e750a5553294f894c2d7b




## [0.7.0] · 2022-05-05
[0.7.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.7.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-control-api-proto-0.6.0...medea-control-api-proto-0.7.0)

### Added

- gRPC:
    - `ControlApi` service:
        - Methods:
            - `Healthz` ([#51]).
        - Messages:
            - `Ping` ([#51]);
            - `Pong` ([#51]).

[#51]: https://github.com/instrumentisto/medea-jason/pull/51




## [0.6.0] · 2022-04-13
[0.6.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.6.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-control-api-proto-0.5.0...medea-control-api-proto-0.6.0)

### Upgraded

- Dependencies: ([b3acc904])
    - [`prost`] to `0.10`;
    - [`tonic`] to `0.7`.

[b3acc904]: https://github.com/instrumentisto/medea-jason/commit/b3acc904165329946d0efbf2f1e7bf9dff1271df




## [0.5.0] · 2021-12-20
[0.5.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.5.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea-jason/compare/medea-control-api-proto-0.4.0...medea-control-api-proto-0.5.0)

### Upgraded

- Dependencies: ([#16])
    - [`prost`] to `0.9`;
    - [`tonic`] to `0.6`.

### Updated

- Switch to [2021 Rust edition][012-1] ([#16]).

[#16]: https://github.com/instrumentisto/medea-jason/pull/16
[012-1]: https://doc.rust-lang.org/edition-guide/rust-2021/index.html




## [0.4.0] · 2021-08-25
[0.4.0]: https://github.com/instrumentisto/medea-jason/tree/medea-control-api-proto-0.4.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea-jason/compare/a2ce6b92...medea-control-api-proto-0.4.0)

### Upgraded

- Dependencies:
    - [`prost`] to `0.8`;
    - [`tonic`] to `0.5`.




## [0.3.0] · 2021-05-12
[0.3.0]: https://github.com/instrumentisto/medea/tree/medea-control-api-proto-0.3.0/proto/control-api

[Diff](https://github.com/instrumentisto/medea/compare/medea-control-api-proto-0.2.0...medea-control-api-proto-0.3.0)

### Upgraded

- Dependencies: ([#199])
    - [`prost`] to `0.7`;
    - [`tonic`] to `0.4`.

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




[`derive_more`]: https://docs.rs/derive_more
[`medea-client-api-proto`]: https://docs.rs/medea-client-api-proto
[`prost`]: https://docs.rs/prost
[`tonic`]: https://docs.rs/tonic
[MSRV]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field
[Semantic Versioning 2.0.0]: https://semver.org
