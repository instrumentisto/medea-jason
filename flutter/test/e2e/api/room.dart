

import 'dart:collection';
import 'package:json_annotation/json_annotation.dart';
import 'member.dart';
part 'room.g.dart';

@JsonSerializable()
class Room {
  String id;

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