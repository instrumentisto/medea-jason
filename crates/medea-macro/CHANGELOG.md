`medea-macro` changelog
=======================

All user visible changes to this project will be documented in this file. This project uses [Semantic Versioning 2.0.0].




## [0.3.3] · 2025-??-??
[0.3.3]: https://github.com/instrumentisto/medea-jason/tree/medea-macro-0.3.3/crates/medea-macro

### Changed

- `#[dart_bridge]` macro:
    - Removed `thread_local!` usage to avoid running out of TSD keys. ([#238])

[#238]: https://github.com/instrumentisto/medea-jason/pull/238




## [0.3.2] · 2025-03-28
[0.3.2]: https://github.com/instrumentisto/medea-jason/tree/medea-macro-0.3.2/crates/medea-macro

### Added

- `#[dart_bridge]` macro:
    - Support `f32` and `f64` for passing through FFI. ([#199])

[#199]: https://github.com/instrumentisto/medea-jason/pull/199




## [0.3.1] · 2025-03-10
[0.3.1]: https://github.com/instrumentisto/medea-jason/tree/medea-macro-0.3.1/crates/medea-macro

### Changed

- Bumped up [MSRV] to 1.85 because of migration to [2024 edition][031-1]. ([6f760c83])

[6f760c83]: https://github.com/instrumentisto/medea-jason/commit/6f760c836f9c5293b5fefae8a0cb4ee2bd5cfda2
[031-1]: https://doc.rust-lang.org/edition-guide/rust-2024/index.html




## [0.3.0] · 2023-06-09
[0.3.0]: https://github.com/instrumentisto/medea-jason/tree/medea-macro-0.3.0/crates/medea-macro

### BC Breaks

- `#[derive(JsCaused)]` ([#4]):
    - Renamed to `#[derive(Caused)]`;
    - `#[js(cause)]` renamed to `#[cause]`;
    - `#[js(error = "...")]` renamed to `#[cause(error = ...)]` ([6234b23f]).

### Added

- `#[dart_bridge]` macro for generating code glue for `extern` Dart functions ([#14]).

### Updated

- Switched to [2021 Rust edition][030-1] ([#16]).
- Switched to 2.0 version of [`syn`] ([6234b23f]).

[#4]: https://github.com/instrumentisto/medea-jason/pull/4
[#14]: https://github.com/instrumentisto/medea-jason/pull/14
[#16]: https://github.com/instrumentisto/medea-jason/pull/16
[030-1]: https://doc.rust-lang.org/edition-guide/rust-2021/index.html
[6234b23f]: https://github.com/instrumentisto/medea-jason/commit/6234b23f66e81c0ce411dfb8cdf983eda51cd2ad




## [0.2.1] · 2021-04-09
[0.2.1]: https://github.com/instrumentisto/medea/tree/medea-macro-0.2.1/crates/medea-macro

[Diff](https://github.com/instrumentisto/medea/compare/medea-macro-0.2.0...medea-macro-0.2.1)

### Updated

- Switch to [v2 Cargo feature resolver][021-1] ([aa10b2e9]).

[aa10b2e9]: https://github.com/instrumentisto/medea/commit/aa10b2e9fc151465f77dc37d7f11f7cf654dbe6f
[021-1]: https://doc.rust-lang.org/cargo/reference/features.html#feature-resolver-version-2




## [0.2.0] · 2021-02-01
[0.2.0]: https://github.com/instrumentisto/medea/tree/medea-macro-0.2.0/crates/medea-macro

[Diff](https://github.com/instrumentisto/medea/compare/medea-macro-0.1.0...medea-macro-0.2.0)

### BC Breaks

- `#[dispatchable]` macro:
    - Handler traits now require specifying `Output` associative type, which is the return type of all handler trait methods ([#66]).

### Added

- `#[derive(JsCaused)]` macro for deriving `JsCaused` trait from `medea-jason` crate ([#68]).
- `#[dispatchable]` macro:
    - Optional argument to specify `self` type for methods of `*Handler` trait (e.g. `#[dispatchable(self: &Self)]`) ([#112]);
    - Optional argument that enables [`async-trait`] integration (e.g. `#[dispatchable(async_trait(?Send))]`) ([#112]).
- `#[watchers]` macro for generating `Component::spawn` method in `medea-jason` crate ([#169]).

### Fixed

- `#[enum_delegate]` macro now works fine on functions with multiple arguments ([#91]);
- `#[dispatchable]` handler trait visibility now corresponds to original enum visibility ([#147]).

[#66]: https://github.com/instrumentisto/medea/pull/66
[#68]: https://github.com/instrumentisto/medea/pull/68
[#91]: https://github.com/instrumentisto/medea/pull/91
[#112]: https://github.com/instrumentisto/medea/pull/112
[#147]: https://github.com/instrumentisto/medea/pull/147
[#169]: https://github.com/instrumentisto/medea/pull/169




## [0.1.0] · 2019-08-21
[0.1.0]: https://github.com/instrumentisto/medea/tree/medea-macro-0.1.0/crates/medea-macro

### Added

- `#[enum_delegate]` macro for delegating function calls to `enum` variants fields ([#23]);
- `#[dispatchable]` macro for dispatching `enum`-based events ([#26]).

[#23]: https://github.com/instrumentisto/medea/pull/23
[#26]: https://github.com/instrumentisto/medea/pull/26




[`async-trait`]: https://docs.rs/async-trait
[`syn`]: https://docs.rs/syn
[Semantic Versioning 2.0.0]: https://semver.org
