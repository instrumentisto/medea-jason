//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <medea_flutter_webrtc/medea_flutter_webrtc_plugin.h>
#include <medea_jason/medea_jason_plugin.h>

void fl_register_plugins(FlPluginRegistry* registry) {
  g_autoptr(FlPluginRegistrar) medea_flutter_webrtc_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "MedeaFlutterWebrtcPlugin");
  medea_flutter_webrtc_plugin_register_with_registrar(medea_flutter_webrtc_registrar);
  g_autoptr(FlPluginRegistrar) medea_jason_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "MedeaJasonPlugin");
  medea_jason_plugin_register_with_registrar(medea_jason_registrar);
}
