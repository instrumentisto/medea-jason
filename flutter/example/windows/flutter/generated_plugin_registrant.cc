//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <medea_flutter_webrtc/medea_flutter_webrtc_plugin_c_api.h>
#include <medea_jason/medea_jason_plugin.h>

void RegisterPlugins(flutter::PluginRegistry* registry) {
  MedeaFlutterWebrtcPluginCApiRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("MedeaFlutterWebrtcPluginCApi"));
  MedeaJasonPluginRegisterWithRegistrar(
      registry->GetRegistrarForPlugin("MedeaJasonPlugin"));
}
