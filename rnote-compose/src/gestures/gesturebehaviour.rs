use std::time::Instant;

use crate::penevents::PenEvent;

/// Behaves like a gesture
pub trait GestureBehaviour {
    /// Feed the gesture with a new pen event.
    fn handle_event(&mut self, event: PenEvent, now: Instant);
    /// When a gesture was recognized
    ///
    /// It is required to check this everytime after a event is handled, else the gesture might be missed.
    fn recognized(&self) -> bool;
}
