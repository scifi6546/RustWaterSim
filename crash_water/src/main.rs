use nalgebra::Vector2;
use water_sim::{Grid, PreferredSolver, Solver, SolverBoundaryConditions};
fn main() {
    fn ground_fn(x: usize, _y: usize) -> f32 {
        if x < 200 {
            0.0
        } else if x < 300 {
            (x as f32 - 200.0) / 100.0
        } else {
            1.0
        }
    }
    let g_h = Grid::from_fn(|x, y| ground_fn(x, y), Vector2::new(400, 200));
    let h = Grid::from_fn(
        |x, y| {
            let g = ground_fn(x, y);
            (1.0 - g).max(0.0) + if x <= 100 { 50.0 } else { 0.0 }
        },
        Vector2::new(400, 200),
    );

    let mut solver = PreferredSolver::new(h, g_h, Vec::new(), SolverBoundaryConditions::default());
    for i in 0..100 {
        println!("{}", i);
        solver.solve(&[]);
    }
}
