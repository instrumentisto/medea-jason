// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'endpoint.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Endpoint _$EndpointFromJson(Map<String, dynamic> json) => Endpoint();

Map<String, dynamic> _$EndpointToJson(Endpoint instance) => <String, dynamic>{};

WebRtcPlayEndpoint _$WebRtcPlayEndpointFromJson(Map<String, dynamic> json) =>
    WebRtcPlayEndpoint(
      json['id'] as String,
      json['src'] as String,
    )..force_relay = json['force_relay'] as bool;

Map<String, dynamic> _$WebRtcPlayEndpointToJson(WebRtcPlayEndpoint instance) =>
    <String, dynamic>{
      'id': instance.id,
      'src': instance.src,
      'force_relay': instance.force_relay,
    };

WebRtcPublishEndpoint _$WebRtcPublishEndpointFromJson(
        Map<String, dynamic> json) =>
    WebRtcPublishEndpoint(
      json['id'] as String,
      $enumDecode(_$P2pModeEnumMap, json['p2p']),
    )
      ..force_relay = json['force_relay'] as bool
      ..audio_settings =
          AudioSettings.fromJson(json['audio_settings'] as Map<String, dynamic>)
      ..video_settings = VideoSettings.fromJson(
          json['video_settings'] as Map<String, dynamic>);

Map<String, dynamic> _$WebRtcPublishEndpointToJson(
        WebRtcPublishEndpoint instance) =>
    <String, dynamic>{
      'id': instance.id,
      'p2p': _$P2pModeEnumMap[instance.p2p],
      'force_relay': instance.force_relay,
      'audio_settings': instance.audio_settings,
      'video_settings': instance.video_settings,
    };

const _$P2pModeEnumMap = {
  P2pMode.Always: 'Always',
  P2pMode.Never: 'Never',
  P2pMode.IfPossible: 'IfPossible',
};

AudioSettings _$AudioSettingsFromJson(Map<String, dynamic> json) =>
    AudioSettings(
      $enumDecode(_$PublishPolicyEnumMap, json['publish_policy']),
    );

Map<String, dynamic> _$AudioSettingsToJson(AudioSettings instance) =>
    <String, dynamic>{
      'publish_policy': _$PublishPolicyEnumMap[instance.publish_policy],
    };

const _$PublishPolicyEnumMap = {
  PublishPolicy.Optional: 'Optional',
  PublishPolicy.Required: 'Required',
  PublishPolicy.Disabled: 'Disabled',
};

VideoSettings _$VideoSettingsFromJson(Map<String, dynamic> json) =>
    VideoSettings(
      $enumDecode(_$PublishPolicyEnumMap, json['publish_policy']),
    );

Map<String, dynamic> _$VideoSettingsToJson(VideoSettings instance) =>
    <String, dynamic>{
      'publish_policy': _$PublishPolicyEnumMap[instance.publish_policy],
    };
