use crate::GameState;
use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
mod my_solver;
use my_solver::MySolver;
use nalgebra::{Vector2, Vector3};
pub struct WaterPlugin;
/// solver for water
pub trait Solver: Send + 'static {
    const HEIGHT_MULTIPLIER: f32 = 100.0;
    /// builds solver
    fn new(water: Grid<f32>, velocities: Grid<Vector2<f32>>) -> Self;
    /// runs water simulation and outputs water heights
    fn solve(&mut self) -> &Grid<f32>;
    fn update_mesh(water: &Grid<f32>, mesh: &mut Mesh) {
        let mut position = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        for x in 0..water.x() - 1 {
            for y in 0..water.y() - 1 {
                let x0_y0 = Vector3::new(
                    x as f32,
                    water.get(x, y) as f32 * Self::HEIGHT_MULTIPLIER,
                    y as f32,
                );
                let x0_y1 = Vector3::new(
                    x as f32,
                    water.get(x, y + 1) as f32 * Self::HEIGHT_MULTIPLIER,
                    y as f32 + 1.0,
                );
                let x1_y0 = Vector3::new(
                    x as f32 + 1.0,
                    water.get(x + 1, y) * Self::HEIGHT_MULTIPLIER,
                    y as f32,
                );
                let x1_y1 = Vector3::new(
                    x as f32 + 1.0,
                    water.get(x + 1, y + 1) as f32 * Self::HEIGHT_MULTIPLIER,
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
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, position);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indicies)));
    }
    /// Builds mesh from grid, todo: make water sim inplace for performance reasons
    fn build_mesh(water: &Grid<f32>) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        Self::update_mesh(water, &mut mesh);
        return mesh;
    }
}
impl Plugin for WaterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(spawn_water_system.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(water_simulation.system()),
        );
    }
}
#[derive(Clone)]
struct Grid<T: Clone + Copy> {
    points: Vec<T>,
    x: usize,
    y: usize,
}
impl<T: Clone + Copy> Grid<T> {
    pub fn from_vec(dimensions: Vector2<usize>, points: Vec<T>) -> Self {
        assert_eq!(dimensions.x * dimensions.y, points.len());
        Self {
            points,
            x: dimensions.x,
            y: dimensions.y,
        }
    }
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    ///
    pub fn get(&self, x: usize, y: usize) -> T {
        self.points[self.y * x + y]
    }
    /// gets points unchecked at point
    pub fn get_unchecked(&self, dim: Vector2<i64>) -> T {
        self.points[self.y * dim.x as usize + dim.y as usize]
    }
    /// gets unchecked mut
    pub fn get_mut_unchecked(&mut self, dim: Vector2<i64>) -> &mut T {
        &mut self.points[self.y * dim.x as usize + dim.y as usize]
    }
}
pub struct WaterMarker;

fn spawn_water_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut heights_data = vec![0.0; 100 * 100];
    heights_data[50 * 50 + 50] = 1.0;
    let water_heights = Grid::from_vec(Vector2::new(100, 100), heights_data);
    let velocities = Grid::from_vec(
        Vector2::new(101, 101),
        vec![Vector2::new(0.0, 0.0); 101 * 101],
    );
    let water = Water::new(water_heights, velocities);
    let mut transform = Transform::from_translation(Vec3::new(0.3, 0.5, 0.3));
    transform.scale = Vec3::new(0.1, 0.1, 0.1);
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(Color::rgb(0.0, 0.5, 0.0).into()),
            transform: transform,
            mesh: meshes.add(water.build_mesh()),
            ..Default::default()
        })
        .insert(water)
        .insert(WaterMarker);
}
fn water_simulation(
    _commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut water_query: Query<
        (&mut Transform, &mut Box<dyn Solver>, &Handle<Mesh>),
        With<WaterMarker>,
    >,
) {
    for (_, mut water, mesh) in water_query.iter_mut() {
        water.water_simulation();
        let mut mesh = mesh_assets.get_mut(mesh).unwrap();
        water.update_mesh(&mut mesh);
    }
}
