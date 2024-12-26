pub mod bezier;
pub mod curve;
mod progress;

use crate::{Animate, Event};
pub use curve::Curve;
pub use progress::Progress;
use std::time::{Duration, Instant};

/// The default duration for animations used for [`Default`] implementations.
pub(crate) const DEFAULT_DURATION: Duration = Duration::from_millis(500);

/// A type of animation that transitions between two values.
#[derive(Debug, Clone, PartialEq)]
pub struct Transition<T> {
    /// The initial value of the transition - the starting point.
    initial: T,
    /// The current value at this point in the transition.
    value: T,
    /// The target value of the transition - the end point.
    target: T,
    /// The curve to use to determine how to update the current value over time.
    curve: Curve,
    /// How long the transition should take to complete.
    duration: Duration,
    /// How far along the transition is.
    progress: Progress,
    /// The time at which the transition was last updated.
    last_update: Instant,
}

impl<T> Transition<T>
where
    T: Animate,
{
    /// Creates a new transition with the given `value`.
    pub fn new(value: T) -> Self {
        Self {
            initial: value.clone(),
            target: value.clone(),
            value,
            curve: Curve::default(),
            duration: DEFAULT_DURATION,
            progress: Progress::default(),
            last_update: Instant::now(),
        }
    }

    /// Sets the curve to use for the transition and returns the updated transition.
    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = curve;
        self
    }

    /// Sets the duration of the transition and returns the updated transition.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the duratoin of the transition.
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    /// Returns a reference to the current `value` of the transition.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns a reference to the current `target` of the transition.
    /// This is the final value that the transition is moving towards.
    ///
    /// If the transition is currently reversing, this will be the `initial` value.
    pub fn target(&self) -> &T {
        match self.progress {
            Progress::Forward(_) => &self.target,
            Progress::Reverse(_) => &self.initial,
        }
    }

    /// Reverses the transition, swapping the initial and target values
    /// and adjusts the animation status to be in the opposite direction.
    pub fn reverse(&mut self) {
        self.progress.reverse();
    }

    /// Ends the transition, immediately setting the current value to the target value.
    pub fn settle(&mut self) {
        self.progress.settle();
        match self.progress {
            Progress::Forward(_) => self.value = self.target.clone(),
            Progress::Reverse(_) => self.value = self.initial.clone(),
        }
    }

    /// Updates the transition with details of the given `event`.
    pub fn update(&mut self, event: Event<T>) {
        match event {
            Event::Settle => self.settle(),
            Event::Tick(now) => self.tick(now),
            Event::Target(target) => self.interrupt(target),
        }
    }

    /// Interrupts the existing transition and starts a new one with the new `target`.
    pub fn interrupt(&mut self, target: T) {
        // Reset the last update if the transition isn't moving.
        // This avoids resetting the last update during continuously interrupted animations.
        if !self.is_animating() {
            self.last_update = Instant::now();
        }

        // Reverse the transition if the new target is the initial
        // value and the animation isn't done. This ensures that the
        // animation follows the same curve when reversing.
        let is_initial_target = match self.progress {
            Progress::Forward(_) => target == self.initial,
            Progress::Reverse(_) => target == self.target,
        };

        if is_initial_target && !self.progress.is_complete() {
            self.reverse();
        } else if &target != self.target() {
            // Target has changed, reset the progress and update the initial value.
            self.progress = Progress::Forward(0.0);
            self.initial = self.value.clone();
            self.target = target;
        }

        self.last_update = Instant::now();
    }

    /// Updates the transition's value based on the elapsed time since the last update.
    pub fn tick(&mut self, now: Instant) {
        if !self.is_animating() {
            return;
        }

        // Figure out how much time has passed since the last update
        let delta = now.duration_since(self.last_update);
        self.last_update = now;

        self.progress
            .update(delta.as_secs_f32() / self.duration.as_secs_f32());
        if self.progress.is_complete() {
            // We're at the target - assign the current value to the target value.
            self.value = self.target().clone();
        } else {
            // Continue to lerp the value towards the target
            self.value.lerp(
                &self.initial,
                &self.target,
                self.curve.value(self.progress.value()),
            );
        }
    }

    /// Whether this transition is currently animating towards its target.
    pub fn is_animating(&self) -> bool {
        !self.progress.is_complete()
    }
}
