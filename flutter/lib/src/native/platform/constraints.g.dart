import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';

typedef _ErrorSetterFnC = Void Function(Handle);
typedef _ErrorSetterFnDart = void Function(Object);

Object Function()? _initDeviceConstraints;
Object Function()? _initDisplayConstraints;
Object Function()? _newVideoConstraints;
Object Function()? _newAudioConstraints;
void Function(Object, int, ForeignValue)? _setVideoConstraintValue;
void Function(Object, int, ForeignValue)? _setAudioConstraintValue;
void Function(Object, int, Object)? _setVideoConstraint;
void Function(Object, int, Object)? _setDisplayVideoConstraint;
void Function(Object, int, Object)? _setAudioConstraint;

_ErrorSetterFnDart? _constraints__init_device_constraints__set_error;
_ErrorSetterFnDart? _constraints__init_display_constraints__set_error;
_ErrorSetterFnDart? _constraints__new_video_constraints__set_error;
_ErrorSetterFnDart? _constraints__new_audio_constraints__set_error;
_ErrorSetterFnDart? _constraints__set_video_constraint_value__set_error;
_ErrorSetterFnDart? _constraints__set_audio_constraint_value__set_error;
_ErrorSetterFnDart? _constraints__set_video_constraint__set_error;
_ErrorSetterFnDart? _constraints__set_display_video_constraint__set_error;
_ErrorSetterFnDart? _constraints__set_audio_constraint__set_error;

void registerFunction(
  DynamicLibrary dl, {
  required Object Function() initDeviceConstraints,
  required Object Function() initDisplayConstraints,
  required Object Function() newVideoConstraints,
  required Object Function() newAudioConstraints,
  required void Function(Object, int, ForeignValue) setVideoConstraintValue,
  required void Function(Object, int, ForeignValue) setAudioConstraintValue,
  required void Function(Object, int, Object) setVideoConstraint,
  required void Function(Object, int, Object) setDisplayVideoConstraint,
  required void Function(Object, int, Object) setAudioConstraint,
}) {
  _initDeviceConstraints = initDeviceConstraints;
  _initDisplayConstraints = initDisplayConstraints;
  _newVideoConstraints = newVideoConstraints;
  _newAudioConstraints = newAudioConstraints;
  _setVideoConstraintValue = setVideoConstraintValue;
  _setAudioConstraintValue = setAudioConstraintValue;
  _setVideoConstraint = setVideoConstraint;
  _setDisplayVideoConstraint = setDisplayVideoConstraint;
  _setAudioConstraint = setAudioConstraint;

  _constraints__init_device_constraints__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'constraints__init_device_constraints__set_error');
  _constraints__init_display_constraints__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'constraints__init_display_constraints__set_error');
  _constraints__new_video_constraints__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'constraints__new_video_constraints__set_error');
  _constraints__new_audio_constraints__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'constraints__new_audio_constraints__set_error');
  _constraints__set_video_constraint_value__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'constraints__set_video_constraint_value__set_error');
  _constraints__set_audio_constraint_value__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'constraints__set_audio_constraint_value__set_error');
  _constraints__set_video_constraint__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'constraints__set_video_constraint__set_error');
  _constraints__set_display_video_constraint__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'constraints__set_display_video_constraint__set_error');
  _constraints__set_audio_constraint__set_error =
      dl.lookupFunction<_ErrorSetterFnC, _ErrorSetterFnDart>(
          'constraints__set_audio_constraint__set_error');

  Pointer<NativeFunction<Handle Function()>> initDeviceConstraints_native =
      Pointer.fromFunction(
    _initDeviceConstraintsProxy,
  );
  Pointer<NativeFunction<Handle Function()>> initDisplayConstraints_native =
      Pointer.fromFunction(
    _initDisplayConstraintsProxy,
  );
  Pointer<NativeFunction<Handle Function()>> newVideoConstraints_native =
      Pointer.fromFunction(
    _newVideoConstraintsProxy,
  );
  Pointer<NativeFunction<Handle Function()>> newAudioConstraints_native =
      Pointer.fromFunction(
    _newAudioConstraintsProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Int64, ForeignValue)>>
      setVideoConstraintValue_native = Pointer.fromFunction(
    _setVideoConstraintValueProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Int64, ForeignValue)>>
      setAudioConstraintValue_native = Pointer.fromFunction(
    _setAudioConstraintValueProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Int64, Handle)>>
      setVideoConstraint_native = Pointer.fromFunction(
    _setVideoConstraintProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Int64, Handle)>>
      setDisplayVideoConstraint_native = Pointer.fromFunction(
    _setDisplayVideoConstraintProxy,
  );
  Pointer<NativeFunction<Void Function(Handle, Int64, Handle)>>
      setAudioConstraint_native = Pointer.fromFunction(
    _setAudioConstraintProxy,
  );

  dl.lookupFunction<
      Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer),
      void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer,
          Pointer, Pointer, Pointer)>('register_constraints')(
    initDeviceConstraints_native,
    initDisplayConstraints_native,
    newVideoConstraints_native,
    newAudioConstraints_native,
    setVideoConstraintValue_native,
    setAudioConstraintValue_native,
    setVideoConstraint_native,
    setDisplayVideoConstraint_native,
    setAudioConstraint_native,
  );
}

Object _initDeviceConstraintsProxy() {
  try {
    return _initDeviceConstraints!();
  } catch (e) {
    _constraints__init_device_constraints__set_error!(e);
    return 0;
  }
}

Object _initDisplayConstraintsProxy() {
  try {
    return _initDisplayConstraints!();
  } catch (e) {
    _constraints__init_display_constraints__set_error!(e);
    return 0;
  }
}

Object _newVideoConstraintsProxy() {
  try {
    return _newVideoConstraints!();
  } catch (e) {
    _constraints__new_video_constraints__set_error!(e);
    return 0;
  }
}

Object _newAudioConstraintsProxy() {
  try {
    return _newAudioConstraints!();
  } catch (e) {
    _constraints__new_audio_constraints__set_error!(e);
    return 0;
  }
}

void _setVideoConstraintValueProxy(Object arg0, int arg1, ForeignValue arg2) {
  try {
    return _setVideoConstraintValue!(arg0, arg1, arg2);
  } catch (e) {
    _constraints__set_video_constraint_value__set_error!(e);
    return;
  }
}

void _setAudioConstraintValueProxy(Object arg0, int arg1, ForeignValue arg2) {
  try {
    return _setAudioConstraintValue!(arg0, arg1, arg2);
  } catch (e) {
    _constraints__set_audio_constraint_value__set_error!(e);
    return;
  }
}

void _setVideoConstraintProxy(Object arg0, int arg1, Object arg2) {
  try {
    return _setVideoConstraint!(arg0, arg1, arg2);
  } catch (e) {
    _constraints__set_video_constraint__set_error!(e);
    return;
  }
}

void _setDisplayVideoConstraintProxy(Object arg0, int arg1, Object arg2) {
  try {
    return _setDisplayVideoConstraint!(arg0, arg1, arg2);
  } catch (e) {
    _constraints__set_display_video_constraint__set_error!(e);
    return;
  }
}

void _setAudioConstraintProxy(Object arg0, int arg1, Object arg2) {
  try {
    return _setAudioConstraint!(arg0, arg1, arg2);
  } catch (e) {
    _constraints__set_audio_constraint__set_error!(e);
    return;
  }
}
