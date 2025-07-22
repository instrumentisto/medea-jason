Pod::Spec.new do |s|
  s.name             = 'medea_jason'
  s.version          = '0.11.0'
  s.summary          = 'Cross-platform client library of Medea media server for Flutter.'
  s.description      = 'Cross-platform client library of Medea media server for Flutter.'
  s.homepage         = 'https://github.com/instrumentisto/medea-jason'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Instrumentisto Team' => 'developer@instrumentisto.com' }

  s.source           = { :path => '.' }
  s.source_files     = 'Classes/**/*'
  s.dependency 'FlutterMacOS'

  s.vendored_libraries = 'lib/*.dylib'

  s.platform = :osx, '10.15'
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES' }
end
