use crate::GameState;
use crate::{actions::Actions, prelude::GroundMarker};
use bevy::prelude::*;

use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMethod, RayCastSource, RaycastSystem};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

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
