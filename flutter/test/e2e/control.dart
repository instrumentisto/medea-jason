import 'package:http/http.dart' as http;
import 'dart:convert';

Map<String,String> headers = {'Content-Type':'application/json'};
class MyClient {
  
  late http.Client inner;
  late String control_api_address;

  MyClient(String control_api_address) {
    this.control_api_address = control_api_address+'/control-api';
    this.inner = http.Client();
  }

  Future<http.Response> create(String path, Object element) async {
    var response = await inner.post(
      Uri.parse('$control_api_address/$path'),
      headers: headers,
      body: json.encode(element));
        if(response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  Future<http.Response> delete(String path) async {
    var response = await inner.delete(
      Uri.parse('$control_api_address/$path'));
    if(response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  Future<http.Response> get(String path) async {
    var response = await inner.get(
      Uri.parse('$control_api_address/$path'));
        if(response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  Future<http.Response> apply(String path, Object element) async {
    var response = await inner.put(
      Uri.parse('$control_api_address/$path'), headers: headers, body: json.encode(element));
    if(response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  Future<http.Response> callbacks() async {
    var response = await inner.get(
      Uri.parse('http://127.0.0.1:8000/callbacks'));
        if(response.statusCode != 200) {
      throw response.body;
    }
    return response;
  }

  String get_url(String path) {
    return '$control_api_address/$path';
  }
}