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

StepDefinitionGeneric when_control_api_removes_member = when1<String, CustomWorld>(
  RegExp(
      r'Control API removes member (Alice|Bob|Carol)'),
  (
      member_id,
      context) async {
        await context.world.delete_member_element(member_id);
      },
);

StepDefinitionGeneric when_control_api_removes_room = when<CustomWorld>(
  RegExp(
      r'Control API removes the room'),
  (
      context) async {
        await context.world.delete_room_element();
      },
);

StepDefinitionGeneric when_interconnects_kind = when3<String, String, String, CustomWorld>(
  RegExp(
      r'Control API interconnects (audio|video) of (Alice|Bob|Carol) and (Alice|Bob|Carol)'),
  (     kind,
         left_member_id,
         right_member_id,
      context) async {
        AudioSettings? audio_setting = null;
        VideoSettings? video_setting = null;
        if(kind == 'audio') {
          audio_setting = AudioSettings();
        }
        else {
          video_setting = VideoSettings();
        }
        // todo
      },
);

StepDefinitionGeneric when_control_api_removes_member_via_apply = when1<String, CustomWorld>(
      r'Control API removes (Alice|Bob|Carol) with `Apply` method',
  (     member_id,
      context) async {
        var spec = await context.world.get_spec();
        spec.pipeline.remove(member_id);
        await context.world.apply(spec);
      },
);


// #[when(regex = r"^$")]
// async fn when_interconnects_kind(
//     world: &mut World,
//     kind: String,
//     left_member_id: String,
//     right_member_id: String,
// ) {
//     let send_video = kind.contains("video").then(|| VideoSettings {
//         publish_policy: proto::PublishPolicy::Optional,
//     });
//     let send_audio = kind.contains("audio").then(|| AudioSettings {
//         publish_policy: proto::PublishPolicy::Optional,
//     });

//     world
//         .interconnect_members(MembersPair {
//             left: PairedMember {
//                 id: left_member_id,
//                 recv: true,
//                 send_video: send_video.clone(),
//                 send_audio: send_audio.clone(),
//             },
//             right: PairedMember {
//                 id: right_member_id,
//                 recv: true,
//                 send_video,
//                 send_audio,
//             },
//         })
//         .await
//         .unwrap();
// }

// #[then(regex = "^Control API sends `OnLeave` callback with `(.+)` reason \
//                  for member (\\S+)$")]
// async fn then_control_api_sends_on_leave(
//     world: &mut World,
//     reason: String,
//     id: String,
// ) {
//     // Assertion is done inside `wait_for_on_leave()` method.
//     timeout(Duration::from_secs(10), world.wait_for_on_leave(id, reason))
//         .await
//         .unwrap();
// }

// #[then(regex = "^Control API doesn't send `OnLeave` callback for \
//                  member (\\S+)$")]
// async fn then_control_api_doesnt_sends_on_leave(world: &mut World, id: String) {
//     assert!(timeout(
//         Duration::from_millis(300),
//         world.wait_for_on_leave(id, String::new()),
//     )
//     .await
//     .is_err());
// }

// #[then(regex = r"^Control API sends `OnJoin` callback for member (\S+)$")]
// async fn then_control_api_sends_on_join(world: &mut World, id: String) {
//     timeout(Duration::from_secs(10), world.wait_for_on_join(id))
//         .await
//         .unwrap()
// }

// #[when(regex = "^Control API starts (\\S+)'s (audio|video|media) publishing \
//                  to (\\S+)$")]
// async fn when_control_api_starts_publishing(
//     world: &mut World,
//     publisher_id: String,
//     kind: String,
//     receiver_id: String,
// ) {
//     let all_kinds = kind.contains("media");
//     let send_audio =
//         (all_kinds || kind.contains("audio")).then(AudioSettings::default);
//     let send_video =
//         (all_kinds || kind.contains("video")).then(VideoSettings::default);
//     world
//         .interconnect_members(MembersPair {
//             left: PairedMember {
//                 id: publisher_id,
//                 recv: false,
//                 send_audio,
//                 send_video,
//             },
//             right: PairedMember {
//                 id: receiver_id,
//                 recv: true,
//                 send_video: None,
//                 send_audio: None,
//             },
//         })
//         .await
//         .unwrap();
// }

// #[when(regex = r"^Control API interconnects (\S+) and (\S+)$")]
// async fn when_control_api_interconnects_members(
//     world: &mut World,
//     id: String,
//     partner_id: String,
// ) {
//     world
//         .interconnect_members(MembersPair {
//             left: PairedMember {
//                 id,
//                 recv: true,
//                 send_video: Some(VideoSettings::default()),
//                 send_audio: Some(AudioSettings::default()),
//             },
//             right: PairedMember {
//                 id: partner_id,
//                 recv: true,
//                 send_video: Some(VideoSettings::default()),
//                 send_audio: Some(AudioSettings::default()),
//             },
//         })
//         .await
//         .unwrap();
// }

// #[when(regex = "^Control API interconnects (\\S+) and (\\S+) with \
//                  `Apply` method$")]
// async fn when_control_api_interconnects_via_apply(
//     world: &mut World,
//     id: String,
//     partner_id: String,
// ) {
//     world
//         .interconnect_members_via_apply(MembersPair {
//             left: PairedMember {
//                 id,
//                 recv: true,
//                 send_video: Some(VideoSettings::default()),
//                 send_audio: Some(AudioSettings::default()),
//             },
//             right: PairedMember {
//                 id: partner_id,
//                 recv: true,
//                 send_video: Some(VideoSettings::default()),
//                 send_audio: Some(AudioSettings::default()),
//             },
//         })
//         .await;
// }

// #[when(regex = r"^Control API deletes (\S+)'s publish endpoint$")]
// async fn when_control_api_deletes_publish_endpoint(
//     world: &mut World,
//     id: String,
// ) {
//     world.delete_publish_endpoint(&id).await;
//     sleep(Duration::from_millis(200)).await;
// }

// #[when(regex = r"^Control API deletes (\S+)'s play endpoint with (\S+)$")]
// async fn when_control_api_deletes_play_endpoint(
//     world: &mut World,
//     id: String,
//     partner_id: String,
// ) {
//     world.delete_play_endpoint(&id, &partner_id).await;
//     sleep(Duration::from_millis(200)).await;
// }
