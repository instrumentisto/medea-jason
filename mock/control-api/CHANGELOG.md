`medea-control-api-mock` changelog
==================================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.2.1] · Unreleased
[0.2.1]: https://github.com/instrumentisto/medea/tree/medea-control-api-mock-0.2.1/mock/control-api

[Diff](https://github.com/instrumentisto/medea/compare/medea-control-api-mock-0.2.0...medea-control-api-mock-0.2.1)

### Upgraded

- Dependencies:
  - [`derive-more`] to `1.0` ([#181]).

[#181]: /../../pull/181




## [0.2.0] · 2021-04-09
[0.2.0]: https://github.com/instrumentisto/medea/tree/medea-control-api-mock-0.2.0/mock/control-api

[Diff](https://github.com/instrumentisto/medea/compare/medea-control-api-mock-0.1.0...medea-control-api-mock-0.2.0)

### Added

- Endpoints:
    - `PUT /control-api/{room_id}` ([#187]);
    - `PUT /control-api/{room_id}/{element_id}` ([#187]).

[#187]: https://github.com/instrumentisto/medea/pull/187




## [0.1.0] · 2021-02-01
[0.1.0]: https://github.com/instrumentisto/medea/tree/medea-control-api-mock-0.1.0/mock/control-api

### Added

- Endpoints:
    - `GET /control-api/{room_id}` ([#36]);
    - `GET /control-api/{room_id}/{element_id}` ([#36]);
    - `GET /control-api/{room_id/{element_id}/{endpoint_id}` ([#36]);
    - `POST /control-api/{room_id}` ([#36]);
    - `POST /control-api/{room_id}/{element_id}` ([#36]);
    - `POST /control-api/{room_id}/{element_id}/{endpoint_id}` ([#36]);
    - `DELETE /control-api/{room_id}` ([#36]);
    - `DELETE /control-api/{room_id}/{element_id}` ([#36]);
    - `DELETE /control-api/{room_id}/{element_id}/{endpoint_id}` ([#36]);
    - `GET /callbacks` ([#36], [#63]);
    - `GET /subscribe/{room_id}` ([#118], [#136]).
- Events:
    - `Created` ([#118]);
    - `Deleted` ([#118]);
    - `Broadcast` ([#136]).

[#36]: https://github.com/instrumentisto/medea/pull/36
[#63]: https://github.com/instrumentisto/medea/pull/63
[#118]: https://github.com/instrumentisto/medea/pull/118
[#136]: https://github.com/instrumentisto/medea/pull/136





[Semantic Versioning 2.0.0]: https://semver.org
