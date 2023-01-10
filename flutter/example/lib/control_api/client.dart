import 'dart:convert';

import 'http.dart';
import 'entities/callback.dart';
import 'entities/endpoint.dart';
import 'entities/member.dart';
import 'entities/room.dart';

// TODO: add docs
class ControlApi {
  final HttpClient _client;

  ControlApi(this._client);

  Future<void> createRoom(String roomId) async {
    await _client.create(roomId, Room(roomId, {}));
  }

  Future<void> createMember(String roomId, Member member) async {
    await _client.create(roomId + '/' + member.id, member);
  }

  Future<void> createPlayEndpoint(
      String roomId, String memberId, WebRtcPlayEndpoint endpoint) async {
    await _client.create(roomId + '/' + memberId + '/' + endpoint.id, endpoint);
  }

  Future<void> createPublishEndpoint(
      String roomId, String memberId, WebRtcPublishEndpoint endpoint) async {
    await _client.create(roomId + '/' + memberId + '/' + endpoint.id, endpoint);
  }

  Future<String> getUrlForElement(
      String roomId, String? memberId, String? endpointId) async {
    var url = roomId;

    if (memberId != null && endpointId != null) {
      url = roomId + '/' + memberId + '/' + endpointId;
    } else if (memberId != null) {
      url = roomId + '/' + memberId;
    }

    return url;
  }

  Future<void> delete(String roomId, String memberId, String endpointId) async {
    var url = await getUrlForElement(roomId, memberId, endpointId);

    await _client.delete(url);
  }

  Future<String> get(String roomId, String memberId, String endpointId) async {
    var url = await getUrlForElement(roomId, memberId, endpointId);
    var resp = await _client.get(url);

    return resp.body;
  }

  Future<List<CallbackItem>> getCallbacks() async {
    var resp = await _client.callbacks();
    List parsed = json.decode(resp.body);

    return parsed.map((item) => CallbackItem.fromJson(item)).toList();
  }
}
