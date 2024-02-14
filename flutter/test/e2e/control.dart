import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:retry/retry.dart';

Map<String, String> headers = {'Content-Type': 'application/json'};

/// Client of a Control API.
class Client {
  http.Client inner = http.Client();
  late String controlApiAddress;

  /// Returns a new Control API [Client].
  Client(this.controlApiAddress);

  /// Creates the provided media `Element` in the provided [path] on a media
  /// server.
  Future<http.Response> create(String path, Object element) async {
    var response = await retry(() => inner.post(
        Uri.parse(getUrl(controlApiAddress, path)),
        headers: headers,
        body: json.encode(element)));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  /// Deletes a media `Element` identified by the provided [path].
  Future<http.Response> delete(String path) async {
    var response = await retry(
        () => inner.delete(Uri.parse(getUrl(controlApiAddress, path))));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  /// Returns a media `Element` identified by the provided [path].
  Future<http.Response> get(String path) async {
    var response = await retry(
        () => inner.get(Uri.parse(getUrl(controlApiAddress, path))));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  /// Applies on a media server the provided media `Element` identified by the
  /// provided [path].
  Future<http.Response> apply(String path, Object element) async {
    var response = await retry(() => inner.put(
        Uri.parse(getUrl(controlApiAddress, path)),
        headers: headers,
        body: json.encode(element)));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  /// Fetches all callbacks received by Control API mock server.
  Future<http.Response> callbacks() async {
    var response =
        await retry(() => inner.get(Uri.parse('$controlApiAddress/callbacks')));
    if (response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  /// Returns URL of a media element identified by the provided [path].
  String getUrl(String controlApiAddress, String path) {
    return '$controlApiAddress/control-api/$path';
  }
}
