// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'constraints.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$ConstrainBoolean {

 bool get field0;
/// Create a copy of ConstrainBoolean
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConstrainBooleanCopyWith<ConstrainBoolean> get copyWith => _$ConstrainBooleanCopyWithImpl<ConstrainBoolean>(this as ConstrainBoolean, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConstrainBoolean&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConstrainBoolean(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConstrainBooleanCopyWith<$Res>  {
  factory $ConstrainBooleanCopyWith(ConstrainBoolean value, $Res Function(ConstrainBoolean) _then) = _$ConstrainBooleanCopyWithImpl;
@useResult
$Res call({
 bool field0
});




}
/// @nodoc
class _$ConstrainBooleanCopyWithImpl<$Res>
    implements $ConstrainBooleanCopyWith<$Res> {
  _$ConstrainBooleanCopyWithImpl(this._self, this._then);

  final ConstrainBoolean _self;
  final $Res Function(ConstrainBoolean) _then;

/// Create a copy of ConstrainBoolean
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? field0 = null,}) {
  return _then(_self.copyWith(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}

}


/// Adds pattern-matching-related methods to [ConstrainBoolean].
extension ConstrainBooleanPatterns on ConstrainBoolean {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ConstrainBoolean_Exact value)?  exact,TResult Function( ConstrainBoolean_Ideal value)?  ideal,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ConstrainBoolean_Exact() when exact != null:
return exact(_that);case ConstrainBoolean_Ideal() when ideal != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ConstrainBoolean_Exact value)  exact,required TResult Function( ConstrainBoolean_Ideal value)  ideal,}){
final _that = this;
switch (_that) {
case ConstrainBoolean_Exact():
return exact(_that);case ConstrainBoolean_Ideal():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ConstrainBoolean_Exact value)?  exact,TResult? Function( ConstrainBoolean_Ideal value)?  ideal,}){
final _that = this;
switch (_that) {
case ConstrainBoolean_Exact() when exact != null:
return exact(_that);case ConstrainBoolean_Ideal() when ideal != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( bool field0)?  exact,TResult Function( bool field0)?  ideal,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ConstrainBoolean_Exact() when exact != null:
return exact(_that.field0);case ConstrainBoolean_Ideal() when ideal != null:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( bool field0)  exact,required TResult Function( bool field0)  ideal,}) {final _that = this;
switch (_that) {
case ConstrainBoolean_Exact():
return exact(_that.field0);case ConstrainBoolean_Ideal():
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( bool field0)?  exact,TResult? Function( bool field0)?  ideal,}) {final _that = this;
switch (_that) {
case ConstrainBoolean_Exact() when exact != null:
return exact(_that.field0);case ConstrainBoolean_Ideal() when ideal != null:
return ideal(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class ConstrainBoolean_Exact extends ConstrainBoolean {
  const ConstrainBoolean_Exact(this.field0): super._();
  

@override final  bool field0;

/// Create a copy of ConstrainBoolean
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConstrainBoolean_ExactCopyWith<ConstrainBoolean_Exact> get copyWith => _$ConstrainBoolean_ExactCopyWithImpl<ConstrainBoolean_Exact>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConstrainBoolean_Exact&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConstrainBoolean.exact(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConstrainBoolean_ExactCopyWith<$Res> implements $ConstrainBooleanCopyWith<$Res> {
  factory $ConstrainBoolean_ExactCopyWith(ConstrainBoolean_Exact value, $Res Function(ConstrainBoolean_Exact) _then) = _$ConstrainBoolean_ExactCopyWithImpl;
@override @useResult
$Res call({
 bool field0
});




}
/// @nodoc
class _$ConstrainBoolean_ExactCopyWithImpl<$Res>
    implements $ConstrainBoolean_ExactCopyWith<$Res> {
  _$ConstrainBoolean_ExactCopyWithImpl(this._self, this._then);

  final ConstrainBoolean_Exact _self;
  final $Res Function(ConstrainBoolean_Exact) _then;

/// Create a copy of ConstrainBoolean
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConstrainBoolean_Exact(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc


class ConstrainBoolean_Ideal extends ConstrainBoolean {
  const ConstrainBoolean_Ideal(this.field0): super._();
  

@override final  bool field0;

/// Create a copy of ConstrainBoolean
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConstrainBoolean_IdealCopyWith<ConstrainBoolean_Ideal> get copyWith => _$ConstrainBoolean_IdealCopyWithImpl<ConstrainBoolean_Ideal>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConstrainBoolean_Ideal&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConstrainBoolean.ideal(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConstrainBoolean_IdealCopyWith<$Res> implements $ConstrainBooleanCopyWith<$Res> {
  factory $ConstrainBoolean_IdealCopyWith(ConstrainBoolean_Ideal value, $Res Function(ConstrainBoolean_Ideal) _then) = _$ConstrainBoolean_IdealCopyWithImpl;
@override @useResult
$Res call({
 bool field0
});




}
/// @nodoc
class _$ConstrainBoolean_IdealCopyWithImpl<$Res>
    implements $ConstrainBoolean_IdealCopyWith<$Res> {
  _$ConstrainBoolean_IdealCopyWithImpl(this._self, this._then);

  final ConstrainBoolean_Ideal _self;
  final $Res Function(ConstrainBoolean_Ideal) _then;

/// Create a copy of ConstrainBoolean
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConstrainBoolean_Ideal(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc
mixin _$ConstrainU32 {

 int get field0;
/// Create a copy of ConstrainU32
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConstrainU32CopyWith<ConstrainU32> get copyWith => _$ConstrainU32CopyWithImpl<ConstrainU32>(this as ConstrainU32, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConstrainU32&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConstrainU32(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConstrainU32CopyWith<$Res>  {
  factory $ConstrainU32CopyWith(ConstrainU32 value, $Res Function(ConstrainU32) _then) = _$ConstrainU32CopyWithImpl;
@useResult
$Res call({
 int field0
});




}
/// @nodoc
class _$ConstrainU32CopyWithImpl<$Res>
    implements $ConstrainU32CopyWith<$Res> {
  _$ConstrainU32CopyWithImpl(this._self, this._then);

  final ConstrainU32 _self;
  final $Res Function(ConstrainU32) _then;

/// Create a copy of ConstrainU32
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? field0 = null,}) {
  return _then(_self.copyWith(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as int,
  ));
}

}


/// Adds pattern-matching-related methods to [ConstrainU32].
extension ConstrainU32Patterns on ConstrainU32 {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ConstrainU32_Exact value)?  exact,TResult Function( ConstrainU32_Ideal value)?  ideal,TResult Function( ConstrainU32_Range value)?  range,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ConstrainU32_Exact() when exact != null:
return exact(_that);case ConstrainU32_Ideal() when ideal != null:
return ideal(_that);case ConstrainU32_Range() when range != null:
return range(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ConstrainU32_Exact value)  exact,required TResult Function( ConstrainU32_Ideal value)  ideal,required TResult Function( ConstrainU32_Range value)  range,}){
final _that = this;
switch (_that) {
case ConstrainU32_Exact():
return exact(_that);case ConstrainU32_Ideal():
return ideal(_that);case ConstrainU32_Range():
return range(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ConstrainU32_Exact value)?  exact,TResult? Function( ConstrainU32_Ideal value)?  ideal,TResult? Function( ConstrainU32_Range value)?  range,}){
final _that = this;
switch (_that) {
case ConstrainU32_Exact() when exact != null:
return exact(_that);case ConstrainU32_Ideal() when ideal != null:
return ideal(_that);case ConstrainU32_Range() when range != null:
return range(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( int field0)?  exact,TResult Function( int field0)?  ideal,TResult Function( int field0,  int field1)?  range,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ConstrainU32_Exact() when exact != null:
return exact(_that.field0);case ConstrainU32_Ideal() when ideal != null:
return ideal(_that.field0);case ConstrainU32_Range() when range != null:
return range(_that.field0,_that.field1);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( int field0)  exact,required TResult Function( int field0)  ideal,required TResult Function( int field0,  int field1)  range,}) {final _that = this;
switch (_that) {
case ConstrainU32_Exact():
return exact(_that.field0);case ConstrainU32_Ideal():
return ideal(_that.field0);case ConstrainU32_Range():
return range(_that.field0,_that.field1);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( int field0)?  exact,TResult? Function( int field0)?  ideal,TResult? Function( int field0,  int field1)?  range,}) {final _that = this;
switch (_that) {
case ConstrainU32_Exact() when exact != null:
return exact(_that.field0);case ConstrainU32_Ideal() when ideal != null:
return ideal(_that.field0);case ConstrainU32_Range() when range != null:
return range(_that.field0,_that.field1);case _:
  return null;

}
}

}

/// @nodoc


class ConstrainU32_Exact extends ConstrainU32 {
  const ConstrainU32_Exact(this.field0): super._();
  

@override final  int field0;

/// Create a copy of ConstrainU32
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConstrainU32_ExactCopyWith<ConstrainU32_Exact> get copyWith => _$ConstrainU32_ExactCopyWithImpl<ConstrainU32_Exact>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConstrainU32_Exact&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConstrainU32.exact(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConstrainU32_ExactCopyWith<$Res> implements $ConstrainU32CopyWith<$Res> {
  factory $ConstrainU32_ExactCopyWith(ConstrainU32_Exact value, $Res Function(ConstrainU32_Exact) _then) = _$ConstrainU32_ExactCopyWithImpl;
@override @useResult
$Res call({
 int field0
});




}
/// @nodoc
class _$ConstrainU32_ExactCopyWithImpl<$Res>
    implements $ConstrainU32_ExactCopyWith<$Res> {
  _$ConstrainU32_ExactCopyWithImpl(this._self, this._then);

  final ConstrainU32_Exact _self;
  final $Res Function(ConstrainU32_Exact) _then;

/// Create a copy of ConstrainU32
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConstrainU32_Exact(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

/// @nodoc


class ConstrainU32_Ideal extends ConstrainU32 {
  const ConstrainU32_Ideal(this.field0): super._();
  

@override final  int field0;

/// Create a copy of ConstrainU32
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConstrainU32_IdealCopyWith<ConstrainU32_Ideal> get copyWith => _$ConstrainU32_IdealCopyWithImpl<ConstrainU32_Ideal>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConstrainU32_Ideal&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ConstrainU32.ideal(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ConstrainU32_IdealCopyWith<$Res> implements $ConstrainU32CopyWith<$Res> {
  factory $ConstrainU32_IdealCopyWith(ConstrainU32_Ideal value, $Res Function(ConstrainU32_Ideal) _then) = _$ConstrainU32_IdealCopyWithImpl;
@override @useResult
$Res call({
 int field0
});




}
/// @nodoc
class _$ConstrainU32_IdealCopyWithImpl<$Res>
    implements $ConstrainU32_IdealCopyWith<$Res> {
  _$ConstrainU32_IdealCopyWithImpl(this._self, this._then);

  final ConstrainU32_Ideal _self;
  final $Res Function(ConstrainU32_Ideal) _then;

/// Create a copy of ConstrainU32
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ConstrainU32_Ideal(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

/// @nodoc


class ConstrainU32_Range extends ConstrainU32 {
  const ConstrainU32_Range(this.field0, this.field1): super._();
  

@override final  int field0;
 final  int field1;

/// Create a copy of ConstrainU32
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ConstrainU32_RangeCopyWith<ConstrainU32_Range> get copyWith => _$ConstrainU32_RangeCopyWithImpl<ConstrainU32_Range>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ConstrainU32_Range&&(identical(other.field0, field0) || other.field0 == field0)&&(identical(other.field1, field1) || other.field1 == field1));
}


@override
int get hashCode => Object.hash(runtimeType,field0,field1);

@override
String toString() {
  return 'ConstrainU32.range(field0: $field0, field1: $field1)';
}


}

/// @nodoc
abstract mixin class $ConstrainU32_RangeCopyWith<$Res> implements $ConstrainU32CopyWith<$Res> {
  factory $ConstrainU32_RangeCopyWith(ConstrainU32_Range value, $Res Function(ConstrainU32_Range) _then) = _$ConstrainU32_RangeCopyWithImpl;
@override @useResult
$Res call({
 int field0, int field1
});




}
/// @nodoc
class _$ConstrainU32_RangeCopyWithImpl<$Res>
    implements $ConstrainU32_RangeCopyWith<$Res> {
  _$ConstrainU32_RangeCopyWithImpl(this._self, this._then);

  final ConstrainU32_Range _self;
  final $Res Function(ConstrainU32_Range) _then;

/// Create a copy of ConstrainU32
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,Object? field1 = null,}) {
  return _then(ConstrainU32_Range(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as int,null == field1 ? _self.field1 : field1 // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

// dart format on
