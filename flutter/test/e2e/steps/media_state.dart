import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';

import 'package:medea_jason/medea_jason.dart';
import '../world/custom_world.dart';
import '../world/more_args.dart';

List<StepDefinitionGeneric> steps() {
  return [
    givenGumDelay,
    thenLocalTrackMuteState,
    thenRemoteMediaDirectionIs,
    thenTrackIsStopped,
    whenEnablesOrMutes,
    whenMemberEnablesRemoteTrack,
    whenMemberFreesAllLocalTracks,
    whenMemberSwitchesDeviceWithLatency
  ];
}

StepDefinitionGeneric whenEnablesOrMutes =
    when4<String, String, String, String, CustomWorld>(
  RegExp(r'(\S+) (enables|disables|mutes|unmutes) (audio|video) and'
      r' (awaits it completes|awaits it errors|ignores the result)?$'),
  (id, action, audioOrVideo, String awaits, context) async {
    var kind = parseMediaKind(audioOrVideo);
    var member = context.world.members[id]!;

    var awaitable = awaits.contains('awaits');
    var error = awaits.contains('errors');
    Future<void> future;
    switch (action) {
      case 'enables':
        future = member.toggleMedia(kind.item1, kind.item2, true);
        break;

      case 'disables':
        future = member.toggleMedia(kind.item1, kind.item2, false);
        break;

      case 'mutes':
        future = member.toggleMute(kind.item1, kind.item2, true);
        break;

      default:
        future = member.toggleMute(kind.item1, kind.item2, false);
        break;
    }

    if (awaitable) {
      try {
        await future;
      } catch (e) {
        if (!error) {
          rethrow;
        }
      }
    } else {
      unawaited(future.catchError((e) => {
            // suppress error
          }));
    }
  },
);

StepDefinitionGeneric whenMemberEnablesRemoteTrack =
    when3<String, String, String, CustomWorld>(
  RegExp(r'(\S+) (enables|disables) remote '
      r'(audio|(?:device |display )?video)$'),
  (id, toggle, String kind, context) async {
    var parsedKind = parseMediaKind(kind);
    var member = context.world.members[id]!;

    if (toggle == 'enables') {
      if (parsedKind.item1 == MediaKind.audio) {
        await member.room.enableRemoteAudio();
      } else {
        await member.room.enableRemoteVideo();
      }
    } else {
      if (parsedKind.item1 == MediaKind.audio) {
        await member.room.disableRemoteAudio();
      } else {
        await member.room.disableRemoteVideo();
      }
    }
  },
);

StepDefinitionGeneric thenRemoteMediaDirectionIs =
    then4<String, String, String, String, CustomWorld>(
  RegExp(r"(\S+)'s (audio|video) from (\S+) has "
      r'`(SendRecv|SendOnly|RecvOnly|Inactive)` direction$'),
  (id, String kind, remoteId, direction, context) async {
    var member = context.world.members[id]!;

    var parsedKind = parseMediaKind(kind);

    await member.waitForConnect(remoteId);
    var track = await member.waitRemoteTrackFrom(
        remoteId, parsedKind.item2, parsedKind.item1);

    var dir = TrackMediaDirection.values
        .firstWhere((e) => e.name.toLowerCase() == direction.toLowerCase());

    await member.waitMediaDirectionTrack(dir, track);
  },
);

StepDefinitionGeneric thenLocalTrackMuteState =
    then3<String, String, String, CustomWorld>(
  RegExp(r"(\S+)'s (audio|(?:device|display) video) local track is "
      r'(not )?muted$'),
  (id, String kind, notMuted, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parseMediaKind(kind);

    var track = await member.waitLocalTrack(parsedKind.item2, parsedKind.item1);
    var muted = !notMuted.contains('not');
    expect(!track.getTrack().isEnabled(), muted);
  },
);

StepDefinitionGeneric thenTrackIsStopped = then2<String, String, CustomWorld>(
  RegExp(r"(\S+)'s (audio|(?:device|display) video) local track is "
      r'stopped$'),
  (id, kind, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parseMediaKind(kind);
    var track = await member.waitLocalTrack(parsedKind.item2, parsedKind.item1);

    await track.free();
  },
);

StepDefinitionGeneric whenMemberFreesAllLocalTracks =
    when1<String, CustomWorld>(
  RegExp(r'(\S+) frees all local tracks$'),
  (id, context) async {
    var member = context.world.members[id]!;
    await member.forgetLocalTracks();
  },
);

StepDefinitionGeneric whenMemberSwitchesDeviceWithLatency =
    when1<String, CustomWorld>(
  RegExp(r'(\S+) switches device with latency$'),
  (id, context) async {
    var member = context.world.members[id]!;
    member.setGumLatency(const Duration(seconds: 3));
    await member.switchVideoDevice();
  },
);

StepDefinitionGeneric givenGumDelay = given1<String, CustomWorld>(
  RegExp(r"(\S+)'s `getUserMedia\(\)` request has added latency$"),
  (id, context) async {
    var member = context.world.members[id]!;
    member.setGumLatency(const Duration(milliseconds: 500));
  },
);
