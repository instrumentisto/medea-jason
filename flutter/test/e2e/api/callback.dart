import 'package:json_annotation/json_annotation.dart';

part 'callback.g.dart';

/// All callbacks which can happen.
/// `OnJoin` callback of Control API or 
/// `OnLeave` callback of Control API.
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

/// Control API callback.
@JsonSerializable()
class CallbackItem {
  /// FID (Full ID) of element with which this event was occurred.
  String fid;

  /// Time on which callback was occurred.
  String at;

  /// Event which occurred.
  CallbackEvent event;

  CallbackItem(this.fid, this.at, this.event);

  factory CallbackItem.fromJson(Map<String, dynamic> json) =>
      _$CallbackItemFromJson(json);
  Map<String, dynamic> toJson() => _$CallbackItemToJson(this);
}

/// `OnJoin` callback for Control API.
@JsonSerializable()
class OnJoin implements CallbackEvent {
  OnJoin();
  factory OnJoin.fromJson(Map<String, dynamic> json) => _$OnJoinFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$OnJoinToJson(this);
}

/// Reason of why `Member` leaves.
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

/// `OnLeave` callback of Control API.
@JsonSerializable()
class OnLeave implements CallbackEvent {
  /// Reason of why `Member` leaves.
  OnLeaveReason reason;

  OnLeave(this.reason);
  factory OnLeave.fromJson(Map<String, dynamic> json) =>
      _$OnLeaveFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$OnLeaveToJson(this);
}