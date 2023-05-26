#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint medea_jason.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name        = 'medea_jason'
  s.version     = '0.3.0'
  s.summary     = 'Cross-platform client library of Medea media server for Flutter.'
  s.description = 'Cross-platform client library of Medea media server for Flutter.'
  s.homepage    = 'https://github.com/instrumentisto/medea-jason'
  s.license     = { :file => '../LICENSE' }
  s.author      = { 'Instrumentisto Team' => 'developer@instrumentisto.com' }

  s.source      = { :path => '.' }
  s.source_files = 'Classes/**/*'
  s.dependency 'Flutter'
  s.platform = :ios, '13.0'

  s.vendored_frameworks = 'lib/MedeaJason.xcframework'

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = {
    'DEFINES_MODULE' => 'YES',
    'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386 arm64',
    'OTHER_LDFLAGS[arch=x86_64]' => "-force_load $(PODS_TARGET_SRCROOT)/lib/MedeaJason.xcframework/ios-x86_64-simulator/libmedea_jason.a",
    'OTHER_LDFLAGS[arch=arm64]' => "-force_load $(PODS_TARGET_SRCROOT)/lib/MedeaJason.xcframework/ios-arm64/libmedea_jason.a",
    'VALID_ARCHS[sdk=iphonesimulator*]' => 'x86_64',
    'VALID_ARCHS[sdk=iphoneos*]' => 'arm64 x86_64',
    'OTHER_LDFLAGS' => '-ObjC'
  }

  s.swift_version = '5.0'
end
