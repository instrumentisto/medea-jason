
import 'package:gherkin/gherkin.dart';
import 'package:medea_jason/src/interface/track_kinds.dart';

import '../world/custom_world.dart';
import '../world/more_args.dart';


// todo recheck await
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
          var future =  member.toggle_media(kind.item1, null, true);
          if(awats) {await future;}
        } 
        break; 
        
        case 'disables': {
          var future =   member.toggle_media(kind.item1, null, false);
                    if(awats) {
                      await future;}
        } 
        break; 

        case 'mutes': { 
          var future =   member.toggle_mute(kind.item1, null, true);
                    if(awats) {await future;}
        } 
        break; 
            
        default: { 
          var future =   member.toggle_mute(kind.item1, null, false);
                    if(awats) {await future;}
        }
        break; 
      } 
    } catch(e) {
      if (!error) { throw 10000; };
    }

  },
);

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

StepDefinitionGeneric then_local_track_mute_state = then3<String, String, String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s local (audio|device video|display video|video) track is (not muted|muted)"),
  (id, String kind, not_muted, context) async {
    var member =context.world.members[id]!;
    var kind_ = parse_media_kind(kind);
    var track = member.connection_store.local_tracks.firstWhere((element) => element.mediaSourceKind() == MediaSourceKind.Device && element.kind() == kind_.item1);
    var muted = !not_muted.contains('not');

    throw 'cant check local track muted';
  },
);


StepDefinitionGeneric given_gum_delay = given1<String, CustomWorld>(
  RegExp(
      r"(Alice|Bob|Carol)'s `getUserMedia\(\)` request has added latency"),
  (id, context) async {
    var member =context.world.members[id]!;
    await member.add_gum_latency(Duration(milliseconds: 500));
  },
);

StepDefinitionGeneric when_member_frees_all_local_tracks = when1<String, CustomWorld>(
  RegExp(
      r'(Alice|Bob|Carol) frees all local tracks'),
  (id, context) async {
    var member = context.world.members[id]!;
    await member.forget_local_tracks();
  },
);
