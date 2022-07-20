// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'member.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Credentials _$CredentialsFromJson(Map<String, dynamic> json) => Credentials();

Map<String, dynamic> _$CredentialsToJson(Credentials instance) =>
    <String, dynamic>{};

Hash _$HashFromJson(Map<String, dynamic> json) => Hash(
      json['data'] as String,
    );

Map<String, dynamic> _$HashToJson(Hash instance) => <String, dynamic>{
      'data': instance.data,
    };

Plain _$PlainFromJson(Map<String, dynamic> json) => Plain(
      json['data'] as String,
    );

Map<String, dynamic> _$PlainToJson(Plain instance) => <String, dynamic>{
      'data': instance.data,
    };

Member _$MemberFromJson(Map<String, dynamic> json) => Member(
      json['id'] as String,
      (json['pipeline'] as Map<String, dynamic>).map(
        (k, e) => MapEntry(k, Endpoint.fromJson(e as Map<String, dynamic>)),
      ),
      json['credentials'] == null
          ? null
          : Credentials.fromJson(json['credentials'] as Map<String, dynamic>),
      json['on_join'] as String?,
      json['on_leave'] as String?,
    )
      ..idle_timeout = json['idle_timeout'] as String?
      ..reconnect_timeout = json['reconnect_timeout'] as String?
      ..ping_interval = json['ping_interval'] as String?;

Map<String, dynamic> _$MemberToJson(Member instance) {
  final val = <String, dynamic>{
    'id': instance.id,
    'pipeline': instance.pipeline,
    'credentials': instance.credentials,
  };

  void writeNotNull(String key, dynamic value) {
    if (value != null) {
      val[key] = value;
    }
  }

  writeNotNull('on_join', instance.on_join);
  writeNotNull('on_leave', instance.on_leave);
  val['idle_timeout'] = instance.idle_timeout;
  val['reconnect_timeout'] = instance.reconnect_timeout;
  val['ping_interval'] = instance.ping_interval;
  return val;
}
