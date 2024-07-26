//! Client for a [WebDriver].
//!
//! [WebDriver]: https://w3.org/TR/webdriver

use std::{
    sync::{mpsc, Arc},
    time::Duration,
};

use fantoccini::{
    wd::{Capabilities, WindowHandle},
    Client, ClientBuilder, Locator,
};
use futures::lock::Mutex;
use serde::Deserialize;
use serde_json::{json, Value as Json};
use tokio::task;

use super::{js::Statement, Error, Result};

/// Arguments for Chrome browser.
const CHROME_ARGS: &[&str] = &[
    "--use-fake-device-for-media-stream",
    "--use-fake-ui-for-media-stream",
    "--disable-web-security",
    "--disable-dev-shm-usage",
    "--no-sandbox",
];

/// Arguments for Firefox browser.
const FIREFOX_ARGS: &[&str] = &[];

/// Result returned from all the JS code executed in a browser.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum JsResult {
    /// [`Json`] value of a successful result.
    Ok(Json),

    /// [`Json`] value of an error result.
    Err(Json),
}

impl From<JsResult> for Result<Json> {
    fn from(from: JsResult) -> Self {
        match from {
            JsResult::Ok(ok) => Self::Ok(ok),
            JsResult::Err(err) => Self::Err(Error::Js(err)),
        }
    }
}

/// Client for interacting with a browser through a [WebDriver] protocol.
///
/// [WebDriver]: https://w3.org/TR/webdriver
#[allow(clippy::module_name_repetitions)] // TODO: Refactor?
#[derive(Clone, Debug)]
pub struct WebDriverClient {
    /// Inner implementation of this [`WebDriverClient`].
    inner: Arc<Mutex<Inner>>,

    /// Host of the file server to load `index.html` page from.
    file_server_host: String,
}

impl WebDriverClient {
    /// Creates a new window in a browser and returns its ID.
    ///
    /// # Errors
    ///
    /// If failed to create or switch to a new browser window.
    pub async fn new_window(&self) -> Result<WindowHandle> {
        self.inner
            .lock()
            .await
            .new_window(&self.file_server_host)
            .await
    }

    /// Switches to the provided browser window and executes the provided
    /// [`Statement`] in it.
    ///
    /// # Errors
    ///
    /// - If failed to switch to the provided browser window.
    /// - If failed to execute JS statement.
    pub async fn switch_to_window_and_execute(
        &self,
        window: WindowHandle,
        exec: Statement,
    ) -> Result<Json> {
        self.inner
            .lock()
            .await
            .switch_to_window_and_execute(window, exec)
            .await
    }

    /// Synchronously closes a [WebDriver] session.
    ///
    /// [WebDriver]: https://w3.org/TR/webdriver
    ///
    /// # Panics
    ///
    /// If [`tokio::spawn()`] panics.
    pub fn blocking_close(&self) {
        let (tx, rx) = mpsc::channel();
        let client = Arc::clone(&self.inner);
        drop(tokio::spawn(async move {
            let inner = client.lock().await;
            inner.0.clone().close().await.unwrap();
            tx.send(()).unwrap();
        }));
        task::block_in_place(move || {
            rx.recv().unwrap();
        });
    }

    /// Synchronously closes the provided browser window.
    ///
    /// # Panics
    ///
    /// If [`tokio::spawn()`] panics.
    pub fn blocking_window_close(&self, window: WindowHandle) {
        let (tx, rx) = mpsc::channel();
        let client = Arc::clone(&self.inner);
        drop(tokio::spawn(async move {
            let client = client.lock().await;
            client.close_window(window).await;
            tx.send(()).unwrap();
        }));
        task::block_in_place(move || {
            rx.recv().unwrap();
        });
    }
}

/// Builder for [`WebDriverClientBuilder`].
#[derive(Clone, Debug)]
pub struct WebDriverClientBuilder<'a, Caps = AutoCapabilities> {
    /// Address of a [WebDriver] server.
    ///
    /// [WebDriver]: https://w3.org/TR/webdriver
    webdriver_address: &'a str,

    /// [`WebDriverClient`] [`Capabilities`].
    capabilities: Caps,
}

impl<'a> WebDriverClientBuilder<'a> {
    /// Creates new [`WebDriverClientBuilder`].
    #[must_use]
    pub const fn new(webdriver_address: &'a str) -> Self {
        Self {
            webdriver_address,
            capabilities: AutoCapabilities {
                headless_firefox: false,
                headless_chrome: false,
            },
        }
    }

    /// Sets manually provided browser [`Capabilities`].
    #[must_use]
    pub const fn capabilities(
        self,
        capabilities: Capabilities,
    ) -> WebDriverClientBuilder<'a, Capabilities> {
        WebDriverClientBuilder {
            webdriver_address: self.webdriver_address,
            capabilities,
        }
    }

    /// Sets `moz:firefoxOptions` `--headless` for Firefox browser.
    #[must_use]
    pub const fn headless_firefox(mut self, value: bool) -> Self {
        self.capabilities.headless_firefox = value;
        self
    }

    /// Sets `goog:chromeOptions` `--headless` for Chrome browser.
    #[must_use]
    pub const fn headless_chrome(mut self, value: bool) -> Self {
        self.capabilities.headless_chrome = value;
        self
    }
}

impl<Caps: Into<Capabilities>> WebDriverClientBuilder<'_, Caps> {
    /// Creates a new [`WebDriverClient`] connected to a [WebDriver].
    ///
    /// # Errors
    ///
    /// If failed to locate [WebDriver] instance.
    ///
    /// [WebDriver]: https://w3.org/TR/webdriver
    pub async fn connect(
        self,
        file_server_host: &str,
    ) -> Result<WebDriverClient> {
        Ok(WebDriverClient {
            inner: Arc::new(Mutex::new(
                Inner::new(self.webdriver_address, self.capabilities.into())
                    .await?,
            )),
            file_server_host: file_server_host.to_owned(),
        })
    }
}

/// Inner implementation of a [`WebDriverClient`].
struct Inner(Client);

impl Inner {
    /// Creates a new [WebDriver] session.
    ///
    /// [WebDriver]: https://w3.org/TR/webdriver
    async fn new(webdriver_address: &str, caps: Capabilities) -> Result<Self> {
        Ok(Self(
            ClientBuilder::rustls()
                .capabilities(caps)
                .connect(webdriver_address)
                .await?,
        ))
    }

    /// Executes the provided [`Statement`] in the current browser window.
    ///
    /// # Errors
    ///
    /// - If JS exception was thrown while executing a JS code.
    /// - If failed to deserialize a result of the executed JS code.
    async fn execute(&self, statement: Statement) -> Result<Json> {
        let (inner_js, args) = statement.prepare();

        // language=JavaScript
        let js = format!(
            r#"
            (
                async () => {{
                    let callback = arguments[arguments.length - 1];
                    try {{
                        {inner_js}
                        callback({{ ok: lastResult }});
                    }} catch (e) {{
                        if (e.__wbg_ptr > 0) {{
                            callback({{
                                err: {{
                                    kind: e.kind ? e.kind() : undefined,
                                    message: e.message(),
                                    trace: e.trace(),
                                    cause: e.cause ? e.cause() : undefined
                                }}
                            }});
                        }} else {{
                            callback({{ err: JSON.stringify(e) }});
                        }}
                    }}
                }}
            )();
            "#,
        );
        let res = self.0.execute_async(&js, args).await?;

        serde_json::from_value::<JsResult>(res)?.into()
    }

    /// Creates a new browser window and returns its ID.
    ///
    /// Creates a `registry` in the created browser window.
    ///
    /// # Errors
    ///
    /// - If failed to create a new browser window.
    /// - If `index.html` wasn't found at `file_server_host`.
    async fn new_window(&self, file_server_host: &str) -> Result<WindowHandle> {
        let window = self.0.new_window(true).await?.handle;
        self.0.switch_to_window(window.clone()).await?;
        self.0
            .goto(&format!("http://{file_server_host}/index.html"))
            .await?;
        self.0
            .wait()
            .at_most(Duration::from_secs(120))
            .for_element(Locator::Id("loaded"))
            .await
            .map(drop)?;

        self.execute(Statement::new(
            // language=JavaScript
            "
            async () => {
                window.registry = new Map();
            }
            ",
            vec![],
        ))
        .await
        .map(drop)?;

        Ok(window)
    }

    /// Switches to the provided browser window and executes the provided
    /// [`Statement`].
    async fn switch_to_window_and_execute(
        &self,
        window: WindowHandle,
        exec: Statement,
    ) -> Result<Json> {
        self.0.switch_to_window(window).await?;
        self.execute(exec).await
    }

    /// Closes the provided browser window.
    async fn close_window(&self, window: WindowHandle) {
        if self.0.switch_to_window(window).await.is_ok() {
            drop(self.0.close_window().await);
        }
    }
}

/// Settings to build [`Capabilities`] automatically.
#[derive(Clone, Copy, Debug)]
pub struct AutoCapabilities {
    /// Indicator whether [`WebDriverClient`] will run against headless Firefox
    /// browser.
    headless_firefox: bool,

    /// Indicator whether [`WebDriverClient`] will run against headless Chrome
    /// browser.
    headless_chrome: bool,
}

impl AutoCapabilities {
    /// Returns `moz:firefoxOptions` for a Firefox browser.
    fn firefox(self) -> serde_json::Value {
        let mut args = FIREFOX_ARGS.to_vec();
        if self.headless_firefox {
            args.push("--headless");
        }
        json!({
            "prefs": {
                "media.navigator.streams.fake": true,
                "media.navigator.permission.disabled": true,
                "media.autoplay.enabled": true,
                "media.autoplay.enabled.user-gestures-needed ": false,
                "media.autoplay.ask-permission": false,
                "media.autoplay.default": 0,
            },
            "args": args,
        })
    }

    /// Returns `goog:chromeOptions` for a Chrome browser.
    fn chrome(self) -> serde_json::Value {
        let mut args = CHROME_ARGS.to_vec();
        if self.headless_chrome {
            args.push("--headless");
        }
        json!({ "args": args })
    }
}

impl From<AutoCapabilities> for Capabilities {
    fn from(auto: AutoCapabilities) -> Self {
        let mut caps = Self::new();
        drop(caps.insert("moz:firefoxOptions".to_owned(), auto.firefox()));
        drop(caps.insert("goog:chromeOptions".to_owned(), auto.chrome()));
        caps
    }
}
