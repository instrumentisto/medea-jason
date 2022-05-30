// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'member.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ApiMember _$ApiMemberFromJson(Map<String, dynamic> json) => ApiMember()
  ..id = json['id'] as String
  ..pipeline = ApiMember.fromJ(json['pipeline'] as Map<String, dynamic>)
  ..credentials = json['credentials'] as Map<String, dynamic>?
  ..on_join = json['on_join'] as String?
  ..on_leave = json['on_leave'] as String?
  ..idle_timeout = json['idle_timeout'] as String?
  ..reconnect_timeout = json['reconnect_timeout'] as String?
  ..ping_interval = json['ping_interval'] as String?;

Map<String, dynamic> _$ApiMemberToJson(ApiMember instance) {
  final val = <String, dynamic>{
    'id': instance.id,
    'pipeline': ApiMember.toJ(instance.pipeline),
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
