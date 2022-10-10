import 'dart:convert';

import 'package:http/http.dart' as http;

Map<String, String> headers = {'Content-Type': 'application/json'};

/// Client of a Control API.
class Client {
  http.Client inner = http.Client();
  late String control_api_address;

  /// Returns a new Control API [Client].
  Client(this.control_api_address);

  /// Creates the provided media `Element` in the provided `path` on a media
  /// server.
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

  /// Deletes a media `Element` identified by the provided [path].
  Future<http.Response> delete(String path) async {
    var response =
        await inner.delete(Uri.parse(get_url(control_api_address, path)));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  /// Returns a media `Element` identified by the provided [path].
  Future<http.Response> get(String path) async {
    var response =
        await inner.get(Uri.parse(get_url(control_api_address, path)));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  /// Applies on a media server the provided media `Element` identified by the
  /// provided [path].
  Future<http.Response> apply(String path, Object element) async {
    var response = await inner.put(
        Uri.parse(get_url(control_api_address, path)),
        headers: headers,
        body: json.encode(element));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  /// Fetches all callbacks received by Control API mock server.
  Future<http.Response> callbacks() async {
    var response = await inner.get(Uri.parse('$control_api_address/callbacks'));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  /// Returns URL of a media element identified by the provided [path].
  String get_url(String control_api_address, String path) {
    return '$control_api_address/control-api/$path';
  }
}
