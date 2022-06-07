import 'package:flutter_webrtc/flutter_webrtc.dart' as fw;
import 'package:gherkin/gherkin.dart';
import 'package:medea_jason/medea_jason.dart';
import '../world/custom_world.dart';
import 'package:flutter_test/flutter_test.dart';
import '../world/more_args.dart';

StepDefinitionGeneric then_member_has_remote_track =
    then3<String, String, String, CustomWorld>(
  RegExp(
      r'(Alice|Bob|Carol) has (audio|video|audio and video) remote track(s) from (Alice|Bob|Carol)'),
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
  RegExp(
      r"(Alice|Bob|Carol) doesn't have remote tracks from (Alice|Bob|Carol)"),
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
  RegExp(
      r'(Alice|Bob|Carol) has {int} (live|stopped) remote tracks from (Alice|Bob|Carol)'),
  (id, expected_count, live_or_stopped, remote_id, context) async {
    var member = context.world.members[id]!;
    await context.world.wait_for_interconnection(id);
    bool stopped;
    if (live_or_stopped == 'live') {
      stopped = false;
    } else {
      stopped = true;
    }

    var actual_count = 0;
    for (var i = 0; i < 5 && actual_count != expected_count; ++i) {
      actual_count = 0;
      member.connection_store.remote_tracks[remote_id]!.forEach((key, value) {
        var stopped_length = member.connection_store.callback_counter[key]!['stopped']!;
        var all_length = value.length;
        var track_stopped = stopped_length == all_length;
        if (stopped == track_stopped) {
          actual_count += 1;
        }
      });
      await Future.delayed(Duration(milliseconds: 300));
    }

    expect(actual_count, expected_count);
  },
);

StepDefinitionGeneric then_member_has_local_tracks =
    then2<String, int, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) has {int} local track(s)'),
  (id, expected_count, context) async {
    await context.world.wait_for_interconnection(id);
    var member = context.world.members[id]!;
    var actual_count = member.connection_store.local_tracks.length;

    expect(actual_count, expected_count);
  },
);

StepDefinitionGeneric then_doesnt_have_remote_track =
    then3<String, String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol) doesn't have (audio|device video|display video|video) remote track from (Alice|Bob|Carol)"),
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
  RegExp(
      r"(Alice|Bob|Carol)'s (audio|device video|display video|video) remote track from (Alice|Bob|Carol) is (enabled|disabled)"),
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
  RegExp(
      r"(Alice|Bob|Carol)'s remote (audio|device video|display video|video) track from (Alice|Bob|Carol) disables"),
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
  RegExp(
      r"`on_(enabled|disabled|muted|unmuted)` callback fires {int} time(s) on (Alice|Bob|Carol)'s remote (audio|device video|display video|video) track from (Alice|Bob|Carol)"),
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
  RegExp(r"(Alice|Bob|Carol) doesn't have live local tracks"),
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
  RegExp(
      r'(Alice|Bob|Carol) has local (audio|device video|display video|video)'),
  (id, kind, context) async {
    var member = context.world.members[id]!;
    var parsedKind = parse_media_kind(kind);

    expect(
        member.connection_store.local_tracks
            .where((element) =>
                element.kind() == parsedKind.item1 &&
                element.mediaSourceKind() == parsedKind.item2)
            .isNotEmpty,
        isTrue);

    if (kind == 'video') {
      expect(
          member.connection_store.local_tracks
              .where((element) =>
                  element.kind() == parsedKind.item1 &&
                  element.mediaSourceKind() == MediaSourceKind.Display)
              .isNotEmpty,
          isTrue);
    }
  },
);