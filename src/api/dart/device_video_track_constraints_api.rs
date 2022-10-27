pub use std::cell::RefCell;

use flutter_rust_bridge::{Opaque, SyncReturn};

pub use crate::media::DeviceVideoTrackConstraints;
use crate::media::FacingMode;

/// Creates new [`DeviceVideoTrackConstraints`] with none constraints
/// configured.
pub fn device_video_track_constraints_new(
) -> SyncReturn<Opaque<RefCell<DeviceVideoTrackConstraints>>> {
    SyncReturn(Opaque::new(
        RefCell::new(DeviceVideoTrackConstraints::new()),
    ))
}

/// Sets an exact [deviceId][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#def-constraint-deviceId
pub fn device_video_track_constraints_device_id(
    constraints: Opaque<RefCell<DeviceVideoTrackConstraints>>,
    device_id: String,
) -> SyncReturn<()> {
    constraints.borrow_mut().device_id(device_id);
    SyncReturn(())
}

/// Sets an exact [facingMode][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
pub fn device_video_track_constraints_exact_facing_mode(
    constraints: Opaque<RefCell<DeviceVideoTrackConstraints>>,
    facing_mode: FacingMode,
) -> SyncReturn<()> {
    constraints.borrow_mut().exact_facing_mode(facing_mode);
    SyncReturn(())
}

/// Sets an ideal [facingMode][1] constraint.
///
/// [1]: https://w3.org/TR/mediacapture-streams#dom-constraindomstring
pub fn device_video_track_constraints_ideal_facing_mode(
    constraints: Opaque<RefCell<DeviceVideoTrackConstraints>>,
    facing_mode: FacingMode,
) -> SyncReturn<()> {
    constraints.borrow_mut().ideal_facing_mode(facing_mode);
    SyncReturn(())
}

/// Sets an exact [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
pub fn device_video_track_constraints_exact_height(
    constraints: Opaque<RefCell<DeviceVideoTrackConstraints>>,
    exact_height: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().exact_height(exact_height);
    SyncReturn(())
}

/// Sets an ideal [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
pub fn device_video_track_constraints_ideal_height(
    constraints: Opaque<RefCell<DeviceVideoTrackConstraints>>,
    ideal_height: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().ideal_height(ideal_height);
    SyncReturn(())
}

/// Sets an exact [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
pub fn device_video_track_constraints_exact_width(
    constraints: Opaque<RefCell<DeviceVideoTrackConstraints>>,
    exact_width: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().exact_width(exact_width);
    SyncReturn(())
}

/// Sets an ideal [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
pub fn device_video_track_constraints_ideal_width(
    constraints: Opaque<RefCell<DeviceVideoTrackConstraints>>,
    ideal_width: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().ideal_width(ideal_width);
    SyncReturn(())
}

/// Sets a range of a [height][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-height
pub fn device_video_track_constraints_height_in_range(
    constraints: Opaque<RefCell<DeviceVideoTrackConstraints>>,
    min: u32,
    max: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().height_in_range(min, max);
    SyncReturn(())
}

/// Sets a range of a [width][1] constraint.
///
/// [1]: https://tinyurl.com/w3-streams#def-constraint-width
pub fn device_video_track_constraints_width_in_range(
    constraints: Opaque<RefCell<DeviceVideoTrackConstraints>>,
    min: u32,
    max: u32,
) -> SyncReturn<()> {
    constraints.borrow_mut().width_in_range(min, max);
    SyncReturn(())
}
