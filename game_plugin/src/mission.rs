mod gui;
mod mission_list;

use crate::prelude::{
    build_water_mesh_system, AABBMaterial, BrushBudget, GameState, GuiRunner, WaterRunPlugin,
    GUI_STYLE,
};

use bevy::prelude::*;

use std::sync::{Arc, Mutex};
use water_sim::{Grid, PreferredSolver, SolverBoundaryConditions, Source};

pub struct MissionPlugin;
impl Plugin for MissionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GuiRunner {
            active_state: GameState::Mission,
        })
        .add_plugin(WaterRunPlugin {
            active_state: GameState::Mission,
        })
        .add_system_set(
            SystemSet::on_enter(GameState::Mission)
                .with_system(gui::build_gui)
                .with_system(build_water),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(mission_list::insert_missions),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Mission)
                .with_system(crate::brush::build_ground_system)
                .with_system(win_condition),
        )
        .add_system_set(SystemSet::on_exit(GameState::Mission).with_system(reset_budget));
    }
}
pub enum WinState {
    Won,
    Lost,
    InProgress(String),
}
pub trait MissionScenario: Send {
    fn get_solver(&self) -> (PreferredSolver, BrushBudget);
    fn name(&self) -> String;
    fn get_win_state(&self, solver: &PreferredSolver, budget: &BrushBudget) -> WinState;
}
#[derive(Clone, Component)]
pub struct Mission {
    pub scenario: Arc<Mutex<dyn MissionScenario>>,
}

#[derive(Component)]
pub struct DebugWin;

fn win_condition(
    current_mission: Res<Mission>,
    asset_server: Res<AssetServer>,
    brush_budget: Res<BrushBudget>,
    water_query: Query<&PreferredSolver, ()>,

    mut text: Query<&mut Text, With<DebugWin>>,
) {
    for water in water_query.iter() {
        let lost = current_mission
            .scenario
            .lock()
            .unwrap()
            .get_win_state(water, &brush_budget);
        let write_text = match lost {
            WinState::Won => "won!".to_string(),
            WinState::InProgress(t) => t,
            WinState::Lost => "lost".to_string(),
        };

        for mut t in text.iter_mut() {
            t.sections = vec![TextSection::new(
                write_text.clone(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: GUI_STYLE.button_text_color,
                },
            )];
        }
    }
}
fn build_water(
    mut commands: Commands,
    active_mission: Res<Mission>,
    meshes: ResMut<Assets<Mesh>>,
    aabb_material: Res<AABBMaterial>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let scenario = active_mission.scenario.lock().unwrap();
    let (water, budget) = scenario.get_solver();
    commands.insert_resource(budget);

    build_water_mesh_system(
        water,
        Vec::new(),
        commands,
        meshes,
        aabb_material,
        materials,
    );
}
fn reset_budget(mut brush_budget: ResMut<BrushBudget>) {
    *brush_budget = BrushBudget {
        used_ground: 0.0,
        max_ground: None,
    };
}
