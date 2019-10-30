use cassowary::strength::{REQUIRED, STRONG};
use cassowary::WeightedRelation::{EQ, GE};
use cassowary::{Constraint, Variable};

/// Single widget element, containing
/// a width, a height, and x and y position
/// a padding, margin
pub struct Element {
    left: Variable,
    _top: Variable,
    right: Variable,
    _bottom: Variable,
    width: Variable,
    height: Variable,
    constraints: Vec<Constraint>,
}

impl Element {
    pub fn new() -> Self {
        let left = Variable::new();
        let top = Variable::new();
        let width = Variable::new();
        let height = Variable::new();
        let right = Variable::new();
        let bottom = Variable::new();
        let constraints: Vec<Constraint> = vec![
            width | GE(REQUIRED) | 0.0,
            height | GE(REQUIRED) | 0.0,
            right | EQ(REQUIRED) | (left + width),
            bottom | EQ(REQUIRED) | (top + height),
        ];

        Self {
            left,
            _top: top,
            width,
            height,
            _bottom: bottom,
            right,
            constraints,
        }
    }

    pub fn min_height(&mut self, value: f64) {
        let constraint = self.height | GE(STRONG) | value;
        self.add_constraint(constraint);
    }

    pub fn min_width(&mut self, value: f64) {
        let constraint = self.width | GE(STRONG) | value;
        self.add_constraint(constraint);
    }

    fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Align the left side of this element with another element.
    pub fn align_left(&mut self, other: &Element) {
        let constraint = self.left | EQ(STRONG) | other.left;
        self.add_constraint(constraint);
    }

    pub fn align_right(&mut self, other: &Element) {
        let constraint = self.right | EQ(STRONG) | other.right;
        self.add_constraint(constraint);
    }
}
