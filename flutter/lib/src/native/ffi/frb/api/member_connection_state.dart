import 'package:freezed_annotation/freezed_annotation.dart' hide protected;

part 'member_connection_state.freezed.dart';

@freezed
sealed class MemberConnectionState with _$MemberConnectionState {
  const MemberConnectionState._();

  /// State in P2P mode.
  const factory MemberConnectionState.p2p(PeerConnectionState peerState) =
      MemberConnectionState_P2P;
}

/// `PeerConnection`'s connection state.
enum PeerConnectionState {
  /// At least one of the connection's ICE transports are in the
  /// [`IceConnectionState::New`] state, and none of them are in one
  /// of the following states: [`IceConnectionState::Checking`],
  /// [`IceConnectionState::Failed`], or
  /// [`IceConnectionState::Disconnected`], or all of the connection's
  /// transports are in the [`IceConnectionState::Closed`] state.
  new_,

  /// One or more of the ICE transports are currently in the process of
  /// establishing a connection; that is, their [`IceConnectionState`] is
  /// either [`IceConnectionState::Checking`] or
  /// [`IceConnectionState::Connected`], and no transports are in the
  /// [`IceConnectionState::Failed`] state.
  connecting,

  /// Every ICE transport used by the connection is either in use (state
  /// [`IceConnectionState::Connected`] or [`IceConnectionState::Completed`])
  /// or is closed ([`IceConnectionState::Closed`]); in addition,
  /// at least one transport is either [`IceConnectionState::Connected`] or
  /// [`IceConnectionState::Completed`].
  connected,

  /// At least one of the ICE transports for the connection is in the
  /// [`IceConnectionState::Disconnected`] state and none of the other
  /// transports are in the state [`IceConnectionState::Failed`] or
  /// [`IceConnectionState::Checking`].
  ///
  /// It's not a terminal state, and it can go back to `Connecting`
  /// and then `Connected` on its own.
  disconnected,

  /// One or more of the ICE transports on the connection is in the
  /// [`IceConnectionState::Failed`] state.
  ///
  /// It's not a terminal state, and it can be fixed with ICE restart if
  /// signalling connection is alive.
  failed,

  /// The `PeerConnection` is closed.
  ///
  /// It's a terminal state.
  closed,
}
