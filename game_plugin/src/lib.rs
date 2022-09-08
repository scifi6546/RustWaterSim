mod actions;
mod file_save;
mod game_menu;
mod gui;
mod input;
mod loading;
mod markdown;
mod menu;
mod mission;
mod player;
mod sandbox;
mod water;

use crate::actions::ActionsPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::winit;

use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};
use water::WaterPlugin;
pub mod prelude {
    pub use super::game_menu::{
        AddBoxButton, ButtonMaterial, GameEntity, GameMenu, GuiState, GuiStyle, LeaveButton,
        LeaveText, PauseButton, PauseTexture, PlayButton, PlayTexture, SaveWaterButton, ShowSpeed,
        ShowVelocities, ShowWater, SolveInfoLabel, SpeedDirection, ViscocityChange, GUI_STYLE,
        MAX_WATER_SPEED,
    };
    pub use super::gui::{build_play_menu, GuiRunner};
    pub use super::loading::FontAssets;
    pub use super::markdown::{
        build_gui, despawn_gui, nav_system, BuiltParentLabel, Document, DocumentGuiParent,
        GuiParent,
    };
    pub use super::menu::SelectStartupInfo;
    pub use super::mission::{Mission, MissionScenario};
    pub use super::player::CameraLabel;
    pub use super::water::{
        aabb::{aabb_barrier_from_transform, build_barrier, AABBMaterial},
        build_water_mesh, get_water_position, AABBBarrier, GroundMarker, InitialConditions,
        SolveInfo, SolveInfoVec, WaterMarker, WaterPlugin, WaterRunPlugin, WATER_SIZE,
    };
    pub use super::GameState;
    pub use water_sim::get_conditions;
}
// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Sandbox,
    /// During this state a player is competing to finish a puzzle
    Mission,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    Page,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
            .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
            .add_plugin(LoadingPlugin)
            .insert_resource(winit::WinitSettings::desktop_app())
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(input::CameraInput)
            .add_plugin(markdown::DocumentPlugin)
            .add_plugin(LookTransformPlugin)
            .add_plugin(mission::MissionPlugin)
            .add_plugin(sandbox::SandboxPlugin)
            .add_plugin(OrbitCameraPlugin {
                override_input_system: false,
            })
            .add_plugin(WaterPlugin {
                active_state: GameState::Sandbox,
            })
            .add_plugin(PlayerPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
        #[cfg(feature = "native")]
        {
            app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());
        }
    }
}
