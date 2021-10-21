import 'dart:ffi';

import 'media_track.dart' as media_track;
import 'peer_connection.dart' as peer_connection;
import 'transceiver.dart' as transceiver;

/// Registers functions needed for platform utils working.
void registerFunctions(DynamicLibrary dl) {
  media_track.registerFunctions(dl);
  peer_connection.registerFunctions(dl);
  transceiver.registerFunctions(dl);
}
