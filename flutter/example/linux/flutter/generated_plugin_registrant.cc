//
//  Generated file. Do not edit.
//

// clang-format off

#include "generated_plugin_registrant.h"

#include <medea_jason/medea_jason_plugin.h>

void fl_register_plugins(FlPluginRegistry* registry) {
  g_autoptr(FlPluginRegistrar) medea_jason_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "MedeaJasonPlugin");
  medea_jason_plugin_register_with_registrar(medea_jason_registrar);
}
