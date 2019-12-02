use std::str::FromStr;

/// Color indication
#[derive(Debug, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn gray() -> Self {
        Self::new(120, 120, 120)
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r, g, b) = match s {
            "red" => (255, 0, 0),
            "green" => (0, 255, 0),
            "blue" => (0, 0, 255),
            "black" => (0, 0, 0),
            other => {
                return Err(format!("Color not recognized: {}", other));
            }
        };

        Ok(Self::new(r, g, b))
    }
}
