import 'package:flutter_test/flutter_test.dart';
import 'package:gherkin/gherkin.dart';
import 'package:http/http.dart';

import '../world/custom_world.dart';

List<StepDefinitionGeneric> steps() {
  return [checkMetrics, metricRespContains, responseCodeIs];
}

StepDefinitionGeneric checkMetrics = when<CustomWorld>(
  RegExp(r'^I check metrics$'),
  (context) async {
    var resp = await get(Uri.http('127.0.0.1:9372', '/metrics'));
    context.world.metricsResponses.add(resp);
  },
);

StepDefinitionGeneric responseCodeIs = then1<int, CustomWorld>(
  RegExp(r'^response code is `(\d+)`$'),
  (expected, context) async {
    var resp = context.world.metricsResponses.last;

    expect(resp.statusCode, expected);
  },
);

StepDefinitionGeneric metricRespContains = then1<String, CustomWorld>(
  RegExp(r'^response contains `(\S+)` metrics?$'),
  (metric, context) async {
    var body = context.world.metricsResponses.last.body;

    expect(body.contains('# TYPE $metric gauge'), isTrue);
  },
);
