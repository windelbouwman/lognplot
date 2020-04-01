//! Data structures to describe a call stack over time.
//!
//! Idea is to enable logging of profile events into a
//! callstack over time manner.

use super::CountMetrics;
use super::Observation;
use crate::time::TimeStamp;
use std::fmt;

/// A single profiling event, such as function enter or function
/// return.
#[derive(Clone, Debug)]
pub enum ProfileEvent {
    FunctionEnter { name: String },
    FunctionExit,
}

// This is a bit lame, but impl<V> From<V> for CountMetrics conflicts with a builtin From implementation.
impl From<ProfileEvent> for CountMetrics {
    fn from(_observation: ProfileEvent) -> Self {
        CountMetrics { count: 1 }
    }
}

impl fmt::Display for ProfileEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfileEvent::FunctionEnter { name } => write!(f, "Function enter: {}", name),
            ProfileEvent::FunctionExit => write!(f, "Function exit"),
        }
    }
}

/// A structure holding a running call stack.
struct CallTrack {
    root: Call,
}

impl CallTrack {
    pub fn add_observation(&mut self, observation: Observation<ProfileEvent>) {
        match observation.value {
            ProfileEvent::FunctionEnter { name } => {
                let call = Call::new(observation.timestamp, name);
                self.root.child_calls.push(call);
            }
            ProfileEvent::FunctionExit => {
                self.root.end = Some(observation.timestamp);
            }
        }
    }
}

/// A single function call node.
///
/// Each function call has a name, a start and end time
/// and other calls to other functions.
struct Call {
    name: String,

    start: Option<TimeStamp>,

    end: Option<TimeStamp>,

    child_calls: Vec<Call>,
}

impl Call {
    fn new(timestamp: TimeStamp, name: String) -> Self {
        Call {
            name,
            start: Some(timestamp),
            end: None,
            child_calls: vec![],
        }
    }
}
