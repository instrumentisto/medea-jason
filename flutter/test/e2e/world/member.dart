

import 'dart:collection';
import 'package:flutter_test/flutter_test.dart';
import 'package:medea_jason/medea_jason.dart';
import 'package:tuple/tuple.dart';

import '../conf.dart';

class MyBuilder {
  late String id;
  late bool is_send;
  late bool is_recv;
  MyBuilder(this.id, this.is_send, this.is_recv);

  MyMember build(
      RoomHandle room,
      HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> send_state,
      HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> recv_state) {
    var result = MyMember();
    result.id = id;
    result.room = room;
    result.is_recv = is_recv;
    result.is_send = is_send;
    result.send_state = send_state;
    result.recv_state = recv_state;
    result.is_joined = false;
    return result;
  }
}

class MyMember {
  late String id;
  late bool is_send;
  late bool is_recv;
  late bool is_joined;
  late HashMap<Tuple2<MediaKind, MediaSourceKind>,bool> send_state;
  late HashMap<Tuple2<MediaKind, MediaSourceKind>,bool> recv_state;
  late RoomHandle room;
  // connection_store: Object<ConnectionStore>,
  // window: Window,

  void join_room(String room_id) async {
    var addrr = envVars['CLIENT_API_ADDR']!;
    await room.join('$addrr/$room_id/$id?token=test');
    is_joined = true;
  }

  void update_send_media_state(
      MediaKind? kind, MediaSourceKind? source_kind, bool enabled) async {
    kinds_combinations(kind, source_kind).forEach((element) {
      send_state.addAll({Tuple2(element.item1, element.item2): enabled});
    });
  }

  void update_recv_media_state(
      MediaKind? kind, MediaSourceKind? source_kind, bool enabled) async {
    kinds_combinations(kind, source_kind).forEach((element) {
      recv_state.addAll({Tuple2(element.item1, element.item2): enabled});
    });
  }

  List<Tuple2<MediaKind,MediaSourceKind>> kinds_combinations(MediaKind? kind, MediaSourceKind? source_kind) {
    var out = List<Tuple2<MediaKind,MediaSourceKind>>.empty();
    if (kind != null) {
      if (source_kind != null) {
        out.add(Tuple2(kind, source_kind));
      }
      else {
        out.add(Tuple2(kind, MediaSourceKind.Device));
      }
    } 
    else if (source_kind != null) {
      out.add(Tuple2(MediaKind.Audio, source_kind));
      out.add(Tuple2(MediaKind.Video, source_kind));
    }
    else {
      out.add(Tuple2(MediaKind.Video, MediaSourceKind.Device));
      out.add(Tuple2(MediaKind.Audio, MediaSourceKind.Device));
    }
    return out;
  } 

  Tuple2<int, int> count_of_tracks_between_members(MyMember other) {
    var send_count = 0;
    send_state.forEach((key, value) { 
      if (other.recv_state[key] != null) {++send_count;}
    });
    var recv_count = 0;
    recv_state.forEach((key, value) { 
      if (other.send_state[key] != null) {++send_count;}
    });
    return Tuple2<int,int>(send_count,recv_count);
  }

  void toggle_media(
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

    void toggle_mute(
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
   void toggle_remote_media(
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
  void switch_video_device() async {
    var setting = MediaStreamSettings(); 
    setting.audio(AudioTrackConstraints());
    setting.deviceVideo(DeviceVideoTrackConstraints());
    await room.setLocalMediaSettings(setting, true, true);
    await room.setLocalMediaSettings(MediaStreamSettings(), true, true);
  }
 
}
