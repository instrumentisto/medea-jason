// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'api.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$ApiConstrainFacingMode {

 FacingMode get field0;
/// Create a copy of ApiConstrainFacingMode
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiConstrainFacingModeCopyWith<ApiConstrainFacingMode> get copyWith => _$ApiConstrainFacingModeCopyWithImpl<ApiConstrainFacingMode>(this as ApiConstrainFacingMode, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiConstrainFacingMode&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ApiConstrainFacingMode(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ApiConstrainFacingModeCopyWith<$Res>  {
  factory $ApiConstrainFacingModeCopyWith(ApiConstrainFacingMode value, $Res Function(ApiConstrainFacingMode) _then) = _$ApiConstrainFacingModeCopyWithImpl;
@useResult
$Res call({
 FacingMode field0
});




}
/// @nodoc
class _$ApiConstrainFacingModeCopyWithImpl<$Res>
    implements $ApiConstrainFacingModeCopyWith<$Res> {
  _$ApiConstrainFacingModeCopyWithImpl(this._self, this._then);

  final ApiConstrainFacingMode _self;
  final $Res Function(ApiConstrainFacingMode) _then;

/// Create a copy of ApiConstrainFacingMode
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? field0 = null,}) {
  return _then(_self.copyWith(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as FacingMode,
  ));
}

}


/// Adds pattern-matching-related methods to [ApiConstrainFacingMode].
extension ApiConstrainFacingModePatterns on ApiConstrainFacingMode {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ApiConstrainFacingMode_Exact value)?  exact,TResult Function( ApiConstrainFacingMode_Ideal value)?  ideal,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ApiConstrainFacingMode_Exact() when exact != null:
return exact(_that);case ApiConstrainFacingMode_Ideal() when ideal != null:
return ideal(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ApiConstrainFacingMode_Exact value)  exact,required TResult Function( ApiConstrainFacingMode_Ideal value)  ideal,}){
final _that = this;
switch (_that) {
case ApiConstrainFacingMode_Exact():
return exact(_that);case ApiConstrainFacingMode_Ideal():
return ideal(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ApiConstrainFacingMode_Exact value)?  exact,TResult? Function( ApiConstrainFacingMode_Ideal value)?  ideal,}){
final _that = this;
switch (_that) {
case ApiConstrainFacingMode_Exact() when exact != null:
return exact(_that);case ApiConstrainFacingMode_Ideal() when ideal != null:
return ideal(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( FacingMode field0)?  exact,TResult Function( FacingMode field0)?  ideal,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ApiConstrainFacingMode_Exact() when exact != null:
return exact(_that.field0);case ApiConstrainFacingMode_Ideal() when ideal != null:
return ideal(_that.field0);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( FacingMode field0)  exact,required TResult Function( FacingMode field0)  ideal,}) {final _that = this;
switch (_that) {
case ApiConstrainFacingMode_Exact():
return exact(_that.field0);case ApiConstrainFacingMode_Ideal():
return ideal(_that.field0);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( FacingMode field0)?  exact,TResult? Function( FacingMode field0)?  ideal,}) {final _that = this;
switch (_that) {
case ApiConstrainFacingMode_Exact() when exact != null:
return exact(_that.field0);case ApiConstrainFacingMode_Ideal() when ideal != null:
return ideal(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class ApiConstrainFacingMode_Exact extends ApiConstrainFacingMode {
  const ApiConstrainFacingMode_Exact(this.field0): super._();
  

@override final  FacingMode field0;

/// Create a copy of ApiConstrainFacingMode
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiConstrainFacingMode_ExactCopyWith<ApiConstrainFacingMode_Exact> get copyWith => _$ApiConstrainFacingMode_ExactCopyWithImpl<ApiConstrainFacingMode_Exact>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiConstrainFacingMode_Exact&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ApiConstrainFacingMode.exact(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ApiConstrainFacingMode_ExactCopyWith<$Res> implements $ApiConstrainFacingModeCopyWith<$Res> {
  factory $ApiConstrainFacingMode_ExactCopyWith(ApiConstrainFacingMode_Exact value, $Res Function(ApiConstrainFacingMode_Exact) _then) = _$ApiConstrainFacingMode_ExactCopyWithImpl;
@override @useResult
$Res call({
 FacingMode field0
});




}
/// @nodoc
class _$ApiConstrainFacingMode_ExactCopyWithImpl<$Res>
    implements $ApiConstrainFacingMode_ExactCopyWith<$Res> {
  _$ApiConstrainFacingMode_ExactCopyWithImpl(this._self, this._then);

  final ApiConstrainFacingMode_Exact _self;
  final $Res Function(ApiConstrainFacingMode_Exact) _then;

/// Create a copy of ApiConstrainFacingMode
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ApiConstrainFacingMode_Exact(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as FacingMode,
  ));
}


}

/// @nodoc


class ApiConstrainFacingMode_Ideal extends ApiConstrainFacingMode {
  const ApiConstrainFacingMode_Ideal(this.field0): super._();
  

@override final  FacingMode field0;

/// Create a copy of ApiConstrainFacingMode
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiConstrainFacingMode_IdealCopyWith<ApiConstrainFacingMode_Ideal> get copyWith => _$ApiConstrainFacingMode_IdealCopyWithImpl<ApiConstrainFacingMode_Ideal>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiConstrainFacingMode_Ideal&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ApiConstrainFacingMode.ideal(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ApiConstrainFacingMode_IdealCopyWith<$Res> implements $ApiConstrainFacingModeCopyWith<$Res> {
  factory $ApiConstrainFacingMode_IdealCopyWith(ApiConstrainFacingMode_Ideal value, $Res Function(ApiConstrainFacingMode_Ideal) _then) = _$ApiConstrainFacingMode_IdealCopyWithImpl;
@override @useResult
$Res call({
 FacingMode field0
});




}
/// @nodoc
class _$ApiConstrainFacingMode_IdealCopyWithImpl<$Res>
    implements $ApiConstrainFacingMode_IdealCopyWith<$Res> {
  _$ApiConstrainFacingMode_IdealCopyWithImpl(this._self, this._then);

  final ApiConstrainFacingMode_Ideal _self;
  final $Res Function(ApiConstrainFacingMode_Ideal) _then;

/// Create a copy of ApiConstrainFacingMode
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ApiConstrainFacingMode_Ideal(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as FacingMode,
  ));
}


}

// dart format on
