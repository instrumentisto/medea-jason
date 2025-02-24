//! State of the media publishing.

use derive_more::with_trait::Display;

use super::{InStable, InTransition};

/// State of the media publishing.
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum Stable {
    /// [`MediaStateControllable`] is enabled.
    ///
    /// [`MediaStateControllable`]: crate::peer::MediaStateControllable
    Enabled,

    /// [`MediaStateControllable`] is disabled.
    ///
    /// [`MediaStateControllable`]: crate::peer::MediaStateControllable
    Disabled,
}

impl Stable {
    /// Returns the opposite value to this [`Stable`].
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Enabled => Self::Disabled,
            Self::Disabled => Self::Enabled,
        }
    }
}

impl InStable for Stable {
    type Transition = Transition;

    /// Converts this [`Stable`] into
    /// [`Transition`].
    ///
    /// [`Stable::Enabled`] =>
    /// [`Transition::Disabling`].
    ///
    /// [`Stable::Disabled`] =>
    /// [`Transition::Enabling`].
    fn start_transition(self) -> Self::Transition {
        match self {
            Self::Enabled => Transition::Disabling(self),
            Self::Disabled => Transition::Enabling(self),
        }
    }
}

impl From<bool> for Stable {
    fn from(enabled: bool) -> Self {
        if enabled { Self::Enabled } else { Self::Disabled }
    }
}

/// [`MediaExchangeState`] in transition to another [`Stable`].
///
/// [`Stable`] which is stored in [`Transition`] variants is a state which we
/// have already, but we still waiting for the desired state update. If the
/// desired state update won't be received, then the stored [`Stable`] will be
/// applied.
///
/// [`MediaExchangeState`]: super::MediaExchangeState
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Transition {
    /// [`MediaStateControllable`] should be enabled, but awaits server
    /// permission.
    ///
    /// [`MediaStateControllable`]: crate::peer::MediaStateControllable
    Enabling(Stable),

    /// [`MediaStateControllable`] should be disabled, but awaits server
    /// permission.
    ///
    /// [`MediaStateControllable`]: crate::peer::MediaStateControllable
    Disabling(Stable),
}

impl InTransition for Transition {
    type Stable = Stable;

    /// Returns intention which this [`Transition`] indicates.
    fn intended(self) -> Self::Stable {
        match self {
            Self::Enabling(_) => Stable::Enabled,
            Self::Disabling(_) => Stable::Disabled,
        }
    }

    /// Sets inner [`Stable`].
    fn set_inner(self, inner: Self::Stable) -> Self {
        match self {
            Self::Enabling(_) => Self::Enabling(inner),
            Self::Disabling(_) => Self::Disabling(inner),
        }
    }

    /// Returns inner [`Stable`].
    fn into_inner(self) -> Self::Stable {
        match self {
            Self::Enabling(s) | Self::Disabling(s) => s,
        }
    }

    /// Converts [`Transition`] to the opposite [`Transition`] with the same
    /// inner [`Stable`].
    fn opposite(self) -> Self {
        match self {
            Self::Enabling(stable) => Self::Disabling(stable),
            Self::Disabling(stable) => Self::Enabling(stable),
        }
    }
}
