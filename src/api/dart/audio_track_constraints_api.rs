pub use std::cell::RefCell;

use flutter_rust_bridge::{Opaque, SyncReturn};

pub use crate::media::AudioTrackConstraints;

/// Creates new [`AudioTrackConstraints`] with none constraints configured.
pub fn audio_track_constraints_new(
) -> SyncReturn<Opaque<RefCell<AudioTrackConstraints>>> {
    SyncReturn(Opaque::new(RefCell::new(AudioTrackConstraints::new())))
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
pub fn audio_track_constraints_device_id(
    track: Opaque<RefCell<AudioTrackConstraints>>,
    device_id: String,
) -> SyncReturn<()> {
    track.borrow_mut().device_id(device_id);
    SyncReturn(())
}
