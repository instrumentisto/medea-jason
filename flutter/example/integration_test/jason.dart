import 'dart:async';
import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
import 'package:medea_jason/src/native/ffi/nullable_pointer.dart';
import 'package:medea_jason/src/native/ffi/result.dart';
import 'package:medea_jason/src/native/room_handle.dart';
import 'package:medea_jason/src/native/media_device_info.dart';
import 'package:medea_jason/src/native/local_media_track.dart';

// TODO(alexlapa): i guess we dont need this ffi tests now
void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('RoomHandle', (WidgetTester tester) async {
    try {
      print('rh 1');
      var jason = Jason();
      print('rh 2');
      var room = jason.initRoom();

      print('rh 3');
      var allFired = List<Completer>.generate(4, (_) => Completer());

      print('rh 4');
      room.onClose((reason) {
        allFired[0].complete();
      });

      print('rh 5');
      room.onConnectionLoss((reconnectHandle) {
        allFired[1].complete();
      });

      print('rh 6');
      room.onLocalTrack((localTrack) {
        allFired[2].complete();
      });

      print('rh 7');
      room.onNewConnection((connection) {
        allFired[3].complete();
      });

      print('rh 8');
      await Future.wait(allFired.map((e) => e.future))
          .timeout(Duration(seconds: 1));

      print('rh 9');
      room.free();

      print('rh 10');
      expect(() => room.onNewConnection((_) {}), throwsA(isA<FfiException>()));
    } catch (e) {
      print('AAAA ${e}');
    }
  });
}

class TestObj {
  TestObj(this.val);

  int val;
}

void testObjMutator(Object o) {
  o as TestObj;
  o.val = 45;
}
