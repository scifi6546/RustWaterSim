/// based off of https://github.com/jostbr/shallow-water/blob/master/swe.py
/// Current Wierdness:
///  - When having circle there is interference that breaks the model  
///     propagating backwards from wave front
use super::{AABBBarrier, Grid, SolveInfo, Solver, Source};
use bevy::prelude::*;
use nalgebra::Vector2;

/// Axis aligned bounding box

fn vec_contains_point(boxes: &[AABBBarrier], x: i32, y: i32) -> bool {
    for barrier in boxes.iter() {
        if barrier.contains_point(x, y) {
            return true;
        }
    }
    return false;
}

#[derive(Component)]
pub struct FiniteSolver {
    /// Ground Height
    g_h: Grid<f32>,
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
    fn new(water: Grid<f32>, ground: Grid<f32>, sources: Vec<Source>) -> Self {
        assert_eq!(water.x(), ground.x());
        assert_eq!(water.y(), ground.y());
        let dim = Vector2::new(water.x(), water.y());
        Self {
            g_h: ground,
            h: water,
            u: Grid::from_fn(|_, _| 0.0, Vector2::new(dim.x + 1, dim.y)),
            v: Grid::from_fn(|_, _| 0.0, Vector2::new(dim.x, dim.y + 1)),
            t: 0,
            sources,
        }
    }
    /// runs water simulation and outputs water heights
    fn solve(&mut self, boxes: &[AABBBarrier]) -> (&Grid<f32>, Vec<SolveInfo>) {
        self.time_step(boxes);

        (
            &self.h,
            vec![], /*
                    vec![
                        SolveInfo {
                            name: "max delta Height",
                            data: format!("{:.2e}", max_delta),
                        },
                        SolveInfo {
                            name: "Volume",
                            data: format!("{:.2e}", volume),
                        },
                    ],*/
        )
    }
    fn water_h(&self) -> &Grid<f32> {
        &self.h
    }
    fn ground_h(&self) -> &Grid<f32> {
        &self.g_h
    }
}
impl FiniteSolver {
    const DX: f32 = 999.0;
    const DY: f32 = 999.0;
    const G: f32 = 9.81;
    const DT: f32 = 0.1;

    /// output reference to h data
    pub fn h(&self) -> &Grid<f32> {
        &self.h
    }
    /// outputs h offset from ground height
    pub fn offset_h(&self) -> Grid<f32> {
        self.h.clone() + self.g_h.clone()
    }
    /// outputs ground heights
    pub fn g_h(&self) -> &Grid<f32> {
        &self.g_h
    }
    /// output velocity in y direction grid
    pub fn v(&self) -> &Grid<f32> {
        &self.v
    }
    /// output velocity in x direction grid
    pub fn u(&self) -> &Grid<f32> {
        &self.u
    }
    /// Returns max displacement in timestep
    pub fn time_step(&mut self, barriers: &[AABBBarrier]) -> f32 {
        for source in self.sources.iter() {
            source.change_h(&mut self.h, self.t);
        }
        let mut u_half = self.u.clone();
        let mut v_half = self.v.clone();

        Self::update_velocity(
            &self.h,
            &self.g_h,
            &mut u_half,
            &mut v_half,
            Self::DT / 2.0,
            barriers,
        );
        let mut half_h = self.h.clone();
        Self::update_heights(
            &self.h,
            &mut half_h,
            &self.u,
            &self.v,
            Self::DT / 2.0,
            barriers,
        );

        Self::update_velocity(
            &half_h,
            &self.g_h,
            &mut self.u,
            &mut self.v,
            Self::DT,
            barriers,
        );
        self.t += 1;
        Self::update_heights(&half_h, &mut self.h, &self.u, &self.v, Self::DT, barriers)
    }
    fn update_velocity(
        heights: &Grid<f32>,
        ground_heights: &Grid<f32>,
        u: &mut Grid<f32>,
        v: &mut Grid<f32>,
        delta_t: f32,
        boxes: &[AABBBarrier],
    ) {
        for x in 0..heights.x() + 1 {
            for y in 0..heights.y() + 1 {
                //handling u
                if y < heights.y() {
                    if x == 0
                        || x == heights.x()
                        || vec_contains_point(&boxes, x as i32, y as i32)
                        || vec_contains_point(&boxes, x as i32 - 1, y as i32)
                    {
                        *u.get_mut(x, y) = 0.0;
                    } else {
                        let hxn1 = heights.get(x - 1, y);
                        let hxp1 = heights.get(x, y);
                        let gh_xn1 = ground_heights.get(x - 1, y);
                        let gh_xp1 = ground_heights.get(x, y);

                        *u.get_mut(x, y) +=
                            Self::G * (delta_t / Self::DX) * ((hxp1 + gh_xp1) - (hxn1 + gh_xn1));
                    }
                }
                if x < heights.x() {
                    if y == 0
                        || y == heights.y()
                        || vec_contains_point(&boxes, x as i32, y as i32)
                        || vec_contains_point(&boxes, x as i32, y as i32 - 1)
                    {
                        *v.get_mut(x, y) = 0.0;
                    } else {
                        let hyn1 = heights.get(x, y - 1);
                        let hyp1 = heights.get(x, y);

                        let gh_yn1 = ground_heights.get(x, y - 1);
                        let gh_yp1 = ground_heights.get(x, y);
                        *v.get_mut(x, y) +=
                            Self::G * (delta_t / Self::DY) * ((hyp1 + gh_yp1) - (hyn1 + gh_yn1));
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
        boxes: &[AABBBarrier],
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
                if x >= 1 || !vec_contains_point(&boxes, x as i32 - 1, y as i32) {
                    dx += un1 * (hxn1 + h0) / 2.0;
                }
                // upper x boundry
                if x <= h.x() - 2 || !vec_contains_point(&boxes, x as i32 + 1, y as i32) {
                    dx -= up1 * (hxp1 + h0) / 2.0;
                }

                //let dx = un1 * (hxn1 + h0) / 2.0 - up1 * (hxp1 + h0) / 2.0;
                let mut dy = 0.0;
                //lower y boundry
                if y >= 1 || !vec_contains_point(&boxes, x as i32, y as i32 - 1) {
                    dy += vn1 * (hyn1 + h0) / 2.0;
                }
                // upper y boundry
                if y <= h.y() - 2 || !vec_contains_point(&boxes, x as i32, y as i32 + 1) {
                    dy -= vp1 * (hyp1 + h0) / 2.0;
                }
                let delta = delta_t * (dx + dy);
                {
                    let h_old = h.get(x, y);
                    if delta >= h_old {
                        println!("use k?");
                    }
                }
                max_delta = if delta > max_delta { delta } else { max_delta };
                *h_apply.get_mut(x, y) -= delta;
            }
        }
        max_delta
    }
}
