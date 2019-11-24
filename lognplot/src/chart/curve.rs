use crate::geometry::Point;
use crate::geometry::Range;
use crate::style::{Color, Stroke};
use crate::time::{TimeSpan, TimeStamp};
use crate::tsdb::{Observation, Sample, SampleMetrics};
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

    /// Retrieve the horizontal range of this curve.
    /// TODO: rename into get_domain? get_range?
    pub fn get_span(&self) -> Option<Range<f64>> {
        match &self.data {
            CurveData::Points(points) => {
                if points.is_empty() {
                    None
                } else {
                    let mut xmin = points[0].x();
                    let mut xmax = xmin;
                    for p in points {
                        if p.x() > xmax {
                            xmax = p.x();
                        }

                        if p.x() < xmin {
                            xmin = p.x();
                        }
                    }
                    Some(Range::new(xmin, xmax))
                }
            }
            x => {
                unimplemented!("TBD: Implement this for {:?} as well?", x);
            }
        }
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
