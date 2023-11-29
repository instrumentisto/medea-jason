import 'dart:async';
import 'dart:collection';

import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as webrtc;
import 'package:tuple/tuple.dart';

import 'package:medea_jason/medea_jason.dart';
import 'package:medea_jason/src/native/platform/media_devices.dart';
import 'package:medea_jason/src/native/platform/transport.dart';
import '../conf.dart';
import 'custom_world.dart';

/// Builder of a [Member].
class MemberBuilder {
  /// ID with which the [Member] will be created.
  String id;

  /// Indicator whether the [Member] will publish media.
  bool isSend;

  /// Indicator whether the [Member] will receive media.
  bool isRecv;

  MemberBuilder(this.id, this.isSend, this.isRecv);

  /// Creates a new [Member] out of this [MemberBuilder] configuration.
  Member build(
      RoomHandle room,
      HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> sendState,
      HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> recvState) {
    return Member(id, isSend, isRecv, false, sendState, recvState, room);
  }
}

/// Storage of [ConnectionHandle]'s and `MediaStreamTrack`'s thrown by this
/// [Member].
class ConnectionStore {
  /// [Completer] for waits close [RoomHandle].
  Completer<RoomCloseReason> closeReason = Completer();

  /// Storage of [ConnectionHandle]s, where the key is [Member.id].
  var connections = HashMap<String, ConnectionHandle>();

  /// Storage of [LocalMediaTrack]s.
  List<LocalMediaTrack> localTracks = List.empty(growable: true);

  /// Storage of [RemoteMediaTrack]s, where the key is [Member.id].
  var remoteTracks = HashMap<String, HashMap<String, List<RemoteMediaTrack>>>();

  /// Storage of [RemoteMediaTrack]'s callbacks fires count, where the key is
  /// `MediaStreamTrack.id`.
  var callbackCounter = HashMap<String, Map<String, int>>();

  /// [Completer]s waiting the [ConnectionHandle]s being closed.
  var closeConnect = HashMap<String, Completer>();

  /// Callbacks calls after [RemoteMediaTrack.onMediaDirectionChanged], where
  /// the key is `track.id`.
  var onMediaDirectionChanged =
      HashMap<String, Function(TrackMediaDirection)>();

  /// Callbacks calls after [RemoteMediaTrack] `callback_kind`, where the key is
  /// `track.id`.
  var onCallbackCounter = HashMap<String, Map<String, Function(int)>>();

  /// Callback calls after [RoomHandle.onNewConnection].
  var onConnect = HashMap<String, Function(ConnectionHandle)>();

  /// Callback calls after [ConnectionHandle.onRemoteTrackAdded].
  var onRemoteTrack = HashMap<String, Function(RemoteMediaTrack)>();

  /// Callback calls after [RoomHandle.onLocalTrack].
  Function(LocalMediaTrack) onLocalTrack = (_) {};

  /// Indicates whether the provided track from the specified `remote_id` is
  /// stopped.
  bool remoteTrackIsStopped(String remoteId, String trackId) {
    var tracks = remoteTracks[remoteId]![trackId]!;
    var stoppedLength = callbackCounter[trackId]!['stopped']!;
    var allLength = tracks.length;
    return stoppedLength == allLength;
  }

  /// Returns count of tracks from `remote_id` by the provided `live` values.
  int countTracksByLived(bool live, String remoteId) {
    var count = 0;
    remoteTracks[remoteId]!.forEach((key, track) {
      var trackStopped = remoteTrackIsStopped(remoteId, key);
      if (live &&
          !track.last.muted() &&
          track.last.mediaDirection() == TrackMediaDirection.sendRecv &&
          !trackStopped) {
        count += 1;
      } else if ((!live && trackStopped) ||
          (!live &&
              track.last.mediaDirection() != TrackMediaDirection.sendRecv)) {
        count += 1;
      }
    });
    return count;
  }
}

/// Representation of a [Member] connected to a media server.
class Member {
  /// ID of this [Member] on a media server.
  String id;

  /// Indicator whether this [Member] should publish media.
  bool isSend;

  /// Indicator whether this [Member] should receive media.
  bool isRecv;

  /// Indicator whether this [Member] should publish audio.
  bool enabledAudio = true;

  /// Indicator whether this [Member] should publish video.
  bool enabledVideo = true;

  /// Indicator whether this [Member] is joined a [RoomHandle] on a media
  /// server.
  bool isJoined;

  /// Media publishing state of this [Member].
  ///
  /// If value is `true` then this [MediaKind] and [MediaSourceKind] is
  /// enabled.
  HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> sendState;

  /// Media receiving state of this [Member].
  ///
  /// If value is `true` then this [MediaKind] and [MediaSourceKind] is
  /// enabled.
  HashMap<Tuple2<MediaKind, MediaSourceKind>, bool> recvState;

  /// [RoomHandle]'s that this [Member] is intended to join.
  RoomHandle room;

  /// Counter of failed local `getUserMedia()` requests.
  int failedLocalStreamCount = 0;

  /// Storage of [ConnectionHandle]s thrown by this [Member]'s [RoomHandle].
  var connectionStore = ConnectionStore();

  /// Last [ReconnectHandle].
  ReconnectHandle? reconnectHandle;

  Member(this.id, this.isSend, this.isRecv, this.isJoined, this.sendState,
      this.recvState, this.room) {
    room.onConnectionLoss((p0) {
      reconnectHandle = p0;
    });
    room.onFailedLocalMedia((p0) {
      ++failedLocalStreamCount;
    });
    room.onClose((reason) {
      connectionStore.closeReason.complete(reason);
    });
    room.onLocalTrack((localTrack) {
      connectionStore.localTracks.add(localTrack);
      connectionStore.onLocalTrack(localTrack);
    });
    room.onNewConnection((connection) async {
      var remoteMemberId = connection.getRemoteMemberId();
      connectionStore.remoteTracks.addAll({remoteMemberId: HashMap()});
      connectionStore.connections.addAll({remoteMemberId: connection});
      connectionStore.closeConnect.addAll({remoteMemberId: Completer()});

      connection.onRemoteTrackAdded((remoteTrack) {
        var remoteTrackId = remoteTrack.getTrack().id();
        if (connectionStore.remoteTracks[remoteMemberId]![remoteTrackId] ==
            null) {
          connectionStore.remoteTracks[remoteMemberId]![remoteTrackId] =
              List.empty(growable: true);
        }
        connectionStore.remoteTracks[remoteMemberId]![remoteTrackId]!
            .add(remoteTrack);

        connectionStore.callbackCounter.addAll({
          remoteTrackId: {
            'enabled': 0,
            'disabled': 0,
            'muted': 0,
            'unmuted': 0,
            'stopped': 0
          }
        });
        connectionStore.onCallbackCounter.addAll({
          remoteTrackId: {
            'enabled': (_) => {},
            'disabled': (_) => {},
            'muted': (_) => {},
            'unmuted': (_) => {},
            'stopped': (_) => {},
          }
        });

        remoteTrack.onMuted(() {
          connectionStore.callbackCounter[remoteTrackId]!
              .update('muted', (value) => value += 1);
          connectionStore.onCallbackCounter[remoteTrackId]!['muted']!(
              connectionStore.callbackCounter[remoteTrackId]!['muted']!);
        });

        remoteTrack.onUnmuted(() {
          connectionStore.callbackCounter[remoteTrackId]!
              .update('unmuted', (value) => value += 1);
          connectionStore.onCallbackCounter[remoteTrackId]!['unmuted']!(
              connectionStore.callbackCounter[remoteTrackId]!['unmuted']!);
        });

        remoteTrack.onMediaDirectionChanged((direction) {
          if (direction != TrackMediaDirection.sendRecv) {
            connectionStore.callbackCounter[remoteTrackId]!
                .update('disabled', (value) => value += 1);

            connectionStore.onCallbackCounter[remoteTrackId]!['disabled']!(
                connectionStore.callbackCounter[remoteTrackId]!['disabled']!);
          } else {
            connectionStore.callbackCounter[remoteTrackId]!
                .update('enabled', (value) => value += 1);
            connectionStore.onCallbackCounter[remoteTrackId]!['enabled']!(
                connectionStore.callbackCounter[remoteTrackId]!['enabled']!);
          }
          connectionStore.onMediaDirectionChanged.forEach((key, value) {
            value(direction);
          });
        });

        remoteTrack.onStopped(() {
          connectionStore.callbackCounter[remoteTrackId]!
              .update('stopped', (value) => value + 1);
          connectionStore.onCallbackCounter[remoteTrackId]!['stopped']!(
              connectionStore.callbackCounter[remoteTrackId]!['stopped']!);
        });

        if (connectionStore.onRemoteTrack[remoteMemberId] != null) {
          connectionStore.onRemoteTrack[remoteMemberId]!(remoteTrack);
        }
      });

      connection.onClose(() {
        connectionStore.closeConnect[remoteMemberId]!.complete();
      });
      if (connectionStore.onConnect[remoteMemberId] != null) {
        connectionStore.onConnect[remoteMemberId]!(connection);
      }
    });
  }

  /// Frees all the [LocalMediaTrack]s of this [Member].
  Future<void> forgetLocalTracks() async {
    for (var track in connectionStore.localTracks) {
      await track.free();
    }
  }

  /// Waits for a [ConnectionHandle] from the [Member] with the provided [id].
  Future<void> waitForConnect(String id) async {
    if (!connectionStore.connections.containsKey(id)) {
      var conn = Completer();
      connectionStore.onConnect[id] = (_) {
        conn.complete();
        connectionStore.onConnect[id] = (_) {};
      };
      return conn.future;
    }
  }

  /// Waits for a `count` of [RemoteMediaTrack]s from the [Member] with the
  /// provided [id].
  Future<void> waitForTrackCount(String id, int count) async {
    if (connectionStore.remoteTracks[id]!.length != count) {
      var trackCompl = Completer();
      connectionStore.onRemoteTrack[id] = (_) {
        if (connectionStore.remoteTracks[id]!.length >= count) {
          trackCompl.complete();
          connectionStore.onRemoteTrack.remove(id);
        }
      };
      return trackCompl.future;
    }
  }

  /// Waits for a [RemoteMediaTrack] from the [Member] with the provided [id],
  /// based on the provided options.
  Future<RemoteMediaTrack> waitRemoteTrackFrom(
      String id, MediaSourceKind? source, MediaKind? kind) async {
    bool sourceCheck(MediaSourceKind a, MediaSourceKind? b) {
      if (b == null) {
        return true;
      }
      return a == b;
    }

    bool kindCheck(MediaKind a, MediaKind? b) {
      if (b == null) {
        return true;
      }
      return a == b;
    }

    if (connectionStore.remoteTracks[id]!.values.any((element) =>
        sourceCheck(element.last.mediaSourceKind(), source) &&
        kindCheck(element.last.kind(), kind))) {
      return connectionStore.remoteTracks[id]!.values
          .lastWhere((element) =>
              sourceCheck(element.last.mediaSourceKind(), source) &&
              kindCheck(element.last.kind(), kind))
          .last;
    } else {
      var trackCompl = Completer<RemoteMediaTrack>();
      connectionStore.onRemoteTrack[id] = (track) {
        if (sourceCheck(track.mediaSourceKind(), source) &&
            kindCheck(track.kind(), kind)) {
          trackCompl.complete(track);
          connectionStore.onRemoteTrack.remove(id);
        }
      };
      return trackCompl.future;
    }
  }

  /// Waits for a [LocalMediaTrack], based on the provided options.
  Future<LocalMediaTrack> waitLocalTrack(
      MediaSourceKind source, MediaKind kind) async {
    if (connectionStore.localTracks.any((element) =>
        element.kind() == kind && element.mediaSourceKind() == source)) {
      return connectionStore.localTracks.firstWhere((element) =>
          element.kind() == kind && element.mediaSourceKind() == source);
    } else {
      var trackCompl = Completer<LocalMediaTrack>();
      connectionStore.onLocalTrack = (track) {
        if (track.kind() == kind && track.mediaSourceKind() == source) {
          trackCompl.complete(track);
          connectionStore.onLocalTrack = (_) {};
        }
      };
      return trackCompl.future;
    }
  }

  /// Joins a [RoomHandle] with the provided ID.
  Future<void> joinRoom(String roomId) async {
    await room.join('$clientApiAddr/$roomId/$id?token=test');
    isJoined = true;
  }

  /// Updates this [Member.sendState].
  void updateSendMediaState(
      MediaKind? kind, MediaSourceKind? sourceKind, bool enabled) async {
    kindsCombinations(kind, sourceKind).forEach((element) {
      sendState[Tuple2(element.item1, element.item2)] = enabled;
    });
  }

  /// Updates this [Member.recvState].
  Future<void> updateRecvMediaState(
      MediaKind? kind, MediaSourceKind? sourceKind, bool enabled) async {
    kindsCombinations(kind, sourceKind).forEach((element) {
      recvState.addAll({Tuple2(element.item1, element.item2): enabled});
    });
  }

  /// Returns a count of [LocalMediaTrack]s and [RemoteMediaTrack]s of this
  /// [Member] with the provided partner [Member].
  Tuple2<int, int> countOfTracksBetweenMembers(Member other) {
    if (isSfu) {
      // All transceivers are always `sendrecv` in SFU mode.
      return const Tuple2<int, int>(3, 3);
    }
    var sendCount = sendState.entries
        .where((element) => other.recvState[element.key]! && element.value)
        .length;
    var recvCount = recvState.entries
        .where((element) => other.sendState[element.key]! && element.value)
        .length;
    return Tuple2<int, int>(sendCount, recvCount);
  }

  /// Toggles a media state of this [Member]'s [RoomHandle].
  Future<void> toggleMedia(
      MediaKind? kind, MediaSourceKind? source, bool enabled) async {
    updateSendMediaState(kind, source, enabled);
    if (enabled) {
      if (kind != null) {
        if (kind == MediaKind.audio) {
          await room.enableAudio();
          enabledAudio = true;
        } else {
          await room.enableVideo(source);
          enabledVideo = true;
        }
      } else {
        await room.enableAudio();
        await room.enableVideo(source);
        enabledAudio = true;
        enabledVideo = true;
      }
    } else {
      if (kind != null) {
        if (kind == MediaKind.audio) {
          await room.disableAudio();
          enabledAudio = false;
        } else {
          await room.disableVideo(source);
          enabledVideo = false;
        }
      } else {
        await room.disableAudio();
        await room.disableVideo(source);
        enabledAudio = false;
        enabledVideo = false;
      }
    }
  }

  /// Toggles a mute state of this [Member]'s [RoomHandle].
  Future<void> toggleMute(
      MediaKind? kind, MediaSourceKind? source, bool muted) async {
    if (!muted) {
      if (kind != null) {
        if (kind == MediaKind.audio) {
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
        if (kind == MediaKind.audio) {
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

  /// Toggles a remote media state of this [Member]'s [RoomHandle].
  Future<void> toggleRemoteMedia(
      MediaKind? kind, MediaSourceKind? source, bool enabled) async {
    await updateRecvMediaState(kind, source, enabled);
    if (enabled) {
      if (kind != null) {
        if (kind == MediaKind.audio) {
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
        if (kind == MediaKind.audio) {
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

  /// Returns a list of [MediaKind]s and a [MediaSourceKind], based on the
  /// provided options.
  List<Tuple2<MediaKind, MediaSourceKind>> kindsCombinations(
      MediaKind? kind, MediaSourceKind? sourceKind) {
    var out = List<Tuple2<MediaKind, MediaSourceKind>>.empty(growable: true);
    if (kind != null) {
      if (sourceKind != null) {
        out.add(Tuple2(kind, sourceKind));
      } else {
        out.add(Tuple2(kind, MediaSourceKind.device));
      }
    } else if (sourceKind != null) {
      out.add(Tuple2(MediaKind.audio, sourceKind));
      out.add(Tuple2(MediaKind.video, sourceKind));
    } else {
      out.add(const Tuple2(MediaKind.video, MediaSourceKind.device));
      out.add(const Tuple2(MediaKind.audio, MediaSourceKind.device));
    }
    return out;
  }

  /// Waits for the [RemoteMediaTrack]'s callbacks of `callback_kind` to happen
  /// the provided `count` times.
  Future<void> waitForTrackCbFireCount(
      String callbackKind, RemoteMediaTrack track, int count) async {
    var id = track.getTrack().id();
    if (connectionStore.callbackCounter[id]![callbackKind] != count) {
      var firesFuture = Completer();
      connectionStore.onCallbackCounter[id]![callbackKind] = (f) {
        if (f <= count) {
          firesFuture.complete();
          connectionStore.onCallbackCounter[id]![callbackKind] = (_) {};
        }
      };
      return firesFuture.future;
    }
  }

  /// Waits for the [RemoteMediaTrack]'s disabled state.
  Future<void> waitDisabledTrack(RemoteMediaTrack track) async {
    var id = track.getTrack().id();
    if (track.mediaDirection() == TrackMediaDirection.sendRecv) {
      var directionFuture = Completer();
      connectionStore.onMediaDirectionChanged[id] = (d) {
        if (d != TrackMediaDirection.sendRecv) {
          directionFuture.complete();
          connectionStore.onMediaDirectionChanged.remove(track.getTrack().id());
        }
      };
      return directionFuture.future;
    }
  }

  /// Waits for the [RemoteMediaTrack]'s enabled state.
  Future<void> waitEnabledTrack(RemoteMediaTrack track) async {
    return waitMediaDirectionTrack(TrackMediaDirection.sendRecv, track);
  }

  /// Waits for the [RemoteMediaTrack]'s direction change to the provided
  /// [direction].
  Future<void> waitMediaDirectionTrack(
      TrackMediaDirection direction, RemoteMediaTrack track) async {
    var id = track.getTrack().id();
    if (track.mediaDirection() != direction) {
      var directionFuture = Completer();
      connectionStore.onMediaDirectionChanged[id] = (d) {
        if (d == direction) {
          directionFuture.complete();
          connectionStore.onMediaDirectionChanged.remove(track.getTrack().id());
        }
      };
      return directionFuture.future;
    }
  }

  /// Waits for a [Member] with the specified [id] to close.
  Future<void> waitForClose(String id) {
    return connectionStore.closeConnect[id]!.future;
  }

  /// Sets the `getUserMedia()` request to return error.
  void getUserMediaMock(bool audio, bool video) {
    MockMediaDevices.gum = (constraints) async {
      if (audio) {
        throw webrtc.GetMediaException(
            webrtc.GetMediaExceptionKind.audio, 'Mock Error');
      } else if (video) {
        throw webrtc.GetMediaException(
            webrtc.GetMediaExceptionKind.video, 'Mock Error');
      }
      return webrtc.getUserMedia(constraints);
    };
  }

  /// Sets latency to the `getUserMedia()` request.
  void setGumLatency(Duration time) {
    MockMediaDevices.gum = (constraints) async {
      await Future.delayed(time);
      return webrtc.getUserMedia(constraints);
    };
  }

  /// Emulates video device switching.
  Future<void> switchVideoDevice() async {
    await room.setLocalMediaSettings(MediaStreamSettings(), true, true);

    var constraints = MediaStreamSettings();
    constraints.deviceVideo(DeviceVideoTrackConstraints());
    await room.setLocalMediaSettings(constraints, true, false);
  }

  /// Waits for the [failedLocalStreamCount] to become [times].
  Future<void> waitFailedLocalStreamCount(int times) async {
    if (failedLocalStreamCount != times) {
      var failedLocalStreamFuture = Completer();
      room.onFailedLocalMedia((err) {
        ++failedLocalStreamCount;
        if (failedLocalStreamCount == times) {
          failedLocalStreamFuture.complete();
          room.onFailedLocalMedia((p0) {
            ++failedLocalStreamCount;
          });
        }
      });
      return failedLocalStreamFuture.future;
    }
  }

  /// Restores the connection.
  Future<void> reconnect() async {
    if (reconnectHandle != null) {
      await reconnectHandle!.reconnectWithBackoff(100, 2.0, 1000, 5000);
    } else {
      await waitConnectionLost();
      await reconnectHandle!.reconnectWithBackoff(100, 2.0, 1000, 5000);
    }
    reconnectHandle = null;
  }

  /// Waits for the connection loss.
  Future<void> waitConnectionLost() async {
    if (reconnectHandle == null) {
      var connectionLostFuture = Completer();
      room.onConnectionLoss((p0) {
        reconnectHandle = p0;
        connectionLostFuture.complete();
      });

      await connectionLostFuture.future;

      room.onConnectionLoss((p0) {
        reconnectHandle = p0;
      });
    }
  }

  /// Closes the [WebSocket] connection.
  Future<void> connectionLoss() async {
    var ws = MockWebSocket.getSocket(id)!;
    await ws.close(9999);
  }
}
