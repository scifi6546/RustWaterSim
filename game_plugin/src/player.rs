use crate::actions::Actions;
use crate::GameState;
use bevy::prelude::*;

use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};
pub struct PlayerPlugin;

pub struct Player;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct CameraLabel;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(spawn_player.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Menu)
                .with_system(spawn_camera.system())
                .label(CameraLabel),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player.system()));
    }
}
const SPAWN_RADIUS: f32 = 10.0;
fn spawn_camera(mut commands: Commands) {
    let target = Vec3::new(3.0, 3.0, 3.0);
    let eye = SPAWN_RADIUS * Vec3::new(-2.0, 2.5, 5.0).normalize() + target;

    let mouse_wheel_zoom_sensitivity = 0.02;
    #[cfg(target_family = "wasm")]
    let mouse_wheel_zoom_sensitivity = 0.002;
    commands
        .spawn_bundle(OrbitCameraBundle::new(
            OrbitCameraController {
                mouse_translate_sensitivity: Vec2::splat(0.001),
                mouse_wheel_zoom_sensitivity,
                ..Default::default()
            },
            PerspectiveCameraBundle::default(),
            eye,
            target,
        ))
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default());
}

fn spawn_player(mut commands: Commands) {
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
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
