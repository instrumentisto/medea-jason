import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
void registerFunction(DynamicLibrary dl, {
required Pointer<NativeFunction<Handle Function()>> initDeviceConstraints,
required Pointer<NativeFunction<Handle Function()>> initDisplayConstraints,
required Pointer<NativeFunction<Handle Function()>> newVideoConstraints,
required Pointer<NativeFunction<Handle Function()>> newAudioConstraints,
required Pointer<NativeFunction<Void Function(Handle, Int64, ForeignValue)>> setVideoConstraintValue,
required Pointer<NativeFunction<Void Function(Handle, Int64, ForeignValue)>> setAudioConstraintValue,
required Pointer<NativeFunction<Void Function(Handle, Int64, Handle)>> setVideoConstraint,
required Pointer<NativeFunction<Void Function(Handle, Int64, Handle)>> setAudioConstraint,
} ) {
dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer)>('register_constraints')(
initDeviceConstraints,
initDisplayConstraints,
newVideoConstraints,
newAudioConstraints,
setVideoConstraintValue,
setAudioConstraintValue,
setVideoConstraint,
setAudioConstraint,
);}
