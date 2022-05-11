
import 'dart:collection';


class ApiCredentials {
  late String type;
  late String data;

  Map<String, dynamic> toJson() {
    return {
      type : {data}
    };
  }
}

class ApiMember {
  late String id;
  late HashMap<String, Object> pipeline; // Endpoint
  late ApiCredentials? credentials; // Credentials
  late String? on_join; // skip null
  late String? on_leave; // skip null
  late Duration? idle_timeout; //humantime_serde
  late Duration? reconnect_timeout; //humantime_serde
  late Duration? ping_interval; //humantime_serde
}