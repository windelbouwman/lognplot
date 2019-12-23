use crate::geometry::Point;
use crate::style::{Color, Stroke};
use crate::time::{TimeSpan, TimeStamp};
use crate::tsdb::{Aggregation, Observation, Sample, SampleMetrics};
use crate::tsdb::{Query, RangeQueryResult, TsDbHandle};
use std::str::FromStr;

/// A single curve with some stroke styling.
#[derive(Debug, Clone)]
pub struct Curve {
    pub data: CurveData,
    stroke: Stroke,
    legend: Option<String>,
}

/// A dataset. Can be either a trace, or a vector of points!
#[derive(Debug, Clone)]
pub enum CurveData {
    /// A tsdb signal with some name.
    Trace {
        name: String,
        db: TsDbHandle,
        // TODO: cache database queries in some data structure?
    },

    /// Raw points.
    Points(Vec<Point>),
}

impl CurveData {
    pub fn points(x: Vec<f64>, y: Vec<f64>) -> Self {
        let points = x
            .iter()
            .zip(y.iter())
            .map(|p| Point::new(*p.0, *p.1))
            .collect();
        CurveData::Points(points)
    }

    pub fn trace(name: &str, db: TsDbHandle) -> Self {
        CurveData::Trace {
            name: name.to_string(),
            db,
        }
    }
}

impl CurveData {
    fn name(&self) -> String {
        match self {
            CurveData::Points(..) => "no-name".to_string(),
            CurveData::Trace { name, .. } => name.clone(),
        }
    }

    /// Pull data in for drawing the graph.
    pub fn query(
        &self,
        timespan: &TimeSpan,
        amount: usize,
    ) -> Option<RangeQueryResult<Sample, SampleMetrics>> {
        match self {
            // In case of raw data, just return them all.
            CurveData::Points(points) => {
                let observations = points
                    .iter()
                    .map(|p| Observation::new(TimeStamp::new(p.x()), Sample::new(p.y())))
                    .collect();
                Some(RangeQueryResult::Observations(observations))
            }

            // In case of a trace, query database for points.
            CurveData::Trace { name, db } => {
                // Time for time series database benefit
                // TODO: cache results?
                let query = Query::create().amount(amount).span(&timespan).build();
                let result = db.query(name, query);
                result.inner
            }
        }
    }

    fn summary(&self, timespan: Option<&TimeSpan>) -> Option<Aggregation<Sample, SampleMetrics>> {
        match &self {
            CurveData::Points(points) => {
                if timespan.is_some() {
                    None // TODO??
                } else {
                    point_summary(points)
                }
            }
            CurveData::Trace { name, db } => db.summary(name, timespan),
        }
    }
}

/// Calculate aggregate information about points.
fn point_summary(points: &[Point]) -> Option<Aggregation<Sample, SampleMetrics>> {
    if points.is_empty() {
        None
    } else {
        let (first, rest) = points
            .split_first()
            .expect("At least a single point at this point.");
        let mut xmin = first.x();
        let mut xmax = xmin;
        for p in rest {
            if p.x() > xmax {
                xmax = p.x();
            }

            if p.x() < xmin {
                xmin = p.x();
            }
        }
        let timespan = TimeSpan::new(TimeStamp::new(xmin), TimeStamp::new(xmax));

        let y_values: Vec<f64> = points.iter().map(|p| p.y()).collect();
        let metrics = SampleMetrics::from_values(&y_values).unwrap();

        let count = points.len();
        let aggregation = Aggregation::new(timespan, metrics, count);
        Some(aggregation)
    }
}

impl Curve {
    pub fn new(data: CurveData, color: &str) -> Self {
        let stroke = Stroke::from_str(color).unwrap();
        let legend = None;

        Self {
            data,
            stroke,
            legend,
        }
    }

    pub fn color(&self) -> Color {
        self.stroke.color.clone()
    }

    pub fn name(&self) -> String {
        self.data.name()
    }

    /// Retrieve a data summary of this curve.
    pub fn data_summary(
        &self,
        timespan: Option<&TimeSpan>,
    ) -> Option<Aggregation<Sample, SampleMetrics>> {
        self.data.summary(timespan)
    }

    /// Pull data in for drawing the graph.
    pub fn query(
        &self,
        timespan: &TimeSpan,
        amount: usize,
    ) -> Option<RangeQueryResult<Sample, SampleMetrics>> {
        self.data.query(timespan, amount)
    }
}
