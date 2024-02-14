#import "MedeaJasonPlugin.h"
#if __has_include(<medea_jason/medea_jason-Swift.h>)
#import <medea_jason/medea_jason-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "medea_jason-Swift.h"
#endif

@implementation MedeaJasonPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftMedeaJasonPlugin registerWithRegistrar:registrar];
}
@end
