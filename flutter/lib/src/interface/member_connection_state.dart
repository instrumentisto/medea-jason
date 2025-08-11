import 'enums.dart';

/// State of member's `Connection`.
sealed class MemberConnectionState {}

/// State of member's `Connection` in [P2P mesh] mode.
///
/// [P2P mesh]: https://webrtcglossary.com/mesh
class MemberConnectionStateP2P extends MemberConnectionState {
  final PeerConnectionState peerState;

  MemberConnectionStateP2P(this.peerState);
}

/// Possible kinds of `Connection`'s state.
enum MemberConnectionStateKind {
  /// `Connection`'s state is in [P2P mesh] mode.
  ///
  /// [P2P mesh]: https://webrtcglossary.com/mesh
  p2p,
}
