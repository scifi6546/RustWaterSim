pub mod aabb;

mod finite_solver;
mod pipe_solver;
mod source;

pub use aabb::AABBBarrier;
use bevy::prelude::*;
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

#[derive(Clone)]
pub struct Grid<T: Clone + Copy> {
    points: Vec<T>,
    x: usize,
    y: usize,
}
impl<T: Clone + Copy + Default> Grid<T> {
    pub fn from_vec(dimensions: Vector2<usize>, points: Vec<T>) -> Self {
        assert_eq!(dimensions.x * dimensions.y, points.len());
        Self {
            points,
            x: dimensions.x,
            y: dimensions.y,
        }
    }
    /// X dimensions
    pub fn x(&self) -> usize {
        self.x
    }
    /// Y dimensions
    pub fn y(&self) -> usize {
        self.y
    }
    ///
    pub fn get(&self, x: usize, y: usize) -> T {
        self.points[self.y * x + y]
    }
    /// gets mut unchecked
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.points[self.y * x + y]
    }
    /// gets points unchecked at point
    pub fn get_unchecked(&self, dim: Vector2<i64>) -> T {
        self.get(dim.x as usize, dim.y as usize)
    }
    /// gets unchecked mut
    pub fn get_mut_unchecked(&mut self, dim: Vector2<i64>) -> &mut T {
        self.get_mut(dim.x as usize, dim.y as usize)
    }
    /// builds grid from function
    pub fn from_fn<F: Fn(usize, usize) -> T>(f: F, dimensions: Vector2<usize>) -> Self {
        let mut s = Self::from_vec(dimensions, vec![T::default(); dimensions.x * dimensions.y]);
        for x in 0..dimensions.x {
            for y in 0..dimensions.y {
                *s.get_mut(x, y) = f(x, y);
            }
        }
        s
    }
}
impl<T: std::ops::Add + std::ops::AddAssign + Clone + Copy> std::ops::Add for Grid<T> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        assert_eq!(self.x, other.x);
        assert_eq!(self.y, other.y);
        for i in 0..self.points.len() {
            self.points[i] += other.points[i];
        }
        Self {
            points: self.points,
            x: self.x,
            y: self.y,
        }
    }
}
pub type PreferredSolver = pipe_solver::PipeSolver;
pub trait Solver {
    fn new(water: Grid<f32>, ground: Grid<f32>, sources: Vec<Source>) -> Self;
    fn solve(&mut self, boxes: &[AABBBarrier]) -> (&Grid<f32>, Vec<SolveInfo>);
    fn water_h(&self) -> &Grid<f32>;
    fn ground_h(&self) -> &Grid<f32>;
    fn offset_water(&self) -> Grid<f32> {
        self.water_h().clone() + self.ground_h().clone()
    }
    fn numpy_data(&self) -> Vec<u8> {
        let water_h = self.water_h();
        let header_str = format!(
            "{{'descr': '<f4', 'fortran_order': False, 'shape': ({}, {}), }}",
            water_h.x(),
            water_h.y()
        );
        let header_bytes = header_str.as_bytes();
        let header_len = header_bytes.len() as u16;
        let mut out_data = vec![
            0x93, 'N' as u8, 'U' as u8, 'M' as u8, 'P' as u8, 'Y' as u8, 0x01, 0x00,
        ];
        for byte in header_len.to_le_bytes().iter() {
            out_data.push(*byte);
        }
        for byte in header_bytes.iter() {
            out_data.push(*byte);
        }
        for x in 0..water_h.x() {
            for y in 0..water_h.y() {
                let h = water_h.get(x, y);
                for byte in h.to_ne_bytes().iter() {
                    out_data.push(*byte);
                }
            }
        }

        return out_data;
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
                    T::new(h, g_h, Vec::new()),
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
                    T::new(h, g_h, Vec::new()),
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

                (T::new(h, g_h, Vec::new()), vec![])
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

                (T::new(h, g_h, Vec::new()), vec![])
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

                (T::new(h, g_h, Vec::new()), vec![])
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
                (T::new(h, g_h, sources), vec![])
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
                (T::new(h, g_h, sources), vec![])
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

                (T::new(h, g_h, Vec::new()), vec![])
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
                    T::new(h, g_h, Vec::new()),
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
