

import 'package:medea_jason/medea_jason.dart';
import 'package:json_annotation/json_annotation.dart';

part 'callback.g.dart';

@JsonSerializable()
class CallbackEvent {

      @JsonKey(
      toJson: toJ,
      fromJson: fromJ)
  late dynamic data;

  static Map<String, dynamic> toJ(dynamic data) {
    var type = data.runtimeType.toString();
    var res = <String, dynamic>{};
    res.addAll({'type' : type});
    res.addAll(data.toJson());
    return res;
  }

  CallbackEvent(this.data);

  static dynamic fromJ(Map<String, dynamic> json) {
    var type = json['type']!;
    json.remove('type');
    var res = null;
    if (type == 'OnJoin') {
      res = OnJoin.fromJson(json);
    }
    else {
      res = OnLeave.fromJson(json);
    }
    return res;
  }

  factory CallbackEvent.fromJson(Map<String, dynamic> json) {
   return CallbackEvent(fromJ(json));
  }


  Map<String, dynamic> toJson() => toJ(data);
}


@JsonSerializable()
class CallbackItem {
  late String fid;
  late String at;
      @JsonKey(
      toJson: toJ,
      fromJson: fromJ)
  late CallbackEvent event;
  CallbackItem();

    static Map<String, dynamic> toJ(dynamic data) {
    return data.toJson();
  }


  static dynamic fromJ(Map<String, dynamic> json) {
    return CallbackEvent.fromJson(json);
  }

  factory CallbackItem.fromJson(Map<String, dynamic> json) => _$CallbackItemFromJson(json);
  Map<String, dynamic> toJson() => _$CallbackItemToJson(this);
}


@JsonSerializable()
class OnJoin {
  OnJoin();
  factory OnJoin.fromJson(Map<String, dynamic> json) => _$OnJoinFromJson(json);
  Map<String, dynamic> toJson() => _$OnJoinToJson(this);
}



enum OnLeaveReason {
  /// `Member` was normally disconnected.
  Disconnected,

  /// Connection with `Member` was lost.
  LostConnection,

  /// Server is shutting down.
  ServerShutdown,

  /// `Member` was forcibly disconnected by server.
  Kicked,
}

@JsonSerializable()
class OnLeave {
  late OnLeaveReason reason;
  OnLeave();

  factory OnLeave.fromJson(Map<String, dynamic> json) => _$OnLeaveFromJson(json);
  Map<String, dynamic> toJson() => _$OnLeaveToJson(this);
}

// pub enum CallbackEvent {
//     /// `OnJoin` callback of Control API.
//     OnJoin(join::OnJoin),

//     /// `OnLeave` callback of Control API.
//     OnLeave(leave::OnLeave),
// }


// //! Control API Callback service implementation.

// pub mod server;

// use medea_control_api_proto::grpc::callback as proto;
// use serde::{Deserialize, Serialize};

// /// All callbacks which can happen.
// #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
// #[serde(tag = "type")]
// pub enum CallbackEvent {
//     /// `OnJoin` callback of Control API.
//     OnJoin(join::OnJoin),

//     /// `OnLeave` callback of Control API.
//     OnLeave(leave::OnLeave),
// }

// impl From<proto::request::Event> for CallbackEvent {
//     fn from(proto: proto::request::Event) -> Self {
//         match proto {
//             proto::request::Event::OnLeave(on_leave) => {
//                 Self::OnLeave(on_leave.into())
//             }
//             proto::request::Event::OnJoin(on_join) => {
//                 Self::OnJoin(on_join.into())
//             }
//         }
//     }
// }

// /// Control API callback.
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct CallbackItem {
//     /// FID (Full ID) of element with which this event was occurred.
//     pub fid: String,

//     /// Event which occurred.
//     pub event: CallbackEvent,

//     /// Time on which callback was occurred.
//     pub at: String,
// }

// #[allow(clippy::fallible_impl_from)]
// impl From<proto::Request> for CallbackItem {
//     fn from(proto: proto::Request) -> Self {
//         Self {
//             fid: proto.fid,
//             at: proto.at,
//             event: proto.event.unwrap().into(),
//         }
//     }
// }

// /// `on_join` callback's related entities and implementations.
// mod join {
//     use medea_control_api_proto::grpc::callback as proto;
//     use serde::{Deserialize, Serialize};

//     /// `OnJoin` callback for Control API.
//     #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
//     pub struct OnJoin;

//     impl From<proto::OnJoin> for OnJoin {
//         fn from(_: proto::OnJoin) -> Self {
//             Self
//         }
//     }
// }

// /// `on_leave` callback's related entities and implementations.
// mod leave {
//     use derive_more::Display;
//     use medea_control_api_proto::grpc::callback as proto;
//     use serde::{Deserialize, Serialize};

//     /// `OnLeave` callback of Control API.
//     #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
//     pub struct OnLeave {
//         /// Reason of why `Member` leaves.
//         pub reason: OnLeaveReason,
//     }

//     impl From<proto::OnLeave> for OnLeave {
//         fn from(proto: proto::OnLeave) -> Self {
//             Self {
//                 reason: proto::on_leave::Reason::from_i32(proto.reason)
//                     .unwrap_or_default()
//                     .into(),
//             }
//         }
//     }

//     /// Reason of why `Member` leaves.
//     #[derive(Clone, Copy, Debug, Deserialize, Display, Serialize)]
//     pub enum OnLeaveReason {
//         /// `Member` was normally disconnected.
//         Disconnected,

//         /// Connection with `Member` was lost.
//         LostConnection,

//         /// Server is shutting down.
//         ServerShutdown,

//         /// `Member` was forcibly disconnected by server.
//         Kicked,
//     }

//     impl From<proto::on_leave::Reason> for OnLeaveReason {
//         fn from(proto: proto::on_leave::Reason) -> Self {
//             use proto::on_leave::Reason as R;

//             match proto {
//                 R::ServerShutdown => Self::ServerShutdown,
//                 R::LostConnection => Self::LostConnection,
//                 R::Disconnected => Self::Disconnected,
//                 R::Kicked => Self::Kicked,
//             }
//         }
//     }
// }
