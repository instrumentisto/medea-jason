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

Future<void> new_given_member(
    String joined,
    first_member_id,
    second_member_id,
    third_member_id,
    String media_settings,
    disabled_media_type,
    String disabled_direction,
    StepContext<CustomWorld> context) async {
  var not_endpoint_direction = '';
  if (media_settings.contains('publish')) {
    not_endpoint_direction = 'publish';
  }
  if (media_settings.contains('play')) {
    not_endpoint_direction = 'play';
  }

  var endpoints_disabled = media_settings.contains(' with no WebRTC endpoints');

  var all_endpoints_disabled =
      endpoints_disabled && not_endpoint_direction == '';
  var is_send_disabled = endpoints_disabled &&
      (all_endpoints_disabled || not_endpoint_direction == 'publish');
  var is_recv_disabled = endpoints_disabled &&
      (all_endpoints_disabled || not_endpoint_direction == 'play');

  var member_builder =
      MemberBuilder(first_member_id, !is_send_disabled, !is_recv_disabled);

  await context.world.create_member(member_builder);
  if (joined.contains('joined ')) {
    await context.world.join_room(first_member_id);
    await context.world.wait_for_interconnection(first_member_id);
  }

  var member = context.world.members[first_member_id]!;

  var is_audio =
      disabled_media_type == ' audio' || disabled_media_type == ' media';
  var is_video =
      disabled_media_type == ' video' || disabled_media_type == ' media';

  if (media_settings.contains(' disabled')) {
    var is_publish = disabled_direction.contains(' publishing') ||
        disabled_direction.isEmpty;
    var is_playing =
        disabled_direction.contains(' playing') || disabled_direction.isEmpty;

    if (is_publish) {
      if (is_audio) {
        await member.toggle_media(MediaKind.Audio, null, false);
      }
      if (is_video) {
        await member.toggle_media(MediaKind.Video, null, false);
      }
    }
    if (is_playing) {
      if (is_audio) {
        await member.toggle_remote_media(MediaKind.Audio, null, false);
      }
      if (is_video) {
        await member.toggle_remote_media(MediaKind.Video, null, false);
      }
    }
  }

  if (media_settings.contains(' muted')) {
    if (is_audio) {
      await member.toggle_mute(MediaKind.Audio, null, true);
    }
    if (is_video) {
      await member.toggle_mute(MediaKind.Video, null, true);
    }
  }

  if (second_member_id != '') {
    await new_given_member(joined, second_member_id, third_member_id, '',
        media_settings, disabled_media_type, disabled_direction, context);
  }
  // await Future.delayed(Duration(seconds: 1));
}

StepDefinitionGeneric givenOneMember =
    given5<String, String, String, String, String, CustomWorld>(
  r'(room with joined |room with |joined |)member (\S+)'
  r'( with no WebRTC endpoints| with no publish WebRTC endpoints| '
  r'with no play WebRTC endpoints| with disabled| with muted|)'
  r'( media| audio| video|)( publishing| playing|)$',
  (joined, first_member_id, endpoints, disabled_media_type, disabled_direction,
      context) async {
    await new_given_member(joined, first_member_id, '', '', endpoints,
        disabled_media_type, disabled_direction, context);
  },
);

StepDefinitionGeneric givenTwoMembers =
    given6<String, String, String, String, String, String, CustomWorld>(
  r'(room with joined |room with |joined )member(s) (\S+) and '
  r'(\S+)( with no WebRTC endpoints| with no publish WebRTC endpoints| '
  r'with no play WebRTC endpoints| with disabled| with muted|)'
  r'( media| audio| video|)( publishing| playing|)$',
  (joined, first_member_id, second_member_id, endpoints, disabled_media_type,
      disabled_direction, context) async {
    await new_given_member(joined, first_member_id, second_member_id, '',
        endpoints, disabled_media_type, disabled_direction, context);
  },
);

StepDefinitionGeneric givenTreeMembers =
    given7<String, String, String, String, String, String, String, CustomWorld>(
  r'(room with joined |room with |joined )member(s) (\S+) and '
  r'(\S+) and (\S+)'
  r'( with no WebRTC endpoints| with no publish WebRTC endpoints| '
  r'with no play WebRTC endpoints| with disabled| with muted|)'
  r'( media| audio| video|)( publishing| playing|)$',
  (joined, first_member_id, second_member_id, third_member_id, endpoints,
      disabled_media_type, disabled_direction, context) async {
    await new_given_member(
        joined,
        first_member_id,
        second_member_id,
        third_member_id,
        endpoints,
        disabled_media_type,
        disabled_direction,
        context);
  },
);
