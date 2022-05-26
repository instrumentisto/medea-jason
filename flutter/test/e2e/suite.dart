import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:flutter_gherkin/flutter_gherkin_with_driver.dart';
import 'package:gherkin/gherkin.dart';

import 'conf.dart';
import 'steps/connection.dart';
import 'steps/control_api.dart';
import 'steps/media_state.dart';
import 'steps/room.dart';
import 'steps/teest.dart';
import 'steps/track.dart';
import 'world/custom_world.dart';

part 'suite.g.dart';

var gg = FlutterTestConfiguration()
  ..stepDefinitions = [

    // control_api
    then_control_api_sends_on_leave,
    when_control_api_removes_member,
    when_control_api_removes_room,
    when_interconnects_kind,
    when_control_api_removes_member_via_apply,
    when_control_api_interconnects_via_apply,
    then_control_api_sends_on_join,
    when_control_api_starts_publishing,
    when_control_api_deletes_publish_endpoint,
    when_control_api_deletes_play_endpoint,
    when_control_api_interconnects_members,
    then_control_api_doesnt_sends_on_leave,

    
    //connection 
    then_connection_closes,
    then_member_receives_connection,
    when_connection_changes_remote_media_state,
    then_member_doesnt_receive_connection,

    //room
    then_on_close_fires,
    given_member_gum_will_error,
    then_room_failed_local_stream_fires,
    when_jason_object_disposes,
    when_room_closed_by_client,
    when_member_joins_room,

    //track
    then_member_has_remote_track,
    then_member_doesnt_have_remote_tracks_with,
    then_doesnt_have_remote_track,
    then_member_has_n_remote_tracks_from,
    then_member_has_local_tracks,
    then_remote_media_track,
    then_remote_track_stops,
    then_callback_fires_on_remote_track,
    then_has_local_track,



    //media_state
    when_enables_or_mutes,
    when_member_enables_remote_track,
    then_remote_media_direction_is,
    given_gum_delay,
    when_member_frees_all_local_tracks,
    then_track_is_stopped,
    then_local_track_mute_state,

    // //websockets
    // ws_connection_loss,
    // ws_connection_restore,
    // connection_is_lost,

    fillField3,
    fillField2,
    fillField1,
  ]
  ..hooks = []
  ..reporters = [
    StdoutReporter(MessageLevel.verbose)
      ..setWriteLineFn(print)
      ..setWriteFn(print),
    ProgressReporter()
      ..setWriteLineFn(print)
      ..setWriteFn(print),
    TestRunSummaryReporter()
      ..setWriteLineFn(print)
      ..setWriteFn(print),
    FlutterDriverReporter(logInfoMessages: true),
  ]
  ..defaultTimeout = const Duration(seconds: 30)
  ..customStepParameterDefinitions = []
  ..createWorld = (config) => Future.sync(() async {
        var world = CustomWorld();
        await world.control_client.create(world.room_id,
            {'kind': 'Room', 'id': world.room_id, 'pipeline': {}});
        return world;
      });

/// Entry point of E2E testing.
@GherkinTestSuite(featurePaths: ['./test/e2e/features/*.feature'])
void main() {
  executeTestSuite(
    gg,
    (World world) async {
      init_VAR();
    },
  );
}
