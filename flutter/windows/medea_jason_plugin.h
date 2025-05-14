#ifndef FLUTTER_PLUGIN_MEDEA_JASON_PLUGIN_H_
#define FLUTTER_PLUGIN_MEDEA_JASON_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace medea_jason {

class MedeaJasonPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  MedeaJasonPlugin();

  virtual ~MedeaJasonPlugin();

  // Disallow copy and assign.
  MedeaJasonPlugin(const MedeaJasonPlugin&) = delete;
  MedeaJasonPlugin& operator=(const MedeaJasonPlugin&) = delete;

  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace medea_jason

#endif  // FLUTTER_PLUGIN_MEDEA_JASON_PLUGIN_H_
