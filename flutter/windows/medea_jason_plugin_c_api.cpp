#include "include/medea_jason/medea_jason_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "medea_jason_plugin.h"

void MedeaJasonPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  medea_jason::MedeaJasonPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
