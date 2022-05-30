import 'dart:async';

import 'package:gherkin/gherkin.dart';
import 'package:medea_jason/src/interface/track_kinds.dart';
import 'package:flutter_test/flutter_test.dart';
import '../world/custom_world.dart';
import '../world/more_args.dart';

StepDefinitionGeneric when_enables_or_mutes =
    when4<String, String, String, String, CustomWorld>(
  RegExp(
      r'(Alice|Bob|Carol) (enables|disables|mutes|unmutes) (audio|video)( and awaits it completes| and awaits it errors|)'),
  (id, action, audio_or_video, awaits, context) async {
    var kind = parse_media_kind(audio_or_video);
    var member = context.world.members[id]!;

    await Future.delayed(Duration(seconds: 1));

    var awaitable = awaits.contains('awaits');
    var error = awaits.contains('errors');

    try {
      switch (action) {
        case 'enables':
          {
            var future = member.toggle_media(kind.item1, kind.item2, true);
            if (awaitable) {
              await future;
            }
          }
          break;

        case 'disables':
          {
            var future = member.toggle_media(kind.item1, kind.item2, false);
            if (awaitable) {
              await future;
            }
          }
          break;

        case 'mutes':
          {
            var future = member.toggle_mute(kind.item1, kind.item2, true);
            if (awaitable) {
              await future;
            }
            
          }
          break;

        default:
          {
            var future = member.toggle_mute(kind.item1, kind.item2, false);
            if (awaitable) {
              await future;
            }
          }
          break;
      }
    } catch (e) {
      if (!error) {
        rethrow;
      }
    }
  },
);

StepDefinitionGeneric when_member_enables_remote_track =
    when3<String, String, String, CustomWorld>(
  RegExp(
      r'(Alice|Bob|Carol) (enables|disables) remote (audio|device video|display video|video)'),
  (id, toggle, String kind, context) async {
    var parsedKind = parse_media_kind(kind);
    var member = context.world.members[id]!;

    if (toggle == 'enables') {
      if (parsedKind.item1 == MediaKind.Audio) {
        await member.room.enableRemoteAudio();
      } else {
        await member.room.enableRemoteVideo();
      }
    } else {
      if (parsedKind.item1 == MediaKind.Audio) {
        await member.room.disableRemoteAudio();
      } else {
        await member.room.disableRemoteVideo();
      }
    }
  },
);

StepDefinitionGeneric then_remote_media_direction_is =
    then4<String, String, String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s (audio|video) from (Alice|Bob|Carol) has `(SendRecv|SendOnly|RecvOnly|Inactive)` direction"),
  (id, String kind, remote_id, direction, context) async {
    var member = context.world.members[id]!;

    var parsedKind = parse_media_kind(kind);

    await member.wait_for_connect(remote_id);
    var tracks = member.connection_store.remote_tracks[remote_id]!.values
        .where((element) => element.isNotEmpty)
        .map((e) => e.last)
        .toList();

    var track = tracks.firstWhere((element) =>
        element.mediaSourceKind() == parsedKind.item2 &&
        element.kind() == parsedKind.item1);

    if (track.mediaDirection().name != direction) {
      var direction_future = Completer();
      member.connection_store.MediaDirectionChangedCB[track.getTrack().id()] = (d) {
        if (direction == d) {
          direction_future.complete();
          member.connection_store.MediaDirectionChangedCB.remove(track.getTrack().id());
        }
      };

      await direction_future.future;
    }

  },
);

StepDefinitionGeneric then_local_track_mute_state =
    then3<String, String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s (audio|device video|display video|video) local track is (not muted|muted)"),
  (id, String kind, not_muted, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parse_media_kind(kind);

    // todo
    await Future.delayed(Duration(milliseconds: 300));
    var track = member.connection_store.local_tracks.firstWhere((element) =>
        element.mediaSourceKind() == MediaSourceKind.Device &&
        element.kind() == parsedKind.item1);
    var muted = !not_muted.contains('not');
    expect(!track.getTrack().isEnabled(), muted);
  },
);

StepDefinitionGeneric then_track_is_stopped =
    then2<String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s (audio|device video|display video|video) local track is stopped"),
  (id, kind, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parse_media_kind(kind);
    var track = member.connection_store.local_tracks.firstWhere((element) =>
        element.kind() == parsedKind.item1 &&
        element.mediaSourceKind() == parsedKind.item2);

    track.free();
    // todo
    // check readyState
    // if(!stopped) {
    //   throw 'not stopped';
    // }
  },
);

StepDefinitionGeneric when_member_frees_all_local_tracks =
    when1<String, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) frees all local tracks'),
  (id, context) async {
    var member = context.world.members[id]!;
    await member.forget_local_tracks();
  },
);
