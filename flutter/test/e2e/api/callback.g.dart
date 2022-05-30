// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'callback.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CallbackEvent _$CallbackEventFromJson(Map<String, dynamic> json) =>
    CallbackEvent(
      CallbackEvent.fromJ(json['data'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$CallbackEventToJson(CallbackEvent instance) =>
    <String, dynamic>{
      'data': CallbackEvent.toJ(instance.data),
    };

CallbackItem _$CallbackItemFromJson(Map<String, dynamic> json) => CallbackItem(
      json['fid'] as String,
      json['at'] as String,
      CallbackItem.fromJ(json['event'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$CallbackItemToJson(CallbackItem instance) =>
    <String, dynamic>{
      'fid': instance.fid,
      'at': instance.at,
      'event': CallbackEvent.toJson(instance.event),
    };

OnJoin _$OnJoinFromJson(Map<String, dynamic> json) => OnJoin();

Map<String, dynamic> _$OnJoinToJson(OnJoin instance) => <String, dynamic>{};

OnLeave _$OnLeaveFromJson(Map<String, dynamic> json) => OnLeave(
      $enumDecode(_$OnLeaveReasonEnumMap, json['reason']),
    );

Map<String, dynamic> _$OnLeaveToJson(OnLeave instance) => <String, dynamic>{
      'reason': _$OnLeaveReasonEnumMap[instance.reason],
    };

const _$OnLeaveReasonEnumMap = {
  OnLeaveReason.Disconnected: 'Disconnected',
  OnLeaveReason.LostConnection: 'LostConnection',
  OnLeaveReason.ServerShutdown: 'ServerShutdown',
  OnLeaveReason.Kicked: 'Kicked',
};
