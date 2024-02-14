// ignore_for_file: depend_on_referenced_packages

import 'package:json_annotation/json_annotation.dart';

import 'member.dart';

part 'room.g.dart';

/// [Control API]'s [Room] representation.
///
/// [Control API]: https://tinyurl.com/yxsqplq7
@JsonSerializable()
class Room {
  /// ID of this [Room].
  String id;

  /// Pipeline of this [Room].
  Map<String, Member> pipeline;

  Room(this.id, this.pipeline);

  factory Room.fromJson(Map<String, dynamic> json) {
    json.remove('kind');
    return _$RoomFromJson(json);
  }

  Map<String, dynamic> toJson() {
    var res = _$RoomToJson(this);
    res.addAll({'kind': 'Room'});
    return res;
  }
}
