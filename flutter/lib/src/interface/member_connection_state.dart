import 'enums.dart';

/// State of a member `Connection`.
sealed class MemberConnectionState {}

/// State of a member `Connection` in `P2P` mode.
class MemberConnectionStateP2P extends MemberConnectionState {
  final PeerConnectionState peerState;

  MemberConnectionStateP2P(this.peerState);
}

/// `Connection`'s state kind.
enum MemberConnectionStateKind {
  /// `Connection`'s state is in P2P mode.
  p2p,
}
