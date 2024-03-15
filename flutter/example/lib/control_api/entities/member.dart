// ignore_for_file: depend_on_referenced_packages, non_constant_identifier_names

import 'package:json_annotation/json_annotation.dart';

import 'endpoint.dart';

part 'member.g.dart';

/// Credentials of a [Member].
///
/// May be either [Hash] or [Plain].
@JsonSerializable()
class Credentials {
  Map<String, dynamic> toJson() => {};

  Credentials();

  factory Credentials.fromJson(Map<String, dynamic> json) {
    if (json.toString().contains('Hash')) {
      return Hash.fromJson(json);
    } else {
      return Plain.fromJson(json);
    }
  }
}

/// [Argon2] hash of the [Member] credentials.
///
/// [Argon2]: https://en.wikipedia.org/wiki/Argon2
@JsonSerializable()
class Hash implements Credentials {
  /// Hash of the credentials.
  String data;

  Hash(this.data);

  factory Hash.fromJson(Map<String, dynamic> json) => Hash(json['hash']);

  @override
  Map<String, dynamic> toJson() {
    return {'hash': data};
  }
}

/// Plain text [Member] credentials.
@JsonSerializable()
class Plain implements Credentials {
  /// Credentials itself as a plain text.
  String data;

  Plain(this.data);

  factory Plain.fromJson(Map<String, dynamic> json) => Plain(json['plain']);

  @override
  Map<String, dynamic> toJson() {
    return {'plain': data};
  }
}

/// Entity representing a [Control API] [Member].
///
/// [Control API]: https://tinyurl.com/yxsqplq7
@JsonSerializable()
class Member {
  /// ID of this [Member].
  String id;

  /// [Control API] pipeline of this [Member].
  ///
  /// [Control API]: https://tinyurl.com/yxsqplq7
  Map<String, Endpoint> pipeline;

  /// Optional credentials of this [Member].
  ///
  /// If `null` then random credentials will be generated on a media server
  /// side.
  Credentials? credentials;

  /// URL to which [OnJoin] Control API callback will be sent.
  @JsonKey(includeIfNull: false)
  String? on_join;

  /// URL to which [OnLeave] Control API callback will be sent.
  @JsonKey(includeIfNull: false)
  String? on_leave;

  /// URL to which [OnStart] Control API callback will be sent.
  @JsonKey(includeIfNull: false)
  String? on_start;

  /// URL to which [OnStop] Control API callback will be sent.
  @JsonKey(includeIfNull: false)
  String? on_stop;

  /// Timeout of receiving heartbeat messages from this [Member] via Client API.
  ///
  /// Once reached, this [Member] is considered being idle.
  String? idle_timeout;

  /// Timeout of this [Member] reconnecting via Client API.
  ///
  /// Once reached, this [Member] is considered disconnected.
  String? reconnect_timeout;

  /// Interval of sending pings from a media server to this [Member] via Client
  /// API.
  String? ping_interval;

  Member(this.id, this.pipeline, this.credentials, this.on_join, this.on_leave, this.on_start, this.on_stop);

  factory Member.fromJson(Map<String, dynamic> json) {
    json.remove('kind');
    return _$MemberFromJson(json);
  }

  Map<String, dynamic> toJson() {
    var res = _$MemberToJson(this);
    res.addAll({'kind': 'Member'});
    return res;
  }
}
