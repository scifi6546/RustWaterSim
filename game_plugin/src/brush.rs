use crate::prelude::{get_water_position, GameState, GroundMarker, WaterMarker};
use crate::water::build_ground_mesh;
use bevy::prelude::*;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastSource};
use nalgebra::RealField;
use water_sim::{PreferredSolver, Solver};
pub struct BrushPlugin;
impl Plugin for BrushPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DefaultRaycastingPlugin::<GroundMarker>::default())
            .add_system_set(
                SystemSet::on_update(GameState::Sandbox).with_system(build_ground_system),
            )
            .insert_resource(BrushBudget {
                used_ground: 0.0,
                max_ground: None,
            });
    }
}
#[derive(Clone, Debug)]
pub struct BrushBudget {
    /// volume of ground used
    pub used_ground: f32,
    pub max_ground: Option<f32>,
}
#[derive(Component, Copy, Clone, Debug)]
pub struct BrushCursorMarker;
/// system that handles player ground brush
pub fn build_ground_system(
    mouse_input: Res<Input<MouseButton>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut brush_budget: ResMut<BrushBudget>,
    ray_cast_iter: Query<&RayCastSource<GroundMarker>>,
    mut ground_query: Query<&Handle<Mesh>, With<GroundMarker>>,
    mut p_set: ParamSet<(
        Query<(&Transform, &mut PreferredSolver), With<WaterMarker>>,
        Query<(&mut Transform, &mut Visibility), With<BrushCursorMarker>>,
    )>,
) {
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }
    let intersect_position = ray_cast_iter
        .iter()
        .filter_map(|s| s.intersect_top())
        .map(|(_a, b)| b.position())
        .next();
    if let Some(pos) = intersect_position {
        for (mut t, mut v) in p_set.p1().iter_mut() {
            t.translation = pos;
            v.is_visible = true;
            if mouse_input.pressed(MouseButton::Left) {
                t.scale = Vec3::new(0.8, 0.8, 0.8);
            } else {
                t.scale = Vec3::new(1.0, 1.0, 1.0);
            }
        }
        for (trans, mut solver) in p_set.p0().iter_mut() {
            let p = get_water_position(pos, trans);
            let p_x = p.x as i32;
            let p_y = p.z as i32;
            let brush_radius = 10;
            let mut added_ground = 0.0f32;

            let v = f32::pi() * (brush_radius as f32).powi(2) * 1.0 / 3.0;
            let h = if let Some(max) = brush_budget.max_ground {
                if brush_budget.used_ground + v > max {
                    let volume_left = max - brush_budget.used_ground;
                    3.0 * volume_left / (f32::pi() * (brush_radius as f32).powi(2))
                } else {
                    1.0
                }
            } else {
                1.0
            };
            for x in p_x - brush_radius..p_x + brush_radius + 1 {
                for y in p_y - brush_radius..p_y + brush_radius + 1 {
                    if x < solver.dim_x() as i32 && x >= 0 && y < solver.dim_y() as i32 && y >= 0 {
                        let r = ((x as f32 - p.x).powi(2) + (y as f32 - p.z).powi(2)).sqrt();
                        let incr = h * (1.0 - r / brush_radius as f32).max(0.0);

                        *solver.get_ground_mut(x as usize, y as usize) += incr;
                        added_ground += incr;
                    }
                }
            }

            for ground_mesh in ground_query.iter_mut() {
                build_ground_mesh(solver.ground_h(), mesh_assets.get_mut(ground_mesh).unwrap())
            }
            brush_budget.used_ground += added_ground;
            info!("added ground: {} predict ground: {}", added_ground, v);
        }
    }
}
