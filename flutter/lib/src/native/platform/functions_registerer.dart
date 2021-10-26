import 'dart:ffi';

import 'object.dart' as object;
import 'media_track.dart' as media_track;
import 'peer_connection.dart' as peer_connection;
import 'transceiver.dart' as transceiver;
import 'ice_servers.dart' as ice_servers;

/// Registers functions needed for platform utils working.
void registerFunctions(DynamicLibrary dl) {
  object.registerFunctions(dl);
  media_track.registerFunctions(dl);
  peer_connection.registerFunctions(dl);
  transceiver.registerFunctions(dl);
  ice_servers.registerFunctions(dl);
}
