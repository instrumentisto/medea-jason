//! State of media mute state.

use derive_more::with_trait::Display;

use super::{InStable, InTransition};

/// State of media mute state.
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum Stable {
    /// [`MediaStateControllable`] is muted.
    ///
    /// [`MediaStateControllable`]: crate::peer::MediaStateControllable
    Muted,

    /// [`MediaStateControllable`] is unmuted.
    ///
    /// [`MediaStateControllable`]: crate::peer::MediaStateControllable
    Unmuted,
}

impl Stable {
    /// Returns the opposite value to this [`Stable`].
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Muted => Self::Unmuted,
            Self::Unmuted => Self::Muted,
        }
    }
}

impl From<bool> for Stable {
    fn from(muted: bool) -> Self {
        if muted { Self::Muted } else { Self::Unmuted }
    }
}

impl InStable for Stable {
    type Transition = Transition;

    fn start_transition(self) -> Self::Transition {
        match self {
            Self::Unmuted => Transition::Muting(self),
            Self::Muted => Transition::Unmuting(self),
        }
    }
}

/// [`MuteState`] in transition to another [`Stable`].
///
/// [`Stable`] which is stored in [`Transition`] variants is a state which we
/// have already, but we still waiting for the desired state update. If the
/// desired state update won't be received, then the stored [`Stable`] will be
/// applied.
///
/// [`MuteState`]: super::MuteState
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Transition {
    /// [`MediaStateControllable`] should be muted, but awaits server
    /// permission.
    ///
    /// [`MediaStateControllable`]: crate::peer::MediaStateControllable
    Muting(Stable),

    /// [`MediaStateControllable`] should be unmuted, but awaits server
    /// permission.
    ///
    /// [`MediaStateControllable`]: crate::peer::MediaStateControllable
    Unmuting(Stable),
}

impl InTransition for Transition {
    type Stable = Stable;

    fn intended(self) -> Self::Stable {
        match self {
            Self::Unmuting(_) => Stable::Unmuted,
            Self::Muting(_) => Stable::Muted,
        }
    }

    fn set_inner(self, inner: Self::Stable) -> Self {
        match self {
            Self::Unmuting(_) => Self::Unmuting(inner),
            Self::Muting(_) => Self::Muting(inner),
        }
    }

    fn into_inner(self) -> Self::Stable {
        match self {
            Self::Unmuting(s) | Self::Muting(s) => s,
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::Unmuting(stable) => Self::Muting(stable),
            Self::Muting(stable) => Self::Unmuting(stable),
        }
    }
}
