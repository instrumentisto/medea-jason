// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'room.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Room _$RoomFromJson(Map<String, dynamic> json) => Room(
  json['id'] as String,
  (json['pipeline'] as Map<String, dynamic>).map(
    (k, e) => MapEntry(k, Member.fromJson(e as Map<String, dynamic>)),
  ),
);

Map<String, dynamic> _$RoomToJson(Room instance) => <String, dynamic>{
  'id': instance.id,
  'pipeline': instance.pipeline,
};
