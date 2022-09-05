mod gui;

use crate::prelude::build_water_mesh;
use crate::{prelude::AABBMaterial, GameState};
use bevy::prelude::*;
use nalgebra::Vector2;
use water_sim::{Grid, PreferredSolver, Solver};

pub struct MissionPlugin;
impl Plugin for MissionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Mission)
                .with_system(gui::build_gui)
                .with_system(build_water),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Mission)
                .with_system(crate::player::build_ground_system)
                .with_system(win_condition),
        );
    }
}
fn win_condition(water_query: Query<&PreferredSolver, ()>) {
    for water in water_query.iter() {
        let water_height = water.water_h();
        let max_vol = 10.0;
        let vol = (90..100)
            .flat_map(|x| (0..50).map(move |y| water_height.get(x, y)))
            .fold(0.0, |acc, x| acc + x);
        info!("vol: {}", vol);
        if vol >= max_vol {
            info!("lost!!!")
        }
    }
}
fn build_water(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    aabb_material: Res<AABBMaterial>,
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
                    10.0
                } else {
                    (8.0 - ground(x, y)).max(0.0)
                }
            },
            Vector2::new(100, 100),
        ),
        Grid::from_fn(|x, y| ground(x, y), Vector2::new(100, 100)),
        Vec::new(),
    );
    build_water_mesh(
        water,
        Vec::new(),
        commands,
        meshes,
        aabb_material,
        materials,
    );
}
