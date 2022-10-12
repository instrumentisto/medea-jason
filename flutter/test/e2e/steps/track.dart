import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';
import 'package:medea_flutter_webrtc/medea_flutter_webrtc.dart' as fw;

import 'package:medea_jason/medea_jason.dart';
import '../world/custom_world.dart';
import '../world/more_args.dart';

List<StepDefinitionGeneric> steps() {
  return [
    then_member_doesnt_have_live_local_tracks,
    then_member_has_remote_track,
    then_member_doesnt_have_remote_tracks_with,
    then_doesnt_have_remote_track,
    then_member_has_n_remote_tracks_from,
    then_member_has_local_tracks,
    then_remote_media_track,
    then_remote_track_stops,
    then_callback_fires_on_remote_track,
    then_has_local_track,
  ];
}

StepDefinitionGeneric then_member_has_remote_track =
    then3<String, String, String, CustomWorld>(
  RegExp(r'(\S+) has (audio|video|audio and video) remote '
      r'track(?:s)? from (\S+)'),
  (id, kind, partner_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_connect(partner_id);
    if (kind.contains('audio')) {
      await member.wait_remote_track_from(partner_id, null, MediaKind.Audio);
    }
    if (kind.contains('video')) {
      await member.wait_remote_track_from(partner_id, null, MediaKind.Video);
    }
  },
);

StepDefinitionGeneric then_member_doesnt_have_remote_tracks_with =
    then2<String, String, CustomWorld>(
  RegExp(r"(\S+) doesn't have remote tracks from (\S+)$"),
  (id, partner_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_connect(partner_id);
    var tracks_count =
        member.connection_store.remote_tracks[partner_id]!.length;
    expect(tracks_count, 0);
  },
);

StepDefinitionGeneric then_member_has_n_remote_tracks_from =
    then4<String, int, String, String, CustomWorld>(
  RegExp(r'(\S+) has {int} (live|stopped) remote tracks from (\S+)$'),
  (id, expected_count, live_or_stopped, remote_id, context) async {
    var member = context.world.members[id]!;
    await context.world.wait_for_interconnection(id);
    var live = (live_or_stopped == 'live');

    var actual_count = 0;
    for (var i = 0; i < 10 && actual_count != expected_count; ++i) {
      actual_count =
          member.connection_store.count_tracks_by_lived(live, remote_id);
      await Future.delayed(Duration(milliseconds: 500));
    }

    expect(actual_count, expected_count);
  },
);

StepDefinitionGeneric then_member_has_local_tracks =
    then2<String, int, CustomWorld>(
  RegExp(r'(\S+) has {int} local track(?:s)?$'),
  (id, expected_count, context) async {
    await context.world.wait_for_interconnection(id);
    var member = context.world.members[id]!;
    var actual_count = member.connection_store.local_tracks.length;

    expect(actual_count, expected_count);
  },
);

StepDefinitionGeneric then_doesnt_have_remote_track =
    then3<String, String, String, CustomWorld>(
  RegExp(r"(\S+) doesn't have (audio|(?:device|display) video) "
      r'remote track from (\S+)$'),
  (id, kind, partner_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_connect(partner_id);
    var parsedKind = parse_media_kind(kind);

    var tracks = member.connection_store.remote_tracks[partner_id]!.values
        .where((element) => element.isNotEmpty)
        .map((e) => e.last)
        .toList();

    var actual_count = tracks
        .where((element) =>
            element.kind() == parsedKind.item1 &&
            element.mediaSourceKind() == parsedKind.item2)
        .length;

    expect(actual_count, 0);
  },
);

StepDefinitionGeneric then_remote_media_track =
    then4<String, String, String, String, CustomWorld>(
  RegExp(r"(\S+)'s (audio|(?:display|device) video) remote track "
      r'from (\S+) is (enabled|disabled)$'),
  (id, kind, partner_id, state, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parse_media_kind(kind);

    var track = await member.wait_remote_track_from(
        partner_id, parsedKind.item2, parsedKind.item1);

    if (state == 'enabled') {
      await member.wait_enabled_track(track);
    } else {
      await member.wait_disabled_track(track);
    }
  },
);

StepDefinitionGeneric then_remote_track_stops =
    then3<String, String, String, CustomWorld>(
  RegExp(r"(\S+)'s remote (audio|(?:device|display) video) "
      r'track from (\S+) disables$'),
  (id, kind, remote_id, context) async {
    var member = context.world.members[id]!;

    var parsedKind = parse_media_kind(kind);
    var track = await member.wait_remote_track_from(
        remote_id, parsedKind.item2, parsedKind.item1);
    await member.wait_disabled_track(track);
  },
);

StepDefinitionGeneric then_callback_fires_on_remote_track =
    fix_then5<String, int, String, String, String, CustomWorld>(
  RegExp(r'`on_(enabled|disabled|muted|unmuted)` callback fires '
      r"{int} time(?:s)? on (\S+)'s "
      r'remote (audio|(?:device|display) video) track from (\S+)$'),
  (callback_kind, int times, id, kind, remote_id, context) async {
    var member = context.world.members[id]!;

    var parsedKind = parse_media_kind(kind);
    var track = await member.wait_remote_track_from(
        remote_id, parsedKind.item2, parsedKind.item1);

    await member.wait_for_track_cb_fire_count(callback_kind, track, times);
  },
);

StepDefinitionGeneric then_member_doesnt_have_live_local_tracks =
    then1<String, CustomWorld>(
  RegExp(r"(\S+) doesn't have live local tracks$"),
  (id, context) async {
    var member = context.world.members[id]!;
    var count = 0;
    member.connection_store.local_tracks.forEach((element) async {
      if (await element.getTrack().state() == fw.MediaStreamTrackState.live) {
        ++count;
      }
    });
    expect(count, 0);
  },
);

StepDefinitionGeneric then_has_local_track = then2<String, String, CustomWorld>(
  RegExp(r'(\S+) has local (audio|(?:device |display )?video)$'),
  (id, kind, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parse_media_kind(kind);

    await member.wait_local_track(parsedKind.item2, parsedKind.item1);

    if (kind == 'video') {
      await member.wait_local_track(MediaSourceKind.Display, parsedKind.item1);
    }
  },
);
