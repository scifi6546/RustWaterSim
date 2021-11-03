use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use nalgebra::Vector3;
pub struct PlayerPlugin;

pub struct Player;
struct Grid {
    points: Vec<f32>,
    x: usize,
    y: usize,
}
impl Grid {
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    /// gets points unchecked at point
    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.points[self.y * x + y]
    }
}
pub struct Water {
    grid: Grid,
}
impl Water {
    pub fn new() -> Self {
        Self {
            grid: Grid {
                points: vec![0.0; 100 * 100],
                x: 100,
                y: 100,
            },
        }
    }
    /// Builds mesh from grid, todo: make water sim inplace for performance reasons
    pub fn build_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut vertices = vec![];
        let mut position = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        for x in 0..self.grid.x() - 1 {
            for y in 0..self.grid.y() - 1 {
                let x0_y0 = Vector3::new(x as f32, self.grid.get(x, y) as f32, y as f32);
                let x0_y1 = Vector3::new(x as f32, self.grid.get(x, y + 1) as f32, y as f32 + 1.0);
                let x1_y0 = Vector3::new(x as f32 + 1.0, self.grid.get(x + 1, y), y as f32);
                let x1_y1 = Vector3::new(
                    x as f32 + 1.0,
                    self.grid.get(x + 1, y + 1) as f32,
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

                vertices.append(&mut vec![
                    //uv
                    0.0,
                    0.0,
                    //normal
                    triangle0_normal.x,
                    triangle0_normal.y,
                    triangle0_normal.z,
                    //position:
                    x0_y1.x,
                    x0_y1.y,
                    x0_y1.z,
                    //uv
                    0.0,
                    1.0,
                    //normal
                    triangle0_normal.x,
                    triangle0_normal.y,
                    triangle0_normal.z,
                    //position:
                    x1_y0.x,
                    x1_y0.y,
                    x1_y0.z,
                    //uv
                    1.0,
                    0.0,
                    //normal
                    triangle0_normal.x,
                    triangle0_normal.y,
                    triangle0_normal.z,
                    //triangle 1

                    //position:
                    x0_y1.x,
                    x0_y1.y,
                    x0_y1.z,
                    //uv
                    0.0,
                    1.0,
                    //normal
                    triangle1_normal.x,
                    triangle1_normal.y,
                    triangle1_normal.z,
                    //position:
                    x1_y1.x,
                    x1_y1.y,
                    x1_y1.z,
                    //uv
                    1.0,
                    1.0,
                    //normal
                    triangle1_normal.x,
                    triangle1_normal.y,
                    triangle1_normal.z,
                    //position:
                    x1_y0.x,
                    x1_y0.y,
                    x1_y0.z,
                    //uv
                    1.0,
                    0.0,
                    //normal
                    triangle1_normal.x,
                    triangle1_normal.y,
                    triangle1_normal.z,
                ]);
            }
        }
        println!("pos length: {}", position.len());
        println!("normal length: {}", normals.len());
        println!("uv length: {}", uvs.len());
        let indicies = (0..position.len()).map(|i| i as u32).collect();
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, position);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indicies)));
        return mesh;
        todo!()
    }
}

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
fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let water = Water::new();
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
        .insert(Player);
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &Water), With<Player>>,
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
