import 'dart:ffi';

import 'codec_capability.dart' as codec_capability;
import 'constraints.dart' as constraints;
import 'ice_candidate.dart' as ice_candidate;
import 'ice_candidate_error.dart' as ice_candidate_error;
import 'ice_servers.dart' as ice_servers;
import 'media_device_info.dart' as media_device_info;
import 'media_devices.dart' as media_devices;
import 'media_display_info.dart' as media_display_info;
import 'media_track.dart' as media_track;
import 'object.dart' as object;
import 'parameters.dart' as parameters;
import 'peer_connection.dart' as peer_connection;
import 'send_encoding_parameters.dart' as send_encoding_parameters;
import 'transceiver.dart' as transceiver;
import 'transport.dart' as transport;

/// Registers functions needed for platform utils working.
void registerFunctions(DynamicLibrary dl) {
  print("platform_functions::registerFunctions 1");
  object.registerFunctions(dl);
  print("platform_functions::registerFunctions 2");
  media_track.registerFunctions(dl);
  print("platform_functions::registerFunctions 3");
  peer_connection.registerFunctions(dl);
  print("platform_functions::registerFunctions 4");
  transceiver.registerFunctions(dl);
  print("platform_functions::registerFunctions 5");
  ice_servers.registerFunctions(dl);
  print("platform_functions::registerFunctions 6");
  constraints.registerFunctions(dl);
  print("platform_functions::registerFunctions 7");
  media_devices.registerFunctions(dl);
  print("platform_functions::registerFunctions 8");
  transport.registerFunctions(dl);
  print("platform_functions::registerFunctions 9");
  codec_capability.registerFunctions(dl);
  print("platform_functions::registerFunctions 10");
  media_display_info.registerFunctions(dl);
  print("platform_functions::registerFunctions 11");
  ice_candidate.registerFunctions(dl);
  print("platform_functions::registerFunctions 12");
  send_encoding_parameters.registerFunctions(dl);
  print("platform_functions::registerFunctions 13");
  parameters.registerFunctions(dl);
  print("platform_functions::registerFunctions 14");
  ice_candidate_error.registerFunctions(dl);
  print("platform_functions::registerFunctions 15");
  media_device_info.registerFunctions(dl);
  print("platform_functions::registerFunctions 16");
}
