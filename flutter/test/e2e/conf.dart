/// Address of a [WebDriver] client.
///
/// Default: `http://127.0.0.1:4444`
///
/// [WebDriver]: https://w3.org/TR/webdriver
const WEBDRIVER_ADDR = String.fromEnvironment('WEBDRIVER_ADDR',
    defaultValue: 'http://127.0.0.1:4444');

/// Address of a Control API mock server.
///
/// Default: `http://127.0.0.1:8000`
const CONTROL_API_ADDR = String.fromEnvironment('CONTROL_API_ADDR',
    defaultValue: 'http://127.0.0.1:8000');

/// Address a Client API WebSocket endpoint.
///
/// Default: `ws://127.0.0.1:8001/ws`
const CLIENT_API_ADDR = String.fromEnvironment('CLIENT_API_ADDR',
    defaultValue: 'ws://127.0.0.1:8001/ws');

/// Host of a [`FileServer`].
///
/// Default: `127.0.0.1:30000`
///
/// [`FileServer`]: crate::file_server::FileServer
const FILE_SERVER_HOST =
    String.fromEnvironment('FILE_SERVER_HOST', defaultValue: '127.0.0.1:30000');

/// Path to a Cucumber features which should be run.
const FEATURES_PATH =
    String.fromEnvironment('FEATURES_PATH', defaultValue: 'tests/features');