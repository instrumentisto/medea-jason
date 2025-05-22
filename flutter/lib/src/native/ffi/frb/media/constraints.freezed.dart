// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'constraints.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$ConstrainBoolean {
  bool get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(bool field0) exact,
    required TResult Function(bool field0) ideal,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(bool field0)? exact,
    TResult? Function(bool field0)? ideal,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(bool field0)? exact,
    TResult Function(bool field0)? ideal,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConstrainBoolean_Exact value) exact,
    required TResult Function(ConstrainBoolean_Ideal value) ideal,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConstrainBoolean_Exact value)? exact,
    TResult? Function(ConstrainBoolean_Ideal value)? ideal,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConstrainBoolean_Exact value)? exact,
    TResult Function(ConstrainBoolean_Ideal value)? ideal,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;

  /// Create a copy of ConstrainBoolean
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $ConstrainBooleanCopyWith<ConstrainBoolean> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ConstrainBooleanCopyWith<$Res> {
  factory $ConstrainBooleanCopyWith(
    ConstrainBoolean value,
    $Res Function(ConstrainBoolean) then,
  ) = _$ConstrainBooleanCopyWithImpl<$Res, ConstrainBoolean>;
  @useResult
  $Res call({bool field0});
}

/// @nodoc
class _$ConstrainBooleanCopyWithImpl<$Res, $Val extends ConstrainBoolean>
    implements $ConstrainBooleanCopyWith<$Res> {
  _$ConstrainBooleanCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ConstrainBoolean
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _value.copyWith(
            field0: null == field0
                ? _value.field0
                : field0 // ignore: cast_nullable_to_non_nullable
                      as bool,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$ConstrainBoolean_ExactImplCopyWith<$Res>
    implements $ConstrainBooleanCopyWith<$Res> {
  factory _$$ConstrainBoolean_ExactImplCopyWith(
    _$ConstrainBoolean_ExactImpl value,
    $Res Function(_$ConstrainBoolean_ExactImpl) then,
  ) = __$$ConstrainBoolean_ExactImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool field0});
}

/// @nodoc
class __$$ConstrainBoolean_ExactImplCopyWithImpl<$Res>
    extends _$ConstrainBooleanCopyWithImpl<$Res, _$ConstrainBoolean_ExactImpl>
    implements _$$ConstrainBoolean_ExactImplCopyWith<$Res> {
  __$$ConstrainBoolean_ExactImplCopyWithImpl(
    _$ConstrainBoolean_ExactImpl _value,
    $Res Function(_$ConstrainBoolean_ExactImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ConstrainBoolean
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$ConstrainBoolean_ExactImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as bool,
      ),
    );
  }
}

/// @nodoc

class _$ConstrainBoolean_ExactImpl extends ConstrainBoolean_Exact {
  const _$ConstrainBoolean_ExactImpl(this.field0) : super._();

  @override
  final bool field0;

  @override
  String toString() {
    return 'ConstrainBoolean.exact(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConstrainBoolean_ExactImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of ConstrainBoolean
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ConstrainBoolean_ExactImplCopyWith<_$ConstrainBoolean_ExactImpl>
  get copyWith =>
      __$$ConstrainBoolean_ExactImplCopyWithImpl<_$ConstrainBoolean_ExactImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(bool field0) exact,
    required TResult Function(bool field0) ideal,
  }) {
    return exact(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(bool field0)? exact,
    TResult? Function(bool field0)? ideal,
  }) {
    return exact?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(bool field0)? exact,
    TResult Function(bool field0)? ideal,
    required TResult orElse(),
  }) {
    if (exact != null) {
      return exact(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConstrainBoolean_Exact value) exact,
    required TResult Function(ConstrainBoolean_Ideal value) ideal,
  }) {
    return exact(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConstrainBoolean_Exact value)? exact,
    TResult? Function(ConstrainBoolean_Ideal value)? ideal,
  }) {
    return exact?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConstrainBoolean_Exact value)? exact,
    TResult Function(ConstrainBoolean_Ideal value)? ideal,
    required TResult orElse(),
  }) {
    if (exact != null) {
      return exact(this);
    }
    return orElse();
  }
}

abstract class ConstrainBoolean_Exact extends ConstrainBoolean {
  const factory ConstrainBoolean_Exact(final bool field0) =
      _$ConstrainBoolean_ExactImpl;
  const ConstrainBoolean_Exact._() : super._();

  @override
  bool get field0;

  /// Create a copy of ConstrainBoolean
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ConstrainBoolean_ExactImplCopyWith<_$ConstrainBoolean_ExactImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ConstrainBoolean_IdealImplCopyWith<$Res>
    implements $ConstrainBooleanCopyWith<$Res> {
  factory _$$ConstrainBoolean_IdealImplCopyWith(
    _$ConstrainBoolean_IdealImpl value,
    $Res Function(_$ConstrainBoolean_IdealImpl) then,
  ) = __$$ConstrainBoolean_IdealImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({bool field0});
}

/// @nodoc
class __$$ConstrainBoolean_IdealImplCopyWithImpl<$Res>
    extends _$ConstrainBooleanCopyWithImpl<$Res, _$ConstrainBoolean_IdealImpl>
    implements _$$ConstrainBoolean_IdealImplCopyWith<$Res> {
  __$$ConstrainBoolean_IdealImplCopyWithImpl(
    _$ConstrainBoolean_IdealImpl _value,
    $Res Function(_$ConstrainBoolean_IdealImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ConstrainBoolean
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$ConstrainBoolean_IdealImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as bool,
      ),
    );
  }
}

/// @nodoc

class _$ConstrainBoolean_IdealImpl extends ConstrainBoolean_Ideal {
  const _$ConstrainBoolean_IdealImpl(this.field0) : super._();

  @override
  final bool field0;

  @override
  String toString() {
    return 'ConstrainBoolean.ideal(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConstrainBoolean_IdealImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of ConstrainBoolean
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ConstrainBoolean_IdealImplCopyWith<_$ConstrainBoolean_IdealImpl>
  get copyWith =>
      __$$ConstrainBoolean_IdealImplCopyWithImpl<_$ConstrainBoolean_IdealImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(bool field0) exact,
    required TResult Function(bool field0) ideal,
  }) {
    return ideal(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(bool field0)? exact,
    TResult? Function(bool field0)? ideal,
  }) {
    return ideal?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(bool field0)? exact,
    TResult Function(bool field0)? ideal,
    required TResult orElse(),
  }) {
    if (ideal != null) {
      return ideal(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConstrainBoolean_Exact value) exact,
    required TResult Function(ConstrainBoolean_Ideal value) ideal,
  }) {
    return ideal(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConstrainBoolean_Exact value)? exact,
    TResult? Function(ConstrainBoolean_Ideal value)? ideal,
  }) {
    return ideal?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConstrainBoolean_Exact value)? exact,
    TResult Function(ConstrainBoolean_Ideal value)? ideal,
    required TResult orElse(),
  }) {
    if (ideal != null) {
      return ideal(this);
    }
    return orElse();
  }
}

abstract class ConstrainBoolean_Ideal extends ConstrainBoolean {
  const factory ConstrainBoolean_Ideal(final bool field0) =
      _$ConstrainBoolean_IdealImpl;
  const ConstrainBoolean_Ideal._() : super._();

  @override
  bool get field0;

  /// Create a copy of ConstrainBoolean
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ConstrainBoolean_IdealImplCopyWith<_$ConstrainBoolean_IdealImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$ConstrainU32 {
  int get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) exact,
    required TResult Function(int field0) ideal,
    required TResult Function(int field0, int field1) range,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? exact,
    TResult? Function(int field0)? ideal,
    TResult? Function(int field0, int field1)? range,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? exact,
    TResult Function(int field0)? ideal,
    TResult Function(int field0, int field1)? range,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConstrainU32_Exact value) exact,
    required TResult Function(ConstrainU32_Ideal value) ideal,
    required TResult Function(ConstrainU32_Range value) range,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConstrainU32_Exact value)? exact,
    TResult? Function(ConstrainU32_Ideal value)? ideal,
    TResult? Function(ConstrainU32_Range value)? range,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConstrainU32_Exact value)? exact,
    TResult Function(ConstrainU32_Ideal value)? ideal,
    TResult Function(ConstrainU32_Range value)? range,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $ConstrainU32CopyWith<ConstrainU32> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $ConstrainU32CopyWith<$Res> {
  factory $ConstrainU32CopyWith(
    ConstrainU32 value,
    $Res Function(ConstrainU32) then,
  ) = _$ConstrainU32CopyWithImpl<$Res, ConstrainU32>;
  @useResult
  $Res call({int field0});
}

/// @nodoc
class _$ConstrainU32CopyWithImpl<$Res, $Val extends ConstrainU32>
    implements $ConstrainU32CopyWith<$Res> {
  _$ConstrainU32CopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _value.copyWith(
            field0: null == field0
                ? _value.field0
                : field0 // ignore: cast_nullable_to_non_nullable
                      as int,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$ConstrainU32_ExactImplCopyWith<$Res>
    implements $ConstrainU32CopyWith<$Res> {
  factory _$$ConstrainU32_ExactImplCopyWith(
    _$ConstrainU32_ExactImpl value,
    $Res Function(_$ConstrainU32_ExactImpl) then,
  ) = __$$ConstrainU32_ExactImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$ConstrainU32_ExactImplCopyWithImpl<$Res>
    extends _$ConstrainU32CopyWithImpl<$Res, _$ConstrainU32_ExactImpl>
    implements _$$ConstrainU32_ExactImplCopyWith<$Res> {
  __$$ConstrainU32_ExactImplCopyWithImpl(
    _$ConstrainU32_ExactImpl _value,
    $Res Function(_$ConstrainU32_ExactImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$ConstrainU32_ExactImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$ConstrainU32_ExactImpl extends ConstrainU32_Exact {
  const _$ConstrainU32_ExactImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'ConstrainU32.exact(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConstrainU32_ExactImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ConstrainU32_ExactImplCopyWith<_$ConstrainU32_ExactImpl> get copyWith =>
      __$$ConstrainU32_ExactImplCopyWithImpl<_$ConstrainU32_ExactImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) exact,
    required TResult Function(int field0) ideal,
    required TResult Function(int field0, int field1) range,
  }) {
    return exact(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? exact,
    TResult? Function(int field0)? ideal,
    TResult? Function(int field0, int field1)? range,
  }) {
    return exact?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? exact,
    TResult Function(int field0)? ideal,
    TResult Function(int field0, int field1)? range,
    required TResult orElse(),
  }) {
    if (exact != null) {
      return exact(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConstrainU32_Exact value) exact,
    required TResult Function(ConstrainU32_Ideal value) ideal,
    required TResult Function(ConstrainU32_Range value) range,
  }) {
    return exact(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConstrainU32_Exact value)? exact,
    TResult? Function(ConstrainU32_Ideal value)? ideal,
    TResult? Function(ConstrainU32_Range value)? range,
  }) {
    return exact?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConstrainU32_Exact value)? exact,
    TResult Function(ConstrainU32_Ideal value)? ideal,
    TResult Function(ConstrainU32_Range value)? range,
    required TResult orElse(),
  }) {
    if (exact != null) {
      return exact(this);
    }
    return orElse();
  }
}

abstract class ConstrainU32_Exact extends ConstrainU32 {
  const factory ConstrainU32_Exact(final int field0) = _$ConstrainU32_ExactImpl;
  const ConstrainU32_Exact._() : super._();

  @override
  int get field0;

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ConstrainU32_ExactImplCopyWith<_$ConstrainU32_ExactImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ConstrainU32_IdealImplCopyWith<$Res>
    implements $ConstrainU32CopyWith<$Res> {
  factory _$$ConstrainU32_IdealImplCopyWith(
    _$ConstrainU32_IdealImpl value,
    $Res Function(_$ConstrainU32_IdealImpl) then,
  ) = __$$ConstrainU32_IdealImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int field0});
}

/// @nodoc
class __$$ConstrainU32_IdealImplCopyWithImpl<$Res>
    extends _$ConstrainU32CopyWithImpl<$Res, _$ConstrainU32_IdealImpl>
    implements _$$ConstrainU32_IdealImplCopyWith<$Res> {
  __$$ConstrainU32_IdealImplCopyWithImpl(
    _$ConstrainU32_IdealImpl _value,
    $Res Function(_$ConstrainU32_IdealImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$ConstrainU32_IdealImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$ConstrainU32_IdealImpl extends ConstrainU32_Ideal {
  const _$ConstrainU32_IdealImpl(this.field0) : super._();

  @override
  final int field0;

  @override
  String toString() {
    return 'ConstrainU32.ideal(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConstrainU32_IdealImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ConstrainU32_IdealImplCopyWith<_$ConstrainU32_IdealImpl> get copyWith =>
      __$$ConstrainU32_IdealImplCopyWithImpl<_$ConstrainU32_IdealImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) exact,
    required TResult Function(int field0) ideal,
    required TResult Function(int field0, int field1) range,
  }) {
    return ideal(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? exact,
    TResult? Function(int field0)? ideal,
    TResult? Function(int field0, int field1)? range,
  }) {
    return ideal?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? exact,
    TResult Function(int field0)? ideal,
    TResult Function(int field0, int field1)? range,
    required TResult orElse(),
  }) {
    if (ideal != null) {
      return ideal(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConstrainU32_Exact value) exact,
    required TResult Function(ConstrainU32_Ideal value) ideal,
    required TResult Function(ConstrainU32_Range value) range,
  }) {
    return ideal(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConstrainU32_Exact value)? exact,
    TResult? Function(ConstrainU32_Ideal value)? ideal,
    TResult? Function(ConstrainU32_Range value)? range,
  }) {
    return ideal?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConstrainU32_Exact value)? exact,
    TResult Function(ConstrainU32_Ideal value)? ideal,
    TResult Function(ConstrainU32_Range value)? range,
    required TResult orElse(),
  }) {
    if (ideal != null) {
      return ideal(this);
    }
    return orElse();
  }
}

abstract class ConstrainU32_Ideal extends ConstrainU32 {
  const factory ConstrainU32_Ideal(final int field0) = _$ConstrainU32_IdealImpl;
  const ConstrainU32_Ideal._() : super._();

  @override
  int get field0;

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ConstrainU32_IdealImplCopyWith<_$ConstrainU32_IdealImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ConstrainU32_RangeImplCopyWith<$Res>
    implements $ConstrainU32CopyWith<$Res> {
  factory _$$ConstrainU32_RangeImplCopyWith(
    _$ConstrainU32_RangeImpl value,
    $Res Function(_$ConstrainU32_RangeImpl) then,
  ) = __$$ConstrainU32_RangeImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int field0, int field1});
}

/// @nodoc
class __$$ConstrainU32_RangeImplCopyWithImpl<$Res>
    extends _$ConstrainU32CopyWithImpl<$Res, _$ConstrainU32_RangeImpl>
    implements _$$ConstrainU32_RangeImplCopyWith<$Res> {
  __$$ConstrainU32_RangeImplCopyWithImpl(
    _$ConstrainU32_RangeImpl _value,
    $Res Function(_$ConstrainU32_RangeImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null, Object? field1 = null}) {
    return _then(
      _$ConstrainU32_RangeImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as int,
        null == field1
            ? _value.field1
            : field1 // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$ConstrainU32_RangeImpl extends ConstrainU32_Range {
  const _$ConstrainU32_RangeImpl(this.field0, this.field1) : super._();

  @override
  final int field0;
  @override
  final int field1;

  @override
  String toString() {
    return 'ConstrainU32.range(field0: $field0, field1: $field1)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$ConstrainU32_RangeImpl &&
            (identical(other.field0, field0) || other.field0 == field0) &&
            (identical(other.field1, field1) || other.field1 == field1));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0, field1);

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$ConstrainU32_RangeImplCopyWith<_$ConstrainU32_RangeImpl> get copyWith =>
      __$$ConstrainU32_RangeImplCopyWithImpl<_$ConstrainU32_RangeImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int field0) exact,
    required TResult Function(int field0) ideal,
    required TResult Function(int field0, int field1) range,
  }) {
    return range(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int field0)? exact,
    TResult? Function(int field0)? ideal,
    TResult? Function(int field0, int field1)? range,
  }) {
    return range?.call(field0, field1);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int field0)? exact,
    TResult Function(int field0)? ideal,
    TResult Function(int field0, int field1)? range,
    required TResult orElse(),
  }) {
    if (range != null) {
      return range(field0, field1);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(ConstrainU32_Exact value) exact,
    required TResult Function(ConstrainU32_Ideal value) ideal,
    required TResult Function(ConstrainU32_Range value) range,
  }) {
    return range(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(ConstrainU32_Exact value)? exact,
    TResult? Function(ConstrainU32_Ideal value)? ideal,
    TResult? Function(ConstrainU32_Range value)? range,
  }) {
    return range?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(ConstrainU32_Exact value)? exact,
    TResult Function(ConstrainU32_Ideal value)? ideal,
    TResult Function(ConstrainU32_Range value)? range,
    required TResult orElse(),
  }) {
    if (range != null) {
      return range(this);
    }
    return orElse();
  }
}

abstract class ConstrainU32_Range extends ConstrainU32 {
  const factory ConstrainU32_Range(final int field0, final int field1) =
      _$ConstrainU32_RangeImpl;
  const ConstrainU32_Range._() : super._();

  @override
  int get field0;
  int get field1;

  /// Create a copy of ConstrainU32
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$ConstrainU32_RangeImplCopyWith<_$ConstrainU32_RangeImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
