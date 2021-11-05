/// based off of https://github.com/jostbr/shallow-water/blob/master/swe.py
use super::{Grid, Solver};
use nalgebra::Vector2;
pub struct FiniteSolver {
    /// Water height
    water: Grid<f32>,
    ///u velocity
    u: Grid<f32>,
    /// v velocity
    v: Grid<f32>,
}
impl Solver for FiniteSolver {
    fn solve(&mut self) -> &Grid<f32> {
        todo!()
    }
}
