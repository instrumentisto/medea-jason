Medea Client API protocol
=========================

[![Latest version](https://img.shields.io/crates/v/medea-client-api-proto "Latest version")](https://crates.io/crates/medea-client-api-proto)
[![Rust 1.85+](https://img.shields.io/badge/rustc-1.85+-lightgray.svg "Rust 1.85+")](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html)
[![Unsafe Forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg "Unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![Rust docs](https://docs.rs/medea-client-api-proto/badge.svg "Rust docs")](https://docs.rs/medea-client-api-proto)

[API docs](https://docs.rs/medea-client-api-proto) |
[Changelog](https://github.com/instrumentisto/medea-jason/blob/master/proto/client-api/CHANGELOG.md)

[Client API] protocol implementation for [Medea] media server.




## Cargo features

- `client` (default): Enables `Deserialize` implementation for `Event`s, and `Serialize` implementation for `Command`s.
- `server`: Enables `Deserialize` implementation for `Command`s, and `Serialize` implementation for `Event`s.




## Contribution guide

Avoid using 64 bit types. [`medea-jason`] uses [wasm-bindgen] to interop with JS, and exposing 64 bit types to JS will make [wasm-bindgen] to use [BigInt64Array][2] / [BigUint64Array][3] in its JS glue, which are not implemented or were implemented too recently in some UAs.

So, it's better to keep protocol 64-bit-types-clean to avoid things breaking by accident.




## License

Copyright © 2019-2025  Instrumentisto Team <https://github.com/instrumentisto>

This software is subject to the terms of the [Blue Oak Model License 1.0.0](https://github.com/instrumentisto/medea-jason/blob/master/proto/client-api/LICENSE.md). If a copy of the [BlueOak-1.0.0](https://spdx.org/licenses/BlueOak-1.0.0.html) license was not distributed with this file, You can obtain one at <https://blueoakcouncil.org/license/1.0.0>.




[`medea-jason`]: https://docs.rs/medea-jason

[Client API]: https://github.com/instrumentisto/medea/blob/master/docs/rfc/0002-webrtc-client-api.md
[Medea]: https://github.com/instrumentisto/medea
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen

[2]: https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt64Array
[3]: https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigUint64Array
