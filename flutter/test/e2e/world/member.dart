import 'dart:async';
import 'dart:collection';
import 'package:flutter_test/flutter_test.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:tuple/tuple.dart';

import '../conf.dart';

var globalConnect = HashMap<String, ConnectionHandle>();

class MyBuilder {
  late String id;
  late bool is_send;
  late bool is_recv;
  MyBuilder(this.id, this.is_send, this.is_recv);

  MyMember build(
      RoomHandle room,
      HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> send_state,
      HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> recv_state) {
    room.onFailedLocalMedia((p0) {
      print('DEAD');
    });
    room.onConnectionLoss((p0) {
      print('Loss');
    });
    room.onClose((p0) => print('$id: Close'));
    room.onLocalTrack((p0) {
      print('Ltrack');
    });
    var result =
        MyMember(id, is_send, is_recv, false, send_state, recv_state, room);
    result.room = room;
    result.is_recv = is_recv;
    result.is_send = is_send;
    result.send_state = send_state;
    result.recv_state = recv_state;
    result.is_joined = false;
    return result;
  }
}

class myConnectionStore {
  var close_conn = HashMap<String, Completer>();
  var connects = HashMap<String, ConnectionHandle>();
  var close = HashMap<String, ConnectionHandle>();

  HashMap<String, Function(ConnectionHandle)> onConnect = HashMap();
  HashMap<String, Function(RoomCloseReason)> onClose = HashMap();
}

class MyMember {
  late String id;
  late bool is_send;
  late bool is_recv;
  late bool is_joined;
  late HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> send_state;
  late HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> recv_state;
  late RoomHandle room;
  var connection_store = myConnectionStore();
  // window: Window,

  MyMember(this.id, this.is_send, this.is_recv, this.is_joined, this.send_state,
      this.recv_state, this.room) {
    room.onClose((p0) {
      connection_store.onClose.forEach((key, value) {
        value(p0);
      });
    });
    room.onNewConnection((p0) {
      connection_store.connects.addAll({p0.getRemoteMemberId(): p0});
      connection_store.close_conn.addAll({p0.getRemoteMemberId(): Completer()});
      p0.onClose(() {
        connection_store.close_conn[p0.getRemoteMemberId()]!.complete();
      });
      print(connection_store.onConnect);
      connection_store.onConnect.forEach((key, value) {
        value(p0);
      });
    });
  }

  Future<void> wait_for_connect(String id) {
    var close = Completer();
    if (!connection_store.connects.containsKey(id)) {
      connection_store.onConnect.addAll({
        id: (p0) {
          if (p0.getRemoteMemberId() == id) {
            close.complete();
            connection_store.onConnect.remove(id);
          }
        }
      });
    } else {
      close.complete();
    }
    return close.future;
  }

    Future<void> wait_for_close(String id) {
      return connection_store.close_conn[id]!.future;
  }

  Future<void> join_room(String room_id) async {
    var addrr = 'ws://127.0.0.1:8001/ws';
    await room.join('$addrr/$room_id/$id?token=test');
    is_joined = true;
  }

  Future<void> update_send_media_state(
      MediaKind? kind, MediaSourceKind? source_kind, bool enabled) async {
    kinds_combinations(kind, source_kind).forEach((element) {
      send_state.addAll({Tuple2(element.item1, element.item2): enabled});
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
    var out = List<Tuple2<MediaKind, MediaSourceKind>>.empty();
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

  Tuple2<int, int> count_of_tracks_between_members(MyMember other) {
    var send_count = 0;
    send_state.forEach((key, value) {
      if (other.recv_state[key] != null) {
        ++send_count;
      }
    });
    var recv_count = 0;
    recv_state.forEach((key, value) {
      if (other.send_state[key] != null) {
        ++send_count;
      }
    });
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
    update_recv_media_state(kind, source, enabled);
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

  // false true ????? todo
  //room.setLocalMediaSettings(false, true, true);
  Future<void> switch_video_device() async {
    var setting = MediaStreamSettings();
    setting.audio(AudioTrackConstraints());
    setting.deviceVideo(DeviceVideoTrackConstraints());
    await room.setLocalMediaSettings(setting, true, true);
    await room.setLocalMediaSettings(MediaStreamSettings(), true, true);
  }
}
