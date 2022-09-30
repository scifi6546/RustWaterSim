pub mod aabb;

mod finite_solver;
mod pipe_solver;
mod source;

pub use aabb::AABBBarrier;
use bevy::prelude::*;
pub use grid::{Grid, Vector};
use std::{fs::File, io::Write};

pub use finite_solver::FiniteSolver;
pub use source::Source;

/// size in x direction of water surface
/// Does not depend on mesh resolution
pub const WATER_SIZE: f32 = 6.0;

use nalgebra::Vector2;

#[derive(Component, Clone, Debug)]
pub struct SolveInfo {
    pub name: &'static str,
    pub data: String,
}
#[derive(Clone, Copy, Debug)]
pub enum BoundaryConditions {
    Reflect,
    Absorb,
    Ocean { level: f32 },
}
#[derive(Clone, Copy, Debug)]
pub struct SolverBoundaryConditions {
    pub x_plus: BoundaryConditions,
    pub x_minus: BoundaryConditions,
    pub y_plus: BoundaryConditions,
    pub y_minus: BoundaryConditions,
}
impl Default for SolverBoundaryConditions {
    fn default() -> Self {
        Self {
            x_plus: BoundaryConditions::Reflect,
            x_minus: BoundaryConditions::Reflect,
            y_plus: BoundaryConditions::Reflect,
            y_minus: BoundaryConditions::Reflect,
        }
    }
}

pub type PreferredSolver = pipe_solver::PipeSolver;
pub trait Solver {
    fn new(
        water: Grid<f32>,
        ground: Grid<f32>,
        sources: Vec<Source>,
        boundary_conditions: SolverBoundaryConditions,
    ) -> Self;
    fn solve(&mut self, boxes: &[AABBBarrier]) -> (&Grid<f32>, Vec<SolveInfo>);
    fn water_h(&self) -> &Grid<f32>;
    fn ground_h(&self) -> &Grid<f32>;
    fn dim_x(&self) -> usize;
    fn dim_y(&self) -> usize;
    fn get_ground_mut(&mut self, x: usize, y: usize) -> &mut f32;
    fn offset_water(&self) -> Grid<f32> {
        self.water_h().clone() + self.ground_h().clone()
    }
    fn numpy_data(&self) -> Vec<u8> {
        self.water_h().numpy_data()
    }
    fn mean_height(&self) -> f32 {
        let mut sum = 0.0;
        let water = self.water_h();
        for x in 0..water.x() {
            for y in 0..water.y() {
                sum += water.get(x, y) / (water.x() * water.y()) as f32
            }
        }
        return sum;
    }
}
pub struct WaterMarker;
pub struct InitialConditions<T: Solver> {
    pub name: &'static str,
    pub build_water_fn: fn() -> (T, Vec<AABBBarrier>),
}
pub fn get_conditions<T: Solver>() -> Vec<InitialConditions<T>> {
    vec![
        InitialConditions {
            name: "Double Slit",
            build_water_fn: || {
                let h = Grid::from_fn(
                    |x, y| {
                        let r = ((x as f32 - 50.0).powi(2) + (y as f32 - 50.0).powi(2)).sqrt();
                        if r <= 10.0 {
                            2.0 * (10.0 - r) / 10.0 + 1.0
                        } else {
                            1.0
                        }
                    },
                    Vector2::new(100, 200),
                );
                let g_h = Grid::from_fn(|_x, _y| 0.0, Vector2::new(100, 200));

                (
                    T::new(h, g_h, Vec::new(), SolverBoundaryConditions::default()),
                    vec![
                        AABBBarrier {
                            top_right: Vector2::new(30, 110),
                            bottom_left: Vector2::new(-10, 109),
                        },
                        AABBBarrier {
                            top_right: Vector2::new(65, 110),
                            bottom_left: Vector2::new(35, 109),
                        },
                        AABBBarrier {
                            top_right: Vector2::new(110, 110),
                            bottom_left: Vector2::new(70, 109),
                        },
                    ],
                )
            },
        },
        InitialConditions {
            name: "Double Slit Large",
            build_water_fn: || {
                let h = Grid::from_fn(
                    |x, y| {
                        let r = ((x as f32 - 50.0).powi(2) + (y as f32 - 50.0).powi(2)).sqrt();
                        if r <= 10.0 {
                            2.0 * (10.0 - r) / 10.0 + 1.0
                        } else {
                            1.0
                        }
                    },
                    Vector2::new(100, 1000),
                );
                let g_h = Grid::from_fn(|_x, _y| 0.0, Vector2::new(100, 1000));

                (
                    T::new(h, g_h, Vec::new(), SolverBoundaryConditions::default()),
                    vec![
                        AABBBarrier {
                            top_right: Vector2::new(30, 110),
                            bottom_left: Vector2::new(-10, 109),
                        },
                        AABBBarrier {
                            top_right: Vector2::new(65, 110),
                            bottom_left: Vector2::new(35, 109),
                        },
                        AABBBarrier {
                            top_right: Vector2::new(110, 110),
                            bottom_left: Vector2::new(70, 109),
                        },
                    ],
                )
            },
        },
        InitialConditions {
            name: "Droplet",
            build_water_fn: || {
                let h = Grid::from_fn(
                    |x, y| {
                        let r = ((x as f32 - 50.0).powi(2) + (y as f32 - 50.0).powi(2)).sqrt();
                        if r <= 10.0 {
                            (10.0 - r) / 10.0 + 1.0
                        } else {
                            1.0
                        }
                    },
                    Vector2::new(100, 100),
                );
                let g_h = Grid::from_fn(|_, _| 0.0, Vector2::new(100, 100));

                (
                    T::new(h, g_h, Vec::new(), SolverBoundaryConditions::default()),
                    vec![],
                )
            },
        },
        InitialConditions {
            name: "Droplet Dry",
            build_water_fn: || {
                let h = Grid::from_fn(
                    |x, y| {
                        let r = ((x as f32 - 50.0).powi(2) + (y as f32 - 50.0).powi(2)).sqrt();
                        if r <= 10.0 {
                            (10.0 - r) / 1.0
                        } else {
                            0.0
                        }
                    },
                    Vector2::new(100, 100),
                );
                let g_h = Grid::from_fn(|_, _| 0.0, Vector2::new(100, 100));

                (
                    T::new(h, g_h, Vec::new(), SolverBoundaryConditions::default()),
                    vec![],
                )
            },
        },
        InitialConditions {
            name: "Island Tsunami",
            build_water_fn: || {
                let dimensions = Vector2::new(400, 400);
                let center_x = 200.0f32;
                let center_y = 200.0f32;
                let bay_center_x = 140.0f32;
                let bay_center_y = 200.0f32;

                let water_level = 10.0;
                let ground_fn = |x: usize, y: usize| {
                    let r = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
                    let island = (-0.3 * r + 40.0).max(0.0);
                    let bay_r = ((x as f32 - bay_center_x).powi(2)
                        + (y as f32 - bay_center_y).powi(2))
                    .sqrt();
                    if bay_r < 40.0 {
                        0.9 * water_level
                    } else {
                        island
                    }
                };

                let g_h = Grid::from_fn(|x, y| ground_fn(x, y), dimensions);
                let h = Grid::from_fn(
                    |x, y| {
                        (water_level - ground_fn(x, y)).max(0.0) + if x < 10 { 100.0 } else { 0.0 }
                    },
                    dimensions,
                );

                (
                    T::new(h, g_h, Vec::new(), SolverBoundaryConditions::default()),
                    vec![],
                )
            },
        },
        InitialConditions {
            name: "Tsunami",
            build_water_fn: || {
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

                (
                    T::new(h, g_h, Vec::new(), SolverBoundaryConditions::default()),
                    vec![],
                )
            },
        },
        InitialConditions {
            name: "Lake",
            build_water_fn: || {
                let g_h = Grid::from_fn(
                    |x, y| {
                        let r = ((x as f32 - 200.0).powi(2) + (y as f32 - 200.0).powi(2)).sqrt();
                        r / 100.0
                    },
                    Vector2::new(400, 400),
                );
                let h = Grid::from_fn(|_, _| 2.0, Vector2::new(400, 400));

                (
                    T::new(h, g_h, Vec::new(), SolverBoundaryConditions::default()),
                    vec![],
                )
            },
        },
        InitialConditions {
            name: "Formed Lake",
            build_water_fn: || {
                let water_height = 0.5;
                let g_h = Grid::from_fn(
                    |x, y| {
                        let r = ((x as f32 - 50.0).powi(2) + (y as f32 - 50.0).powi(2)).sqrt();
                        r / 20.0
                    },
                    Vector2::new(100, 100),
                );
                let h = Grid::from_fn(
                    |x, y| {
                        let h = water_height - g_h.get(x, y);
                        if h > 0.0 {
                            h
                        } else {
                            0.0
                        }
                    },
                    Vector2::new(100, 100),
                );

                (
                    T::new(h, g_h, Vec::new(), SolverBoundaryConditions::default()),
                    vec![],
                )
            },
        },
        InitialConditions {
            name: "Single Source",
            build_water_fn: || {
                let h = Grid::from_fn(|_, _| 2.0, Vector2::new(200, 200));
                let g_h = Grid::from_fn(|_x, _y| 0.0, Vector2::new(200, 200));

                let sources = vec![Source {
                    center: Vector2::new(100.0, 100.0),
                    height: 2.2,
                    radius: 10.0,
                    period: 1000.0,
                }];
                (
                    T::new(h, g_h, sources, SolverBoundaryConditions::default()),
                    vec![],
                )
            },
        },
        InitialConditions {
            name: "Two Sources",
            build_water_fn: || {
                let h = Grid::from_fn(|_, _| 2.0, Vector2::new(300, 300));
                let g_h = Grid::from_fn(|_x, _y| 0.0, Vector2::new(300, 300));

                let sources = vec![
                    Source {
                        center: Vector2::new(160.0, 150.0),
                        height: 2.2,
                        radius: 5.0,
                        period: 400.0,
                    },
                    Source {
                        center: Vector2::new(140.0, 150.0),
                        height: 2.2,
                        radius: 5.0,
                        period: 400.0,
                    },
                ];
                (
                    T::new(h, g_h, sources, SolverBoundaryConditions::default()),
                    vec![],
                )
            },
        },
        InitialConditions {
            name: "Big Droplet (warning slow)",
            build_water_fn: || {
                let h = Grid::from_fn(
                    |x, y| {
                        let floor = 5.0;
                        let droplet_size = 50.0;
                        let height = 10.0;
                        let drop_x = 125.0;
                        let drop_y = 125.0;
                        let r = ((x as f32 - drop_x).powi(2) + (y as f32 - drop_y).powi(2)).sqrt();
                        if r <= droplet_size {
                            height * (droplet_size - r) / droplet_size + floor
                        } else {
                            floor
                        }
                    },
                    Vector2::new(250, 250),
                );
                let g_h = Grid::from_fn(|_x, _y| 0.0, Vector2::new(250, 250));

                (
                    T::new(h, g_h, Vec::new(), SolverBoundaryConditions::default()),
                    vec![],
                )
            },
        },
        InitialConditions {
            name: "Wall",
            build_water_fn: || {
                let g_h = Grid::from_fn(|_x, _y| 0.0, Vector2::new(100, 300));
                let h = Grid::from_fn(
                    |_x, y| {
                        let top_height = 1.5;
                        let base_height = 1.0;
                        let top_cutoff = 10;
                        let slope_cutoff = 40;
                        let slope =
                            (top_height - base_height) / (top_cutoff as f32 - slope_cutoff as f32);
                        if y < top_cutoff {
                            top_height
                        } else if y < slope_cutoff as usize {
                            slope * (y as f32 - top_cutoff as f32) + top_height
                        } else {
                            base_height
                        }
                    },
                    Vector2::new(100, 300),
                );

                (
                    T::new(h, g_h, Vec::new(), SolverBoundaryConditions::default()),
                    vec![
                        AABBBarrier {
                            top_right: Vector2::new(20, 50),
                            bottom_left: Vector2::new(15, 45),
                        },
                        AABBBarrier {
                            top_right: Vector2::new(45, 50),
                            bottom_left: Vector2::new(40, 45),
                        },
                        AABBBarrier {
                            top_right: Vector2::new(70, 50),
                            bottom_left: Vector2::new(65, 45),
                        },
                        AABBBarrier {
                            top_right: Vector2::new(95, 50),
                            bottom_left: Vector2::new(90, 45),
                        },
                    ],
                )
            },
        },
    ]
}
