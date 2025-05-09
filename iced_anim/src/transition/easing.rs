use crate::animated::DEFAULT_DURATION;

use super::Curve;
use std::time::Duration;

/// A configuration for creating a `Transition`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Easing {
    /// The curve to use to determine how to update the current value over time.
    pub curve: Curve,
    /// How long the transition should take to complete.
    pub duration: Duration,
    /// Whether the transition will move in reverse if a new target value
    /// is the same as the initial value.
    ///
    /// Reversible animations will transition the current value along the curve backwards if the
    /// new target value is the same as the initial value, e.g. `0.0 -> 1.0 -> 0.0`. Otherwise,
    /// changing the target value will always be treated as moving forward along the curve and
    /// restart the transition from the beginning.
    pub reversible: bool,
}

impl Default for Easing {
    fn default() -> Self {
        Self {
            curve: Curve::default(),
            duration: DEFAULT_DURATION,
            reversible: false,
        }
    }
}

impl Easing {
    /// A default easing that uses [`Curve::Linear`] and the default duration.
    pub const LINEAR: Self = Self {
        curve: Curve::Linear,
        duration: DEFAULT_DURATION,
        reversible: false,
    };

    /// A default easing that uses [`Curve::Ease`] and the default duration.
    pub const EASE: Self = Self {
        curve: Curve::Ease,
        duration: DEFAULT_DURATION,
        reversible: false,
    };

    /// A default easing that uses [`Curve::EaseIn`] and the default duration.
    pub const EASE_IN: Self = Self {
        curve: Curve::EaseIn,
        duration: DEFAULT_DURATION,
        reversible: false,
    };

    /// A default easing that uses [`Curve::EaseOut`] and the default duration.
    pub const EASE_OUT: Self = Self {
        curve: Curve::EaseOut,
        duration: DEFAULT_DURATION,
        reversible: false,
    };

    /// A default easing that uses [`Curve::EaseInOut`] and the default duration.
    pub const EASE_IN_OUT: Self = Self {
        curve: Curve::EaseInOut,
        duration: DEFAULT_DURATION,
        reversible: false,
    };

    /// Creates a new [`Easing`] with the given `curve`.
    pub fn new(curve: Curve) -> Self {
        Self {
            curve,
            duration: DEFAULT_DURATION,
            reversible: false,
        }
    }

    /// Sets the `curve` of the easing and returns the updated easing.
    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = curve;
        self
    }

    /// Sets the `duration` of the easing and returns the updated easing.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the duration of the [`Easing`] value to 100ms.
    pub fn very_quick(self) -> Self {
        self.with_duration(Duration::from_millis(100))
    }

    /// Sets the duration of the [`Easing`] to 200ms.
    pub fn quick(self) -> Self {
        self.with_duration(Duration::from_millis(200))
    }

    /// Sets the duration of the [`Easing`] to 400ms.
    pub fn slow(self) -> Self {
        self.with_duration(Duration::from_millis(400))
    }

    /// Sets the duration of the [`Easing`] to 500ms.
    pub fn very_slow(self) -> Self {
        self.with_duration(Duration::from_millis(500))
    }

    /// Sets whether the easing is reversible and returns the updated easing.
    ///
    /// Reversible animations will transition the current value along the curve backwards if the
    /// new target value is the same as the initial value, e.g. `0.0 -> 1.0 -> 0.0`. Otherwise,
    /// changing the target value will always be treated as moving forward along the curve and
    /// restart the transition from the beginning.
    pub fn reversible(mut self, reversible: bool) -> Self {
        self.reversible = reversible;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_easings() {
        assert_eq!(Easing::LINEAR.curve, Curve::Linear);
        assert_eq!(Easing::EASE.curve, Curve::Ease);
        assert_eq!(Easing::EASE_IN.curve, Curve::EaseIn);
        assert_eq!(Easing::EASE_OUT.curve, Curve::EaseOut);
        assert_eq!(Easing::EASE_IN_OUT.curve, Curve::EaseInOut);
    }

    #[test]
    fn default() {
        let easing = Easing::default();

        assert_eq!(easing.curve, Curve::default());
        assert_eq!(easing.duration, DEFAULT_DURATION);
        assert!(!easing.reversible);
    }

    #[test]
    fn initialization() {
        let easing = Easing::new(Curve::EaseInOut)
            .with_duration(Duration::from_millis(300))
            .reversible(true);

        assert_eq!(easing.curve, Curve::EaseInOut);
        assert_eq!(easing.duration, Duration::from_millis(300));
        assert!(easing.reversible);
    }
}
