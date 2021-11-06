/// based off of https://github.com/jostbr/shallow-water/blob/master/swe.py
use super::{Grid, Solver};
use nalgebra::Vector2;
pub struct FiniteSolver {
    /// Water height
    h: Grid<f32>,
    ///u velocity
    u: Grid<f32>,
    /// v velocity
    v: Grid<f32>,
}
impl Solver for FiniteSolver {
    fn solve(&mut self) -> &Grid<f32> {
        for _ in 0..300 {
            Self::update_velocity(&self.h, &mut self.u, &mut self.v);
            Self::update_heights(&mut self.h, &self.u, &self.v);
        }
        &self.h
    }
}
impl FiniteSolver {
    const DX: f32 = 999.0;
    const DY: f32 = 999.0;
    const G: f32 = 9.81;
    const DT: f32 = 0.001;
    const VISC: f32 = 0.001;
    fn update_velocity(heights: &Grid<f32>, u: &mut Grid<f32>, v: &mut Grid<f32>) {
        for x in 0..heights.x() {
            for y in 0..heights.y() {
                if x != 0 && y != 0 {
                    let hxn1 = heights.get(x - 1, y);
                    let hxp1 = heights.get(x, y);

                    *u.get_mut(x, y) += Self::G * (Self::DT / Self::DX) * (hxp1 - hxn1);
                    *u.get_mut(x, y) *= 1.0 - Self::VISC * Self::DT;
                    let hyn1 = heights.get(x, y - 1);
                    let hyp1 = heights.get(x, y);
                    *v.get_mut(x, y) += Self::G * (Self::DT / Self::DY) * (hyp1 - hyn1);
                    *v.get_mut(x, y) *= (1.0 - Self::VISC * Self::DT);
                }
            }
        }
    }
    fn update_heights(h: &mut Grid<f32>, u: &Grid<f32>, v: &Grid<f32>) {
        let h_old = h.clone();
        for x in 0..h.x() {
            for y in 0..h.y() {
                let un1 = u.get(x, y);
                let up1 = u.get(x + 1, y);
                let vn1 = v.get(x, y);
                let vp1 = v.get(x, y + 1);

                let hxn1 = if x >= 1 {
                    h_old.get(x - 1, y)
                } else {
                    h_old.get(x, y)
                };
                let hxp1 = if x <= h.x() - 2 {
                    h_old.get(x + 1, y)
                } else {
                    h_old.get(x, y)
                };

                let hyn1 = if y >= 1 {
                    h_old.get(x, y - 1)
                } else {
                    h_old.get(x, y)
                };
                let hyp1 = if y <= h.y() - 2 {
                    h_old.get(x, y + 1)
                } else {
                    h_old.get(x, y)
                };
                let h0 = h_old.get(x, y);
                let dx = un1 * (hxn1 + h0) / 2.0 - up1 * (hxp1 + h0) / 2.0;
                let dy = vn1 * (hyn1 + h0) / 2.0 - vp1 * (hyp1 + h0) / 2.0;
                // bad scheme lets see how it goes anyways
                *h.get_mut(x, y) -= Self::DT * (dx + dy);
            }
        }
    }
    pub fn new() -> Self {
        let mut water_data = vec![0.0f32; 100 * 100];
        //water_data[50 * 50 + 50] = 0.5;
        let u = vec![0.0f32; 101 * 100];
        let v_data = u.clone();
        let mut h = Grid::from_vec(Vector2::new(100, 100), water_data);
        for x in 0..100 {
            for y in 0..100 {
                let r = ((x as f32 - 50.0).powi(2) + (y as f32 - 50.0).powi(2)).sqrt();
                if r <= 10.0 {
                    *h.get_mut(x, y) = (10.0 - r) / 10.0;
                }
            }
        }
        let u = Grid::from_vec(Vector2::new(101, 100), u);
        let v = Grid::from_vec(Vector2::new(100, 101), v_data);
        Self { h, u, v }
    }
}
