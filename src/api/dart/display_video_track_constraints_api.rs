pub use std::cell::RefCell;

use flutter_rust_bridge::{Opaque, SyncReturn};

pub use crate::media::DisplayVideoTrackConstraints;

/// Creates new [`DisplayVideoTrackConstraints`] with none constraints
/// configured.
pub fn display_video_track_constraints_new(
) -> SyncReturn<Opaque<RefCell<DisplayVideoTrackConstraints>>> {
    SyncReturn(Opaque::new(RefCell::new(
        DisplayVideoTrackConstraints::new(),
    )))
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
pub fn display_video_track_constraints_device_id(
    constraints: Opaque<RefCell<DisplayVideoTrackConstraints>>,
    device_id: String,
) -> SyncReturn<()> {
    constraints.borrow_mut().device_id(device_id);
    SyncReturn(())
}

/// Sets an exact [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
pub fn display_video_track_constraints_exact_height(
    constraints: Opaque<RefCell<DisplayVideoTrackConstraints>>,
    exact_height: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().exact_height(exact_height);
    SyncReturn(())
}

/// Sets an ideal [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
pub fn display_video_track_constraints_ideal_height(
    constraints: Opaque<RefCell<DisplayVideoTrackConstraints>>,
    ideal_height: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().ideal_height(ideal_height);
    SyncReturn(())
}

/// Sets an exact [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
pub fn display_video_track_constraints_exact_width(
    constraints: Opaque<RefCell<DisplayVideoTrackConstraints>>,
    exact_width: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().exact_width(exact_width);
    SyncReturn(())
}

/// Sets an ideal [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
pub fn display_video_track_constraints_ideal_width(
    constraints: Opaque<RefCell<DisplayVideoTrackConstraints>>,
    ideal_width: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().ideal_width(ideal_width);
    SyncReturn(())
}

/// Sets an ideal [frameRate][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
pub fn display_video_track_constraints_ideal_frame_rate(
    constraints: Opaque<RefCell<DisplayVideoTrackConstraints>>,
    ideal_frame_rate: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().ideal_frame_rate(ideal_frame_rate);
    SyncReturn(())
}

/// Sets an exact [frameRate][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dfn-framerate
pub fn display_video_track_constraints_exact_frame_rate(
    constraints: Opaque<RefCell<DisplayVideoTrackConstraints>>,
    exact_frame_rate: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().exact_frame_rate(exact_frame_rate);
    SyncReturn(())
}
