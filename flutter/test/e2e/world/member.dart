import 'dart:async';
import 'dart:collection';
import 'package:flutter_test/flutter_test.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:tuple/tuple.dart';

import 'package:flutter_webrtc/flutter_webrtc.dart' as fw;

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
    room.onFailedLocalMedia((p0) {});
    room.onConnectionLoss((p0) {});
    room.onClose((p0) {});
    room.onLocalTrack((p0) {});
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

  var stopped_tracks = HashMap<String, bool>();

  var callback_counter = HashMap<String, Map<String, int>>();

  var remote_tracks = HashMap<String, List<RemoteMediaTrack>>();
  List<LocalMediaTrack> local_tracks = List.empty(growable: true);

  HashMap<String, Function(ConnectionHandle)> onConnect = HashMap();
}

class MyMember {
  late String id;
  late bool is_send;
  late bool is_recv;
  late bool is_joined;
  late HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> send_state;
  late HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> recv_state;
  late Completer<RoomCloseReason> close_reason = Completer();
  late RoomHandle room;
  var connection_store = myConnectionStore();
  // window: Window,

  MyMember(this.id, this.is_send, this.is_recv, this.is_joined, this.send_state,
      this.recv_state, this.room) {
    room.onClose((p0) {
      close_reason.complete(p0);
    });

    room.onLocalTrack((p0) {
      connection_store.local_tracks.add(p0);
    });

    room.onNewConnection((p0) {
      var id = p0.getRemoteMemberId();
      connection_store.remote_tracks.addAll({id: List.empty(growable: true)});
      p0.onRemoteTrackAdded((p0) {
        connection_store.callback_counter.addAll({
          p0.getTrack().id(): {
            'enabled': 0,
            'disabled': 0,
            'muted': 0,
            'unmuted': 0
          }
        });
        p0.onMuted(() {
          var c = connection_store.callback_counter[p0.getTrack().id()]!;
          var old = c['muted']!;
          c['muted'] = old + 1;
        });

        p0.onUnmuted(() {
          var c = connection_store.callback_counter[p0.getTrack().id()]!;
          var old = c['unmuted']!;
          c['unmuted'] = old + 1;
        });

        p0.onMediaDirectionChanged((p0_) {
          if (p0_ != TrackMediaDirection.SendRecv) {
            var c = connection_store.callback_counter[p0.getTrack().id()]!;
            var old = c['disabled']!;
            c['disabled'] = old + 1;
          } else {
            var c = connection_store.callback_counter[p0.getTrack().id()]!;
            var old = c['enabled']!;
            c['enabled'] = old + 1;
          }
        });

        connection_store.stopped_tracks.addAll({p0.getTrack().id(): false});
        connection_store.remote_tracks[id]!.add(p0);
        p0.onStopped(() {
          connection_store.stopped_tracks[p0.getTrack().id()] = true;
        });
      });

      connection_store.connects.addAll({p0.getRemoteMemberId(): p0});
      connection_store.close_conn.addAll({p0.getRemoteMemberId(): Completer()});
      p0.onClose(() {
        connection_store.close_conn[p0.getRemoteMemberId()]!.complete();
      });
      connection_store.onConnect.forEach((key, value) {
        value(p0);
      });
    });
  }

  Future<void> forget_local_tracks() async {
    connection_store.local_tracks.forEach((element) {element.free();});
  }

  Future<void> wait_for_connect(String id) {
    var connect = Completer();
    if (!connection_store.connects.containsKey(id)) {
      connection_store.onConnect.addAll({
        id: (p0) {
          if (p0.getRemoteMemberId() == id) {
            connect.complete();
            connection_store.onConnect.remove(id);
          }
        }
      });
    } else {
      connect.complete();
    }
    return connect.future;
  }

  Future<void> wait_for_track_count(String id, int count) async {
    var count_f = Completer();
    if (connection_store.remote_tracks[id]!.length == count) {
      count_f.complete();
    } else {
      while (connection_store.remote_tracks[id]!.length < count) {
        await Future.delayed(Duration(milliseconds: 100));
      }
      count_f.complete();
    }
    return count_f.future;
    return Future.delayed(Duration(seconds: 1));
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

  //   let send_count = self
  //     .send_state
  //     .borrow()
  //     .iter()
  //     .filter(|(key, enabled)| {
  //         other.recv_state.borrow().get(key).copied().unwrap_or(false)
  //             && **enabled
  //     })
  //     .count() as u64;
  // let recv_count = self
  //     .recv_state
  //     .borrow()
  //     .iter()
  //     .filter(|(key, enabled)| {
  //         other.send_state.borrow().get(key).copied().unwrap_or(false)
  //             && **enabled
  //     })
  //     .count() as u64;

  Tuple2<int, int> count_of_tracks_between_members(MyMember other) {
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
    await update_send_media_state(kind, source, enabled);
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

  Future<void> add_gum_latency(Duration latency) async {
    var caps = fw.DeviceConstraints();
    caps.video.mandatory = fw.DeviceVideoConstraints();
    caps.audio.mandatory = fw.AudioConstraints();
    caps.video.mandatory!.width = 640;
    caps.video.mandatory!.height = 480;
    caps.video.mandatory!.fps = 30;
    await fw.getUserMedia(caps).timeout(latency);
  }

  //   /// Emulates the provided `latency` for `getUserMedia()` requests.
  // pub async fn add_gum_latency(&self, latency: Duration) {
  //     self.window
  //         .execute(Statement::new(
  //             r#"
  //                 async () => {
  //                     const [duration] = args;

  //                     var gUM = navigator.mediaDevices.getUserMedia.bind(
  //                         navigator.mediaDevices
  //                     );
  //                     navigator.mediaDevices.getUserMedia =
  //                         async function (cons) {
  //                             await new Promise(r => setTimeout(r, duration));
  //                             return await gUM(cons);
  //                         };
  //                 }
  //             "#,
  //             [(latency.as_millis() as u64).into()],
  //         ))
  //         .await
  //         .unwrap();
  // }

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
