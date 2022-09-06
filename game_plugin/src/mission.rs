mod gui;

use crate::prelude::{
    build_water_mesh, AABBMaterial, GameEntity, GameState, GuiRunner, WaterPlugin, WaterRunPlugin,
    GUI_STYLE,
};
use bevy::prelude::*;
use nalgebra::Vector2;
use water_sim::{Grid, PreferredSolver, Solver};

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
        .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(insert_mission))
        .add_system_set(
            SystemSet::on_update(GameState::Mission)
                .with_system(crate::player::build_ground_system)
                .with_system(win_condition),
        );
    }
}
pub trait MissionScenario: Send {
    fn get_solver(&self) -> PreferredSolver;
    fn name(&self) -> String;
    fn get_lost(&self, solver: &PreferredSolver) -> bool;
}
unsafe impl Send for Box<dyn MissionScenario> {}
pub fn get_missions() -> Vec<Box<dyn MissionScenario>> {
    vec![Box::new(TsunamiScenario {})]
}
fn insert_mission(mut commands: Commands) {
    commands.insert_resource(get_missions())
}
struct TsunamiScenario {}
impl MissionScenario for TsunamiScenario {
    fn get_solver(&self) -> PreferredSolver {
        fn ground(x: usize, y: usize) -> f32 {
            if x < 60 {
                0.0
            } else if x <= 80 {
                10.0 * (x as f32 - 60.0) / 20.0
            } else {
                10.0
            }
        }
        PreferredSolver::new(
            Grid::from_fn(
                |x, y| {
                    if x < 20 {
                        20.0
                    } else {
                        (8.0 - ground(x, y)).max(0.0)
                    }
                },
                Vector2::new(100, 100),
            ),
            Grid::from_fn(|x, y| ground(x, y), Vector2::new(100, 100)),
            Vec::new(),
        )
    }
    fn name(&self) -> String {
        "Tsunami".to_string()
    }
    fn get_lost(&self, solver: &PreferredSolver) -> bool {
        let loose_vol = 100.0f32;
        let water_height = solver.water_h();
        let vol = (90..100)
            .flat_map(|x| (0..50).map(move |y| water_height.get(x, y)))
            .fold(0.0, |acc, x| acc + x);
        loose_vol < vol
    }
}
#[derive(Component)]
pub struct DebugWin;

fn win_condition(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    water_query: Query<&PreferredSolver, ()>,
    mut text: Query<&mut Text, With<DebugWin>>,
) {
    const LOOSE_VOL: f32 = 100.0;
    for water in water_query.iter() {
        let water_height = water.water_h();
        let max_vol = 10.0;
        let vol = (90..100)
            .flat_map(|x| (0..50).map(move |y| water_height.get(x, y)))
            .fold(0.0, |acc, x| acc + x);
        info!("vol: {}", vol);
        let write_text = if vol < LOOSE_VOL {
            format!("volume of water in town:\n{}", vol)
        } else {
            format!("lost!!")
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
        if vol >= max_vol {
            info!("lost!!!")
        }
    }
}
fn build_water(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    aabb_material: Res<AABBMaterial>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    fn ground(x: usize, y: usize) -> f32 {
        if x < 60 {
            0.0
        } else if x <= 80 {
            10.0 * (x as f32 - 60.0) / 20.0
        } else {
            10.0
        }
    }
    let water = PreferredSolver::new(
        Grid::from_fn(
            |x, y| {
                if x < 20 {
                    20.0
                } else {
                    (8.0 - ground(x, y)).max(0.0)
                }
            },
            Vector2::new(100, 100),
        ),
        Grid::from_fn(|x, y| ground(x, y), Vector2::new(100, 100)),
        Vec::new(),
    );
    commands
        .spawn_bundle(TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: GUI_STYLE.button_text_color,
            },
        ))
        .insert(GameEntity)
        .insert(DebugWin);
    build_water_mesh(
        water,
        Vec::new(),
        commands,
        meshes,
        aabb_material,
        materials,
    );
}
