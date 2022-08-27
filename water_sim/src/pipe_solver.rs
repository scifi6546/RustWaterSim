use super::{AABBBarrier, Grid, SolveInfo, Solver, Source};
use bevy::prelude::Component;
use nalgebra::Vector2;
/// used https://github.com/bshishov/UnityTerrainErosionGPU as reference
#[derive(Clone, Copy)]
struct Pipes {
    l: f32,
    r: f32,
    u: f32,
    d: f32,
}
impl Default for Pipes {
    fn default() -> Self {
        Pipes {
            l: 0.0,
            r: 0.0,
            u: 0.0,
            d: 0.0,
        }
    }
}
#[derive(Component)]
pub struct PipeSolver {
    water: Grid<f32>,
    velocity: Grid<Pipes>,
    ground: Grid<f32>,
    sources: Vec<Source>,
    t: u32,
}

impl Solver for PipeSolver {
    fn new(water: Grid<f32>, ground: Grid<f32>, sources: Vec<Source>) -> Self {
        let dimensions = Vector2::new(water.x(), water.y());
        Self {
            water,
            velocity: Grid::from_fn(|_, _| Pipes::default(), dimensions),
            ground,
            sources,
            t: 0,
        }
    }

    fn solve(&mut self, boxes: &[AABBBarrier]) -> (&Grid<f32>, Vec<SolveInfo>) {
        self.solve_pipe();
        (&self.water, vec![])
    }

    fn water_h(&self) -> &Grid<f32> {
        &self.water
    }

    fn ground_h(&self) -> &Grid<f32> {
        &self.ground
    }
}

impl PipeSolver {
    const L_X: f32 = 1.0;
    const L_Y: f32 = 1.0;
    const DELTA_T: f32 = 0.001;
    const G: f32 = 9.81;
    fn get_w_g_h(&self, x: usize, y: usize) -> f32 {
        self.water.get(x, y) + self.ground.get(x, y)
    }
    fn solve_pipe(&mut self) {
        for source in self.sources.iter() {
            source.change_h(&mut self.water, self.t);
        }
        let mut new_v = self.velocity.clone();
        let dim_x = self.water.x();
        let dim_y = self.water.y();
        for x in 1..dim_x - 1 {
            for y in 1..dim_y - 1 {
                let f = self.velocity.get(x, y);
                let k = 1.0f32.min(
                    self.water.get(x, y) * Self::L_X * Self::L_Y
                        / (Self::DELTA_T + (f.l + f.r + f.u + f.d)),
                );
                let delta_h_left = self.get_w_g_h(x, y) - self.get_w_g_h(x - 1, y);

                let f_left_new = k * 0.0f32.max(
                    self.velocity.get(x, y).l
                        + Self::DELTA_T * Self::G * Self::L_X * Self::L_Y * delta_h_left
                            / Self::L_Y,
                );

                let d_h_right = self.get_w_g_h(x, y) - self.get_w_g_h(x + 1, y);
                let f_right_new = k * 0.0f32.max(
                    self.velocity.get(x, y).r
                        + Self::DELTA_T * Self::G * Self::L_X * Self::L_Y * d_h_right / Self::L_X,
                );

                let d_h_up = self.get_w_g_h(x, y) - self.get_w_g_h(x, y + 1);
                let f_up_new = k * 0.0f32.max(
                    self.velocity.get(x, y).u
                        + Self::DELTA_T * Self::G * Self::L_X * Self::L_Y * d_h_up / Self::L_Y,
                );
                let d_h_down = self.get_w_g_h(x, y) - self.get_w_g_h(x, y - 1);
                let f_down_new = k * 0.0f32.max(
                    self.velocity.get(x, y).d
                        + Self::DELTA_T * Self::G * Self::L_X * Self::L_Y * d_h_down / Self::L_Y,
                );
                *new_v.get_mut(x, y) = Pipes {
                    l: f_left_new,
                    r: f_right_new,
                    d: f_down_new,
                    u: f_up_new,
                };
            }
        }
        self.velocity = new_v;

        for x in 1..dim_x - 1 {
            for y in 1..dim_y - 1 {
                let f_out = self.velocity.get(x, y);
                let f_out = f_out.l + f_out.u + f_out.d + f_out.r;
                let f_in = self.velocity.get(x - 1, y).r
                    + self.velocity.get(x + 1, y).l
                    + self.velocity.get(x, y - 1).u
                    + self.velocity.get(x, y + 1).d;
                let volume_change = Self::DELTA_T * (f_in - f_out) / (Self::L_X * Self::L_Y);
                *self.water.get_mut(x, y) += volume_change
            }
        }
        self.t += 1;
    }
}
