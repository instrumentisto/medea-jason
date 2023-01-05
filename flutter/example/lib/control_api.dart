import 'dart:convert';

import 'package:medea_jason_example/stuff/api/endpoint.dart';
import 'package:medea_jason_example/stuff/api/member.dart';
import 'package:medea_jason_example/stuff/api/room.dart';
import 'package:medea_jason_example/stuff/control.dart';

import 'call.dart';

class ControlApi {
  final Client _client;
  ControlApi(this._client);

  Future<void> createRoom(String roomId) async {
    try {
      await _client.create(roomId, Room(roomId, {}));
    } catch (e) {
      print('Control api createRoom error: $e');
    }
  }

  Future<void> createMember(String roomId, Member member) async {
    try {
      await _client.create(roomId + '/' + member.id, member);
    } catch (e) {
      print('Control api createMember error: $e');
    }
  }

  Future<bool> createPlayEndpoint(
      String roomId, String memberId, WebRtcPlayEndpoint endpoint) async {
    try {
      await _client.create(
          roomId + '/' + memberId + '/' + endpoint.id, endpoint);
      return true;
    } catch (e) {
      print('Control api createPlayEndpoint error: $e');
      return false;
    }
  }

  Future<bool> createPublishEndpoint(
      String roomId, String memberId, WebRtcPublishEndpoint endpoint) async {
    try {
      await _client.create(
          roomId + '/' + memberId + '/' + endpoint.id, endpoint);
      return true;
    } catch (e) {
      print('Control api createPublishEndpoint error: $e');
      return false;
    }
  }

  Future<String> getUrlForElement(
      String roomId, String? memberId, String? endpointId) async {
    var url = CONTROL_API_ADDR + roomId;
    if (memberId != null && endpointId != null) {
      url = CONTROL_API_ADDR + roomId + '/' + memberId + '/' + endpointId;
    } else if (memberId != null) {
      url = CONTROL_API_ADDR + roomId + '/' + memberId;
    }
    return url;
  }

  Future<void> delete(String roomId, String memberId, String endpointId) async {
    try {
      var url = await getUrlForElement(roomId, memberId, endpointId);
      var resp = await _client.delete(url);
      return jsonDecode(resp.body);
    } catch (e) {
      print('Control api delete error: $e');
    }
  }

  Future<String> get(String roomId, String memberId, String endpointId) async {
    try {
      var url = await getUrlForElement(roomId, memberId, endpointId);
      var resp = await _client.get(url);
      return resp.body;
    } catch (e) {
      print('Control api get error: $e');
      rethrow;
    }
  }

  Future<String> getCallbacks(
      String roomId, String memberId, String endpointId) async {
    try {
      var resp = await _client.get(CONTROL_API_ADDR + '/callbacks');
      return resp.body;
    } catch (e) {
      print('Control api getCallbacks error: $e');
      rethrow;
    }
  }
}
