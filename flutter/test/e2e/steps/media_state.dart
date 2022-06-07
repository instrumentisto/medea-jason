import 'package:flutter_webrtc/flutter_webrtc.dart' as fw;
import 'package:gherkin/gherkin.dart';
import 'package:medea_jason/medea_jason.dart';
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
    var track = await member.wait_remote_track_from(
        remote_id, parsedKind.item2, parsedKind.item1);

    var dir =
        TrackMediaDirection.values.firstWhere((e) => e.name == direction);

    await member.wait_media_direction_track(dir, track);
  },
);

StepDefinitionGeneric then_local_track_mute_state =
    then3<String, String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s (audio|device video|display video|video) local track is (not muted|muted)"),
  (id, String kind, not_muted, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parse_media_kind(kind);

    var track =
        await member.wait_local_track(parsedKind.item2, parsedKind.item1);
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
    var track =
        await member.wait_local_track(parsedKind.item2, parsedKind.item1);

    var track_ = track.getTrack();
    track.free();
    // await Future.delayed(Duration(milliseconds:100));
    expect(await track_.state(), fw.MediaStreamTrackState.ended);
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
