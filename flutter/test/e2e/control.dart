import 'package:http/http.dart' as http;
import 'dart:convert';

Map<String, String> headers = {'Content-Type': 'application/json'};

class Client {
  http.Client inner = http.Client();
  late String control_api_address;

  Client(this.control_api_address);

  Future<http.Response> create(String path, Object element) async {
    var response = await inner.post(
        Uri.parse(get_url(control_api_address, path)),
        headers: headers,
        body: json.encode(element));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  Future<http.Response> delete(String path) async {
    var response = await inner.delete(Uri.parse(get_url(control_api_address, path)));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  Future<http.Response> get(String path) async {
    var response = await inner.get(Uri.parse(get_url(control_api_address, path)));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  Future<http.Response> apply(String path, Object element) async {
    var response = await inner.put(Uri.parse(get_url(control_api_address, path)),
        headers: headers, body: json.encode(element));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  Future<http.Response> callbacks() async {
    var response =
        await inner.get(Uri.parse('$control_api_address/callbacks'));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  String get_url(String control_api_address, String path) {
    return '$control_api_address/control-api/$path';
  }
}
