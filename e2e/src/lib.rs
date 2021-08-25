//! Tools for testing [`medea-jason`] through a [WebDriver] protocol.
//!
//! [`medea-jason`]: https://docs.rs/medea-jason
//! [WebDriver]: https://w3.org/TR/webdriver

#![forbid(non_ascii_idents, unsafe_code)]

pub mod browser;
pub mod object;

pub use browser::{WebDriverClient, WebDriverClientBuilder};
