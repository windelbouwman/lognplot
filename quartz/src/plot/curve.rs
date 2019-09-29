use crate::geometry::Point;
use crate::style::{Color, Stroke};
use crate::tsdb::Trace;
use std::str::FromStr;
use std::sync::Arc;

/// A single curve with some stroke styling.
#[derive(Debug)]
pub struct Curve {
    pub data: CurveData,
    stroke: Stroke,
    legend: Option<String>,
}

/// A dataset. Can be either a trace, or a vector of points!
#[derive(Debug)]
pub enum CurveData {
    Trace(Arc<Trace>),
    Points(Vec<Point>),
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

    pub fn get_points(&self) -> Vec<Point> {
        match &self.data {
            CurveData::Points(p) => p.clone(),
            CurveData::Trace(_) => {
                unimplemented!("TBD: Implement this for trace as well?");
            }
        }
    }

    // TODO: create point pairs!
    // pub fn point_pairs(&self) -> dyn Iterator<Item=(Point, Point)> {
    // self.points.iter().zip(self.points.iter().skip(1)).into_iter()
    // }
}
