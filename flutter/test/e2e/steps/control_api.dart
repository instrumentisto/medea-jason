// use std::time::Duration;

import 'package:flutter/material.dart';
import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:gherkin/gherkin.dart';

import '../api/endpoint.dart';
import '../api/room.dart';
import '../parameters/user.dart';
import '../world/custom_world.dart';
import '../world/member.dart';
import '../world/custom_world.dart';

StepDefinitionGeneric when_control_api_removes_member =
    when1<String, CustomWorld>(
  RegExp(r'Control API removes member (Alice|Bob|Carol)'),
  (member_id, context) async {
    await context.world.delete_member_element(member_id);
  },
);

StepDefinitionGeneric when_control_api_removes_room = when<CustomWorld>(
  RegExp(r'Control API removes the room'),
  (context) async {
    await context.world.delete_room_element();
  },
);

StepDefinitionGeneric when_interconnects_kind =
    when3<String, String, String, CustomWorld>(
  RegExp(
      r'Control API interconnects (audio|video) of (Alice|Bob|Carol) and (Alice|Bob|Carol)'),
  (kind, left_member_id, right_member_id, context) async {
    AudioSettings? audio_setting = null;
    VideoSettings? video_setting = null;
    if (kind == 'audio') {
      audio_setting = AudioSettings(PublishPolicy.Optional);
    } else {
      video_setting = VideoSettings(PublishPolicy.Optional);
    }
    // todo
  },
);

StepDefinitionGeneric when_control_api_removes_member_via_apply =
    when1<String, CustomWorld>(
  r'Control API removes (Alice|Bob|Carol) with `Apply` method',
  (member_id, context) async {
    var spec = await context.world.get_spec();
    spec.pipeline.forEach((key, value) {
      value.pipeline.removeWhere((key, value) => key.contains(member_id)); // todo kostyl
    });
    spec.pipeline.remove(member_id);
    await context.world.apply(spec);
  },
);

StepDefinitionGeneric when_control_api_interconnects_members =
    when2<String, String, CustomWorld>(
  r'Control API interconnects (Alice|Bob|Carol) and (Alice|Bob|Carol)',
  (id, partner_id, context) async {
    var member_pair = MembersPair(
      PairedMember(id, AudioSettings(PublishPolicy.Optional),
          VideoSettings(PublishPolicy.Optional), true),
      PairedMember(partner_id, AudioSettings(PublishPolicy.Optional),
          VideoSettings(PublishPolicy.Optional), true),
    );

    await context.world.interconnect_members(member_pair);
  },
);

StepDefinitionGeneric when_control_api_interconnects_via_apply =
    when2<String, String, CustomWorld>(
  r'Control API interconnects (Alice|Bob|Carol) and (Alice|Bob|Carol) with `Apply` method',
  (id, partner_id, context) async {
    var member_pair = MembersPair(
      PairedMember(id, AudioSettings(PublishPolicy.Optional),
          VideoSettings(PublishPolicy.Optional), true),
      PairedMember(partner_id, AudioSettings(PublishPolicy.Optional),
          VideoSettings(PublishPolicy.Optional), true),
    );

    await context.world.interconnect_members_via_apply(member_pair);
  },
);

StepDefinitionGeneric then_control_api_sends_on_join =
    then1<String, CustomWorld>(
  r'Control API sends `OnJoin` callback for member (Alice|Bob|Carol)',
  (id, context) async {
    var future = context.world.wait_for_on_join(id);
    await future.timeout(Duration(seconds: 10));
  },
);

StepDefinitionGeneric then_control_api_doesnt_sends_on_leave =
    then1<String, CustomWorld>(
  r"Control API doesn't send `OnLeave` callback for member (Alice|Bob|Carol)",
  (id, context) async {
    var res = false;
    try {
      var future = context.world.wait_for_on_leave(id, '');
      await future.timeout(Duration(seconds: 10));
    } catch (e) {
      if (e.toString().contains('TimeoutException')) {
        res = true;
      }
    }
    if (!res) {
      throw 'send onLeav';
    }
  },
);

StepDefinitionGeneric then_control_api_sends_on_leave =
    then2<String, String, CustomWorld>(
  r'Control API sends `OnLeave` callback with `(.+)` reason for member (Alice|Bob|Carol)',
  (reason, id, context) async {
    await context.world
        .wait_for_on_leave(id, reason)
        .timeout(Duration(seconds: 10));
  },
);

StepDefinitionGeneric when_control_api_starts_publishing =
    then3<String, String, String, CustomWorld>(
  r"Control API starts (Alice|Bob|Carol)'s (audio|video|media) publishing to (Alice|Bob|Carol)",
  (publisher_id, kind, receiver_id, context) async {
    var all_kinds = kind.contains('media');

    AudioSettings? a_setting;
    if (all_kinds || kind.contains('audio')) {
      a_setting = AudioSettings(PublishPolicy.Optional);
    }

    VideoSettings? v_setting;
    if (all_kinds || kind.contains('video')) {
      v_setting = VideoSettings(PublishPolicy.Optional);
    }

    var member_pair = MembersPair(
      PairedMember(publisher_id, a_setting, v_setting, false),
      PairedMember(receiver_id, null, null, true),
    );
    await context.world.interconnect_members(member_pair);

  },
);

StepDefinitionGeneric when_control_api_deletes_publish_endpoint =
    when1<String, CustomWorld>(
  r"Control API deletes (Alice|Bob|Carol)'s publish endpoint",
  (id, context) async {
    var future = context.world.delete_publish_endpoint(id);
    await future.timeout(Duration(milliseconds: 200));

    // todo delete
    {
      var member = context.world.members[id]!;
      var iter = member.connection_store.connects.keys.iterator;
      while (iter.moveNext()) {
        var key = iter.current;
        await context.world
            .delete_play_endpoint(id, key);
      }
    }
  },
);

StepDefinitionGeneric when_control_api_deletes_play_endpoint =
    when2<String, String, CustomWorld>(
  r"Control API deletes (Alice|Bob|Carol)'s play endpoint with (Alice|Bob|Carol)",
  (id, partner_id, context) async {

    // todo delete
    {
      var future = context.world.delete_publish_endpoint(id);
      await future.timeout(Duration(milliseconds: 200));
    }
    var future = context.world.delete_play_endpoint(id, partner_id);
    await future.timeout(Duration(milliseconds: 200));

  },
);
