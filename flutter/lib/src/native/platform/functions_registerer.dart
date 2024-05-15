import 'dart:ffi';

import 'constraints.dart' as constraints;
import 'ice_candidate.dart' as ice_candidate;
import 'ice_candidate_error.dart' as ice_candidate_error;
import 'ice_servers.dart' as ice_servers;
import 'media_device_info.dart' as media_device_info;
import 'media_devices.dart' as media_devices;
import 'media_display_info.dart' as media_display_info;
import 'media_track.dart' as media_track;
import 'object.dart' as object;
import 'peer_connection.dart' as peer_connection;
import 'transceiver.dart' as transceiver;
import 'transport.dart' as transport;

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
  media_device_info.registerFunctions(dl);
  media_display_info.registerFunctions(dl);
  ice_candidate.registerFunctions(dl);
  ice_candidate_error.registerFunctions(dl);
}
