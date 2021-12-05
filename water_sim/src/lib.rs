pub mod aabb;
mod finite_solver;
pub use aabb::AABBBarrier;
pub use finite_solver::FiniteSolver;
/// size in x direction of water surface
/// Does not depend on mesh resolution
pub const WATER_SIZE: f32 = 6.0;

use nalgebra::Vector2;

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
pub struct WaterMarker;

pub struct InitialConditions {
    pub name: &'static str,
    pub build_water_fn: fn() -> (FiniteSolver, Vec<AABBBarrier>),
}
pub const CONDITIONS: &[InitialConditions] = &[
    InitialConditions {
        name: "Double Slit",
        build_water_fn: || finite_solver::FiniteSolver::barrier(),
    },
    InitialConditions {
        name: "Double Slit Large",
        build_water_fn: || finite_solver::FiniteSolver::barrier_long(),
    },
    InitialConditions {
        name: "Droplet",
        build_water_fn: || finite_solver::FiniteSolver::droplet(),
    },
    InitialConditions {
        name: "Single Source",
        build_water_fn: || finite_solver::FiniteSolver::single_dynamic(),
    },
    InitialConditions {
        name: "Two Sources",
        build_water_fn: || finite_solver::FiniteSolver::dynamic_droplet(),
    },
    InitialConditions {
        name: "Big Droplet (warning slow)",
        build_water_fn: || finite_solver::FiniteSolver::big_droplet(),
    },
    InitialConditions {
        name: "Wall",
        build_water_fn: || finite_solver::FiniteSolver::bridge_poles(),
    },
];
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
