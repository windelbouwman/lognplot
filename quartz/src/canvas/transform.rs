use crate::linalg::Matrix;

/// A 2D transformation.
/// This means, rotation and translation.
pub struct SimpleTransform {
    matrix: Matrix,
}

impl Default for SimpleTransform {
    fn default() -> Self {
        Self::identity()
    }
}

impl SimpleTransform {
    /// Return identity transformation.
    fn identity() -> Self {
        Self {
            matrix: Matrix::identity(3),
        }
    }
}

/// Transformation trait. Supports translation, scaling and rotation.
pub trait Transform {
    fn translate(&mut self, dx: f64, dy: f64);
    fn scale(&mut self, sx: f64, sy: f64);
    fn rotate(&mut self, angle: f64);
}

impl Transform for SimpleTransform {
    fn translate(&mut self, dx: f64, dy: f64) {
        // update matrix:
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        // update matrix:
    }

    fn rotate(&mut self, angle: f64) {
        let ca = angle.cos();
        let sa = angle.sin();
        let mut m = Matrix::identity(3);
        m[(0,0)] = ca;
        m[(0,1)] = sa;
        m[(1,1)] = ca;
        m[(1,0)] = -sa;
        // update matrix:
    }
}
