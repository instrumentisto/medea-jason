

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
    var res;
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

  Map<String, dynamic> _toJson() => toJ(data);
  static Map<String, dynamic> toJson(CallbackEvent value) => value._toJson();
}


@JsonSerializable()
class CallbackItem {
  String fid;
  String at;
      @JsonKey(
      toJson: CallbackEvent.toJson,
      fromJson: fromJ)
  CallbackEvent event;
  CallbackItem(this.fid, this.at, this.event);

    static Map<String, dynamic> toJ(dynamic data) {
    return data.toJson();
  }

  static dynamic fromJ(Map<String, dynamic> json) {
    return CallbackEvent.fromJson(json);
  }

  factory CallbackItem.fromJson(Map<String, dynamic> json) => _$CallbackItemFromJson(json);
  Map<String, dynamic> _toJson() => _$CallbackItemToJson(this);
  static Map<String, dynamic> toJson(CallbackItem value) => value._toJson();

}


@JsonSerializable()
class OnJoin {
  OnJoin();
  factory OnJoin.fromJson(Map<String, dynamic> json) => _$OnJoinFromJson(json);
  Map<String, dynamic> toJson() => _$OnJoinToJson(this);
}



enum OnLeaveReason {
  Disconnected,
  LostConnection,
  ServerShutdown,
  Kicked,
}

@JsonSerializable()
class OnLeave {
  OnLeaveReason reason;
  OnLeave(this.reason);

  factory OnLeave.fromJson(Map<String, dynamic> json) => _$OnLeaveFromJson(json);
  Map<String, dynamic> toJson() => _$OnLeaveToJson(this);
}