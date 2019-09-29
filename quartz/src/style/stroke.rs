use super::color::Color;
use std::str::FromStr;

/// A stroking style, with color and width
#[derive(Debug)]
pub struct Stroke {
    pub color: Color,
    pub width: f64,
}

impl Stroke {
    fn new(color: Color, width: f64) -> Self {
        Stroke { color, width }
    }
}

impl FromStr for Stroke {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = Color::from_str(s)?;
        Ok(Self::new(color, 1.0))
    }
}
