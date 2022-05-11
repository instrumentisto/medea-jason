import 'package:http/http.dart' as http;
import 'dart:convert';


class MyClient {
  
  late http.Client inner;
  late String control_api_address;

  MyClient(String control_api_address) {
    this.control_api_address = control_api_address;
    this.inner = http.Client();
  }

  Future<http.Response> create(String path, Object element) async {
    var response = await inner.post(
      Uri.parse('$control_api_address/$path'),
      body: json.encode(element));
    return response;
  }

  Future<http.Response> delete(String path) async {
    var response = await inner.delete(
      Uri.parse('$control_api_address/$path'));
    return response;
  }

  Future<http.Response> get(String path) async {
    var response = await inner.get(
      Uri.parse('$control_api_address/$path'));
    return response;
  }

  Future<http.Response> apply(String path, Object element) async {
    var response = await inner.put(
      Uri.parse('$control_api_address/$path'), body: json.encode(element));
    return response;
  }

  Future<http.Response> callbacks() async {
    var response = await inner.get(
      Uri.parse('$control_api_address/$callbacks'));
    return response;
  }

  String get_url(String path) {
    return '$control_api_address/control-api/$path';
  }
}