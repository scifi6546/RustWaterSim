use super::{Grid, SolveInfo, Solver};

use nalgebra::Vector2;
pub struct MySolver {
    /// height of water
    heights: Grid<f32>,
    /// water velocity
    velocity: Grid<Vector2<f32>>,
    dimensions: Vector2<usize>,
    /// viscosity
    viscosity: f32,
}
impl Solver for MySolver {
    fn solve(&mut self) -> (&Grid<f32>, Vec<SolveInfo>) {
        self.water_simulation();
        (&self.heights, vec![])
    }
}
impl MySolver {
    /// Time step
    const DELTA_T: f32 = 0.001;
    /// Gravity constant
    const G: f32 = 0.1;
    /// Viscosity

    pub fn new(water: Grid<f32>, velocities: Grid<Vector2<f32>>) -> Self {
        assert!(velocities.x() == water.x() + 1);
        assert!(velocities.y() == water.y() + 1);
        let dimensions = Vector2::new(water.x(), water.y());
        Self {
            heights: water,
            velocity: velocities,
            dimensions,
            viscosity: 0.000004,
        }
    }
    fn update_velocity(
        heights: &Grid<f32>,
        _velocity: &Grid<Vector2<f32>>,
        velocity_apply: &Grid<Vector2<f32>>,
        dimensions: &Vector2<usize>,
        delta_t: f32,
        viscosity: f32,
    ) -> Grid<Vector2<f32>> {
        let mut new_velocities = velocity_apply.clone();
        //Update Velocities
        for x in 0..dimensions.x {
            for y in 0..dimensions.y {
                let water_x_n1 = if x > 0 {
                    heights.get_unchecked(Vector2::new(x as i64 - 1, y as i64))
                } else {
                    heights.get_unchecked(Vector2::new(x as i64, y as i64))
                };
                let water_y_n1 = if y > 0 {
                    heights.get_unchecked(Vector2::new(x as i64, y as i64 - 1))
                } else {
                    heights.get_unchecked(Vector2::new(x as i64, y as i64))
                };

                let v = new_velocities.get_mut_unchecked(Vector2::new(x as i64, y as i64));
                let center = heights.get_unchecked(Vector2::new(x as i64, y as i64));
                v.x += (water_x_n1 - center) * delta_t * Self::G;
                if x == 0 {
                    v.x = 0.0;
                }
                v.y += (water_y_n1 - center) * delta_t * Self::G;

                *v -= *v * viscosity;
                if y == 0 {
                    v.y = 0.0;
                }
            }
        }
        return new_velocities;
    }
    fn update_water(
        heights: &Grid<f32>,
        velocity: &Grid<Vector2<f32>>,
        heights_apply: &mut Grid<f32>,
        dimensions: &Vector2<usize>,
        delta_t: f32,
    ) {
        for x in 0..dimensions.x {
            for y in 0..dimensions.y {
                let water_yn1 = if y > 0 {
                    heights.get_unchecked(Vector2::new(x as i64, y as i64 - 1))
                } else {
                    heights.get_unchecked(Vector2::new(x as i64, y as i64))
                };
                let (water_0, v_y0, u_x0) = (
                    heights.get_unchecked(Vector2::new(x as i64, y as i64)),
                    velocity.get_unchecked(Vector2::new(x as i64, y as i64)).y,
                    velocity.get_unchecked(Vector2::new(x as i64, y as i64)).x,
                );
                let (water_y1, v_y1) = if y <= dimensions.y - 2 {
                    (
                        heights.get_unchecked(Vector2::new(x as i64, y as i64 + 1)),
                        velocity
                            .get_unchecked(Vector2::new(x as i64, y as i64 + 1))
                            .y,
                    )
                } else {
                    (heights.get_unchecked(Vector2::new(x as i64, y as i64)), 0.0)
                };
                let water_xn1 = if x > 0 {
                    heights.get_unchecked(Vector2::new(x as i64 - 1, y as i64))
                } else {
                    heights.get_unchecked(Vector2::new(x as i64, y as i64))
                };
                let (water_x1, u_x1) = if x <= dimensions.x - 2 {
                    (
                        heights.get_unchecked(Vector2::new(x as i64 + 1, y as i64)),
                        velocity
                            .get_unchecked(Vector2::new(x as i64 + 1, y as i64))
                            .x,
                    )
                } else {
                    (heights.get_unchecked(Vector2::new(x as i64, y as i64)), 0.0)
                };
                let water_xn1_avg = (water_xn1 + water_0) / 2.0;
                let water_x1_avg = (water_x1 + water_0) / 2.0;

                let water_yn1_avg = (water_yn1 + water_0) / 2.0;
                let water_y1_avg = (water_y1 + water_0) / 2.0;
                let deltax = (u_x1 * water_x1_avg) - (u_x0 * water_xn1_avg);
                let deltay = (v_y1 * water_y1_avg) - (v_y0 * water_yn1_avg);
                *heights_apply.get_mut_unchecked(Vector2::new(x as i64, y as i64)) +=
                    -1.0 * (deltax + deltay) * delta_t;
            }
        }
    }
    pub fn water_simulation(&mut self) {
        //Update Velocities
        for _ in 0..20 {
            let half_uv = Self::update_velocity(
                &self.heights,
                &self.velocity,
                &self.velocity,
                &self.dimensions,
                Self::DELTA_T / 2.0,
                self.viscosity,
            );
            let mut half_h = self.heights.clone();
            Self::update_water(
                &self.heights,
                &self.velocity,
                &mut half_h,
                &self.dimensions,
                Self::DELTA_T / 2.0,
            );

            self.velocity = Self::update_velocity(
                &half_h,
                &half_uv,
                &self.velocity,
                &self.dimensions,
                Self::DELTA_T,
                self.viscosity,
            );
            Self::update_water(
                &half_h,
                &half_uv,
                &mut self.heights,
                &self.dimensions,
                Self::DELTA_T,
            );
        }
    }
}
