import 'dart:ffi';

import 'object.dart' as object;
import 'media_track.dart' as media_track;
import 'peer_connection.dart' as peer_connection;
import 'transceiver.dart' as transceiver;
import 'ice_servers.dart' as ice_servers;
import 'constraints.dart' as constraints;
import 'media_devices.dart' as media_devices;
import 'transport.dart' as transport;
import 'input_device_info.dart' as input_device_info;
import 'ice_candidate.dart' as ice_candidate;

/// Registers functions needed for platform utils working.
void registerFunctions(DynamicLibrary dl) {
  object.registerFunctions(dl);
  media_track.registerFunctions(dl);
  peer_connection.registerFunctions(dl);
  transceiver.registerFunctions(dl);
  ice_servers.registerFunctions(dl);
  constraints.registerFunctions(dl);
  media_devices.registerFunctions(dl);
  transport.registerFunctions(dl);
  input_device_info.registerFunctions(dl);
  ice_candidate.registerFunctions(dl);
}
