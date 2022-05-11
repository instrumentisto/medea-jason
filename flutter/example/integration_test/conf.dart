import 'dart:io';

Map<String, String> envVars = Platform.environment;

void main() {

  var check = envVars['WEBDRIVER_ADDR'];
  if (check == '') {
    envVars['WEBDRIVER_ADDR'] = 'http://127.0.0.1:4444';
  }

  check = envVars['CONTROL_API_ADDR'];
  if (check == '') {
    envVars['CONTROL_API_ADDR'] = 'http://127.0.0.1:8000';
  }

  check = envVars['CLIENT_API_ADDR'];
  if (check == '') {
    envVars['CLIENT_API_ADDR'] = 'ws://127.0.0.1:8001/ws';
  }

  check = envVars['FILE_SERVER_HOST'];
  if (check == '') {
    envVars['FILE_SERVER_HOST'] = '127.0.0.1:30000';
  }

  check = envVars['FEATURES_PATH'];
  if (check == '') {
    envVars['FEATURES_PATH'] = 'tests/features';
  }
}