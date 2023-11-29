import 'package:gherkin/gherkin.dart';

import 'package:medea_jason/src/interface/media_track.dart';
import '../world/custom_world.dart';
import '../world/member.dart';
import '../world/more_args.dart';

List<StepDefinitionGeneric> steps() {
  return [
    givenTreeMembers,
    givenTwoMembers,
    givenOneMember,
  ];
}

Future<void> newGivenMember(
    String joined,
    firstMemberId,
    secondMemberId,
    thirdMemberId,
    String mediaSettings,
    disabledMediaType,
    String disabledDirection,
    StepContext<CustomWorld> context) async {
  var notEndpointDirection = '';
  if (mediaSettings.contains('publish')) {
    notEndpointDirection = 'publish';
  }
  if (mediaSettings.contains('play')) {
    notEndpointDirection = 'play';
  }

  var endpointsDisabled = mediaSettings.contains(' with no WebRTC endpoints');

  var allEndpointsDisabled = endpointsDisabled && notEndpointDirection == '';
  var isSendDisabled = endpointsDisabled &&
      (allEndpointsDisabled || notEndpointDirection == 'publish');
  var isRecvDisabled = endpointsDisabled &&
      (allEndpointsDisabled || notEndpointDirection == 'play');

  var memberBuilder =
      MemberBuilder(firstMemberId, !isSendDisabled, !isRecvDisabled);

  await context.world.createMember(memberBuilder);

  var member = context.world.members[firstMemberId]!;

  var isAudio = disabledMediaType == ' audio' || disabledMediaType == ' media';
  var isVideo = disabledMediaType == ' video' || disabledMediaType == ' media';
  var isPublish =
      disabledDirection.contains(' publishing') || disabledDirection.isEmpty;
  var isPlaying =
      disabledDirection.contains(' playing') || disabledDirection.isEmpty;

  if (mediaSettings.contains(' disabled')) {
    if (isPublish) {
      if (isAudio) {
        await member.toggleMedia(MediaKind.audio, null, false);
      }
      if (isVideo) {
        await member.toggleMedia(MediaKind.video, null, false);
      }
    }
  }

  if (mediaSettings.contains(' muted')) {
    if (isAudio) {
      await member.toggleMute(MediaKind.audio, null, true);
    }
    if (isVideo) {
      await member.toggleMute(MediaKind.video, null, true);
    }
  }

  if (joined.contains('joined ')) {
    await context.world.joinRoom(firstMemberId);
    await context.world.waitForInterconnection(firstMemberId);
  }

  if (isPlaying) {
    if (isAudio) {
      await member.toggleRemoteMedia(MediaKind.audio, null, false);
    }
    if (isVideo) {
      await member.toggleRemoteMedia(MediaKind.video, null, false);
    }
  }

  if (secondMemberId != '') {
    await newGivenMember(joined, secondMemberId, thirdMemberId, '',
        mediaSettings, disabledMediaType, disabledDirection, context);
  }
}

StepDefinitionGeneric givenOneMember =
    given5<String, String, String, String, String, CustomWorld>(
  r'(room with joined |room with |joined |)member (\S+)'
  r'( with no WebRTC endpoints| with no publish WebRTC endpoints| '
  r'with no play WebRTC endpoints| with disabled| with muted|)'
  r'( media| audio| video|)( publishing| playing|)$',
  (joined, firstMemberId, endpoints, disabledMediaType, disabledDirection,
      context) async {
    await newGivenMember(joined, firstMemberId, '', '', endpoints,
        disabledMediaType, disabledDirection, context);
  },
);

StepDefinitionGeneric givenTwoMembers =
    given6<String, String, String, String, String, String, CustomWorld>(
  r'(room with joined |room with |joined )member(s) (\S+) and '
  r'(\S+)( with no WebRTC endpoints| with no publish WebRTC endpoints| '
  r'with no play WebRTC endpoints| with disabled| with muted|)'
  r'( media| audio| video|)( publishing| playing|)$',
  (joined, firstMemberId, secondMemberId, endpoints, disabledMediaType,
      disabledDirection, context) async {
    await newGivenMember(joined, firstMemberId, secondMemberId, '', endpoints,
        disabledMediaType, disabledDirection, context);
  },
);

StepDefinitionGeneric givenTreeMembers =
    given7<String, String, String, String, String, String, String, CustomWorld>(
  r'(room with joined |room with |joined )member(s) (\S+) and '
  r'(\S+) and (\S+)'
  r'( with no WebRTC endpoints| with no publish WebRTC endpoints| '
  r'with no play WebRTC endpoints| with disabled| with muted|)'
  r'( media| audio| video|)( publishing| playing|)$',
  (joined, firstMemberId, secondMemberId, thirdMemberId, endpoints,
      disabledMediaType, disabledDirection, context) async {
    await newGivenMember(joined, firstMemberId, secondMemberId, thirdMemberId,
        endpoints, disabledMediaType, disabledDirection, context);
  },
);
