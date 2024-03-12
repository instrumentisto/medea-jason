import 'dart:io';

/// Base address of a servers/clients.
/// Use `10.0.2.2` for windows test in VM.
///
/// Default: `127.0.0.1`
final String ipTestBase = Platform.environment['IP_TEST_BASE'] ?? '127.0.0.1';

/// Address of a [WebDriver] client.
///
/// Default: `http://127.0.0.1:4444`
///
/// [WebDriver]: https://w3.org/TR/webdriver
final String webdriverAddr =
    Platform.environment['WEBDRIVER_ADDR'] ?? 'http://$ipTestBase:4444';

/// Address of a Control API mock server.
///
/// Default: `http://127.0.0.1:8000`
final String controlApiAddr =
    Platform.environment['CONTROL_API_ADDR'] ?? 'http://$ipTestBase:8000';

/// Address a Client API WebSocket endpoint.
///
/// Default: `ws://127.0.0.1:8001/ws`
final String clientApiAddr =
    Platform.environment['CLIENT_API_ADDR'] ?? 'ws://$ipTestBase:8001/ws';

/// Host of a [`FileServer`].
///
/// Default: `127.0.0.1:30000`
///
/// [`FileServer`]: crate::file_server::FileServer
final String fileServerHost =
    Platform.environment['FILE_SERVER_HOST'] ?? '$ipTestBase:30000';

/// Path to a Cucumber features which should be run.
final String featuresPath =
    Platform.environment['FEATURES_PATH'] ?? 'tests/features';
