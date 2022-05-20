
import 'package:gherkin/gherkin.dart';
import 'package:medea_jason/src/interface/track_kinds.dart';

import '../world/custom_world.dart';
import '../world/more_args.dart';

StepDefinitionGeneric when_enables_or_mutes = when4<String, String, String, String, CustomWorld>(
  RegExp(
      r'(Alice|Bob|Carol) (enables|disables|mutes|unmutes) (audio|video)( and awaits it completes| and awaits it errors|)'),
  (id, action, audio_or_video, awaits, context) async {
    var kind = parse_media_kind(audio_or_video);
    var member = context.world.members[id]!;
    
    var awats = awaits.contains('awaits');
    var error = awaits.contains('errors');

    try {
      switch(action) { 
        case 'enables': { 
          print("ENABLE");
          print(kind);
          var future =  member.toggle_media(kind.item1, null, true);
          if (awats) {
            await future;
          }
        } 
        break; 
        
        case 'disables': {
          await member.toggle_media(kind.item1, null, false);
        } 
        break; 

        case 'mutes': { 
          await member.toggle_mute(kind.item1, null, true);
        } 
        break; 
            
        default: { 
          await member.toggle_mute(kind.item1, null, false);
        }
        break; 
      } 
    } catch(e) {
      if (!error) { rethrow; };
    }

  },
);


// #[then(regex = "^(\\S+)'s (audio|video) from (\\S+) has \
//                  `(SendRecv|SendOnly|RecvOnly|Inactive)` direction$")]
// async fn then_remote_media_direction_is(
//     world: &mut World,
//     id: String,
//     kind: String,
//     remote_id: String,
//     direction: String,
// ) {
//     let media_kind = kind.parse().unwrap();
//     let media_direction = match direction.as_str() {
//         "SendRecv" => MediaDirection::SendRecv,
//         "SendOnly" => MediaDirection::SendOnly,
//         "RecvOnly" => MediaDirection::RecvOnly,
//         _inactive => MediaDirection::Inactive,
//     };

//     let member = world.get_member(&id).unwrap();
//     let connection = member
//         .connections()
//         .wait_for_connection(remote_id)
//         .await
//         .unwrap();
//     let tracks_store = connection.tracks_store().await.unwrap();
//     let track = tracks_store
//         .get_track(media_kind, MediaSourceKind::Device)
//         .await
//         .unwrap();
//     track
//         .wait_for_media_direction(media_direction)
//         .await
//         .unwrap();
// }


StepDefinitionGeneric when_member_enables_remote_track = when3<String, String, String, CustomWorld>(
  RegExp(
      r'(Alice|Bob|Carol) (enables|disables) remote (audio|device video|display video|video)'),
  (id, toggle, String kind, context) async {
    var kind_ = parse_media_kind(kind);
    var member = context.world.members[id]!;

    if (toggle == 'enables') {
      if (kind_.item1 == MediaKind.Audio) {
        await member.room.enableRemoteAudio();
      }
      else {
        await member.room.enableRemoteVideo();
      }
    } else {
      if (kind_.item1 == MediaKind.Audio) {
        await member.room.disableRemoteAudio();
      }
      else {
        await member.room.disableRemoteVideo();
      }
    }
  },
);

StepDefinitionGeneric then_remote_media_direction_is = then4<String, String, String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s (audio|video) from (Alice|Bob|Carol) has `(SendRecv|SendOnly|RecvOnly|Inactive)` direction"),
  (id, String kind, remote_id, direction, context) async {
    var member =context.world.members[id]!;
    await member.wait_for_connect(remote_id);
    var kind_ = parse_media_kind(kind);
    var track = member.connection_store.remote_tracks[remote_id]!.firstWhere((element) => element.mediaSourceKind() == MediaSourceKind.Device && element.kind() == kind_.item1);
    while (track.mediaDirection().name != direction) {
      await Future.delayed(Duration(milliseconds: 100));
    }
  },
);

// #[when(regex = "^(\\S+) (enables|disables) remote \
//                  (audio|(?:device |display )?video)$")]
// async fn when_member_enables_remote_track(
//     world: &mut World,
//     id: String,
//     toggle: String,
//     kind: String,
// ) {
//     let member = world.get_member(&id).unwrap();
//     let media_kind = kind.parse().unwrap();
//     let source_kind = kind.parse().ok();

//     if toggle == "enables" {
//         member
//             .room()
//             .enable_remote_media(media_kind, source_kind)
//             .await
//             .unwrap();
//     } else {
//         member
//             .room()
//             .disable_remote_media(media_kind, source_kind)
//             .await
//             .unwrap();
//     }
// }
