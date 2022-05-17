// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'room.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Room _$RoomFromJson(Map<String, dynamic> json) => Room()
  ..id = json['id'] as String
  ..pipeline = Room.fromPipe(json['pipeline'] as Map<String, dynamic>);

Map<String, dynamic> _$RoomToJson(Room instance) => <String, dynamic>{
      'id': instance.id,
      'pipeline': Room.toPipe(instance.pipeline),
    };
