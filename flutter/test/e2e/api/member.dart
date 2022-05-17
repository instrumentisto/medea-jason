
import 'dart:collection';

import 'endpoint.dart';
import 'package:json_annotation/json_annotation.dart';

part 'member.g.dart';

class ApiCredentials {
  late String type;
  late String data;

  Map<String, dynamic> toJson() {
    return {
      type : {data}
    };
  }
}

@JsonSerializable()
class ApiMember {
  late String id;

      @JsonKey(
      toJson: toJ,
      fromJson: fromJ)
  late Map<String, Endpoint> pipeline;

  Map<String, dynamic>? credentials; // ApiCredentials

  @JsonKey(includeIfNull: false)
  String? on_join; // skip null

  @JsonKey(includeIfNull: false)
  String? on_leave; // skip null

  @JsonKey(ignore: true)
  Duration? idle_timeout; //humantime_serde

  @JsonKey(ignore: true)
  Duration? reconnect_timeout; //humantime_serde
  
  @JsonKey(ignore: true)
  Duration? ping_interval; //humantime_serde

  ApiMember();

  factory ApiMember.fromJson(Map<String, dynamic> json) => _$ApiMemberFromJson(json);

  static Map<String, dynamic> toJ(Map<String, Endpoint> pipeline) {
    var res = HashMap<String, dynamic>();
    pipeline.forEach((key, value) {
      if (value.data is WebRtcPlayEndpoint || value.data is WebRtcPublishEndpoint) {
        res.addAll({key: value.toJson()});
      }
      else {
        throw 'ERrr';
      }
    });
    return res;
  }
  static Map<String, Endpoint> fromJ(Map<String, dynamic> json) {
    var res = HashMap<String, Endpoint>();
    json.forEach((key, value) {
      var ep = Endpoint();
      try {
        ep.data = WebRtcPublishEndpoint.fromJson(value);
        res.addAll({key: ep});
      }
      catch (e) {
        ep.data = WebRtcPlayEndpoint.fromJson(value);
        res.addAll({key: ep});
      }
    });
    return res;
  }

  Map<String, dynamic> toJson() {
    var res = _$ApiMemberToJson(this);
    res.addAll({'kind':'Member'});
    return res;
    }
}