
use crate::geometry::Point;
use crate::canvas::{Color};
use std::str::FromStr;

/// A single curve with some color.
#[derive(Debug)]
pub struct Curve {
    pub points: Vec<Point>,
    color: Color,
    legend: Option<String>,
}

impl Curve {
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Self {
        let points = x
            .iter()
            .zip(y.iter())
            .map(|p| Point::new(*p.0, *p.1))
            .collect();
        let color = Color::from_str("blue").unwrap();

        Self {
            points,
            color,
            legend: None,
        }
    }

    pub fn color(&self) -> Color {
        self.color.clone()
    }

    // TODO: create point pairs!
    // pub fn point_pairs(&self) -> dyn Iterator<Item=(Point, Point)> {
    // self.points.iter().zip(self.points.iter().skip(1)).into_iter()
    // }
}
