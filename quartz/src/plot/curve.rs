use crate::canvas::{Color, Stroke};
use crate::geometry::Point;
use std::str::FromStr;

/// A single curve with some color.
#[derive(Debug)]
pub struct Curve {
    pub points: Vec<Point>,
    stroke: Stroke,
    legend: Option<String>,
}

impl Curve {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Self {
        let points = x
            .iter()
            .zip(y.iter())
            .map(|p| Point::new(*p.0, *p.1))
            .collect();
        let stroke = Stroke::from_str("blue").unwrap();

        Self {
            points,
            stroke,
            legend: None,
        }
    }

    pub fn color(&self) -> Color {
        self.stroke.color.clone()
    }

    // TODO: create point pairs!
    // pub fn point_pairs(&self) -> dyn Iterator<Item=(Point, Point)> {
    // self.points.iter().zip(self.points.iter().skip(1)).into_iter()
    // }
}
