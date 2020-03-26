use super::trace::Trace;
use super::Observation;
use super::TrackType;
use super::{CountMetrics, Text};
use super::{Query, QueryResult, QuickSummary, Sample, SampleMetrics, Summary};
use crate::time::TimeSpan;

#[derive(Debug)]
pub enum Track {
    Value(Trace<Sample, SampleMetrics>),
    Text(Trace<Text, CountMetrics>),
}

impl Track {
    pub fn new_with_type(typ: TrackType) -> Self {
        match typ {
            TrackType::Value => Track::Value(Default::default()),
            TrackType::Text => Track::Text(Default::default()),
        }
    }

    pub fn get_type(&self) -> TrackType {
        match self {
            Track::Value(..) => TrackType::Value,
            Track::Text(..) => TrackType::Text,
        }
    }

    pub fn add_value_observation(&mut self, observation: Observation<Sample>) {
        if let Track::Value(trace) = self {
            trace.add_observation(observation)
        } else {
            panic!("Cannot add value observation to non-value track")
        }
    }

    pub fn add_value_observations(&mut self, observations: Vec<Observation<Sample>>) {
        if let Track::Value(trace) = self {
            trace.add_observations(observations)
        } else {
            panic!("Cannot add value observations to non-value track")
        }
    }

    pub fn add_text_observation(&mut self, observation: Observation<Text>) {
        if let Track::Text(trace) = self {
            trace.add_observation(observation)
        } else {
            panic!("Cannot add text observation to non-text track")
        }
    }

    pub fn query(&self, query: Query) -> QueryResult {
        match self {
            Track::Value(trace) => QueryResult::Value(trace.query(query)),
            Track::Text(trace) => QueryResult::Text(trace.query(query)),
        }
    }

    pub fn quick_summary(&self) -> Option<QuickSummary> {
        match self {
            Track::Value(trace) => {
                let (count, last) = trace.quick_summary()?;
                Some(QuickSummary::new_value(count, last))
            }
            Track::Text(trace) => {
                let (count, last) = trace.quick_summary()?;
                Some(QuickSummary::new_text(count, last))
            }
        }
    }

    pub fn summary(&self, timespan: Option<&TimeSpan>) -> Option<Summary> {
        match self {
            Track::Value(trace) => Some(Summary::Value(trace.summary(timespan)?)),
            Track::Text(trace) => Some(Summary::Text(trace.summary(timespan)?)),
        }
    }

    pub fn to_vec(&self) -> Vec<Observation<Sample>> {
        if let Track::Value(trace) = self {
            trace.to_vec()
        } else {
            vec![]
        }
    }
}
