/// Base address of a servers/clients.
/// Use `10.0.2.2` for windows test in VM.
///
/// Default: `127.0.0.1`
const String ipTestBase =
    String.fromEnvironment('IP_TEST_BASE', defaultValue: '127.0.0.1');

/// Address of a [WebDriver] client.
///
/// Default: `http://127.0.0.1:4444`
///
/// [WebDriver]: https://w3.org/TR/webdriver
const String webdriverAddr = String.fromEnvironment('WEBDRIVER_ADDR',
    defaultValue: 'http://$ipTestBase:4444');

/// Address of a Control API mock server.
///
/// Default: `http://127.0.0.1:8000`
const String controlApiAddr = String.fromEnvironment('CONTROL_API_ADDR',
    defaultValue: 'http://$ipTestBase:8000');

/// Address a Client API WebSocket endpoint.
///
/// Default: `ws://127.0.0.1:8001/ws`
const String clientApiAddr = String.fromEnvironment('CLIENT_API_ADDR',
    defaultValue: 'ws://$ipTestBase:8001/ws');

/// Host of a [`FileServer`].
///
/// Default: `127.0.0.1:30000`
///
/// [`FileServer`]: crate::file_server::FileServer
const String fileServerHost = String.fromEnvironment('FILE_SERVER_HOST',
    defaultValue: '$ipTestBase:30000');

/// Path to a Cucumber features which should be run.
const String featuresPath =
    String.fromEnvironment('FEATURES_PATH', defaultValue: 'tests/features');
