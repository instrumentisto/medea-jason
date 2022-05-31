import 'dart:async';
import 'dart:collection';
import 'package:medea_jason/medea_jason.dart';
import 'package:tuple/tuple.dart';
import '../conf.dart';

class MemberBuilder {
  String id;
  bool is_send;
  bool is_recv;
  MemberBuilder(this.id, this.is_send, this.is_recv);

  Member build(
      RoomHandle room,
      HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> send_state,
      HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> recv_state) {
    room.onFailedLocalMedia((p0) {});
    room.onConnectionLoss((p0) {});
    return Member(id, is_send, is_recv, false, send_state, recv_state, room);
  }
}

class ConnectionStore {
  var close_connect = HashMap<String, Completer>();
  var connects = HashMap<String, ConnectionHandle>();
  var stopped_tracks = HashMap<String, int>();
  var callback_counter = HashMap<String, Map<String, int>>();
  var remote_tracks =
      HashMap<String, HashMap<String, List<RemoteMediaTrack>>>();
  List<LocalMediaTrack> local_tracks = List.empty(growable: true);

  var OnMediaDirectionChanged =
      HashMap<String, Function(TrackMediaDirection)>();
  var OnCallbackCounter = HashMap<String, Map<String, Function(int)>>();
  var OnConnect = HashMap<String, Function(ConnectionHandle)>();
  var OnRemoteTrack = HashMap<String, Function(RemoteMediaTrack)>();
  Function(LocalMediaTrack) OnLocalTrack = (_) {};
}

class Member {
  String id;
  bool is_send;
  bool is_recv;
  bool is_joined;
  HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> send_state;
  HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> recv_state;

  Completer<RoomCloseReason> close_reason = Completer();
  RoomHandle room;
  var connection_store = ConnectionStore();

  Member(this.id, this.is_send, this.is_recv, this.is_joined, this.send_state,
      this.recv_state, this.room) {
    room.onClose((reason) {
      close_reason.complete(reason);
    });
    room.onLocalTrack((local_track) {
      connection_store.local_tracks.add(local_track);
      connection_store.OnLocalTrack(local_track);
    });
    room.onNewConnection((connection) {
      var remote_member_id = connection.getRemoteMemberId();
      connection_store.remote_tracks.addAll({remote_member_id: HashMap()});
      connection_store.connects.addAll({remote_member_id: connection});
      connection_store.close_connect.addAll({remote_member_id: Completer()});

      connection.onRemoteTrackAdded((remote_track) {
        var remote_track_id = remote_track.getTrack().id();

        if (connection_store
                .remote_tracks[remote_member_id]![remote_track_id] ==
            null) {
          connection_store.remote_tracks[remote_member_id]![remote_track_id] =
              List.empty(growable: true);
        }
        connection_store.remote_tracks[remote_member_id]![remote_track_id]!
            .add(remote_track);

        connection_store.callback_counter.addAll({
          remote_track_id: {
            'enabled': 0,
            'disabled': 0,
            'muted': 0,
            'unmuted': 0
          }
        });
        connection_store.OnCallbackCounter.addAll({
          remote_track_id: {
            'enabled': (_) => {},
            'disabled': (_) => {},
            'muted': (_) => {},
            'unmuted': (_) => {}
          }
        });

        remote_track.onMuted(() {
          connection_store.callback_counter[remote_track_id]!
              .update('muted', (value) => value += 1);
          connection_store.OnCallbackCounter[remote_track_id]!['muted']!(
              connection_store.callback_counter[remote_track_id]!['muted']!);
        });

        remote_track.onUnmuted(() {
          connection_store.callback_counter[remote_track_id]!
              .update('unmuted', (value) => value += 1);
          connection_store.OnCallbackCounter[remote_track_id]!['unmuted']!(
              connection_store.callback_counter[remote_track_id]!['unmuted']!);
        });

        remote_track.onMediaDirectionChanged((direction) {
          if (direction != TrackMediaDirection.SendRecv) {
            connection_store.callback_counter[remote_track_id]!
                .update('disabled', (value) => value += 1);

            connection_store.OnCallbackCounter[remote_track_id]!['disabled']!(
                connection_store
                    .callback_counter[remote_track_id]!['disabled']!);
          } else {
            connection_store.callback_counter[remote_track_id]!
                .update('enabled', (value) => value += 1);
            connection_store.OnCallbackCounter[remote_track_id]!['enabled']!(
                connection_store
                    .callback_counter[remote_track_id]!['enabled']!);
          }
          connection_store.OnMediaDirectionChanged.forEach((key, value) {
            value(direction);
          });
        });

        connection_store.stopped_tracks[remote_track_id] = 0;
        remote_track.onStopped(() {
          connection_store.stopped_tracks
              .update(remote_track_id, (value) => value + 1);
        });

        if (connection_store.OnRemoteTrack[remote_member_id] != null) {
          connection_store.OnRemoteTrack[remote_member_id]!(remote_track);
        }
      });

      connection.onClose(() {
        connection_store.close_connect[remote_member_id]!.complete();
      });
      if (connection_store.OnConnect[remote_member_id] != null) {
        connection_store.OnConnect[remote_member_id]!(connection);
      }
    });
  }

  Future<void> forget_local_tracks() async {
    connection_store.local_tracks.forEach((track) {
      track.free();
    });
  }

  Future<void> wait_for_connect(String id) async {
    if (!connection_store.connects.containsKey(id)) {
      var conn = Completer();
      connection_store.OnConnect[id] = (_) {
        conn.complete();
        connection_store.OnConnect[id] = (_) {};
      };
      return conn.future;
    }
  }

  Future<void> wait_for_track_count(String id, int count) async {
    if (connection_store.remote_tracks[id]!.length != count) {
      var track_compl = Completer();
      connection_store.OnRemoteTrack[id] = (_) {
        if (connection_store.remote_tracks[id]!.length >= count) {
          track_compl.complete();
          connection_store.OnRemoteTrack.remove(id);
        }
      };
      return track_compl.future;
    }
  }

  Future<RemoteMediaTrack> wait_remote_track_from(
      String id, MediaSourceKind? source, MediaKind? kind) async {
    bool source_check(MediaSourceKind a, MediaSourceKind? b) {
      if (b == null) {
        return true;
      }
      return a == b;
    }

    bool kind_check(MediaKind a, MediaKind? b) {
      if (b == null) {
        return true;
      }
      return a == b;
    }

    if (connection_store.remote_tracks[id]!.values.any((element) =>
        source_check(element.last.mediaSourceKind(), source) &&
        kind_check(element.last.kind(), kind))) {
      return connection_store.remote_tracks[id]!.values
          .lastWhere((element) =>
              source_check(element.last.mediaSourceKind(), source) &&
              kind_check(element.last.kind(), kind))
          .last;
    } else {
      var track_compl = Completer<RemoteMediaTrack>();
      connection_store.OnRemoteTrack[id] = (track) {
        if (source_check(track.mediaSourceKind(), source) &&
            kind_check(track.kind(), kind)) {
          track_compl.complete(track);
          connection_store.OnRemoteTrack.remove(id);
        }
      };
      return track_compl.future;
    }
  }

  Future<LocalMediaTrack> wait_local_track(
      MediaSourceKind source, MediaKind kind) async {
    if (connection_store.local_tracks.any((element) =>
        element.kind() == kind && element.mediaSourceKind() == source)) {
      return connection_store.local_tracks.firstWhere((element) =>
          element.kind() == kind && element.mediaSourceKind() == source);
    } else {
      var track_compl = Completer<LocalMediaTrack>();
      connection_store.OnLocalTrack = (track) {
        if (track.kind() == kind && track.mediaSourceKind() == source) {
          track_compl.complete(track);
          connection_store.OnLocalTrack = (_) {};
        }
      };
      return track_compl.future;
    }
  }

  Future<void> wait_for_close(String id) {
    return connection_store.close_connect[id]!.future;
  }

  Future<void> join_room(String room_id) async {
    await room.join('$CLIENT_API_ADDR/$room_id/$id?token=test');
    is_joined = true;
  }

  void update_send_media_state(
      MediaKind? kind, MediaSourceKind? source_kind, bool enabled) async {
    kinds_combinations(kind, source_kind).forEach((element) {
      send_state[Tuple2(element.item1, element.item2)] = enabled;
    });
  }

  Future<void> update_recv_media_state(
      MediaKind? kind, MediaSourceKind? source_kind, bool enabled) async {
    kinds_combinations(kind, source_kind).forEach((element) {
      recv_state.addAll({Tuple2(element.item1, element.item2): enabled});
    });
  }

  List<Tuple2<MediaKind, MediaSourceKind>> kinds_combinations(
      MediaKind? kind, MediaSourceKind? source_kind) {
    var out = List<Tuple2<MediaKind, MediaSourceKind>>.empty(growable: true);
    if (kind != null) {
      if (source_kind != null) {
        out.add(Tuple2(kind, source_kind));
      } else {
        out.add(Tuple2(kind, MediaSourceKind.Device));
      }
    } else if (source_kind != null) {
      out.add(Tuple2(MediaKind.Audio, source_kind));
      out.add(Tuple2(MediaKind.Video, source_kind));
    } else {
      out.add(Tuple2(MediaKind.Video, MediaSourceKind.Device));
      out.add(Tuple2(MediaKind.Audio, MediaSourceKind.Device));
    }
    return out;
  }

  Future<void> wait_for_track_cb_fire_count(
      String callback_kind, RemoteMediaTrack track, int count) async {
    var id = track.getTrack().id();
    if (connection_store.callback_counter[id]![callback_kind] != count) {
      var fires_future = Completer();
      connection_store.OnCallbackCounter[id]![callback_kind] = (f) {
        if (f <= count) {
          fires_future.complete();
          connection_store.OnCallbackCounter[id]![callback_kind] = (_) {};
        }
      };
      return fires_future.future;
    }
  }

  Future<void> wait_disabled_track(RemoteMediaTrack track) async {
    var id = track.getTrack().id();
    if (track.mediaDirection() == TrackMediaDirection.SendRecv) {
      var direction_future = Completer();
      connection_store.OnMediaDirectionChanged[id] = (d) {
        if (d != TrackMediaDirection.SendRecv) {
          direction_future.complete();
          connection_store.OnMediaDirectionChanged.remove(
              track.getTrack().id());
        }
      };
      return direction_future.future;
    }
  }

  Future<void> wait_enabled_track(RemoteMediaTrack track) async {
    return wait_media_direction_track(TrackMediaDirection.SendRecv, track);
  }

  Future<void> wait_media_direction_track(
      TrackMediaDirection direction, RemoteMediaTrack track) async {
    var id = track.getTrack().id();
    if (track.mediaDirection() != direction) {
      var direction_future = Completer();
      connection_store.OnMediaDirectionChanged[id] = (d) {
        if (d == direction) {
          direction_future.complete();
          connection_store.OnMediaDirectionChanged.remove(
              track.getTrack().id());
        }
      };
      return direction_future.future;
    }
  }

  Tuple2<int, int> count_of_tracks_between_members(Member other) {
    var send_count = send_state.entries
        .where((element) => other.recv_state[element.key]! && element.value)
        .length;
    var recv_count = recv_state.entries
        .where((element) => other.send_state[element.key]! && element.value)
        .length;
    return Tuple2<int, int>(send_count, recv_count);
  }

  Future<void> toggle_media(
      MediaKind? kind, MediaSourceKind? source, bool enabled) async {
    update_send_media_state(kind, source, enabled);
    if (enabled) {
      if (kind != null) {
        if (kind == MediaKind.Audio) {
          await room.enableAudio();
        } else {
          await room.enableVideo(source);
        }
      } else {
        await room.enableAudio();
        await room.enableVideo(source);
      }
    } else {
      if (kind != null) {
        if (kind == MediaKind.Audio) {
          await room.disableAudio();
        } else {
          await room.disableVideo(source);
        }
      } else {
        await room.disableAudio();
        await room.disableVideo(source);
      }
    }
  }

  Future<void> toggle_mute(
      MediaKind? kind, MediaSourceKind? source, bool muted) async {
    if (!muted) {
      if (kind != null) {
        if (kind == MediaKind.Audio) {
          await room.unmuteAudio();
        } else {
          await room.unmuteVideo(source);
        }
      } else {
        await room.unmuteAudio();
        await room.unmuteVideo(source);
      }
    } else {
      if (kind != null) {
        if (kind == MediaKind.Audio) {
          await room.muteAudio();
        } else {
          await room.muteVideo(source);
        }
      } else {
        await room.muteAudio();
        await room.muteVideo(source);
      }
    }
  }

  Future<void> toggle_remote_media(
      MediaKind? kind, MediaSourceKind? source, bool enabled) async {
    await update_recv_media_state(kind, source, enabled);
    if (enabled) {
      if (kind != null) {
        if (kind == MediaKind.Audio) {
          await room.enableRemoteAudio();
        } else {
          await room.enableRemoteVideo();
        }
      } else {
        await room.enableRemoteAudio();
        await room.enableRemoteVideo();
      }
    } else {
      if (kind != null) {
        if (kind == MediaKind.Audio) {
          await room.disableRemoteAudio();
        } else {
          await room.disableRemoteVideo();
        }
      } else {
        await room.disableRemoteAudio();
        await room.disableRemoteVideo();
      }
    }
  }
}
