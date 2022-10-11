import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';

import '../api/endpoint.dart';
import '../world/custom_world.dart';

List<StepDefinitionGeneric> steps() {
  return [
    when_control_api_removes_member,
    when_control_api_removes_room,
    when_interconnects_kind,
    when_control_api_removes_member_via_apply,
    when_control_api_interconnects_members,
    when_control_api_interconnects_via_apply,
    then_control_api_sends_on_join,
    then_control_api_doesnt_sends_on_leave,
    then_control_api_sends_on_leave,
    when_control_api_starts_publishing,
    when_control_api_deletes_publish_endpoint,
    when_control_api_deletes_play_endpoint
  ];
}

StepDefinitionGeneric when_control_api_removes_member =
    when1<String, CustomWorld>(
  RegExp(r'Control API removes member (\S+)$'),
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
  RegExp(r'Control API interconnects (audio|video) of (\S+) and (\S+)$'),
  (kind, left_member_id, right_member_id, context) async {
    AudioSettings? audio_setting;
    VideoSettings? video_setting;
    if (kind == 'audio') {
      audio_setting = AudioSettings(PublishPolicy.Optional);
    } else {
      video_setting = VideoSettings(PublishPolicy.Optional);
    }

    var member_pair = MembersPair(
      PairedMember(left_member_id, audio_setting, video_setting, true),
      PairedMember(right_member_id, audio_setting, video_setting, true),
    );
    await context.world.interconnect_members(member_pair);
  },
);

StepDefinitionGeneric when_control_api_removes_member_via_apply =
    when1<String, CustomWorld>(
  r'Control API removes (\S+) with `Apply` method$',
  (member_id, context) async {
    var spec = await context.world.get_spec();
    spec.pipeline.forEach((key, value) {
      value.pipeline.removeWhere((key, value) => key.contains(member_id));
    });
    spec.pipeline.remove(member_id);
    await context.world.apply(spec);
  },
);

StepDefinitionGeneric when_control_api_interconnects_members =
    when2<String, String, CustomWorld>(
  r'Control API interconnects (\S+) and (\S+)$',
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
  r'Control API interconnects (\S+) and (\S+) with '
  r'`Apply` method$',
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
  r'Control API sends `OnJoin` callback for member (\S+)$',
  (id, context) async {
    var future = context.world.wait_for_on_join(id);
    await future.timeout(Duration(seconds: 10));
  },
);

StepDefinitionGeneric then_control_api_doesnt_sends_on_leave =
    then1<String, CustomWorld>(
  r"Control API doesn't send `OnLeave` callback for "
  r'member (\S+)$',
  (id, context) async {
    var sendOnLeave = true;
    try {
      var future = context.world.wait_for_on_leave(id, '');
      await future.timeout(Duration(seconds: 10));
    } on TimeoutException catch (_) {
      sendOnLeave = false;
    }
    expect(sendOnLeave, isFalse);
  },
);

StepDefinitionGeneric then_control_api_sends_on_leave =
    then2<String, String, CustomWorld>(
  r'Control API sends `OnLeave` callback with `(.+)` reason '
  r'for member (\S+)$',
  (reason, id, context) async {
    await context.world
        .wait_for_on_leave(id, reason)
        .timeout(Duration(seconds: 10));
  },
);

StepDefinitionGeneric when_control_api_starts_publishing =
    then3<String, String, String, CustomWorld>(
  r"Control API starts (\S+)'s (audio|video|media) publishing "
  r'to (\S+)$',
  (publisher_id, kind, receiver_id, context) async {
    var all_kinds = kind.contains('media');

    AudioSettings? audio_setting;
    if (all_kinds || kind.contains('audio')) {
      audio_setting = AudioSettings(PublishPolicy.Optional);
    }

    VideoSettings? video_setting;
    if (all_kinds || kind.contains('video')) {
      video_setting = VideoSettings(PublishPolicy.Optional);
    }

    var member_pair = MembersPair(
      PairedMember(publisher_id, audio_setting, video_setting, false),
      PairedMember(receiver_id, null, null, true),
    );
    await context.world.interconnect_members(member_pair);
  },
);

StepDefinitionGeneric when_control_api_deletes_publish_endpoint =
    when1<String, CustomWorld>(
  r"Control API deletes (\S+)'s publish endpoint$",
  (id, context) async {
    var future = context.world.delete_publish_endpoint(id);
    await future.timeout(Duration(milliseconds: 200));
  },
);

StepDefinitionGeneric when_control_api_deletes_play_endpoint =
    when2<String, String, CustomWorld>(
  r"Control API deletes (\S+)'s play endpoint with (\S+)$",
  (id, partner_id, context) async {
    var future = context.world.delete_play_endpoint(id, partner_id);
    await future.timeout(Duration(milliseconds: 200));
  },
);
