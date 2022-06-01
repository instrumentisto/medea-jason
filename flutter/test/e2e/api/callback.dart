import 'package:json_annotation/json_annotation.dart';

part 'callback.g.dart';

@JsonSerializable()
class CallbackEvent {
  Map<String, dynamic> toJson() => {};
  CallbackEvent();
  factory CallbackEvent.fromJson(Map<String, dynamic> json) {
    if (json.toString().contains('OnLeave')) {
      return OnLeave.fromJson(json);
    } else {
      return OnJoin.fromJson(json);
    }
  }
}

@JsonSerializable()
class CallbackItem {
  String fid;
  String at;
  CallbackEvent event;

  CallbackItem(this.fid, this.at, this.event);

  factory CallbackItem.fromJson(Map<String, dynamic> json) =>
      _$CallbackItemFromJson(json);
  Map<String, dynamic> toJson() => _$CallbackItemToJson(this);
}

@JsonSerializable()
class OnJoin implements CallbackEvent {
  OnJoin();
  factory OnJoin.fromJson(Map<String, dynamic> json) => _$OnJoinFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$OnJoinToJson(this);
}

enum OnLeaveReason {
  Disconnected,
  LostConnection,
  ServerShutdown,
  Kicked,
}

@JsonSerializable()
class OnLeave implements CallbackEvent {
  OnLeaveReason reason;

  OnLeave(this.reason);
  factory OnLeave.fromJson(Map<String, dynamic> json) =>
      _$OnLeaveFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$OnLeaveToJson(this);
}
