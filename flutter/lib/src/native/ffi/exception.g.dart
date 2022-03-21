import 'dart:ffi';
import 'package:ffi/ffi.dart';
import 'package:medea_jason/src/native/ffi/foreign_value.dart';
void registerFunction(DynamicLibrary dl, {
required Pointer<NativeFunction<Handle Function(ForeignValue, Pointer<Utf8>, Pointer<Utf8>)>> newArgumentError,
required Pointer<NativeFunction<Handle Function(Pointer<Utf8>)>> newStateError,
required Pointer<NativeFunction<Handle Function(Pointer<Utf8>)>> newFormatException,
required Pointer<NativeFunction<Handle Function(Int64, Pointer<Utf8>, ForeignValue, Pointer<Utf8>)>> newLocalMediaInitException,
required Pointer<NativeFunction<Handle Function(Pointer<Handle>, Pointer<Utf8>)>> newEnumerateDevicesException,
required Pointer<NativeFunction<Handle Function(Int64, Pointer<Utf8>, ForeignValue, Pointer<Utf8>)>> newRpcClientException,
required Pointer<NativeFunction<Handle Function(Pointer<Utf8>, Pointer<Utf8>)>> newMediaStateTransitionException,
required Pointer<NativeFunction<Handle Function(Pointer<Utf8>, ForeignValue, Pointer<Utf8>)>> newInternalException,
required Pointer<NativeFunction<Handle Function(Pointer<Utf8>, Pointer<Handle>, Bool)>> newMediaSettingsUpdateException,
required Pointer<NativeFunction<Handle Function(Pointer<Utf8>)>> newInvalidOutputAudioDeviceIdException,
} ) {
dl.lookupFunction<Void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer), void Function(Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer, Pointer)>('register_exception')(
newArgumentError,
newStateError,
newFormatException,
newLocalMediaInitException,
newEnumerateDevicesException,
newRpcClientException,
newMediaStateTransitionException,
newInternalException,
newMediaSettingsUpdateException,
newInvalidOutputAudioDeviceIdException,
);}
