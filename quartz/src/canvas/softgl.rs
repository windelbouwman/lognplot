use super::transform::{SimpleTransform, Transform};

/// Software GL like drawing API.
struct SoftGl {
    transforms: Vec<SimpleTransform>,
}

impl Default for SoftGl {
    fn default() -> Self {
        Self { transforms: vec![] }
    }
}

impl SoftGl {
    /// Restore identity matrix. No scaling anymore.
    fn identity(&self) {}

    /// Save the current transformation
    fn push(&mut self) {
        self.transforms.push(SimpleTransform::default());
    }

    fn pop(&mut self) {
        self.transforms.pop();
    }
}

impl Transform for SoftGl {
    /// Move cursor some amount
    fn translate(&mut self, dx: f64, dy: f64) {
        self.transforms.last_mut().unwrap().translate(dx, dy);
    }

    /// Scale some amount
    fn scale(&mut self, sx: f64, sy: f64) {
        self.transforms.last_mut().unwrap().scale(sx, sy);
    }

    /// Rotate some amount of radians
    fn rotate(&mut self, angle: f64) {
        self.transforms.last_mut().unwrap().rotate(angle);
    }
}
