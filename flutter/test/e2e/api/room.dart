

import 'dart:collection';
import 'package:json_annotation/json_annotation.dart';
import 'member.dart';
import 'dart:convert';

part 'room.g.dart';

@JsonSerializable()
class Room {
  late String id;
    @JsonKey(
      fromJson: fromPipe,
      toJson: toPipe)
  late HashMap<String, ApiMember> pipeline;

  static HashMap<String, ApiMember> fromPipe(Map<String, dynamic> pipeline) {
    var res = HashMap<String, ApiMember>();
    pipeline.forEach((key, value) => res.addAll({key:ApiMember.fromJson(value)}));
    return res;
  }
  static Map<String, Map<String, dynamic>> toPipe(Map<String, ApiMember> pipeline) {
    var gg = pipeline.map((key, value) => MapEntry(key, value.toJson()));
    return gg;
  }

  Room();

  factory Room.fromJson(Map<String, dynamic> json) {
    var gg = _$RoomFromJson(json); 
    return gg;
  }

  Map<String, dynamic> toJson() {
    var gg = _$RoomToJson(this);
    gg.addAll({'kind': 'Room'});

    return gg;
  }

}

  // //! `Room` element related methods and entities.

// use std::collections::HashMap;

// use medea_control_api_proto::grpc::api as proto;
// use serde::{Deserialize, Serialize};

// use super::member::Member;

// /// [Control API]'s `Room` representation.
// ///
// /// [Control API]: https://tinyurl.com/yxsqplq7
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Room {
//     /// ID of this [`Room`].
//     #[serde(skip_deserializing)]
//     pub id: String,

//     /// Pipeline of this [`Room`].
//     pub pipeline: HashMap<String, RoomElement>,
// }

// impl Room {
//     /// Converts [`Room`] into protobuf [`proto::Room`].
//     #[must_use]
//     pub fn into_proto(self, room_id: String) -> proto::Room {
//         proto::Room {
//             id: room_id,
//             pipeline: self
//                 .pipeline
//                 .into_iter()
//                 .map(|(id, member)| (id.clone(), member.into_proto(id)))
//                 .collect(),
//         }
//     }
// }

// /// Element of [`Room`]'s pipeline.
// #[derive(Debug, Deserialize, Serialize)]
// #[serde(tag = "kind")]
// pub enum RoomElement {
//     /// [`Member`] of the [`Room`].
//     Member(Member),
// }

// impl RoomElement {
//     /// Converts this [`RoomElement`] into a [`proto::room::Element`] with the
//     /// specified `id`.
//     #[must_use]
//     pub fn into_proto(self, id: String) -> proto::room::Element {
//         let el = match self {
//             Self::Member(m) => {
//                 proto::room::element::El::Member(m.into_proto(id))
//             }
//         };
//         proto::room::Element { el: Some(el) }
//     }
// }

// #[allow(clippy::fallible_impl_from)]
// impl From<proto::room::Element> for RoomElement {
//     fn from(proto: proto::room::Element) -> Self {
//         match proto.el.unwrap() {
//             proto::room::element::El::Member(member) => {
//                 Self::Member(member.into())
//             }
//             proto::room::element::El::WebrtcPlay(_)
//             | proto::room::element::El::WebrtcPub(_) => unimplemented!(),
//         }
//     }
// }

// impl From<proto::Room> for Room {
//     fn from(proto: proto::Room) -> Self {
//         Self {
//             id: proto.id,
//             pipeline: proto
//                 .pipeline
//                 .into_iter()
//                 .map(|(id, member)| (id, member.into()))
//                 .collect(),
//         }
//     }
// }
