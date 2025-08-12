// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'member_connection_state.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$MemberConnectionState {

 PeerConnectionState get field0;
/// Create a copy of MemberConnectionState
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$MemberConnectionStateCopyWith<MemberConnectionState> get copyWith => _$MemberConnectionStateCopyWithImpl<MemberConnectionState>(this as MemberConnectionState, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is MemberConnectionState&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'MemberConnectionState(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $MemberConnectionStateCopyWith<$Res>  {
  factory $MemberConnectionStateCopyWith(MemberConnectionState value, $Res Function(MemberConnectionState) _then) = _$MemberConnectionStateCopyWithImpl;
@useResult
$Res call({
 PeerConnectionState field0
});




}
/// @nodoc
class _$MemberConnectionStateCopyWithImpl<$Res>
    implements $MemberConnectionStateCopyWith<$Res> {
  _$MemberConnectionStateCopyWithImpl(this._self, this._then);

  final MemberConnectionState _self;
  final $Res Function(MemberConnectionState) _then;

/// Create a copy of MemberConnectionState
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? field0 = null,}) {
  return _then(_self.copyWith(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as PeerConnectionState,
  ));
}

}


/// Adds pattern-matching-related methods to [MemberConnectionState].
extension MemberConnectionStatePatterns on MemberConnectionState {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( MemberConnectionState_P2P value)?  p2P,required TResult orElse(),}){
final _that = this;
switch (_that) {
case MemberConnectionState_P2P() when p2P != null:
return p2P(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( MemberConnectionState_P2P value)  p2P,}){
final _that = this;
switch (_that) {
case MemberConnectionState_P2P():
return p2P(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( MemberConnectionState_P2P value)?  p2P,}){
final _that = this;
switch (_that) {
case MemberConnectionState_P2P() when p2P != null:
return p2P(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( PeerConnectionState field0)?  p2P,required TResult orElse(),}) {final _that = this;
switch (_that) {
case MemberConnectionState_P2P() when p2P != null:
return p2P(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( PeerConnectionState field0)  p2P,}) {final _that = this;
switch (_that) {
case MemberConnectionState_P2P():
return p2P(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( PeerConnectionState field0)?  p2P,}) {final _that = this;
switch (_that) {
case MemberConnectionState_P2P() when p2P != null:
return p2P(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class MemberConnectionState_P2P extends MemberConnectionState {
  const MemberConnectionState_P2P(this.field0): super._();
  

@override final  PeerConnectionState field0;

/// Create a copy of MemberConnectionState
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$MemberConnectionState_P2PCopyWith<MemberConnectionState_P2P> get copyWith => _$MemberConnectionState_P2PCopyWithImpl<MemberConnectionState_P2P>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is MemberConnectionState_P2P&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'MemberConnectionState.p2P(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $MemberConnectionState_P2PCopyWith<$Res> implements $MemberConnectionStateCopyWith<$Res> {
  factory $MemberConnectionState_P2PCopyWith(MemberConnectionState_P2P value, $Res Function(MemberConnectionState_P2P) _then) = _$MemberConnectionState_P2PCopyWithImpl;
@override @useResult
$Res call({
 PeerConnectionState field0
});




}
/// @nodoc
class _$MemberConnectionState_P2PCopyWithImpl<$Res>
    implements $MemberConnectionState_P2PCopyWith<$Res> {
  _$MemberConnectionState_P2PCopyWithImpl(this._self, this._then);

  final MemberConnectionState_P2P _self;
  final $Res Function(MemberConnectionState_P2P) _then;

/// Create a copy of MemberConnectionState
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(MemberConnectionState_P2P(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as PeerConnectionState,
  ));
}


}

// dart format on
