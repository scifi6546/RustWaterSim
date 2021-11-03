use crate::actions::Actions;
use crate::GameState;
use bevy::prelude::*;

use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};
pub struct PlayerPlugin;

pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system())
                .with_system(debug_text.system())
                .with_system(spawn_camera.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player.system()));
    }
}
fn debug_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "hello bevy".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        ..Default::default()
    });
}
<<<<<<< HEAD

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let water = Water::new();
    let mut transform = Transform::from_translation(Vec3::new(0.3, 0.5, 0.3));
    transform.scale = Vec3::new(0.1, 0.1, 0.1);
=======
fn spawn_camera(mut commands: Commands) {
    let eye = Vec3::new(-2.0, 2.5, 5.0);
    let target = Vec3::ZERO;
>>>>>>> 97faa976935312d75a0f20bcaf716c6743ce53ad
    commands
        .spawn_bundle(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            PerspectiveCameraBundle::default(),
            eye,
            target,
        ))
        .insert(PerspectiveCameraBundle::default());
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
