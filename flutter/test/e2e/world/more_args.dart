import 'package:gherkin/gherkin.dart';
import 'package:tuple/tuple.dart';

import 'package:medea_jason/src/interface/media_track.dart';

StepDefinitionGeneric<TWorld> step6<TWorld extends World, TInput1, TInput2,
    TInput3, TInput4, TInput5, TInput6>(
  Pattern pattern,
  int expectedParameterCount,
  Function onInvoke, {
  StepDefinitionConfiguration? configuration,
}) {
  return GenericFunctionStepDefinition<TWorld>(
    pattern,
    onInvoke,
    expectedParameterCount,
    configuration: configuration,
  );
}

StepDefinitionGeneric<TWorld> step7<TWorld extends World, TInput1, TInput2,
    TInput3, TInput4, TInput5, TInput6, TInput7>(
  Pattern pattern,
  int expectedParameterCount,
  Function onInvoke, {
  StepDefinitionConfiguration? configuration,
}) {
  return GenericFunctionStepDefinition<TWorld>(
    pattern,
    onInvoke,
    expectedParameterCount,
    configuration: configuration,
  );
}

StepDefinitionGeneric<TWorld> step8<TWorld extends World, TInput1, TInput2,
    TInput3, TInput4, TInput5, TInput6, TInput7, TInput8>(
  Pattern pattern,
  int expectedParameterCount,
  Function onInvoke, {
  StepDefinitionConfiguration? configuration,
}) {
  return GenericFunctionStepDefinition<TWorld>(
    pattern,
    onInvoke,
    expectedParameterCount,
    configuration: configuration,
  );
}

StepDefinitionGeneric<TWorld> step9<TWorld extends World, TInput1, TInput2,
    TInput3, TInput4, TInput5, TInput6, TInput7, TInput8, TInput9>(
  Pattern pattern,
  int expectedParameterCount,
  Function onInvoke, {
  StepDefinitionConfiguration? configuration,
}) {
  return GenericFunctionStepDefinition<TWorld>(
    pattern,
    onInvoke,
    expectedParameterCount,
    configuration: configuration,
  );
}

StepDefinitionGeneric<TWorld> given6<TInput1, TInput2, TInput3, TInput4,
        TInput5, TInput6, TWorld extends World>(
  Pattern pattern,
  Future<void> Function(
    TInput1 input1,
    TInput2 input2,
    TInput3 input3,
    TInput4 input4,
    TInput5 input5,
    TInput6 input6,
    StepContext<TWorld> context,
  ) onInvoke, {
  StepDefinitionConfiguration? configuration,
}) =>
    step6<TWorld, TInput1, TInput2, TInput3, TInput4, TInput5, TInput6>(
      pattern,
      6,
      onInvoke,
      configuration: configuration,
    );

StepDefinitionGeneric<TWorld> given7<TInput1, TInput2, TInput3, TInput4,
        TInput5, TInput6, TInput7, TWorld extends World>(
  Pattern pattern,
  Future<void> Function(
    TInput1 input1,
    TInput2 input2,
    TInput3 input3,
    TInput4 input4,
    TInput5 input5,
    TInput6 input6,
    TInput7 input7,
    StepContext<TWorld> context,
  ) onInvoke, {
  StepDefinitionConfiguration? configuration,
}) =>
    step7<TWorld, TInput1, TInput2, TInput3, TInput4, TInput5, TInput6,
        TInput7>(
      pattern,
      7,
      onInvoke,
      configuration: configuration,
    );

StepDefinitionGeneric<TWorld> given8<TInput1, TInput2, TInput3, TInput4,
        TInput5, TInput6, TInput7, TInput8, TWorld extends World>(
  Pattern pattern,
  Future<void> Function(
    TInput1 input1,
    TInput2 input2,
    TInput3 input3,
    TInput4 input4,
    TInput5 input5,
    TInput6 input6,
    TInput7 input7,
    TInput8 input8,
    StepContext<TWorld> context,
  ) onInvoke, {
  StepDefinitionConfiguration? configuration,
}) =>
    step8<TWorld, TInput1, TInput2, TInput3, TInput4, TInput5, TInput6, TInput7,
        TInput8>(
      pattern,
      8,
      onInvoke,
      configuration: configuration,
    );

StepDefinitionGeneric<TWorld> given9<TInput1, TInput2, TInput3, TInput4,
        TInput5, TInput6, TInput7, TInput8, TInput9, TWorld extends World>(
  Pattern pattern,
  Future<void> Function(
    TInput1 input1,
    TInput2 input2,
    TInput3 input3,
    TInput4 input4,
    TInput5 input5,
    TInput6 input6,
    TInput7 input7,
    TInput8 input8,
    TInput9 input9,
    StepContext<TWorld> context,
  ) onInvoke, {
  StepDefinitionConfiguration? configuration,
}) =>
    step9<TWorld, TInput1, TInput2, TInput3, TInput4, TInput5, TInput6, TInput7,
        TInput8, TInput9>(
      pattern,
      9,
      onInvoke,
      configuration: configuration,
    );

Tuple2<MediaKind, MediaSourceKind> parseMediaKind(String kind) {
  var kind_ = MediaKind.Video;
  var source = MediaSourceKind.Device;
  if (kind.contains('audio')) {
    kind_ = MediaKind.Audio;
  } else {
    if (kind.contains('display')) {
      source = MediaSourceKind.Display;
    }
  }
  return Tuple2(kind_, source);
}

StepDefinitionGeneric<TWorld>
    fixThen5<TInput1, TInput2, TInput3, TInput4, TInput5, TWorld extends World>(
  Pattern pattern,
  Future<void> Function(
    TInput1 input1,
    TInput2 input2,
    TInput3 input3,
    TInput4 input4,
    TInput5 input5,
    StepContext<TWorld> context,
  ) onInvoke, {
  StepDefinitionConfiguration? configuration,
}) =>
        step<TWorld, TInput1, TInput2, TInput3, TInput4, TInput5>(
          pattern,
          5,
          onInvoke,
          configuration: configuration,
        );

StepDefinitionGeneric<TWorld> fixGiven5<TInput1, TInput2, TInput3, TInput4,
        TInput5, TWorld extends World>(
  Pattern pattern,
  Future<void> Function(
    TInput1 input1,
    TInput1 input2,
    TInput1 input3,
    TInput1 input4,
    TInput1 input5,
    StepContext<TWorld> context,
  ) onInvoke, {
  StepDefinitionConfiguration? configuration,
}) =>
    step<TWorld, TInput1, TInput2, TInput3, TInput4, TInput5>(
      pattern,
      5,
      onInvoke,
      configuration: configuration,
    );
