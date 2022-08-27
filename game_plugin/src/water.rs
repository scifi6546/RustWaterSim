use crate::prelude::{CameraLabel, GameEntity, GuiState, SelectStartupInfo};
use crate::GameState;
pub use aabb::aabb_barrier_from_transform;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
pub use water_sim::{get_conditions, AABBBarrier, PreferredSolver, SolveInfo, Solver};
pub mod aabb;
use aabb::AABBMaterial;
//pub use finite_solver::FiniteSolver;
mod uv_show;
/// size in x direction of water surface
/// Does not depend on mesh resolution
pub const WATER_SIZE: f32 = 6.0;
const HEIGHT_MULTIPLIER: f32 = 30.0;

use nalgebra::{Vector2, Vector3};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum WaterLabel {
    InsertAABBMaterial,
}
#[derive(Component, Clone, Debug)]
pub struct SolveInfoVec {
    pub data: Vec<SolveInfo>,
}
pub struct WaterPlugin;
impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading)
                .with_system(aabb::insert_aabb_material)
                .label(WaterLabel::InsertAABBMaterial),
        );
        // build system set
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .after(CameraLabel)
                .after(WaterLabel::InsertAABBMaterial)
                .with_system(spawn_water_system)
                .with_system(uv_show::build_uv_cubes),
        )
        // update system set
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .after(CameraLabel)
                .after(WaterLabel::InsertAABBMaterial)
                .with_system(water_simulation)
                .with_system(show_water)
                .with_system(aabb::aabb_transform)
                .with_system(uv_show::run_uv_cubes),
        );
    }
}

fn build_mesh(water: &water_sim::Grid<f32>, mesh: &mut Mesh) {
    let mut position = vec![];
    let mut normals = vec![];
    let mut uvs = vec![];
    for x in 0..water.x() - 1 {
        for y in 0..water.y() - 1 {
            let x0_y0 = Vector3::new(
                x as f32,
                water.get(x, y) as f32 * HEIGHT_MULTIPLIER,
                y as f32,
            );
            let x0_y1 = Vector3::new(
                x as f32,
                water.get(x, y + 1) as f32 * HEIGHT_MULTIPLIER,
                y as f32 + 1.0,
            );
            let x1_y0 = Vector3::new(
                x as f32 + 1.0,
                water.get(x + 1, y) * HEIGHT_MULTIPLIER,
                y as f32,
            );
            let x1_y1 = Vector3::new(
                x as f32 + 1.0,
                water.get(x + 1, y + 1) as f32 * HEIGHT_MULTIPLIER,
                y as f32 + 1.0,
            );
            let triangle0_normal = (x0_y1 - x0_y0).cross(&(x1_y0 - x0_y0)).normalize();
            let triangle1_normal = (x1_y0 - x1_y1).cross(&(x0_y1 - x1_y1)).normalize();

            //vert 0
            position.push([x0_y0.x, x0_y0.y, x0_y0.z]);
            normals.push([triangle0_normal.x, triangle0_normal.y, triangle0_normal.z]);
            uvs.push([0.0, 0.0]);

            //vert 1
            position.push([x0_y1.x, x0_y1.y, x0_y1.z]);
            normals.push([triangle0_normal.x, triangle0_normal.y, triangle0_normal.z]);
            uvs.push([0.0, 1.0]);
            //vert 2
            position.push([x1_y0.x, x1_y0.y, x1_y0.z]);
            normals.push([triangle0_normal.x, triangle0_normal.y, triangle0_normal.z]);
            uvs.push([1.0, 0.0]);

            //Triangle 1
            //vert3
            position.push([x1_y1.x, x1_y1.y, x1_y1.z]);
            normals.push([triangle1_normal.x, triangle1_normal.y, triangle1_normal.z]);
            uvs.push([1.0, 0.0]);

            //vert4
            position.push([x1_y0.x, x1_y0.y, x1_y0.z]);
            normals.push([triangle1_normal.x, triangle1_normal.y, triangle1_normal.z]);
            uvs.push([1.0, 1.0]);
            //vert5
            position.push([x0_y1.x, x0_y1.y, x0_y1.z]);
            normals.push([triangle1_normal.x, triangle1_normal.y, triangle1_normal.z]);
            uvs.push([0.0, 1.0]);
        }
    }

    let indicies = (0..position.len()).map(|i| i as u32).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, position);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indicies)));
}

#[derive(Component)]
pub struct WaterMarker;
#[derive(Component)]
pub struct GroundMarker;
fn spawn_water_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    aabb_material: Res<AABBMaterial>,
    startup_info: Res<SelectStartupInfo>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let conditions = get_conditions::<PreferredSolver>();
    let water_fn = conditions[startup_info.index].build_water_fn;
    let (water, mut barriers) = water_fn();
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    let scale = WATER_SIZE / water.water_h().x() as f32;

    transform.scale = Vec3::new(scale, scale, scale);
    let info: Vec<SolveInfo> = vec![];
    let mean_h = water.mean_height();
    let water_dimensions = Vector2::new(water.water_h().x(), water.water_h().y());

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut ground_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    build_mesh(&water.ground_h(), &mut ground_mesh);
    build_mesh(&water.offset_water(), &mut mesh);
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(Color::rgb(0.25, 0.25, 0.7).into()),
            transform: transform,
            mesh: meshes.add(mesh),
            ..Default::default()
        })
        .insert(water)
        .insert(SolveInfoVec { data: info })
        .insert(GameEntity)
        .insert(WaterMarker);
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(Color::rgb(0.8, 0.25, 0.7).into()),
            transform: transform,
            mesh: meshes.add(ground_mesh),
            ..Default::default()
        })
        .insert(GameEntity)
        .insert(GroundMarker);

    for barrier in barriers.drain(..) {
        aabb::build_barrier(
            &mut commands,
            barrier,
            &aabb_material,
            &mut meshes,
            mean_h,
            water_dimensions,
        );
    }
}
pub struct InitialConditions {
    pub name: &'static str,
    pub build_water_fn: fn() -> (PreferredSolver, Vec<AABBBarrier>),
}
fn water_simulation(
    _commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    gui_query: Query<&GuiState, With<GuiState>>,
    mut water_query: Query<
        (
            &mut Transform,
            &mut PreferredSolver,
            &Handle<Mesh>,
            &mut SolveInfoVec,
        ),
        With<WaterMarker>,
    >,
    aabb_query: Query<&AABBBarrier, ()>,
) {
    let gui_state = gui_query.iter().next();
    if gui_state.is_none() {
        return;
    }
    let gui_state = gui_state.unwrap();
    if gui_state.water_speed == 0 {
        return;
    }
    let aabb_vec = aabb_query.iter().copied().collect::<Vec<_>>();
    for (_, mut water, mesh, mut info) in water_query.iter_mut() {
        (0..(gui_state.water_speed - 1)).for_each(|_| {
            water.solve(&aabb_vec);
        });
        let (_, out_info) = water.solve(&aabb_vec);

        let heights = water.offset_water();

        let mut mesh = mesh_assets.get_mut(mesh).unwrap();
        build_mesh(&heights, &mut mesh);
        info.data = out_info;
    }
}
/// Handles showing velocities and water
fn show_water(
    mut water_query: Query<
        (&mut Transform, &mut Visibility, &mut PreferredSolver),
        With<WaterMarker>,
    >,
    gui_query: Query<&GuiState, Changed<GuiState>>,
) {
    let gui_state = gui_query.iter().next();
    if gui_state.is_none() {
        return;
    }
    let gui_state = gui_state.unwrap();
    for (_t, mut visible, _solver) in water_query.iter_mut() {
        visible.is_visible = gui_state.show_water;
    }
}
