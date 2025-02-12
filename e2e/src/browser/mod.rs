//! Interaction with browser through a [WebDriver] protocol.
//!
//! [WebDriver]: https://w3.org/TR/webdriver

pub mod client;
mod js;
pub mod mock;

use std::{
    io,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use derive_more::with_trait::{Display, Error as StdError, From};
use fantoccini::wd::WindowHandle;
use serde_json::Value as Json;

pub use self::client::{WebDriverClient, WebDriverClientBuilder};

#[doc(inline)]
pub use self::js::Statement;

/// All errors which can happen while working with a browser.
#[derive(Debug, Display, From, StdError)]
pub enum Error {
    /// Failed to deserialize a result of the executed JS code.
    ///
    /// Should never happen.
    Deserialize(serde_json::Error),

    /// JS exception was thrown while executing a JS code.
    #[from(ignore)]
    Js(#[error(not(source))] Json),

    /// Failed to initialize TLS for establishing a [WebDriver] session.
    ///
    /// [WebDriver]: https://w3.org/TR/webdriver
    #[display("Failed to initialize TLS: {_0}")]
    TlsInit(io::Error),

    /// Error occurred while executing some browser action by a [WebDriver].
    ///
    /// [WebDriver]: https://w3.org/TR/webdriver
    WebDriverCmd(fantoccini::error::CmdError),

    /// Error occurred while attempting to establish a [WebDriver] session.
    ///
    /// [WebDriver]: https://w3.org/TR/webdriver
    WebDriverSession(fantoccini::error::NewSessionError),
}

/// Shortcut for a [`Result`] with an [`Error`](enum@Error) inside.
///
/// [`Result`]: std::result::Result
#[expect(clippy::absolute_paths, reason = "one liner")]
type Result<T> = std::result::Result<T, Error>;

/// [WebDriver] handle of a browser window.
///
/// All JS code executed by [`Window::execute()`] will run in the right browser
/// window.
///
/// Window is closed once all [`WindowHandle`]s for this window are [`Drop`]ped.
///
/// [WebDriver]: https://w3.org/TR/webdriver
#[derive(Debug)]
pub struct Window {
    /// Client for interacting with a browser through [WebDriver].
    ///
    /// [WebDriver]: https://w3.org/TR/webdriver
    client: WebDriverClient,

    /// Handle of the browser window in which this [`Window`] should execute
    /// everything.
    window: WindowHandle,

    /// Count of this [`Window`] references.
    ///
    /// Used in a [`Drop`] implementation of this [`Window`].
    rc: Arc<AtomicUsize>,
}

impl Clone for Window {
    fn clone(&self) -> Self {
        _ = self.rc.fetch_add(1, Ordering::SeqCst);
        Self {
            client: self.client.clone(),
            window: self.window.clone(),
            rc: Arc::clone(&self.rc),
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if self.rc.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.client.blocking_window_close(self.window.clone());
        }
    }
}

impl Window {
    /// Creates a new [`Window`] in the provided [`WebDriverClient`].
    async fn new(client: WebDriverClient) -> Self {
        let window = client.new_window().await.unwrap();

        let this = Self {
            client,
            window,
            rc: Arc::new(AtomicUsize::new(1)),
        };
        mock::instantiate_mocks(&this).await;
        this
    }

    /// Executes the provided [`Statement`] in this [`Window`].
    ///
    /// # Errors
    ///
    /// - If failed to switch browser to this [`Window`].
    /// - If failed to execute JS statement.
    pub async fn execute(&self, exec: Statement) -> Result<Json> {
        self.client
            .switch_to_window_and_execute(self.window.clone(), exec)
            .await
    }
}

/// Root [WebDriver] client for some browser.
///
/// This client can create new [`Window`]s.
///
/// [WebDriver] session will be closed on this object's [`Drop`].
///
/// [WebDriver]: https://w3.org/TR/webdriver
#[derive(Debug, From)]
pub struct WindowFactory(WebDriverClient);

impl WindowFactory {
    /// Returns a new [`WindowFactory`] from [`WebDriverClient`].
    #[must_use]
    pub const fn new(client: WebDriverClient) -> Self {
        Self(client)
    }

    /// Creates and returns a new [`Window`].
    pub async fn new_window(&self) -> Window {
        Window::new(self.0.clone()).await
    }
}

impl Drop for WindowFactory {
    fn drop(&mut self) {
        self.0.blocking_close();
    }
}
