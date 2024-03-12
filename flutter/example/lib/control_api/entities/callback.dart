// ignore_for_file: constant_identifier_names, depend_on_referenced_packages

import 'package:json_annotation/json_annotation.dart';

part 'callback.g.dart';

/// Possible Control API callbacks events which may happen on a media server.
///
/// May be either [OnJoin] or [OnLeave].
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

/// Event notifying about a [Member] joining a [Room].
@JsonSerializable()
class OnJoin implements CallbackEvent {
  OnJoin();
  factory OnJoin.fromJson(Map<String, dynamic> json) => _$OnJoinFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$OnJoinToJson(this);
}

/// Reason of why a [Member] leaves its [Room].
enum OnLeaveReason {
  /// [Member] was disconnected normally.
  Disconnected,

  /// Connection with the [Member] was lost.
  Lost,

  /// [Member[ was forcibly disconnected by a media server.
  Kicked,

  /// Media server was shut down.
  Shutdown,
}

/// Event notifying about a [Member] leaving its [Room].
@JsonSerializable()
class OnLeave implements CallbackEvent {
  /// Reason of why the [Member] leaves.
  OnLeaveReason reason;

  OnLeave(this.reason);
  factory OnLeave.fromJson(Map<String, dynamic> json) =>
      _$OnLeaveFromJson(json);

  @override
  Map<String, dynamic> toJson() => _$OnLeaveToJson(this);
}
