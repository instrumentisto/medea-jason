#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint medea_jason.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'medea_jason'
  s.version          = '0.0.1'
  s.summary          = 'A new Flutter project.'
  s.description      = <<-DESC
A new Flutter project.
                       DESC
  s.homepage         = 'http://example.com'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Your Company' => 'email@example.com' }
  s.source           = { :path => '.' }
  s.source_files = 'Classes/**/*'
  s.dependency 'Flutter'
  s.platform = :ios, '13.0'

  s.vendored_libraries = 'lib/aarch64-apple-ios/*.a'

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig = {
    'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386',
    "OTHER_LDFLAGS" => "-force_load $(PODS_TARGET_SRCROOT)/lib/aarch64-apple-ios/libmedea_jason.a"
  }
  s.swift_version = '5.0'
end
