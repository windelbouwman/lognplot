struct Grid {}

impl Grid {
    fn create_solver(&self) {
        let mut solver = cassowary::Solver::new();
        // solver

        // Width and height must be larger than 0:
        let var_w = cassowary::Variable::new();
        let var_h = cassowary::Variable::new();

        solver
            .add_constraint(var_w | cassowary::WeightedRelation::GE(1.0) | 0.0)
            .unwrap();
        solver
            .add_constraint(var_h | cassowary::WeightedRelation::GE(1.0) | 0.0)
            .unwrap();

        // Now add rows and columns:
    }
}
