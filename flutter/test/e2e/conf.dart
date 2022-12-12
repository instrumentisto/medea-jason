import 'dart:io';

/// Base address of a servers/clients.
/// Use `10.0.2.2` for windows test in VM.
///
/// Default: `127.0.0.1`
final String IP_TEST_BASE = Platform.environment['IP_TEST_BASE'] ?? '127.0.0.1';

/// Address of a [WebDriver] client.
///
/// Default: `http://127.0.0.1:4444`
///
/// [WebDriver]: https://w3.org/TR/webdriver
final String WEBDRIVER_ADDR =
    Platform.environment['WEBDRIVER_ADDR'] ?? 'http://$IP_TEST_BASE:4444';

/// Address of a Control API mock server.
///
/// Default: `http://127.0.0.1:8000`
final String CONTROL_API_ADDR =
    Platform.environment['CONTROL_API_ADDR'] ?? 'http://$IP_TEST_BASE:8000';

/// Address a Client API WebSocket endpoint.
///
/// Default: `ws://127.0.0.1:8001/ws`
final String CLIENT_API_ADDR =
    Platform.environment['CLIENT_API_ADDR'] ?? 'ws://$IP_TEST_BASE:8001/ws';

/// Host of a [`FileServer`].
///
/// Default: `127.0.0.1:30000`
///
/// [`FileServer`]: crate::file_server::FileServer
final String FILE_SERVER_HOST =
    Platform.environment['FILE_SERVER_HOST'] ?? '$IP_TEST_BASE:30000';

/// Path to a Cucumber features which should be run.
final String FEATURES_PATH =
    Platform.environment['FEATURES_PATH'] ?? 'tests/features';
