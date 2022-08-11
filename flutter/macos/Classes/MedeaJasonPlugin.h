#import "FlutterMacOS/FlutterMacOS.h"

@interface MedeaJasonPlugin : NSObject <FlutterPlugin>
- (instancetype)initWithChannel:(FlutterMethodChannel*)
                        channel:(NSObject<FlutterBinaryMessenger>*)messenger;
@end
