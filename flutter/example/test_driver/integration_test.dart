import 'dart:io';

import 'package:flutter_driver/flutter_driver.dart';
import 'package:integration_test/integration_test_driver_extended.dart';

const String _packageName = 'com.instrumentisto.medea_jason_example';
const List<String> _androidPermissions = ['CAMERA', 'RECORD_AUDIO'];

Future<void> main() async {
  var driver = await FlutterDriver.connect();
  print('PERMISSIONS 000');
  if ((await driver.serviceClient.getVM()).operatingSystem == 'android') {
    print('PERMISSIONS 111');
    for (var permission in _androidPermissions) {
      print('PERMISSIONS 2222');
      await Process.run('adb', [
        'shell',
        'pm',
        'grant',
        _packageName,
        'android.permission.$permission',
      ]);
    }
    print('PERMISSIONS 3333');
  }
  print('PERMISSIONS 44444');
  await integrationDriver(driver: driver);
  print('PERMISSIONS 5555');
}
