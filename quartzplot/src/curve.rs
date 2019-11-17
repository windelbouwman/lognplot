use quartzcanvas::geometry::Point;
use quartzcanvas::geometry::Range;
use quartzcanvas::style::{Color, Stroke};
use quartztsdb::Trace;
use std::str::FromStr;
use std::sync::Arc;

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
    Trace(Arc<Trace>),
    Points(Vec<Point>),
    Aggregations(Vec<Aggregate>),
}

/// An aggregation of multiple samples into a range with properties
/// like min, max, median, average, std-deviation etc..
#[derive(Debug, Clone)]
pub struct Aggregate {
    /// Horizontal span of this aggreation.
    domain: Range<f64>,
    min: f64,
    max: f64,
    mean: f64,
    stddev: f64,
}

impl CurveData {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Self {
        let points = x
            .iter()
            .zip(y.iter())
            .map(|p| Point::new(*p.0, *p.1))
            .collect();
        CurveData::Points(points)
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

    pub fn get_points(&self) -> Vec<Point> {
        match &self.data {
            CurveData::Points(p) => p.clone(),
            x => {
                unimplemented!("TBD: Implement this for {:?} as well?", x);
            }
        }
    }

    // TODO: create point pairs!
    // pub fn point_pairs(&self) -> dyn Iterator<Item=(Point, Point)> {
    // self.points.iter().zip(self.points.iter().skip(1)).into_iter()
    // }
}

/*
fn get_range(points: &Vec<Point>) -> Range<f64> {
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
}
*/
