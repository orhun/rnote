use std::time::{Duration, Instant};

use crate::penevents::PenEvent;

use super::GestureBehaviour;

/// double click gesture state
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GestureDoubleClickState {
    /// idle
    Idle,
    /// one down
    ///
    /// holds initial time
    OneDown(Instant),
    /// one down click, one up.
    ///
    // holds time of the last mouse release
    OneDownOneUp(Instant),
    /// Gesture recognized
    Recognized,
}

impl Default for GestureDoubleClickState {
    fn default() -> Self {
        Self::Idle
    }
}

/// A double click gesture
#[derive(Debug, Clone)]
pub struct GestureDoubleClick {
    timeout: Duration,
    state: GestureDoubleClickState,
}

impl Default for GestureDoubleClick {
    fn default() -> Self {
        Self {
            timeout: Duration::from_millis(400),
            state: Default::default(),
        }
    }
}

impl GestureBehaviour for GestureDoubleClick {
    fn handle_event(&mut self, event: PenEvent, now: Instant) {
        /*
               log::debug!(
                   "double click gesture handle_event: state: {:#?}, event: {:#?}",
                   self.state,
                   event
               );
        */

        match (&mut self.state, event) {
            (GestureDoubleClickState::Idle, PenEvent::Down { .. }) => {
                self.state = GestureDoubleClickState::OneDown(now);
            }
            (GestureDoubleClickState::OneDown(_), PenEvent::Down { .. }) => {}
            (GestureDoubleClickState::OneDownOneUp(then), PenEvent::Down { .. }) => {
                if now.duration_since(*then) < self.timeout {
                    self.state = GestureDoubleClickState::Recognized;
                } else {
                    self.state = GestureDoubleClickState::Idle;
                }
            }
            (GestureDoubleClickState::OneDown(then), PenEvent::Up { .. }) => {
                if now.duration_since(*then) < self.timeout {
                    self.state = GestureDoubleClickState::OneDownOneUp(now);
                } else {
                    self.state = GestureDoubleClickState::Idle;
                }
            }
            (GestureDoubleClickState::OneDownOneUp(_), PenEvent::Up { .. }) => {}
            (_, _) => self.state = GestureDoubleClickState::Idle,
        }
    }

    fn recognized(&self) -> bool {
        self.state == GestureDoubleClickState::Recognized
    }
}

impl GestureDoubleClick {
    /// Specify a double click timeout. It is defined as the time that can pass between every down -> up -> down state
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}
