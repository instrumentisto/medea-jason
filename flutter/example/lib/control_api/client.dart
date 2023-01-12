import 'dart:convert';

import 'http.dart';
import 'entities/callback.dart';
import 'entities/endpoint.dart';
import 'entities/member.dart';
import 'entities/room.dart';

class ControlApi {
  final HttpClient _client;

  ControlApi(this._client);

  /// Creates a room with the given id.
  Future<void> createRoom(String roomId) async {
    await _client.create(roomId, Room(roomId, {}));
  }

  /// Creates room member by id.
  Future<void> createMember(String roomId, Member member) async {
    await _client.create(roomId + '/' + member.id, member);
  }

  /// Creates [WebRtcPlayEndpoint] of member for the room.
  Future<void> createPlayEndpoint(
      String roomId, String memberId, WebRtcPlayEndpoint endpoint) async {
    await _client.create(roomId + '/' + memberId + '/' + endpoint.id, endpoint);
  }

  /// Creates [WebRtcPublishEndpoint] of member for the room.
  Future<void> createPublishEndpoint(
      String roomId, String memberId, WebRtcPublishEndpoint endpoint) async {
    await _client.create(roomId + '/' + memberId + '/' + endpoint.id, endpoint);
  }

  /// Returns url by endpointId.
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

  /// Deletes control api element.
  Future<void> delete(String roomId, String memberId, String endpointId) async {
    var url = await getUrlForElement(roomId, memberId, endpointId);

    await _client.delete(url);
  }

  /// Returns control api element.
  Future<String> get(String roomId, String memberId, String endpointId) async {
    var url = await getUrlForElement(roomId, memberId, endpointId);
    var resp = await _client.get(url);

    return resp.body;
  }

  /// Returns all control api callbacks.
  Future<List<CallbackItem>> getCallbacks() async {
    var resp = await _client.callbacks();
    List parsed = json.decode(resp.body);

    return parsed.map((item) => CallbackItem.fromJson(item)).toList();
  }
}
