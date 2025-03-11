import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';
import 'package:retry/retry.dart';

import 'package:medea_jason/medea_jason.dart';
import '../world/custom_world.dart';
import '../world/more_args.dart';

import 'package:medea_jason/src/interface/enums.dart'
    show MediaDirection, MediaStreamTrackState;

List<StepDefinitionGeneric> steps() {
  return [
    thenMemberDoesntHaveLiveLocalTracks,
    thenMemberHasRemoteTrack,
    thenMemberDoesntHaveRemoteTracksWith,
    thenDoesntHaveRemoteTrack,
    thenMemberHasNRemoteTracksFrom,
    thenMemberHasLocalTracks,
    thenRemoteMediaTrack,
    thenRemoteTrackStops,
    thenCallbackFiresOnRemoteTrack,
    thenHasLocalTrack,
  ];
}

StepDefinitionGeneric thenMemberHasRemoteTrack =
    then3<String, String, String, CustomWorld>(
      RegExp(
        r'(\S+) has (audio|video|audio and video) remote '
        r'track(?:s)? from (\S+)',
      ),
      (id, kind, partnerId, context) async {
        var member = context.world.members[id]!;
        await member.waitForConnect(partnerId);
        if (kind.contains('audio')) {
          await member.waitRemoteTrackFrom(partnerId, null, MediaKind.audio);
        }
        if (kind.contains('video')) {
          await member.waitRemoteTrackFrom(partnerId, null, MediaKind.video);
        }
      },
    );

StepDefinitionGeneric thenMemberDoesntHaveRemoteTracksWith =
    then2<String, String, CustomWorld>(
      RegExp(r"(\S+) doesn't have remote tracks from (\S+)$"),
      (id, partnerId, context) async {
        var member = context.world.members[id]!;
        await member.waitForConnect(partnerId);
        var tracksCount =
            member.connectionStore.remoteTracks[partnerId]!.length;
        expect(tracksCount, 0);
      },
    );

StepDefinitionGeneric thenMemberHasNRemoteTracksFrom =
    then4<String, int, String, String, CustomWorld>(
      RegExp(r'(\S+) has {int} (live|stopped) remote tracks from (\S+)$'),
      (id, expectedCount, liveOrStopped, remoteId, context) async {
        var member = context.world.members[id]!;
        await member.waitForConnect(remoteId);
        var live = (liveOrStopped == 'live');

        // We might have to wait for Rust side for a little bit.
        await retry(() async {
          var actualCount = member.connectionStore.countTracksByLived(
            live,
            remoteId,
          );
          expect(actualCount, expectedCount);
        });
      },
    );

StepDefinitionGeneric thenMemberHasLocalTracks =
    then2<String, int, CustomWorld>(
      RegExp(r'(\S+) has {int} local track(?:s)?$'),
      (id, expectedCount, context) async {
        await context.world.waitForInterconnection(id);
        var member = context.world.members[id]!;
        var actualCount = member.connectionStore.localTracks.length;

        expect(actualCount, expectedCount);
      },
    );

StepDefinitionGeneric thenDoesntHaveRemoteTrack =
    then4<String, String, String, String, CustomWorld>(
      RegExp(
        r"(\S+) doesn't have (live )?(audio|(?:device|display) video) "
        r'remote track from (\S+)$',
      ),
      (id, live, kind, partnerId, context) async {
        var member = context.world.members[id]!;
        await member.waitForConnect(partnerId);
        var parsedKind = parseMediaKind(kind);

        var tracks =
            member.connectionStore.remoteTracks[partnerId]!.values
                .where((element) => element.isNotEmpty)
                .map((e) => e.last)
                .where(
                  (element) =>
                      element.kind() == parsedKind.item1 &&
                      element.mediaSourceKind() == parsedKind.item2,
                )
                .toList();

        if (isSfu && live.isNotEmpty) {
          await retry(() async {
            var length =
                tracks
                    .where(
                      (element) =>
                          !member.connectionStore.remoteTrackIsStopped(
                            partnerId,
                            element.getTrack().id(),
                          ) &&
                          element.mediaDirection() == MediaDirection.sendRecv,
                    )
                    .length;
            expect(length, 0);
          });
        } else {
          expect(tracks.length, 0);
        }
      },
    );

StepDefinitionGeneric thenRemoteMediaTrack =
    then4<String, String, String, String, CustomWorld>(
      RegExp(
        r"(\S+)'s (audio|(?:display|device) video) remote track "
        r'from (\S+) is (enabled|disabled)$',
      ),
      (id, kind, partnerId, state, context) async {
        var member = context.world.members[id]!;
        var parsedKind = parseMediaKind(kind);

        await member.waitForConnect(partnerId);

        var track = await member.waitRemoteTrackFrom(
          partnerId,
          parsedKind.item2,
          parsedKind.item1,
        );

        if (state == 'enabled') {
          await member.waitEnabledTrack(track);
        } else {
          await member.waitDisabledTrack(track);
        }
      },
    );

StepDefinitionGeneric thenRemoteTrackStops =
    then3<String, String, String, CustomWorld>(
      RegExp(
        r"(\S+)'s remote (audio|(?:device|display) video) "
        r'track from (\S+) disables$',
      ),
      (id, kind, remoteId, context) async {
        var member = context.world.members[id]!;

        var parsedKind = parseMediaKind(kind);
        var track = await member.waitRemoteTrackFrom(
          remoteId,
          parsedKind.item2,
          parsedKind.item1,
        );
        await member.waitDisabledTrack(track);
      },
    );

StepDefinitionGeneric thenCallbackFiresOnRemoteTrack =
    fixThen5<String, int, String, String, String, CustomWorld>(
      RegExp(
        r'`on_(enabled|disabled|muted|unmuted)` callback fires '
        r"{int} time(?:s)? on (\S+)'s "
        r'remote (audio|(?:device|display) video) track from (\S+)$',
      ),
      (callbackKind, int times, id, kind, remoteId, context) async {
        var member = context.world.members[id]!;
        await member.waitForConnect(remoteId);

        var parsedKind = parseMediaKind(kind);
        var track = await member.waitRemoteTrackFrom(
          remoteId,
          parsedKind.item2,
          parsedKind.item1,
        );

        await member.waitForTrackCbFireCount(callbackKind, track, times);
      },
    );

StepDefinitionGeneric thenMemberDoesntHaveLiveLocalTracks =
    then1<String, CustomWorld>(
      RegExp(r"(\S+) doesn't have live local tracks$"),
      (id, context) async {
        var member = context.world.members[id]!;
        var count = 0;
        for (var element in member.connectionStore.localTracks) {
          if (await element.getTrack().state() == MediaStreamTrackState.live) {
            ++count;
          }
        }
        expect(count, 0);
      },
    );

StepDefinitionGeneric thenHasLocalTrack = then2<String, String, CustomWorld>(
  RegExp(r'(\S+) has local (audio|(?:device |display )?video)$'),
  (id, kind, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parseMediaKind(kind);

    await member.waitLocalTrack(parsedKind.item2, parsedKind.item1);

    if (kind == 'video') {
      await member.waitLocalTrack(MediaSourceKind.display, parsedKind.item1);
    }
  },
);
