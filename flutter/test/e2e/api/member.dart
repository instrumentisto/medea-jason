
import 'dart:collection';

import 'endpoint.dart';
import 'package:json_annotation/json_annotation.dart';
part 'member.g.dart';

@JsonSerializable()
class Credentials {
  Map<String, dynamic> toJson() => {};
  Credentials();
  factory Credentials.fromJson(Map<String, dynamic> json) {
    if (json.toString().contains('Hash')) {
      return Hash.fromJson(json) as Credentials;
    } else {
      return Plain.fromJson(json) as Credentials;
    }
  }
}

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



@JsonSerializable()
class Member {
  String id;
  Map<String, Endpoint> pipeline;
  Credentials? credentials;

  @JsonKey(includeIfNull: false)
  String? on_join;

  @JsonKey(includeIfNull: false)
  String? on_leave;

  String? idle_timeout; //humantime_serde

  String? reconnect_timeout; //humantime_serde

  String? ping_interval; //humantime_serde

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