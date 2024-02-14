#import "MedeaJasonPlugin.h"

@implementation MedeaJasonPlugin
+ (void)registerWithRegistrar:(nonnull id<FlutterPluginRegistrar>)registrar {
    FlutterMethodChannel* channel = [FlutterMethodChannel
        methodChannelWithName:@"medea_jason"
              binaryMessenger:[registrar messenger]];
    MedeaJasonPlugin* instance = [MedeaJasonPlugin alloc];
    MedeaJasonPlugin* finalInstance =
        [instance initWithChannel:channel:[registrar messenger]];
    [registrar addMethodCallDelegate:finalInstance channel:channel];
}

// Handles provided FlutterMethodCall.
- (void)handleMethodCall:(nonnull FlutterMethodCall*)methodCall
                  result:(nonnull FlutterResult)result {
    result(FlutterMethodNotImplemented);
}

// Initializes this FlutterWebRTCPlugin with a FlutterMethodChannel.
- (instancetype)initWithChannel:(FlutterMethodChannel*)
                        channel:(NSObject<FlutterBinaryMessenger>*)messenger {
    self = [super init];
    return self;
}
@end
