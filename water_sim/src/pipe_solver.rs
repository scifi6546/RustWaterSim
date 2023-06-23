use super::{
    AABBBarrier, BoundaryConditions, Grid, SolveInfo, Solver, SolverBoundaryConditions, Source,
    Vector,
};
use bevy::prelude::Component;
use grid::DebugBuffer;
use nalgebra::Vector2;
use std::ops::Neg;
/// bounds x by negative and positive version of value
/// for example, max_min(1.0,3.0) -> 1.0
/// max_min(4.0,3.0)->3.0
/// max_min(-5.0,3.0)->-3.0
fn max_min(x: f32, bounds: f32) -> f32 {
    x.min(bounds.abs()).max(bounds.abs().neg())
}
/// used https://github.com/bshishov/UnityTerrainErosionGPU as reference
#[derive(Clone, Copy)]
#[repr(C)]
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

impl Vector for Pipes {
    const DIM: usize = 1;

    fn to_le_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn from_le_bytes(bytes: &[u8]) -> Self {
        todo!()
    }
}

#[derive(Component)]
pub struct PipeSolver {
    water: Grid<f32>,
    water_debug_buffer: DebugBuffer<f32>,
    velocity: Grid<Pipes>,
    velocity_debug_buffer: DebugBuffer<Vector2<f32>>,
    ground: Grid<f32>,
    ground_debug_buffer: DebugBuffer<f32>,
    dissolved_ground: Grid<f32>,
    dissolved_ground_debug_buffer: DebugBuffer<f32>,
    sources: Vec<Source>,
    boundary_conditions: SolverBoundaryConditions,
    t: u32,
}

impl Solver for PipeSolver {
    fn new(
        water: Grid<f32>,
        ground: Grid<f32>,
        sources: Vec<Source>,
        boundary_conditions: SolverBoundaryConditions,
    ) -> Self {
        let dimensions = Vector2::new(water.x(), water.y());
        Self {
            water,
            water_debug_buffer: DebugBuffer::new(Self::DEBUG_INTERVAL as u32),
            velocity: Grid::from_fn(|_, _| Pipes::default(), dimensions),
            velocity_debug_buffer: DebugBuffer::new(Self::DEBUG_INTERVAL as u32),
            ground,
            ground_debug_buffer: DebugBuffer::new(Self::DEBUG_INTERVAL as u32),
            dissolved_ground: Grid::from_fn(|_, _| 0.0, dimensions),
            dissolved_ground_debug_buffer: DebugBuffer::new(Self::DEBUG_INTERVAL as u32),
            sources,
            boundary_conditions,
            t: 0,
        }
    }

    fn solve(&mut self, _boxes: &[AABBBarrier]) -> (&Grid<f32>, Vec<SolveInfo>) {
        self.solve_pipe();
        self.solve_erode();

        // self.debug_save();
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
    const DELTA_T: f32 = 0.1;
    const G: f32 = 9.81;
    const DEBUG_INTERVAL: u32 = 10;
    fn get_g_h(&self, x: usize, y: usize) -> f32 {
        self.ground.get(x, y)
    }
    fn get_velocity(pipe: &Pipes) -> Vector2<f32> {
        let x = pipe.l - pipe.r;
        let y = pipe.d - pipe.u;
        Vector2::new(x, y)
    }
    fn debug_save(&mut self) {
        self.water_debug_buffer.push(self.water.clone());
        self.velocity_debug_buffer.push(Grid::from_fn(
            |x, y| Self::get_velocity(&self.velocity.get(x, y)),
            Vector2::new(self.dim_x(), self.dim_y()),
        ));
        self.ground_debug_buffer.push(self.ground.clone());
        self.dissolved_ground_debug_buffer
            .push(self.dissolved_ground.clone());

        if self.t % Self::DEBUG_INTERVAL == 0 {
            let save_dir = std::path::PathBuf::from("./debug_data");
            std::fs::create_dir_all(&save_dir).expect("failed to create dir");
            println!("saving ground");
            let ground_name = format!("ground_{}.npz", self.t);
            self.ground_debug_buffer
                .save(save_dir.as_path().join(&ground_name))
                .expect("failed to save");
            let dimensions = Vector2::new(self.dim_x(), self.dim_y());
            let slope = Grid::from_fn(|x, y| Self::get_slope(&self.ground, x, y), dimensions);

            let velocity_name = format!("velocity_{}.npz", self.t);
            self.velocity_debug_buffer
                .save(save_dir.join(&velocity_name))
                .expect("failed to save");

            //let slope_name = format!("slope_{}.np", self.t);
            //slope.debug_save(save_dir.join(&slope_name));

            let water_name = format!("water_{}.npz", self.t);
            self.water_debug_buffer
                .save(save_dir.join(water_name))
                .expect("failed to save");

            let dissolve_name = format!("dissolved_{}.npz", self.t);

            self.dissolved_ground_debug_buffer
                .save(save_dir.join(dissolve_name))
                .expect("failed to save");
        }
    }
    fn get_slope(g: &Grid<f32>, x: usize, y: usize) -> f32 {
        fn abs_max(a: f32, b: f32) -> f32 {
            if a.abs() >= b.abs() {
                a
            } else {
                b
            }
        }
        let height = g.get(x, y);

        abs_max(
            (height - g.get_or(x as i32 - 1, y as i32, height)),
            (height - g.get_or(x as i32 + 1, y as i32, height)),
        );
        [
            (height - g.get_or(x as i32 - 1, y as i32, height)),
            (height - g.get_or(x as i32 + 1, y as i32, height)),
            (height - g.get_or(x as i32, y as i32 - 1, height)),
            (height - g.get_or(x as i32, y as i32 + 1, height)),
        ]
        .iter()
        .fold(0.0, |acc, x| abs_max(acc, *x))
    }
    fn get_w_g_h(&self, x: usize, y: usize) -> f32 {
        self.water.get(x, y) + self.get_g_h(x, y)
    }
    fn solve_erode(&mut self) {
        const GROUND_DELTA_T: f32 = 0.5;
        let softness = 1.0;
        let dim_x = self.water.x();
        let dim_y = self.water.y();

        let mut water_new = self.water.clone();
        let mut new_ground = self.ground.clone();
        let mut new_dissolved = self.dissolved_ground.clone();
        for x in 0..dim_x {
            for y in 0..dim_y {
                let v_out = self.velocity.get(x, y);
                let v_out = [v_out.l, v_out.d, v_out.r, v_out.u];

                let v_in = [
                    if x == 0 {
                        0.0
                    } else {
                        self.velocity.get(x - 1, y).r
                    },
                    if x == dim_x - 1 {
                        0.0
                    } else {
                        self.velocity.get(x + 1, y).r
                    },
                    if y == 0 {
                        0.0
                    } else {
                        self.velocity.get(x, y - 1).u
                    },
                    if y == dim_y - 1 {
                        0.0
                    } else {
                        self.velocity.get(x, y + 1).d
                    },
                ];
                let v = v_out.iter().fold(0.0, |acc, x| acc + x)
                    + v_in.iter().fold(0.0, |acc, x| acc + x);
                // max concentration to take
                let cap = (softness * v).min(0.01) * Self::get_slope(&self.ground, x, y);
                let cap = (v.max(0.0).atan() * self.water.get(x, y)).max(0.0)
                    * Self::get_slope(&self.ground, x, y);
                //let cap = 0.00001;
                let to_take = (cap - self.dissolved_ground.get(x, y))
                    * Self::DELTA_T
                    * GROUND_DELTA_T
                    * softness;

                //let to_take = to_take.max(-self.water.get(x, y));
                //let to_take = to_take.max(-self.dissolved_ground.get(x, y));
                *new_ground.get_mut(x, y) -= to_take;
                *new_dissolved.get_mut(x, y) += to_take;
                *water_new.get_mut(x, y) += to_take;
            }
        }
        self.ground = new_ground;
        self.dissolved_ground = new_dissolved;
        self.water = water_new;
        let max_move = Grid::from_fn(
            |x, y| {
                let velocity = self.velocity.get(x, y);
                let water = self.water.get(x, y);
                let velocity_sum = velocity.r + velocity.l + velocity.u + velocity.d;
                if velocity_sum >= water {
                    Pipes {
                        l: water * velocity.l / velocity_sum,
                        r: water * velocity.r / velocity_sum,
                        u: water * velocity.u / velocity_sum,
                        d: water * velocity.d / velocity_sum,
                    }
                } else {
                    velocity
                }
            },
            Vector2::new(self.velocity.x(), self.velocity.y()),
        );
        let mut new_dissoved_g = self.dissolved_ground.clone();

        let mut water_new = self.water.clone();

        for x in 0..dim_x {
            for y in 0..dim_y {
                let d_xm1y0 = if x == 0 {
                    0.0
                } else {
                    self.dissolved_ground.get(x - 1, y)
                };
                let d_xp1y0 = if x == dim_x - 1 {
                    0.0
                } else {
                    self.dissolved_ground.get(x + 1, y)
                };
                let d_x0ym1 = if y == 0 {
                    0.0
                } else {
                    self.dissolved_ground.get(x, y - 1)
                };

                let d_x0yp1 = if y == dim_y - 1 {
                    0.0
                } else {
                    self.dissolved_ground.get(x, y + 1)
                };

                let d_x0y0 = self.dissolved_ground.get(x, y);
                let v_xm1y0 = if x == 0 {
                    0.0
                } else {
                    max_min(self.velocity.get(x - 1, y).r, max_move.get(x - 1, y).r)
                };
                let v_xp1y0 = if x == dim_x - 1 {
                    0.0
                } else {
                    max_min(self.velocity.get(x + 1, y).l, max_move.get(x + 1, y).l)
                };
                let v_x0ym1 = if y == 0 {
                    0.0
                } else {
                    max_min(self.velocity.get(x, y - 1).u, max_move.get(x, y - 1).u)
                };
                let v_x0yp1 = if y == dim_y - 1 {
                    0.0
                } else {
                    max_min(self.velocity.get(x, y + 1).d, max_move.get(x, y + 1).d)
                };

                let v_x0y0 = self.velocity.get(x, y);

                let ground_out = d_x0y0
                    * Self::DELTA_T
                    * GROUND_DELTA_T
                    * (v_x0y0.l * Self::L_X
                        + v_x0y0.r * Self::L_X
                        + v_x0y0.d * Self::L_Y
                        + v_x0y0.u * Self::L_Y);
                let ground_in = Self::DELTA_T
                    * GROUND_DELTA_T
                    * (v_xm1y0 * d_xm1y0 * Self::L_X
                        + v_xp1y0 * d_xp1y0 * Self::L_X
                        + v_x0ym1 * d_x0ym1 * Self::L_Y
                        + v_x0yp1 * d_x0yp1 * Self::L_Y);
                let delta = ground_in - ground_out;
                *new_dissoved_g.get_mut(x, y) += delta;
                *water_new.get_mut(x, y) += delta;
            }
        }
        self.dissolved_ground = new_dissoved_g;

        self.water = water_new;
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
            f.l = match self.boundary_conditions.x_minus {
                BoundaryConditions::Absorb => f.l,
                BoundaryConditions::Ocean { .. } => f.l,
                BoundaryConditions::Reflect => 0.0,
            };
            *new_v.get_mut(0, y) = match self.boundary_conditions.x_minus {
                BoundaryConditions::Reflect => Self::kernel(
                    f,
                    self.water.get(0, y),
                    self.get_w_g_h(0, y),
                    self.get_w_g_h(0, y),
                    self.get_w_g_h(1, y),
                    self.get_w_g_h(0, y - 1),
                    self.get_w_g_h(0, y + 1),
                ),
                BoundaryConditions::Ocean { level } => Self::kernel(
                    f,
                    self.water.get(0, y),
                    self.get_w_g_h(0, y),
                    self.get_w_g_h(0, y),
                    level,
                    self.get_w_g_h(0, y - 1),
                    self.get_w_g_h(0, y + 1),
                ),
                BoundaryConditions::Absorb => Self::kernel(
                    f,
                    self.water.get(0, y),
                    self.get_w_g_h(0, y),
                    self.get_g_h(0, y),
                    self.get_w_g_h(1, y),
                    self.get_w_g_h(0, y - 1),
                    self.get_w_g_h(0, y + 1),
                ),
            };

            let mut f = self.velocity.get(dim_x - 1, y);
            f.r = match self.boundary_conditions.x_plus {
                BoundaryConditions::Absorb => f.r,
                BoundaryConditions::Ocean { .. } => f.r,
                BoundaryConditions::Reflect => 0.0,
            };
            *new_v.get_mut(dim_x - 1, y) = match self.boundary_conditions.x_plus {
                BoundaryConditions::Absorb => Self::kernel(
                    f,
                    self.water.get(dim_x - 1, y),
                    self.get_w_g_h(dim_x - 1, y),
                    self.get_w_g_h(dim_x - 2, y),
                    self.get_g_h(dim_x - 1, y),
                    self.get_w_g_h(dim_x - 1, y - 1),
                    self.get_w_g_h(dim_x - 1, y + 1),
                ),
                BoundaryConditions::Ocean { level } => Self::kernel(
                    f,
                    self.water.get(dim_x - 1, y),
                    self.get_w_g_h(dim_x - 1, y),
                    self.get_w_g_h(dim_x - 2, y),
                    level,
                    self.get_w_g_h(dim_x - 1, y - 1),
                    self.get_w_g_h(dim_x - 1, y + 1),
                ),
                BoundaryConditions::Reflect => Self::kernel(
                    f,
                    self.water.get(dim_x - 1, y),
                    self.get_w_g_h(dim_x - 1, y),
                    self.get_w_g_h(dim_x - 2, y),
                    self.get_w_g_h(dim_x - 1, y),
                    self.get_w_g_h(dim_x - 1, y - 1),
                    self.get_w_g_h(dim_x - 1, y + 1),
                ),
            };
        }
        for x in 1..dim_x - 2 {
            let mut f = self.velocity.get(x, 0);
            f.d = match self.boundary_conditions.y_minus {
                BoundaryConditions::Absorb => f.d,
                BoundaryConditions::Ocean { .. } => f.d,
                BoundaryConditions::Reflect => 0.0,
            };
            *new_v.get_mut(x, 0) = match self.boundary_conditions.y_minus {
                BoundaryConditions::Absorb => Self::kernel(
                    f,
                    self.water.get(x, 0),
                    self.get_w_g_h(x, 0),
                    self.get_w_g_h(x - 1, 0),
                    self.get_w_g_h(x + 1, 0),
                    self.get_g_h(x, 0),
                    self.get_w_g_h(x, 1),
                ),
                BoundaryConditions::Ocean { level } => Self::kernel(
                    f,
                    self.water.get(x, 0),
                    self.get_w_g_h(x, 0),
                    self.get_w_g_h(x - 1, 0),
                    self.get_w_g_h(x + 1, 0),
                    level,
                    self.get_w_g_h(x, 1),
                ),
                BoundaryConditions::Reflect => Self::kernel(
                    f,
                    self.water.get(x, 0),
                    self.get_w_g_h(x, 0),
                    self.get_w_g_h(x - 1, 0),
                    self.get_w_g_h(x + 1, 0),
                    self.get_w_g_h(x, 0),
                    self.get_w_g_h(x, 1),
                ),
            };

            let mut f = self.velocity.get(x, dim_y - 1);
            f.u = match self.boundary_conditions.y_plus {
                BoundaryConditions::Reflect => 0.0,
                BoundaryConditions::Ocean { .. } => f.u,
                BoundaryConditions::Absorb => f.u,
            };

            *new_v.get_mut(x, dim_y - 1) = match self.boundary_conditions.y_plus {
                BoundaryConditions::Absorb => Self::kernel(
                    f,
                    self.water.get(x, dim_y - 1),
                    self.get_w_g_h(x, dim_y - 1),
                    self.get_w_g_h(x - 1, dim_y - 1),
                    self.get_w_g_h(x + 1, dim_y - 1),
                    self.get_w_g_h(x, dim_y - 2),
                    self.get_g_h(x, dim_y - 1),
                ),
                BoundaryConditions::Ocean { level } => Self::kernel(
                    f,
                    self.water.get(x, dim_y - 1),
                    self.get_w_g_h(x, dim_y - 1),
                    self.get_w_g_h(x - 1, dim_y - 1),
                    self.get_w_g_h(x + 1, dim_y - 1),
                    self.get_w_g_h(x, dim_y - 2),
                    level,
                ),
                BoundaryConditions::Reflect => Self::kernel(
                    f,
                    self.water.get(x, dim_y - 1),
                    self.get_w_g_h(x, dim_y - 1),
                    self.get_w_g_h(x - 1, dim_y - 1),
                    self.get_w_g_h(x + 1, dim_y - 1),
                    self.get_w_g_h(x, dim_y - 2),
                    self.get_w_g_h(x, dim_y - 1),
                ),
            };
        }
        {
            let mut f = self.velocity.get(0, 0);
            f.d = match self.boundary_conditions.y_minus {
                BoundaryConditions::Absorb => f.d,
                BoundaryConditions::Ocean { .. } => f.d,
                BoundaryConditions::Reflect => 0.0,
            };
            f.l = match self.boundary_conditions.x_minus {
                BoundaryConditions::Absorb => f.l,
                BoundaryConditions::Ocean { .. } => f.l,
                BoundaryConditions::Reflect => 0.0,
            };
            let wg_xm1y0 = match self.boundary_conditions.x_minus {
                BoundaryConditions::Absorb => self.get_g_h(0, 0),
                BoundaryConditions::Ocean { level } => level,
                BoundaryConditions::Reflect => self.get_w_g_h(0, 0),
            };
            let wg_x0ym1 = match self.boundary_conditions.y_minus {
                BoundaryConditions::Absorb => self.get_g_h(0, 0),
                BoundaryConditions::Ocean { level } => level,
                BoundaryConditions::Reflect => self.get_w_g_h(0, 0),
            };
            *new_v.get_mut(0, 0) = Self::kernel(
                f,
                self.water.get(0, 0),
                self.get_w_g_h(0, 0),
                wg_xm1y0,
                self.get_w_g_h(1, 0),
                wg_x0ym1,
                self.get_w_g_h(0, 1),
            );
        }
        {
            let mut f = self.velocity.get(dim_x - 1, 0);
            f.d = match self.boundary_conditions.y_minus {
                BoundaryConditions::Absorb => f.d,
                BoundaryConditions::Ocean { .. } => f.d,
                BoundaryConditions::Reflect => 0.0,
            };
            f.r = match self.boundary_conditions.x_plus {
                BoundaryConditions::Absorb => f.r,
                BoundaryConditions::Ocean { .. } => f.r,
                BoundaryConditions::Reflect => 0.0,
            };
            let wg_xp1y0 = match self.boundary_conditions.x_plus {
                BoundaryConditions::Absorb => self.get_g_h(dim_x - 1, 0),
                BoundaryConditions::Ocean { level } => level,
                BoundaryConditions::Reflect => self.get_w_g_h(dim_x - 1, 0),
            };
            let wg_x0ym1 = match self.boundary_conditions.y_minus {
                BoundaryConditions::Absorb => self.get_g_h(dim_x - 1, 0),
                BoundaryConditions::Ocean { level } => level,
                BoundaryConditions::Reflect => self.get_w_g_h(dim_x - 1, 0),
            };
            *new_v.get_mut(dim_x - 1, 0) = Self::kernel(
                f,
                self.water.get(dim_x - 1, 0),
                self.get_w_g_h(dim_x - 1, 0),
                self.get_w_g_h(dim_x - 2, 0),
                wg_xp1y0,
                wg_x0ym1,
                self.get_w_g_h(dim_x - 1, 1),
            );
        }
        {
            let mut f = self.velocity.get(dim_x - 1, dim_y - 1);
            f.u = match self.boundary_conditions.y_plus {
                BoundaryConditions::Absorb => f.u,
                BoundaryConditions::Ocean { .. } => f.u,
                BoundaryConditions::Reflect => 0.0,
            };
            f.r = match self.boundary_conditions.x_plus {
                BoundaryConditions::Absorb => f.r,
                BoundaryConditions::Ocean { .. } => f.r,
                BoundaryConditions::Reflect => 0.0,
            };
            let wg_xp1y0 = match self.boundary_conditions.x_plus {
                BoundaryConditions::Absorb => self.get_g_h(dim_x - 1, dim_y - 1),
                BoundaryConditions::Ocean { level } => level,
                BoundaryConditions::Reflect => self.get_w_g_h(dim_x - 2, dim_y - 1),
            };
            let wg_x0yp1 = match self.boundary_conditions.y_plus {
                BoundaryConditions::Absorb => self.get_g_h(dim_x - 1, dim_y - 1),
                BoundaryConditions::Ocean { level } => level,
                BoundaryConditions::Reflect => self.get_w_g_h(dim_x - 1, dim_y - 1),
            };
            *new_v.get_mut(dim_x - 1, dim_y - 1) = Self::kernel(
                f,
                self.water.get(dim_x - 1, dim_y - 1),
                self.get_w_g_h(dim_x - 1, dim_y - 1),
                self.get_w_g_h(dim_x - 2, dim_y - 1),
                wg_xp1y0,
                self.get_w_g_h(dim_x - 1, dim_y - 2),
                wg_x0yp1,
            );
        }
        {
            let mut f = self.velocity.get(0, dim_y - 1);
            f.u = match self.boundary_conditions.y_plus {
                BoundaryConditions::Absorb => f.u,
                BoundaryConditions::Ocean { .. } => f.u,
                BoundaryConditions::Reflect => 0.0,
            };
            f.l = match self.boundary_conditions.x_minus {
                BoundaryConditions::Absorb => f.l,
                BoundaryConditions::Ocean { .. } => f.l,
                BoundaryConditions::Reflect => 0.0,
            };
            let wg_xm1y0 = match self.boundary_conditions.x_minus {
                BoundaryConditions::Absorb => self.get_g_h(0, dim_y - 1),
                BoundaryConditions::Ocean { level } => level,
                BoundaryConditions::Reflect => self.get_w_g_h(0, dim_y - 1),
            };
            let wg_x0yp1 = match self.boundary_conditions.y_plus {
                BoundaryConditions::Absorb => self.get_g_h(0, dim_y - 1),
                BoundaryConditions::Ocean { level } => level,
                BoundaryConditions::Reflect => self.get_w_g_h(0, dim_y - 1),
            };
            *new_v.get_mut(0, dim_y - 1) = Self::kernel(
                f,
                self.water.get(0, dim_y - 1),
                self.get_w_g_h(0, dim_y - 1),
                wg_xm1y0,
                self.get_w_g_h(1, dim_y - 1),
                self.get_w_g_h(0, dim_y - 2),
                wg_x0yp1,
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
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn min_max() {
        assert!((max_min(1.0, 2.0) - 1.0).abs() < 0.01)
    }
}
