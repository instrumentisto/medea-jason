import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:gherkin/gherkin.dart';
import 'package:medea_jason/src/interface/track_kinds.dart';

import '../parameters/user.dart';
import '../world/custom_world.dart';
import '../world/member.dart';
import '../world/more_args.dart';

Future<void> new_given_member(
    joined,
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

  var endpoints_disabled = media_settings == ' with no WebRTC endpoints';
  var all_endpoints_disabled =
      endpoints_disabled && not_endpoint_direction == '';
  var is_send_disabled = endpoints_disabled &&
      (all_endpoints_disabled || not_endpoint_direction == 'publish');
  var is_recv_disabled = endpoints_disabled &&
      (all_endpoints_disabled || not_endpoint_direction == 'play');

  var member_builder =
      MyBuilder(first_member_id, !is_send_disabled, !is_recv_disabled);

  await context.world.create_member(member_builder);
  if (joined == 'joined ') {
    await context.world.join_room(first_member_id);
    await context.world.wait_for_interconnection(first_member_id);
  }

  var member = context.world.members[first_member_id]!;

  var is_audio =
      disabled_media_type == ' audio' || disabled_media_type == ' media';
  var is_video =
      disabled_media_type == ' video' || disabled_media_type == ' media';

  if (media_settings.contains('disabled')) {
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

  if (media_settings.contains('muted')) {
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
}

StepDefinitionGeneric fillField1 =
    given5<String, String, String, String, String, CustomWorld>(
  r'room with (joined |)member (Alice|Bob|Carol)( with no WebRTC endpoints| with no publish WebRTC endpoints| with no play WebRTC endpoints| with disabled| with muted|)( media| audio| video|)( publishing| playing|)',
  (joined, first_member_id, webrtc, disabled_media_type, disabled_direction,
      context) async {
    await new_given_member(joined, first_member_id, '', '', webrtc,
        disabled_media_type, disabled_direction, context);
  },
);

StepDefinitionGeneric fillField2 =
    given6<String, String, String, String, String, String, CustomWorld>(
  RegExp(
      r'room with (joined |)member(s) (Alice|Bob|Carol) and (Alice|Bob|Carol)( with no WebRTC endpoints| with no publish WebRTC endpoints| with no play WebRTC endpoints| disabled| muted|)( media| audio| video|)( publishing| playing|)'),
  (joined, first_member_id, second_member_id, webrtc, disabled_media_type,
      disabled_direction, context) async {
    await new_given_member(joined, first_member_id, second_member_id, '',
        webrtc, disabled_media_type, disabled_direction, context);
  },
);

StepDefinitionGeneric fillField3 =
    given7<String, String, String, String, String, String, String, CustomWorld>(
  RegExp(
      r'room with (joined |)member(s) (Alice|Bob|Carol) and (Alice|Bob|Carol) and (Alice|Bob|Carol)( with no WebRTC endpoints| with no publish WebRTC endpoints| with no play WebRTC endpoints| disabled| muted|)( media| audio| video|)( publishing| playing|)'),
  (joined, first_member_id, second_member_id, third_member_id, webrtc,
      disabled_media_type, disabled_direction, context) async {
    await new_given_member(
        joined,
        first_member_id,
        second_member_id,
        third_member_id,
        webrtc,
        disabled_media_type,
        disabled_direction,
        context);
  },
);

StepDefinitionGeneric fillField01 =
    given5<String, String, String, String, String, CustomWorld>(
  r'(joined |)member(s) (Alice|Bob|Carol)( with no WebRTC endpoints| with no publish WebRTC endpoints| with no play WebRTC endpoints| with disabled| with muted|)( media| audio| video|)( publishing| playing|)',
  (joined, first_member_id, webrtc, disabled_media_type, disabled_direction,
      context) async {
    await new_given_member(joined, first_member_id, '', '', webrtc,
        disabled_media_type, disabled_direction, context);
  },
);

StepDefinitionGeneric fillField02 =
    given6<String, String, String, String, String, String, CustomWorld>(
  RegExp(
      r'(joined |)member(s) (Alice|Bob|Carol) and (Alice|Bob|Carol)( with no WebRTC endpoints| with no publish WebRTC endpoints| with no play WebRTC endpoints| disabled| muted|)( media| audio| video|)( publishing| playing|)'),
  (joined, first_member_id, second_member_id, webrtc, disabled_media_type,
      disabled_direction, context) async {
    await new_given_member(joined, first_member_id, second_member_id, '',
        webrtc, disabled_media_type, disabled_direction, context);
  },
);

StepDefinitionGeneric fillField03 =
    given7<String, String, String, String, String, String, String, CustomWorld>(
  RegExp(
      r'(joined |)member(s) (Alice|Bob|Carol) and (Alice|Bob|Carol) and (Alice|Bob|Carol)( with no WebRTC endpoints| with no publish WebRTC endpoints| with no play WebRTC endpoints| disabled| muted|)( media| audio| video|)( publishing| playing|)'),
  (joined, first_member_id, second_member_id, third_member_id, webrtc,
      disabled_media_type, disabled_direction, context) async {
    await new_given_member(
        joined,
        first_member_id,
        second_member_id,
        third_member_id,
        webrtc,
        disabled_media_type,
        disabled_direction,
        context);
  },
);


// todo recheck Control API removes member Alice (Создает нового пользователя)