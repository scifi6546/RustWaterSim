use super::Grid;
use nalgebra::Vector2;
use std::f32::consts::PI;
/// Water Source, dynamically adds droplet in order to create pretty waves
pub struct Source {
    /// center of source
    pub center: Vector2<f32>,
    /// radius of cone
    pub radius: f32,
    /// height of added cone
    pub height: f32,
    /// period in number of timesteps of pattern
    pub period: f32,
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
