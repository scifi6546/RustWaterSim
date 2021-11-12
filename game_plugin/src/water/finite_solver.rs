/// based off of https://github.com/jostbr/shallow-water/blob/master/swe.py
/// Current Wierdness:
///  - When having circle there is interference that breaks the model  
///     propagating backwards from wave front
use super::{Grid, SolveInfo, Solver};
use bevy::prelude::*;
use nalgebra::Vector2;
use std::f32::consts::PI;
/// Axis aligned bounding box
pub struct AABBBArrier {
    pub top_right: Vector2<usize>,
    pub bottom_left: Vector2<usize>,
}
/// Water Source, dynamically adds droplet in order to create pretty waves
pub struct Source {
    /// center of source
    center: Vector2<f32>,
    /// radius of cone
    radius: f32,
    /// height of added cone
    height: f32,
    /// period in number of timesteps of pattern
    period: f32,
}
impl Source {
    pub fn change_h(&self, height: &mut Grid<f32>, t: u32) {
        let t_i = t;
        let t = t as f32;
        let s = (2.0 * PI * t / self.period).sin();
        if t_i % 10 == 0 {
            info!("s: {}", s);
        }

        for x in 0..height.x() {
            for y in 0..height.y() {
                let distance = ((x as f32 - self.center.x).powi(2)
                    + (y as f32 - self.center.y).powi(2))
                .sqrt();
                let dh = if distance < self.radius {
                    self.height * (self.radius - distance) / self.radius
                } else {
                    0.0
                };
                *height.get_mut(x, y) += s * dh / self.period;
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoundryType {
    Source,
    Reflection,
}
pub struct FiniteSolver {
    /// Water height
    h: Grid<f32>,
    ///u velocity
    u: Grid<f32>,
    /// v velocity
    v: Grid<f32>,
    /// time counter
    t: u32,
    /// sources to be added at runtime
    sources: Vec<Source>,
}
impl Solver for FiniteSolver {
    fn solve(&mut self) -> (&Grid<f32>, Vec<SolveInfo>) {
        let mut max_delta = 0.0;
        for _ in 0..Self::NUM_STEPS_PER_FRAME {
            //Self::update_velocity(&self.h, &mut self.u, &mut self.v, Self::DT);
            //let old_h = self.h.clone();
            //let delta = Self::update_heights(&old_h, &mut self.h, &self.u, &self.v, Self::DT);
            let delta = self.time_step();
            max_delta = if delta > max_delta { delta } else { max_delta };
        }
        let mut volume = 0.0;
        for x in 0..self.h.x() {
            for y in 0..self.h.y() {
                volume += self.h.get(x, y);
            }
        }
        (
            &self.h,
            vec![
                SolveInfo {
                    name: "max delta Height",
                    data: format!("{:.2e}", max_delta),
                },
                SolveInfo {
                    name: "Volume",
                    data: format!("{:.2e}", volume),
                },
            ],
        )
    }
    fn h(&self) -> &Grid<f32> {
        &self.h
    }
    fn v(&self) -> &Grid<f32> {
        &self.v
    }
    fn u(&self) -> &Grid<f32> {
        &self.u
    }
}
impl FiniteSolver {
    const NUM_STEPS_PER_FRAME: usize = 2;
    const DX: f32 = 999.0;
    const DY: f32 = 999.0;
    const G: f32 = 9.81;
    const DT: f32 = 0.1;
    const VISC: f32 = 0.0;
    const BOUNDRY: BoundryType = BoundryType::Reflection;
    /// Returns max displacement in timestep
    pub fn time_step(&mut self) -> f32 {
        for source in self.sources.iter() {
            source.change_h(&mut self.h, self.t);
        }
        let mut u_half = self.u.clone();
        let mut v_half = self.v.clone();
        let half_uv = Self::update_velocity(&self.h, &mut u_half, &mut v_half, Self::DT / 2.0);
        let mut half_h = self.h.clone();
        Self::update_heights(&self.h, &mut half_h, &self.u, &self.v, Self::DT / 2.0);

        Self::update_velocity(&half_h, &mut self.u, &mut self.v, Self::DT);
        self.t += 1;
        Self::update_heights(&half_h, &mut self.h, &self.u, &self.v, Self::DT)
    }
    fn update_velocity(heights: &Grid<f32>, u: &mut Grid<f32>, v: &mut Grid<f32>, delta_t: f32) {
        for x in 0..heights.x() + 1 {
            for y in 0..heights.y() + 1 {
                if x != 0 && y != 0 {
                    //handling u
                    if y < heights.y() {
                        if x == 0 || x == heights.x() {
                            *u.get_mut(x, y) = 0.0;
                        } else {
                            let hxn1 = heights.get(x - 1, y);
                            let hxp1 = heights.get(x, y);
                            *u.get_mut(x, y) += Self::G * (delta_t / Self::DX) * (hxp1 - hxn1);
                            *u.get_mut(x, y) *= 1.0 - Self::VISC * delta_t;
                        }
                    }
                    if x < heights.x() {
                        if y == 0 || y == heights.y() {
                            *v.get_mut(x, y) = 0.0;
                        } else {
                            let hyn1 = heights.get(x, y - 1);
                            let hyp1 = heights.get(x, y);
                            *v.get_mut(x, y) += Self::G * (delta_t / Self::DY) * (hyp1 - hyn1);
                            *v.get_mut(x, y) *= 1.0 - Self::VISC * delta_t;
                        }
                    }
                }
            }
        }
    }
    fn update_heights(
        h: &Grid<f32>,
        h_apply: &mut Grid<f32>,
        u: &Grid<f32>,
        v: &Grid<f32>,
        delta_t: f32,
    ) -> f32 {
        let mut max_delta = 0.0;
        for x in 0..h.x() {
            for y in 0..h.y() {
                let un1 = u.get(x, y);
                let up1 = u.get(x + 1, y);
                let vn1 = v.get(x, y);
                let vp1 = v.get(x, y + 1);

                let hxn1 = if x >= 1 { h.get(x - 1, y) } else { h.get(x, y) };
                let hxp1 = if x <= h.x() - 2 {
                    h.get(x + 1, y)
                } else {
                    h.get(x, y)
                };

                let hyn1 = if y >= 1 { h.get(x, y - 1) } else { h.get(x, y) };
                let hyp1 = if y <= h.y() - 2 {
                    h.get(x, y + 1)
                } else {
                    h.get(x, y)
                };
                let h0 = h.get(x, y);
                let mut dx = 0.0;
                //lower x boundry
                if x >= 1 {
                    dx += un1 * (hxn1 + h0) / 2.0;
                } else {
                    match Self::BOUNDRY {
                        BoundryType::Source => continue,
                        BoundryType::Reflection => {}
                    }
                }
                // upper x boundry
                if x <= h.x() - 2 {
                    dx -= up1 * (hxp1 + h0) / 2.0;
                } else {
                    match Self::BOUNDRY {
                        BoundryType::Source => continue,
                        BoundryType::Reflection => {}
                    }
                }

                //let dx = un1 * (hxn1 + h0) / 2.0 - up1 * (hxp1 + h0) / 2.0;
                let mut dy = 0.0;
                //lower y boundry
                if y >= 1 {
                    dy += vn1 * (hyn1 + h0) / 2.0;
                } else {
                    match Self::BOUNDRY {
                        BoundryType::Source => continue,
                        BoundryType::Reflection => {}
                    }
                }
                // upper y boundry
                if y <= h.y() - 2 {
                    dy -= vp1 * (hyp1 + h0) / 2.0;
                } else {
                    match Self::BOUNDRY {
                        BoundryType::Source => continue,
                        BoundryType::Reflection => {}
                    }
                }
                let delta = delta_t * (dx + dy);
                max_delta = if delta > max_delta { delta } else { max_delta };
                *h_apply.get_mut(x, y) -= delta;
            }
        }
        max_delta
    }
    pub fn droplet() -> Self {
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
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(101, 100));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(100, 101));
        Self {
            h,
            u,
            v,
            sources: vec![],
            t: 0,
        }
    }
    pub fn dynamic_droplet() -> Self {
        let h = Grid::from_fn(|_, _| 2.0, Vector2::new(300, 300));
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(301, 300));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(300, 301));
        Self {
            h,
            u,
            v,
            sources: vec![
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
            ],
            t: 0,
        }
    }
    pub fn big_droplet() -> Self {
        let h = Grid::from_fn(
            |x, y| {
                let floor = 5.0;
                let droplet_size = 50.0;
                let height = 10.0;
                let drop_x = 125.0;
                let drop_y = 125.0;
                let r = ((x as f32 - drop_x).powi(2) + (y as f32 - drop_x).powi(2)).sqrt();
                if r <= droplet_size {
                    height * (droplet_size - r) / droplet_size + floor
                } else {
                    floor
                }
            },
            Vector2::new(250, 250),
        );
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(251, 250));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(250, 251));
        Self {
            h,
            u,
            v,
            sources: vec![],
            t: 0,
        }
    }
    pub fn wave_wall() -> Self {
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(101, 100));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(100, 101));
        let h = Grid::from_fn(
            |x, y| {
                if x > 5 && x < 20 {
                    if y > 5 && y < 95 {
                        1.5
                    } else {
                        1.0
                    }
                } else {
                    1.0
                }
            },
            Vector2::new(100, 100),
        );
        Self {
            u,
            v,
            h,
            sources: vec![],
            t: 0,
        }
    }
}
