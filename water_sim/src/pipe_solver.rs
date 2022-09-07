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
    dissolved_ground: Grid<f32>,
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
            dissolved_ground: Grid::from_fn(|_, _| 0.0, dimensions),
            sources,
            t: 0,
        }
    }

    fn solve(&mut self, _boxes: &[AABBBarrier]) -> (&Grid<f32>, Vec<SolveInfo>) {
        self.solve_pipe();
        self.solve_erode();
        (&self.water, vec![])
    }

    fn water_h(&self) -> &Grid<f32> {
        &self.water
    }

    fn ground_h(&self) -> &Grid<f32> {
        &self.ground
    }

    fn dim_x(&self) -> usize {
        self.water.x()
    }
    fn dim_y(&self) -> usize {
        self.water.y()
    }
    fn get_ground_mut(&mut self, x: usize, y: usize) -> &mut f32 {
        self.ground.get_mut(x, y)
    }
}

impl PipeSolver {
    const L_X: f32 = 1.0;
    const L_Y: f32 = 1.0;
    const DELTA_T: f32 = 0.01;
    const G: f32 = 9.81;
    fn get_w_g_h(&self, x: usize, y: usize) -> f32 {
        self.water.get(x, y) + self.ground.get(x, y)
    }
    fn solve_erode(&mut self) {
        let softness = 0.01;
        let dim_x = self.water.x();
        let dim_y = self.water.y();
        for x in 0..dim_x + 1 {
            for y in 0..dim_y + 1 {
                let ground_height = self.ground.get_mut(x, y);
                let pipe = self.velocity.get(x, y);
                let v_x = pipe.r - pipe.l;
                let v_y = pipe.u - pipe.d;
                let v = (v_x.powi(2) + v_y.powi(2)).sqrt();
                let cap = softness * v;
            }
        }
    }
    fn kernel(
        f_x0y0: Pipes,

        w_x0y0: f32,

        wg_x0y0: f32,
        wg_xm1y0: f32,
        wg_xp1y0: f32,
        wg_x0ym1: f32,
        wg_x0yp1: f32,
    ) -> Pipes {
        let k = 1.0f32.min(
            w_x0y0 * Self::L_X * Self::L_Y
                / (Self::DELTA_T + (f_x0y0.l + f_x0y0.r + f_x0y0.u + f_x0y0.d)),
        );
        let delta_h_left = wg_x0y0 - wg_xm1y0;

        let f_left_new = k * 0.0f32.max(
            f_x0y0.l + Self::DELTA_T * Self::G * Self::L_X * Self::L_Y * delta_h_left / Self::L_Y,
        );

        let d_h_right = wg_x0y0 - wg_xp1y0;
        let f_right_new = k * 0.0f32.max(
            f_x0y0.r + Self::DELTA_T * Self::G * Self::L_X * Self::L_Y * d_h_right / Self::L_X,
        );

        let d_h_up = wg_x0y0 - wg_x0yp1;
        let f_up_new = k * 0.0f32
            .max(f_x0y0.u + Self::DELTA_T * Self::G * Self::L_X * Self::L_Y * d_h_up / Self::L_Y);
        let d_h_down = wg_x0y0 - wg_x0ym1;
        let f_down_new = k * 0.0f32
            .max(f_x0y0.d + Self::DELTA_T * Self::G * Self::L_X * Self::L_Y * d_h_down / Self::L_Y);
        Pipes {
            l: f_left_new,
            r: f_right_new,
            d: f_down_new,
            u: f_up_new,
        }
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
                let t = Self::kernel(
                    self.velocity.get(x, y),
                    self.water.get(x, y),
                    self.get_w_g_h(x, y),
                    self.get_w_g_h(x - 1, y),
                    self.get_w_g_h(x + 1, y),
                    self.get_w_g_h(x, y - 1),
                    self.get_w_g_h(x, y + 1),
                );
                *new_v.get_mut(x, y) = t;
            }
        }
        for y in 1..dim_y - 1 {
            let mut f = self.velocity.get(0, y);
            f.l = 0.0;
            *new_v.get_mut(0, y) = Self::kernel(
                f,
                self.water.get(0, y),
                self.get_w_g_h(0, y),
                self.get_w_g_h(0, y),
                self.get_w_g_h(1, y),
                self.get_w_g_h(0, y - 1),
                self.get_w_g_h(0, y + 1),
            );
            let mut f = self.velocity.get(dim_x - 1, y);
            f.r = 0.0;
            *new_v.get_mut(dim_x - 1, y) = Self::kernel(
                f,
                self.water.get(dim_x - 1, y),
                self.get_w_g_h(dim_x - 1, y),
                self.get_w_g_h(dim_x - 2, y),
                self.get_w_g_h(dim_x - 1, y),
                self.get_w_g_h(dim_x - 1, y - 1),
                self.get_w_g_h(dim_x - 1, y + 1),
            );
        }
        for x in 1..dim_x - 2 {
            let mut f = self.velocity.get(x, 0);
            f.d = 0.0;
            *new_v.get_mut(x, 0) = Self::kernel(
                f,
                self.water.get(x, 0),
                self.get_w_g_h(x, 0),
                self.get_w_g_h(x - 1, 0),
                self.get_w_g_h(x + 1, 0),
                self.get_w_g_h(x, 0),
                self.get_w_g_h(x, 1),
            );
            let mut f = self.velocity.get(x, dim_y - 1);
            f.u = 0.0;
            *new_v.get_mut(x, dim_y - 1) = Self::kernel(
                f,
                self.water.get(x, dim_y - 1),
                self.get_w_g_h(x, dim_y - 1),
                self.get_w_g_h(x - 1, dim_y - 1),
                self.get_w_g_h(x + 1, dim_y - 1),
                self.get_w_g_h(x, dim_y - 2),
                self.get_w_g_h(x, dim_y - 1),
            );
        }
        {
            let mut f = self.velocity.get(0, 0);
            f.d = 0.0;
            f.l = 0.0;
            *new_v.get_mut(0, 0) = Self::kernel(
                f,
                self.water.get(0, 0),
                self.get_w_g_h(0, 0),
                self.get_w_g_h(0, 0),
                self.get_w_g_h(1, 0),
                self.get_w_g_h(0, 0),
                self.get_w_g_h(0, 1),
            );
        }
        {
            let mut f = self.velocity.get(dim_x - 1, 0);
            f.d = 0.0;
            f.r = 0.0;
            *new_v.get_mut(dim_x - 1, 0) = Self::kernel(
                f,
                self.water.get(dim_x - 1, 0),
                self.get_w_g_h(dim_x - 1, 0),
                self.get_w_g_h(dim_x - 2, 0),
                self.get_w_g_h(dim_x - 1, 0),
                self.get_w_g_h(dim_x - 1, 0),
                self.get_w_g_h(dim_x - 1, 1),
            );
        }
        {
            let mut f = self.velocity.get(dim_x - 1, dim_y - 1);
            f.u = 0.0;
            f.r = 0.0;
            *new_v.get_mut(dim_x - 1, dim_y - 1) = Self::kernel(
                f,
                self.water.get(dim_x - 1, dim_y - 1),
                self.get_w_g_h(dim_x - 1, dim_y - 1),
                self.get_w_g_h(dim_x - 2, dim_y - 1),
                self.get_w_g_h(dim_x - 1, dim_y - 1),
                self.get_w_g_h(dim_x - 1, dim_y - 2),
                self.get_w_g_h(dim_x - 1, dim_y - 1),
            );
        }
        {
            let mut f = self.velocity.get(0, dim_y - 1);
            f.u = 0.0;
            f.l = 0.0;
            *new_v.get_mut(0, dim_y - 1) = Self::kernel(
                f,
                self.water.get(0, dim_y - 1),
                self.get_w_g_h(0, dim_y - 1),
                self.get_w_g_h(0, dim_y - 1),
                self.get_w_g_h(1, dim_y - 1),
                self.get_w_g_h(0, dim_y - 2),
                self.get_w_g_h(0, dim_y - 1),
            );
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
            {
                let f_out = self.velocity.get(x, 0);
                let f_out = f_out.l + f_out.u + f_out.d + f_out.r;
                let f_in = self.velocity.get(x - 1, 0).r
                    + self.velocity.get(x + 1, 0).l
                    + 0.0
                    + self.velocity.get(x, 1).d;
                let volume_change = Self::DELTA_T * (f_in - f_out) / (Self::L_X * Self::L_Y);
                *self.water.get_mut(x, 0) += volume_change
            }
            {
                let f_out = self.velocity.get(x, dim_y - 1);
                let f_out = f_out.l + f_out.u + f_out.d + f_out.r;
                let f_in = self.velocity.get(x - 1, dim_y - 1).r
                    + self.velocity.get(x + 1, dim_y - 1).l
                    + self.velocity.get(x, dim_y - 2).u
                    + 0.0;
                let volume_change = Self::DELTA_T * (f_in - f_out) / (Self::L_X * Self::L_Y);
                *self.water.get_mut(x, dim_y - 1) += volume_change
            }
        }
        for y in 1..dim_y - 1 {
            {
                let f_out = self.velocity.get(0, y);
                let f_out = f_out.l + f_out.u + f_out.d + f_out.r;
                let f_in = 0.0
                    + self.velocity.get(1, y).l
                    + self.velocity.get(0, y - 1).u
                    + self.velocity.get(0, y + 1).d;
                let volume_change = Self::DELTA_T * (f_in - f_out) / (Self::L_X * Self::L_Y);
                *self.water.get_mut(0, y) += volume_change;
            }
            {
                let f_out = self.velocity.get(dim_x - 1, y);
                let f_out = f_out.l + f_out.u + f_out.d + f_out.r;
                let f_in = self.velocity.get(dim_x - 2, y).r
                    + 0.0
                    + self.velocity.get(dim_x - 1, y - 1).u
                    + self.velocity.get(dim_x - 1, y + 1).d;
                let volume_change = Self::DELTA_T * (f_in - f_out) / (Self::L_X * Self::L_Y);
                *self.water.get_mut(dim_x - 1, y) += volume_change
            }
        }
        {
            let f_out = self.velocity.get(0, 0);
            let f_out = f_out.l + f_out.u + f_out.d + f_out.r;
            let f_in = 0.0 + self.velocity.get(1, 0).l + 0.0 + self.velocity.get(0, 1).d;
            let volume_change = Self::DELTA_T * (f_in - f_out) / (Self::L_X * Self::L_Y);
            *self.water.get_mut(0, 0) += volume_change
        }
        {
            let f_out = self.velocity.get(dim_x - 1, 0);
            let f_out = f_out.l + f_out.u + f_out.d + f_out.r;
            let f_in =
                self.velocity.get(dim_x - 2, 0).r + 0.0 + 0.0 + self.velocity.get(dim_x - 1, 1).d;
            let volume_change = Self::DELTA_T * (f_in - f_out) / (Self::L_X * Self::L_Y);
            *self.water.get_mut(dim_x - 1, 0) += volume_change
        }
        {
            let f_out = self.velocity.get(dim_x - 1, dim_y - 1);
            let f_out = f_out.l + f_out.u + f_out.d + f_out.r;
            let f_in = self.velocity.get(dim_x - 2, dim_y - 1).r
                + 0.0
                + self.velocity.get(dim_x - 1, dim_y - 2).u
                + 0.0;
            let volume_change = Self::DELTA_T * (f_in - f_out) / (Self::L_X * Self::L_Y);
            *self.water.get_mut(dim_x - 1, dim_y - 1) += volume_change
        }
        {
            let f_out = self.velocity.get(0, dim_y - 1);
            let f_out = f_out.l + f_out.u + f_out.d + f_out.r;
            let f_in =
                0.0 + self.velocity.get(1, dim_y - 1).l + self.velocity.get(0, dim_y - 2).u + 0.0;
            let volume_change = Self::DELTA_T * (f_in - f_out) / (Self::L_X * Self::L_Y);
            *self.water.get_mut(0, dim_y - 1) += volume_change
        }

        self.t += 1;
    }
}
