use crate::GameState;
use crate::{actions::Actions, prelude::GroundMarker, water::WaterMarker};
use bevy::prelude::*;

use crate::water::{build_mesh, get_water_position};
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMethod, RayCastSource, RaycastSystem};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};
use water_sim::{PreferredSolver, Solver};

pub struct PlayerPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Component, SystemLabel)]
pub struct Player;
#[derive(Debug, Hash, PartialEq, Eq, Clone, Component, SystemLabel)]
pub struct CameraLabel;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DefaultRaycastingPlugin::<GroundMarker>::default())
            .add_system_set(SystemSet::on_enter(GameState::Sandbox))
            .add_startup_system(spawn_camera)
            .add_system_set(
                SystemSet::on_update(GameState::Sandbox)
                    .with_system(move_player)
                    .with_system(build_ground_system)
                    .with_system(update_raycast_cursor),
            )
            .add_system_to_stage(
                CoreStage::First,
                update_raycast_cursor.before(RaycastSystem::BuildRays::<GroundMarker>),
            );
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, SystemLabel)]
pub struct RayCastBuildLabel;
#[derive(Component, Copy, Clone, Debug)]
pub struct BrushCursorMarker;
const SPAWN_RADIUS: f32 = 10.0;
fn spawn_camera(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    info!("spawining camera");

    let target = Vec3::new(3.0, 3.0, 3.0);
    let eye = SPAWN_RADIUS * Vec3::new(-2.0, 2.5, 5.0).normalize() + target;

    let mouse_wheel_zoom_sensitivity = 0.02;
    #[cfg(target_family = "wasm")]
    let mouse_wheel_zoom_sensitivity = 0.002;
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_translate_sensitivity: Vec2::splat(0.001),
                mouse_wheel_zoom_sensitivity,
                ..Default::default()
            },
            eye,
            target,
        ))
        .insert(RayCastSource::<GroundMarker>::default())
        .insert(UiCameraConfig { show_ui: true })
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default());
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
pub struct RayCastCursorLabel;
fn update_raycast_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<GroundMarker>>,
) {
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };
    for mut pick_source in &mut query {
        pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
    }
}
/// system that handles player ground brush
pub fn build_ground_system(
    mouse_input: Res<Input<MouseButton>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
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
        .map(|(a, b)| b.position())
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
            for x in p_x - brush_radius..p_x + brush_radius + 1 {
                for y in p_y - brush_radius..p_y + brush_radius + 1 {
                    if x < solver.dim_x() as i32 && x >= 0 && y < solver.dim_y() as i32 && y >= 0 {
                        let r = ((x as f32 - p.x).powi(2) + (y as f32 - p.z).powi(2)).sqrt();
                        let incr = (1.0 - r / brush_radius as f32).max(0.0);
                        *solver.get_ground_mut(x as usize, y as usize) += 0.8 * incr;
                    }
                }
            }

            for ground_mesh in ground_query.iter_mut() {
                build_mesh(solver.ground_h(), mesh_assets.get_mut(ground_mesh).unwrap())
            }
            info!("{}", p);
        }
    }
}
fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &Handle<Mesh>), With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 1.0;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for (mut player_transform, _) in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}
