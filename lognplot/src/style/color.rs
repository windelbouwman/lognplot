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

    pub fn white() -> Self {
        Self::new(255, 255, 255)
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
        let (r, g, b) = if s.starts_with('#') {
            if s.len() != 7 {
                return Err(format!("Color code {} must have 7 digits", s));
            }
            let r = u8::from_str_radix(&s[1..3], 16).unwrap();
            let g = u8::from_str_radix(&s[3..5], 16).unwrap();
            let b = u8::from_str_radix(&s[5..7], 16).unwrap();
            (r, g, b)
        } else {
            match s {
                "red" => (255, 0, 0),
                "green" => (0, 255, 0),
                "blue" => (0, 0, 255),
                "black" => (0, 0, 0),
                other => {
                    return Err(format!("Color not recognized: {}", other));
                }
            }
        };

        Ok(Self::new(r, g, b))
    }
}
