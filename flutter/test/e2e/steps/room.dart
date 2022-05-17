

import 'package:flutter/material.dart';
import 'package:flutter_gherkin/flutter_gherkin.dart';
import 'package:gherkin/gherkin.dart';

import '../api/endpoint.dart';
import '../api/room.dart';
import '../parameters/user.dart';
import '../world/custom_world.dart';
import '../world/member.dart';
import '../world/custom_world.dart';

// #[then(regex = "^(\\S+)'s `on_close` room's callback fires with `(\\S+)` \
//                  reason$")]
// async fn then_on_close_fires(
//     world: &mut World,
//     id: String,
//     expect_reason: String,
// ) {
//     let reason = world.wait_for_on_close(&id).await.unwrap();
//     assert_eq!(expect_reason, reason);
// }

// pub enum CloseReason {
//     /// Client session was finished on a server side.
//     Finished,

//     /// Old connection was closed due to a client reconnection.
//     Reconnected,

//     /// Connection has been inactive for a while and thus considered idle
//     /// by a server.
//     Idle,

//     /// Establishing of connection with a server was rejected on server side.
//     ///
//     /// Most likely because of incorrect `Member` credentials.
//     Rejected,

//     /// Server internal error has occurred while connecting.
//     ///
//     /// This close reason is similar to 500 HTTP status code.
//     InternalError,

//     /// Client was evicted on the server side.
//     Evicted,
// }

StepDefinitionGeneric then_on_close_fires =
    then2<String, String, CustomWorld>(
  RegExp(r"(Alice|Bob|Carol)'s `on_close` room's callback fires with `(Finished|Reconnected|Idle|Rejected|InternalError|Evicted)`"),
  (id, expect_reason, context) async {
    var reason = await context.world.wait_for_on_close(id);
    if (reason.reason() != expect_reason) {
      throw 42;
    }
  },
);
