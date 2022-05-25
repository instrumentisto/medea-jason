import 'package:flutter/material.dart';
import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:gherkin/gherkin.dart';
// import 'package:json_serializable/type_helper.dart';
import 'package:medea_jason/medea_jason.dart';

import '../api/endpoint.dart';
import '../api/room.dart';
import '../parameters/user.dart';
import '../world/custom_world.dart';
import '../world/member.dart';
import '../world/custom_world.dart';
import '../world/more_args.dart';

StepDefinitionGeneric then_member_has_remote_track =
    then3<String, String, String, CustomWorld>(
  RegExp(
      r'(Alice|Bob|Carol) has (audio|video|audio and video) remote track(s) from (Alice|Bob|Carol)'),
  (id, kind, partner_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_connect(partner_id);
    var track_store = member.connection_store.remote_tracks;
    await Future.delayed(Duration(milliseconds: 1000)); // todo delete

    var kind_ = parse_media_kind(kind);
    track_store[partner_id]!
        .firstWhere((element) => element.kind() == kind_.item1);
  },
);

StepDefinitionGeneric then_member_doesnt_have_remote_tracks_with =
    then2<String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol) doesn't have remote tracks from (Alice|Bob|Carol)"),
  (id, partner_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_connect(partner_id);
    var track_store = member.connection_store.remote_tracks[partner_id]!;
    var tracks_count = track_store.length;
    if (tracks_count != 0) {
      throw '$tracks_count != 0';
    }
  },
);

StepDefinitionGeneric then_member_has_n_remote_tracks_from =
    then4<String, int, String, String, CustomWorld>(
  RegExp(
      r'(Alice|Bob|Carol) has {int} (live|stopped) remote tracks from (Alice|Bob|Carol)'),
  (id, expected_count, live_or_stopped, remote_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_connect(remote_id);
    // await member.wait_for_track_count(remote_id, 4);
    var muted;
    var stopped;
    if (live_or_stopped == 'live') {
      muted = false;
      stopped = false;
    } else {
      muted = true;
      stopped = true;
    }

    // member.connection_store.remote_tracks[remote_id]!.forEach((element) {print(element.mediaDirection());});

    // todo check muted
    var actual_count = 0;
    for (var i = 0; i < 5; ++i) {
      actual_count = member.connection_store.remote_tracks[remote_id]!
          .where((element) =>
              member.connection_store
                      .stopped_tracks[element.getTrack().id()]! ==
                  stopped)
          .length;
      if (actual_count < expected_count) {
        await Future.delayed(Duration(milliseconds: 300));
        actual_count = member.connection_store.remote_tracks[remote_id]!
            .where((element) =>
                member.connection_store
                        .stopped_tracks[element.getTrack().id()]! ==
                    stopped)
            .length;
      }
      else {
        break;
      }
    }

    print(member.connection_store.remote_tracks[remote_id]!.length);
    print('HHHERRRREEE');
    if (actual_count != expected_count) {
      throw '$actual_count != $expected_count';
    }
  },
);

StepDefinitionGeneric then_member_has_local_tracks =
    then2<String, int, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) has {int} local track(s)'),
  (id, expected_count, context) async {
    await context.world.wait_for_interconnection(id);
    var member = context.world.members[id]!;
    var actual_count = member.connection_store.local_tracks.length;

    if (actual_count != expected_count) {
      throw '$actual_count != $expected_count';
    }
  },
);

StepDefinitionGeneric then_doesnt_have_remote_track =
    then3<String, String, String, CustomWorld>(
  RegExp(r"(Alice|Bob|Carol) doesn't have (audio|device video|display video|video) remote track from (Alice|Bob|Carol)"),
  (id, kind, partner_id, context) async {
    var member = context.world.members[id]!;
    await member.wait_for_connect(partner_id);
    var kind_ = parse_media_kind(kind);
    var actual_count = member.connection_store.remote_tracks[partner_id]!.where((element) => element.kind() == kind_.item1 && element.mediaSourceKind() == kind_.item2).length;
    if (actual_count != 0) {
      throw '$actual_count != 0';
    }
  },
);

StepDefinitionGeneric then_remote_media_track =
    then4<String, String, String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s (audio|device video|display video|video) remote track from (Alice|Bob|Carol) is (enabled|disabled)"),
  (id, kind, partner_id, state, context) async {

    // todo трэки не всегда успевают появиться
    var member = context.world.members[id]!;
    await member.wait_for_connect(partner_id);

    var tracks = member.connection_store.remote_tracks[partner_id]!;

    var kind_ = parse_media_kind(kind);

    await Future.delayed(Duration(milliseconds: 500));

    var track = tracks.firstWhere((element) =>
        element.kind() == kind_.item1 &&
        element.mediaSourceKind() == kind_.item2);


    if (state == 'enabled') {
      while (track.mediaDirection() != TrackMediaDirection.SendRecv) {
        await Future.delayed(Duration(milliseconds: 100));
      }
    } else {
      while (track.mediaDirection() == TrackMediaDirection.SendRecv) {
        await Future.delayed(Duration(milliseconds: 100));
      }
    }
  },
);

StepDefinitionGeneric then_remote_track_stops =
    then3<String, String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s remote (audio|device video|display video|video) track from (Alice|Bob|Carol) disables"),
  (id, kind, remote_id, context) async {
    var member = context.world.members[id]!;

    var kind_ = parse_media_kind(kind);
    var track = member.connection_store.remote_tracks[remote_id]!.firstWhere(
        (element) =>
            element.kind() == kind_.item1 &&
            element.mediaSourceKind() == kind_.item2);
    if (track.mediaDirection() != TrackMediaDirection.SendOnly) {
      throw 1042;
    }
  },
);

StepDefinitionGeneric then_callback_fires_on_remote_track =
    fix_then5<String, int, String, String, String, CustomWorld>(
  RegExp(
      r"`on_(enabled|disabled|muted|unmuted)` callback fires {int} time(s) on (Alice|Bob|Carol)'s remote (audio|device video|display video|video) track from (Alice|Bob|Carol)"),
  (callback_kind, int times, id, kind, remote_id, context) async {
    var member = context.world.members[id]!;
    await context.world.wait_for_interconnection(id);

    var kind_ = parse_media_kind(kind);
    var track = member.connection_store.remote_tracks[remote_id]!.firstWhere(
        (element) =>
            element.kind() == kind_.item1 &&
            element.mediaSourceKind() == kind_.item2);

    var count = member.connection_store
        .callback_counter[track.getTrack().id()]![callback_kind]!;
    while (count != times) {
      await Future.delayed(Duration(milliseconds: 100));
      count = member.connection_store
          .callback_counter[track.getTrack().id()]![callback_kind]!;
    }
  },
);

// todo recheck
StepDefinitionGeneric then_has_local_track =
    then2<String, String, CustomWorld>(
  RegExp(r'(Alice|Bob|Carol) has local (audio|device video|display video|video)'),
  (id, kind, context) async {
    var member = context.world.members[id]!;
    var kind_ = parse_media_kind(kind);

    if (kind == 'video') {
      member.connection_store.local_tracks.firstWhere((element) => element.kind() == kind_.item1 && element.mediaSourceKind() == kind_.item2);
      member.connection_store.local_tracks.firstWhere((element) => element.kind() == kind_.item1 && element.mediaSourceKind() == MediaSourceKind.Display);
    } else {
      member.connection_store.local_tracks.firstWhere((element) => element.kind() == kind_.item1 && element.mediaSourceKind() == kind_.item2);
    }

  },
);
