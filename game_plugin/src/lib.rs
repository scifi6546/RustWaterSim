mod actions;
mod game_menu;
mod input;
mod loading;
mod menu;
mod player;
mod water;
use crate::actions::ActionsPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use bevy::app::AppBuilder;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use game_menu::GameMenuPlugin;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};
use water::WaterPlugin;
pub mod prelude {
    pub use super::game_menu::{GameEntity, GuiState};
    pub use super::menu::SelectStartupInfo;
    pub use super::player::CameraLabel;
    pub use super::water::{
        aabb::{build_barrier, AABBBArrier, AABBMaterial},
        FiniteSolver, InitialConditions, SolveInfo, WaterMarker, CONDITIONS, WATER_SIZE,
    };
}
// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(bevy_mod_picking::DefaultPickingPlugins)
            .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(input::CameraInput)
            .add_plugin(LookTransformPlugin)
            .add_plugin(OrbitCameraPlugin)
            .add_plugin(WaterPlugin)
            .add_plugin(GameMenuPlugin)
            .add_plugin(PlayerPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
