import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';

import '../api/endpoint.dart';
import '../world/custom_world.dart';

List<StepDefinitionGeneric> steps() {
  return [
    whenControlApiRemovesMember,
    whenControlApiRemovesRoom,
    whenInterconnectsKind,
    whenControlApiRemovesMemberViaApply,
    whenControlApiCreatesMemberViaApply,
    whenControlApiInterconnectsMembers,
    whenControlApiInterconnectsViaApply,
    thenControlApiSendsOnJoin,
    thenControlApiDoesntSendsOnLeave,
    thenControlApiSendsOnLeave,
    whenControlApiStartsPublishing,
    whenControlApiDeletesPublishEndpoint,
    whenControlApiDeletesPlayEndpoint
  ];
}

StepDefinitionGeneric whenControlApiRemovesMember = when1<String, CustomWorld>(
  RegExp(r'Control API removes member (\S+)$'),
  (memberId, context) async {
    await context.world.deleteMemberElement(memberId);
  },
);

StepDefinitionGeneric whenControlApiRemovesRoom = when<CustomWorld>(
  RegExp(r'Control API removes the room'),
  (context) async {
    await context.world.deleteRoomElement();
  },
);

StepDefinitionGeneric whenInterconnectsKind =
    when3<String, String, String, CustomWorld>(
  RegExp(r'Control API interconnects (audio|video) of (\S+) and (\S+)$'),
  (kind, leftMemberId, rightMemberId, context) async {
    AudioSettings? audioSetting;
    VideoSettings? videoSetting;
    if (kind == 'audio') {
      audioSetting = AudioSettings(PublishPolicy.Optional);
    } else {
      videoSetting = VideoSettings(PublishPolicy.Optional);
    }

    var memberPair = MembersPair(
      PairedMember(leftMemberId, audioSetting, videoSetting, true),
      PairedMember(rightMemberId, audioSetting, videoSetting, true),
    );
    await context.world.interconnectMembers(memberPair);
  },
);

StepDefinitionGeneric whenControlApiCreatesMemberViaApply =
    when1<String, CustomWorld>(
  r'Control API creates member (\S+) with `Apply` method$',
  (memberId, context) async {
    var memberBuilder = MemberBuilder(memberId, false, false);

    await context.world.createMember(memberBuilder);
  },
);

StepDefinitionGeneric whenControlApiRemovesMemberViaApply =
    when1<String, CustomWorld>(
  r'Control API removes (\S+) with `Apply` method$',
  (memberId, context) async {
    var spec = await context.world.getSpec();
    spec.pipeline.forEach((key, value) {
      value.pipeline.removeWhere((key, value) => key.contains(memberId));
    });
    spec.pipeline.remove(memberId);
    await context.world.apply(spec);
  },
);

StepDefinitionGeneric whenControlApiInterconnectsMembers =
    when2<String, String, CustomWorld>(
  r'Control API interconnects (\S+) and (\S+)$',
  (id, partnerId, context) async {
    var memberPair = MembersPair(
      PairedMember(id, AudioSettings(PublishPolicy.Optional),
          VideoSettings(PublishPolicy.Optional), true),
      PairedMember(partnerId, AudioSettings(PublishPolicy.Optional),
          VideoSettings(PublishPolicy.Optional), true),
    );

    await context.world.interconnectMembers(memberPair);
  },
);

StepDefinitionGeneric whenControlApiInterconnectsViaApply =
    when2<String, String, CustomWorld>(
  r'Control API interconnects (\S+) and (\S+) with '
  r'`Apply` method$',
  (id, partnerId, context) async {
    var memberPair = MembersPair(
      PairedMember(id, AudioSettings(PublishPolicy.Optional),
          VideoSettings(PublishPolicy.Optional), true),
      PairedMember(partnerId, AudioSettings(PublishPolicy.Optional),
          VideoSettings(PublishPolicy.Optional), true),
    );

    await context.world.interconnectMembersViaApply(memberPair);
  },
);

StepDefinitionGeneric thenControlApiSendsOnJoin = then1<String, CustomWorld>(
  r'Control API sends `OnJoin` callback for member (\S+)$',
  (id, context) async {
    var future = context.world.waitForOnJoin(id);
    await future.timeout(const Duration(seconds: 10));
  },
);

StepDefinitionGeneric thenControlApiDoesntSendsOnLeave =
    then1<String, CustomWorld>(
  r"Control API doesn't send `OnLeave` callback for "
  r'member (\S+)$',
  (id, context) async {
    var sendOnLeave = true;
    try {
      var future = context.world.waitForOnLeave(id, '');
      await future.timeout(const Duration(seconds: 10));
    } on TimeoutException catch (_) {
      sendOnLeave = false;
    }
    expect(sendOnLeave, isFalse);
  },
);

StepDefinitionGeneric thenControlApiSendsOnLeave =
    then2<String, String, CustomWorld>(
  r'Control API sends `OnLeave` callback with `(.+)` reason '
  r'for member (\S+)$',
  (reason, id, context) async {
    await context.world
        .waitForOnLeave(id, reason)
        .timeout(const Duration(seconds: 10));
  },
);

StepDefinitionGeneric whenControlApiStartsPublishing =
    then3<String, String, String, CustomWorld>(
  r"Control API starts (\S+)'s (audio|video|media) publishing "
  r'to (\S+)$',
  (publisherId, kind, receiverId, context) async {
    var allKinds = kind.contains('media');

    AudioSettings? audioSetting;
    if (allKinds || kind.contains('audio')) {
      audioSetting = AudioSettings(PublishPolicy.Optional);
    }

    VideoSettings? videoSetting;
    if (allKinds || kind.contains('video')) {
      videoSetting = VideoSettings(PublishPolicy.Optional);
    }

    var memberPair = MembersPair(
      PairedMember(publisherId, audioSetting, videoSetting, false),
      PairedMember(receiverId, null, null, true),
    );
    await context.world.interconnectMembers(memberPair);
  },
);

StepDefinitionGeneric whenControlApiDeletesPublishEndpoint =
    when1<String, CustomWorld>(
  r"Control API deletes (\S+)'s publish endpoint$",
  (id, context) async {
    var future = context.world.deletePublishEndpoint(id);
    await future.timeout(const Duration(seconds: 5));
  },
);

StepDefinitionGeneric whenControlApiDeletesPlayEndpoint =
    when2<String, String, CustomWorld>(
  r"Control API deletes (\S+)'s play endpoint with (\S+)$",
  (id, partnerId, context) async {
    var future = context.world.deletePlayEndpoint(id, partnerId);
    await future.timeout(const Duration(seconds: 5));
  },
);
