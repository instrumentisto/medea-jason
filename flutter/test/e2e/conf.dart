
const WEBDRIVER_ADDR = String.fromEnvironment('WEBDRIVER_ADDR', defaultValue: 'http://127.0.0.1:4444');
const CONTROL_API_ADDR = String.fromEnvironment('CONTROL_API_ADDR', defaultValue: 'http://127.0.0.1:8000');
const CLIENT_API_ADDR = String.fromEnvironment('CLIENT_API_ADDR', defaultValue: 'ws://127.0.0.1:8001/ws');
const FILE_SERVER_HOST = String.fromEnvironment('FILE_SERVER_HOST', defaultValue: '127.0.0.1:30000');
const FEATURES_PATH = String.fromEnvironment('FEATURES_PATH', defaultValue: 'tests/features');