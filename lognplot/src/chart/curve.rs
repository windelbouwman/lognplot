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
    /// Pull data in for drawing the graph.
    pub fn query(
        &self,
        timespan: &TimeSpan,
        amount: usize,
    ) -> RangeQueryResult<Sample, SampleMetrics> {
        match self {
            // In case of raw data, just return them all.
            CurveData::Points(points) => {
                let observations = points
                    .iter()
                    .map(|p| Observation::new(TimeStamp::new(p.x()), Sample::new(p.y())))
                    .collect();
                RangeQueryResult::Observations(observations)
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

    fn summary(&self) -> Option<Aggregation<Sample, SampleMetrics>> {
        match &self {
            CurveData::Points(points) => point_summary(points),
            CurveData::Trace { name, db } => db.summary(name),
        }
    }

    fn range_summary(&self, timespan: &TimeSpan) -> Option<Aggregation<Sample, SampleMetrics>> {
        match &self {
            CurveData::Points(_points) => None, // TODO??
            CurveData::Trace { name, db } => db.range_summary(name, timespan),
        }
    }
}

/// Calculate aggregate information about points.
fn point_summary(points: &[Point]) -> Option<Aggregation<Sample, SampleMetrics>> {
    if points.is_empty() {
        None
    } else {
        let count = points.len();
        let (first, rest) = points.split_first().unwrap();
        let mut xmin = first.x();
        let mut xmax = xmin;
        let mut ymin = first.y();
        let mut ymax = ymin;
        let mut ysum = ymin;
        let mut ysum_squared = ymin * ymin;
        for p in rest {
            if p.x() > xmax {
                xmax = p.x();
            }

            if p.x() < xmin {
                xmin = p.x();
            }

            if p.y() > ymax {
                ymax = p.y();
            }

            if p.y() < ymin {
                ymin = p.y();
            }

            ysum += p.y();
            ysum_squared += p.y() * p.y();
        }

        let timespan = TimeSpan::new(TimeStamp::new(xmin), TimeStamp::new(xmax));
        let metrics = SampleMetrics::new(ymin, ymax, ysum, ysum_squared, count);
        let aggregation = Aggregation::new(timespan, metrics, count);
        Some(aggregation)
    }
}

impl Curve {
    pub fn new(data: CurveData) -> Self {
        let stroke = Stroke::from_str("blue").unwrap();
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

    /// Retrieve a data summary of a time slice of this curve.
    pub fn range_summary(&self, timespan: &TimeSpan) -> Option<Aggregation<Sample, SampleMetrics>> {
        self.data.range_summary(timespan)
    }

    /// Retrieve a data summary of this curve.
    pub fn summary(&self) -> Option<Aggregation<Sample, SampleMetrics>> {
        self.data.summary()
    }

    /// Pull data in for drawing the graph.
    pub fn query(
        &self,
        timespan: &TimeSpan,
        amount: usize,
    ) -> RangeQueryResult<Sample, SampleMetrics> {
        self.data.query(timespan, amount)
    }
}
