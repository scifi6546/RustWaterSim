/// based off of https://github.com/jostbr/shallow-water/blob/master/swe.py
/// Current Wierdness:
///  - When having circle there is interference that breaks the model  
///     propagating backwards from wave front
use super::{AABBBarrier, Grid, SolveInfo};
use nalgebra::Vector2;
use std::f32::consts::PI;
/// Axis aligned bounding box

fn vec_contains_point(boxes: &[AABBBarrier], x: i32, y: i32) -> bool {
    for barrier in boxes.iter() {
        if barrier.contains_point(x, y) {
            return true;
        }
    }
    return false;
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
        let t = t as f32;
        let s = (2.0 * PI * t / self.period).sin();

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

impl FiniteSolver {
    const DX: f32 = 999.0;
    const DY: f32 = 999.0;
    const G: f32 = 9.81;
    const DT: f32 = 0.1;

    /// gets mean height of water
    pub fn mean_height(&self) -> f32 {
        let mut sum = 0.0;
        for x in 0..self.h.x() {
            for y in 0..self.h.y() {
                sum += self.h.get(x, y) / (self.h.x() * self.h.y()) as f32
            }
        }
        return sum;
    }
    /// runs water simulation and outputs water heights
    pub fn solve(&mut self, boxes: &[AABBBarrier]) -> (&Grid<f32>, Vec<SolveInfo>) {
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
                max_delta = if delta > max_delta { delta } else { max_delta };
                *h_apply.get_mut(x, y) -= delta;
            }
        }
        max_delta
    }
    pub fn numpy_data(&self) -> Vec<u8> {
        let header_str = format!(
            "{{'descr': '<f4', 'fortran_order': False, 'shape': ({}, {}), }}",
            self.h.x(),
            self.h.y()
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
        for x in 0..self.h.x() {
            for y in 0..self.h.y() {
                let h = self.h.get(x, y);
                for byte in h.to_ne_bytes().iter() {
                    out_data.push(*byte);
                }
            }
        }

        return out_data;
    }
    pub fn droplet() -> (Self, Vec<AABBBarrier>) {
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
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(101, 100));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(100, 101));
        (
            Self {
                h,
                g_h,
                u,
                v,
                sources: vec![],
                t: 0,
            },
            vec![],
        )
    }
    pub fn cup() -> (Self, Vec<AABBBarrier>) {
        let g_h = Grid::from_fn(
            |x, y| {
                let r = ((x as f32 - 200.0).powi(2) + (y as f32 - 200.0).powi(2)).sqrt();
                r / 100.0
            },
            Vector2::new(400, 400),
        );
        let h = Grid::from_fn(|_, _| 2.0, Vector2::new(400, 400));
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(401, 400));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(400, 401));
        (
            Self {
                h,
                g_h,
                u,
                v,
                sources: vec![],
                t: 0,
            },
            vec![],
        )
    }
    pub fn lake() -> (Self, Vec<AABBBarrier>) {
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
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(101, 100));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(100, 101));
        (
            Self {
                h,
                g_h,
                u,
                v,
                sources: vec![],
                t: 0,
            },
            vec![],
        )
    }
    pub fn dynamic_droplet() -> (Self, Vec<AABBBarrier>) {
        let h = Grid::from_fn(|_, _| 2.0, Vector2::new(300, 300));
        let g_h = Grid::from_fn(|_x, _y| 0.0, Vector2::new(300, 300));
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(301, 300));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(300, 301));
        (
            Self {
                h,
                g_h,
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
            },
            vec![],
        )
    }
    pub fn single_dynamic() -> (Self, Vec<AABBBarrier>) {
        let h = Grid::from_fn(|_, _| 2.0, Vector2::new(200, 200));
        let g_h = Grid::from_fn(|_x, _y| 0.0, Vector2::new(200, 200));
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(201, 200));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(200, 201));
        (
            Self {
                h,
                g_h,
                u,
                v,
                sources: vec![Source {
                    center: Vector2::new(100.0, 100.0),
                    height: 2.2,
                    radius: 10.0,
                    period: 1000.0,
                }],
                t: 0,
            },
            vec![],
        )
    }
    pub fn big_droplet() -> (Self, Vec<AABBBarrier>) {
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
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(251, 250));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(250, 251));
        (
            Self {
                h,
                g_h,
                u,
                v,
                sources: vec![],
                t: 0,
            },
            vec![],
        )
    }
    pub fn bridge_poles() -> (Self, Vec<AABBBarrier>) {
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(101, 300));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(100, 301));
        let g_h = Grid::from_fn(|_x, _y| 0.0, Vector2::new(100, 300));
        let h = Grid::from_fn(
            |_x, y| {
                let top_height = 1.5;
                let base_height = 1.0;
                let top_cutoff = 10;
                let slope_cutoff = 40;
                let slope = (top_height - base_height) / (top_cutoff as f32 - slope_cutoff as f32);
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
            Self {
                u,
                v,
                h,
                g_h,
                sources: vec![],
                t: 0,
            },
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
    }
    pub fn barrier() -> (Self, Vec<AABBBarrier>) {
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
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(101, 200));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(100, 201));
        (
            Self {
                h,
                g_h,
                u,
                v,
                sources: vec![],
                t: 0,
            },
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
    }
    pub fn barrier_long() -> (Self, Vec<AABBBarrier>) {
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
        let u = Grid::from_fn(|_, _| 0.0, Vector2::new(101, 1000));
        let v = Grid::from_fn(|_, _| 0.0, Vector2::new(100, 1001));
        (
            Self {
                h,
                g_h,
                u,
                v,
                sources: vec![],
                t: 0,
            },
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
    }
}
