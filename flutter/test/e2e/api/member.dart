
import 'endpoint.dart';
import 'package:json_annotation/json_annotation.dart';
part 'member.g.dart';

/// Credentials of the [Member].
/// [Hash] or [Plain].
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
  String data;

  Hash(this.data);
  factory Hash.fromJson(Map<String, dynamic> json) => Hash(json['hash']);

  @override
  Map<String, dynamic> toJson() {
    return {'hash': data};
  }
}

/// Plain text [`Member`] credentials.
@JsonSerializable()
class Plain implements Credentials {
  String data;

  Plain(this.data);
  factory Plain.fromJson(Map<String, dynamic> json) => Plain(json['plain']);

  @override
  Map<String, dynamic> toJson() {
    return {'plain': data};
  }
}


/// Entity that represents a [Control API] [Member].
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
  /// If [`None`] then random credentials will be generated on Medea side.
  Credentials? credentials;

  /// URL to which `OnJoin` Control API callback will be sent.
  @JsonKey(includeIfNull: false)
  String? on_join;

  /// URL to which `OnLeave` Control API callback will be sent.
  @JsonKey(includeIfNull: false)
  String? on_leave;

  /// Timeout of receiving heartbeat messages from this [`Member`] via Client
  /// API. Once reached, the [`Member`] is considered being idle.
  String? idle_timeout; 

  /// Timeout of this [`Member`] reconnecting via Client API.
  /// Once reached, the [`Member`] is considered disconnected.
  String? reconnect_timeout; 

  /// Interval of sending pings from Medea to this [`Member`] via Client API.
  String? ping_interval;

  Member(this.id, this.pipeline, this.credentials, this.on_join, this.on_leave);

  factory Member.fromJson(Map<String, dynamic> json) {
    json.remove('kind');
    return _$MemberFromJson(json);
    }

  Map<String, dynamic> toJson() {
    var res = _$MemberToJson(this);
    res.addAll({'kind':'Member'});
    return res;
    }
}