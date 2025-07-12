// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'member_connection_state.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$MemberConnectionState {
  PeerConnectionState get field0 => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(PeerConnectionState field0) p2P,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(PeerConnectionState field0)? p2P,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(PeerConnectionState field0)? p2P,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MemberConnectionState_P2P value) p2P,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MemberConnectionState_P2P value)? p2P,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MemberConnectionState_P2P value)? p2P,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;

  /// Create a copy of MemberConnectionState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $MemberConnectionStateCopyWith<MemberConnectionState> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $MemberConnectionStateCopyWith<$Res> {
  factory $MemberConnectionStateCopyWith(
    MemberConnectionState value,
    $Res Function(MemberConnectionState) then,
  ) = _$MemberConnectionStateCopyWithImpl<$Res, MemberConnectionState>;
  @useResult
  $Res call({PeerConnectionState field0});
}

/// @nodoc
class _$MemberConnectionStateCopyWithImpl<
  $Res,
  $Val extends MemberConnectionState
>
    implements $MemberConnectionStateCopyWith<$Res> {
  _$MemberConnectionStateCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of MemberConnectionState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _value.copyWith(
            field0: null == field0
                ? _value.field0
                : field0 // ignore: cast_nullable_to_non_nullable
                      as PeerConnectionState,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$MemberConnectionState_P2PImplCopyWith<$Res>
    implements $MemberConnectionStateCopyWith<$Res> {
  factory _$$MemberConnectionState_P2PImplCopyWith(
    _$MemberConnectionState_P2PImpl value,
    $Res Function(_$MemberConnectionState_P2PImpl) then,
  ) = __$$MemberConnectionState_P2PImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({PeerConnectionState field0});
}

/// @nodoc
class __$$MemberConnectionState_P2PImplCopyWithImpl<$Res>
    extends
        _$MemberConnectionStateCopyWithImpl<
          $Res,
          _$MemberConnectionState_P2PImpl
        >
    implements _$$MemberConnectionState_P2PImplCopyWith<$Res> {
  __$$MemberConnectionState_P2PImplCopyWithImpl(
    _$MemberConnectionState_P2PImpl _value,
    $Res Function(_$MemberConnectionState_P2PImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of MemberConnectionState
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? field0 = null}) {
    return _then(
      _$MemberConnectionState_P2PImpl(
        null == field0
            ? _value.field0
            : field0 // ignore: cast_nullable_to_non_nullable
                  as PeerConnectionState,
      ),
    );
  }
}

/// @nodoc

class _$MemberConnectionState_P2PImpl extends MemberConnectionState_P2P {
  const _$MemberConnectionState_P2PImpl(this.field0) : super._();

  @override
  final PeerConnectionState field0;

  @override
  String toString() {
    return 'MemberConnectionState.p2P(field0: $field0)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$MemberConnectionState_P2PImpl &&
            (identical(other.field0, field0) || other.field0 == field0));
  }

  @override
  int get hashCode => Object.hash(runtimeType, field0);

  /// Create a copy of MemberConnectionState
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$MemberConnectionState_P2PImplCopyWith<_$MemberConnectionState_P2PImpl>
  get copyWith =>
      __$$MemberConnectionState_P2PImplCopyWithImpl<
        _$MemberConnectionState_P2PImpl
      >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(PeerConnectionState field0) p2P,
  }) {
    return p2P(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(PeerConnectionState field0)? p2P,
  }) {
    return p2P?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(PeerConnectionState field0)? p2P,
    required TResult orElse(),
  }) {
    if (p2P != null) {
      return p2P(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(MemberConnectionState_P2P value) p2P,
  }) {
    return p2P(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(MemberConnectionState_P2P value)? p2P,
  }) {
    return p2P?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(MemberConnectionState_P2P value)? p2P,
    required TResult orElse(),
  }) {
    if (p2P != null) {
      return p2P(this);
    }
    return orElse();
  }
}

abstract class MemberConnectionState_P2P extends MemberConnectionState {
  const factory MemberConnectionState_P2P(final PeerConnectionState field0) =
      _$MemberConnectionState_P2PImpl;
  const MemberConnectionState_P2P._() : super._();

  @override
  PeerConnectionState get field0;

  /// Create a copy of MemberConnectionState
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$MemberConnectionState_P2PImplCopyWith<_$MemberConnectionState_P2PImpl>
  get copyWith => throw _privateConstructorUsedError;
}
