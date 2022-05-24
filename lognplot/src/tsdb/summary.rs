use super::observations::{
    Aggregation, CountMetrics, Observation, ProfileEvent, Sample, SampleMetrics, Text,
};
use crate::time::{TimeSpan, TimeStamp};

/// Data summary
pub enum Summary {
    Value(Aggregation<Sample, SampleMetrics>),
    Text(Aggregation<Text, CountMetrics>),
    Profile(Aggregation<ProfileEvent, CountMetrics>),
}

impl Summary {
    pub fn count(&self) -> usize {
        match self {
            Summary::Value(summary) => summary.count,
            Summary::Text(summary) => summary.count,
            Summary::Profile(summary) => summary.count,
        }
    }

    pub fn timespan(&self) -> &TimeSpan {
        match self {
            Summary::Value(summary) => &summary.timespan,
            Summary::Text(summary) => &summary.timespan,
            Summary::Profile(summary) => &summary.timespan,
        }
    }
}

/// Less detailed summary, but easier to keep track of.
#[derive(Debug, Clone)]
pub struct QuickSummary {
    pub count: usize,
    pub last: LastValue,
}

impl QuickSummary {
    pub fn new_value(count: usize, last: Observation<Sample>) -> Self {
        QuickSummary {
            count,
            last: LastValue::Value(last),
        }
    }

    pub fn new_text(count: usize, last: Observation<Text>) -> Self {
        QuickSummary {
            count,
            last: LastValue::Text(last),
        }
    }

    pub fn new_profile(count: usize, last: Observation<ProfileEvent>) -> Self {
        QuickSummary {
            count,
            last: LastValue::Profile(last),
        }
    }

    pub fn last_timestamp(&self) -> &TimeStamp {
        match &self.last {
            LastValue::Value(last) => &last.timestamp,
            LastValue::Text(last) => &last.timestamp,
            LastValue::Profile(last) => &last.timestamp,
        }
    }

    pub fn last_value(&self) -> String {
        match &self.last {
            LastValue::Value(last) => last.value.value.to_string(),
            LastValue::Text(last) => last.value.text.clone(),
            LastValue::Profile(last) => last.value.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LastValue {
    Value(Observation<Sample>),
    Text(Observation<Text>),
    Profile(Observation<ProfileEvent>),
}
