use super::{Solver, WATER_SCALE};
use crate::prelude::GuiState;
use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::PrimitiveTopology,
    },
};
/// appends cube to mesh
pub fn append_cube(mesh: &mut Mesh, translation: Vec3, box_scale: f32, height: f32) {
    let x: f32 = translation.x;
    let y: f32 = translation.y;
    let z: f32 = translation.z;
    let offset = mesh.count_vertices();
    if mesh.indices_mut().is_none() {
        let data: Vec<u32> = vec![];
        mesh.set_indices(Some(Indices::U32(vec![])));
    }
    let indices = mesh.indices_mut().unwrap();

    let indices = match indices {
        Indices::U32(v) => v,
        _ => panic!("invalid index width"),
    };

    let mut indices_push: Vec<u32> = vec![
        [0, 1, 2],
        [0, 2, 3],
        [5, 6, 4],
        [5, 7, 6],
        [8, 9, 10],
        [10, 9, 11],
        [12, 13, 15],
        [12, 15, 14],
        [16, 17, 19],
        [16, 19, 18],
        [20, 23, 21],
        [20, 22, 23],
    ]
    .iter()
    .flatten()
    .map(|i| i + offset as u32)
    .collect();

    indices.append(&mut indices_push);
    if mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).is_none() {
        let data: Vec<[f32; 3]> = vec![];
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, data);
    }
    let position = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
    let position = match position {
        VertexAttributeValues::Float3(v) => v,
        _ => panic!("invalid vertex type"),
    };
    // face 0

    position.push([
        0.5 * box_scale + x,
        -0.5 * box_scale + y,
        0.5 * box_scale + z,
    ]);
    position.push([
        0.5 * box_scale + x,
        -0.5 * box_scale + y,
        -0.5 * box_scale + z,
    ]);
    position.push([0.5 * box_scale + x, y * box_scale + y, -0.5 * box_scale + z]);
    position.push([0.5 * box_scale + x, y * box_scale + y, 0.5 * box_scale + z]);
    //face 1
    position.push([
        0.5 * box_scale + x,
        -0.5 * box_scale + y,
        -0.5 * box_scale + z,
    ]);
    position.push([
        -0.5 * box_scale + x,
        -0.5 * box_scale + y,
        -0.5 * box_scale + z,
    ]);
    position.push([0.5 * box_scale + x, y * box_scale + y, -0.5 * box_scale + z]);
    position.push([
        -0.5 * box_scale + x,
        height * box_scale + y,
        -0.5 * box_scale + z,
    ]);
    // face 2
    position.push([
        -0.5 * box_scale + x,
        -0.5 * box_scale + y,
        -0.5 * box_scale + z,
    ]);
    position.push([
        -0.5 * box_scale + x,
        -0.5 * box_scale + y,
        0.5 * box_scale + z,
    ]);
    position.push([
        -0.5 * box_scale + x,
        height * box_scale + y,
        -0.5 * box_scale + z,
    ]);
    position.push([
        -0.5 * box_scale + x,
        height * box_scale + y,
        0.5 * box_scale + z,
    ]);
    // face 3
    position.push([
        -0.5 * box_scale + x,
        -0.5 * box_scale + y,
        0.5 * box_scale + z,
    ]);
    position.push([
        0.5 * box_scale + x,
        -0.5 * box_scale + y,
        0.5 * box_scale + z,
    ]);
    position.push([
        -0.5 * box_scale + x,
        height * box_scale + y,
        0.5 * box_scale + z,
    ]);
    position.push([
        0.5 * box_scale + x,
        height * box_scale + y,
        0.5 * box_scale + z,
    ]);
    // face 4
    position.push([
        0.5 * box_scale + x,
        height * box_scale + y,
        0.5 * box_scale + z,
    ]);
    position.push([
        0.5 * box_scale + x,
        height * box_scale + y,
        -0.5 * box_scale + z,
    ]);
    position.push([
        -0.5 * box_scale + x,
        height * box_scale + y,
        0.5 * box_scale + z,
    ]);
    position.push([
        -0.5 * box_scale + x,
        height * box_scale + y,
        -0.5 * box_scale + z,
    ]);
    // face 5
    position.push([
        0.5 * box_scale + x,
        -0.5 * box_scale + y,
        0.5 * box_scale + z,
    ]);
    position.push([
        0.5 * box_scale + x,
        -0.5 * box_scale + y,
        -0.5 * box_scale + z,
    ]);
    position.push([
        -0.5 * box_scale + x,
        -0.5 * box_scale + y,
        0.5 * box_scale + z,
    ]);
    position.push([
        -0.5 * box_scale + x,
        -0.5 * box_scale + y,
        -0.5 * box_scale + z,
    ]);

    if mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_none() {
        let data: Vec<[f32; 2]> = vec![];
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, data);
    }
    let uv = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
    let uv = match uv {
        VertexAttributeValues::Float2(v) => v,
        _ => panic!("invalid vertex type"),
    };
    //face 0
    uv.push([2.0 / 6.0, 0.0]);
    uv.push([2.0 / 6.0, 1.0]);
    uv.push([1.0 / 6.0, 1.0]);
    uv.push([1.0 / 6.0, 0.0]);
    // face 1
    uv.push([3.0 / 6.0, 0.0]);
    uv.push([3.0 / 6.0, 1.0]);
    uv.push([2.0 / 6.0, 0.0]);
    uv.push([2.0 / 6.0, 1.0]);
    // face 2
    uv.push([4.0 / 6.0, 0.0]);
    uv.push([4.0 / 6.0, 1.0]);
    uv.push([3.0 / 6.0, 0.0]);
    uv.push([3.0 / 6.0, 1.0]);
    // face 3
    uv.push([5.0 / 6.0, 0.0]);
    uv.push([5.0 / 6.0, 1.0]);
    uv.push([4.0 / 6.0, 0.0]);
    uv.push([4.0 / 6.0, 1.0]);
    // face 4
    uv.push([0.0, 0.0]);
    uv.push([1.0 / 6.0, 0.0]);
    uv.push([0.0, 1.0]);
    uv.push([1.0 / 6.0, 1.0]);
    // face 5
    uv.push([5.0 / 6.0, 0.0]);
    uv.push([5.0 / 6.0, 1.0]);
    uv.push([1.0, 0.0]);
    uv.push([1.0, 1.0]);
    if mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_none() {
        let data: Vec<[f32; 3]> = vec![];
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, data);
    }

    let normal = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL).unwrap();
    let normal = match normal {
        VertexAttributeValues::Float3(v) => v,
        _ => panic!("invalid vertex type"),
    };

    //face 0
    normal.push([1.0, 0.0, 0.0]);
    normal.push([1.0, 0.0, 0.0]);
    normal.push([1.0, 0.0, 0.0]);
    normal.push([1.0, 0.0, 0.0]);

    //face 1
    normal.push([0.0, 1.0, 0.0]);
    normal.push([0.0, 1.0, 0.0]);
    normal.push([0.0, 1.0, 0.0]);
    normal.push([0.0, 1.0, 0.0]);
    //face 2
    normal.push([-1.0, 0.0, 0.0]);
    normal.push([-1.0, 0.0, 0.0]);
    normal.push([-1.0, 0.0, 0.0]);
    normal.push([-1.0, 0.0, 0.0]);
    //face 3

    normal.push([0.0, 0.0, 1.0]);
    normal.push([0.0, 0.0, 1.0]);
    normal.push([0.0, 0.0, 1.0]);
    normal.push([0.0, 0.0, 1.0]);
    //face 4
    normal.push([0.0, 1.0, 0.0]);
    normal.push([0.0, 1.0, 0.0]);
    normal.push([0.0, 1.0, 0.0]);
    normal.push([0.0, 1.0, 0.0]);
    // face 5
    normal.push([0.0, -1.0, 0.0]);
    normal.push([0.0, -1.0, 0.0]);
    normal.push([0.0, -1.0, 0.0]);
    normal.push([0.0, -1.0, 0.0]);
}
pub struct UShow;
pub struct VShow;
pub fn build_uv_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut cube = Mesh::new(PrimitiveTopology::TriangleList);
    let position: Vec<[f32; 3]> = vec![];
    let normal: Vec<[f32; 3]> = vec![];
    let uv: Vec<[f32; 2]> = vec![];
    cube.set_attribute(Mesh::ATTRIBUTE_POSITION, position);
    cube.set_attribute(Mesh::ATTRIBUTE_NORMAL, normal);
    cube.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);

    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: transform,
            mesh: meshes.add(cube.clone()),
            ..Default::default()
        })
        .insert(UShow);
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: transform,
            mesh: meshes.add(cube),
            ..Default::default()
        })
        .insert(VShow);
}
pub fn run_uv_cubes(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    gui_query: Query<&GuiState>,
    water_query: Query<&Box<dyn Solver>, ()>,
    mut queries: QuerySet<(
        Query<(&Handle<Mesh>, &mut Visible), With<UShow>>,
        Query<(&Handle<Mesh>, &mut Visible), With<VShow>>,
    )>,
) {
    const CUBE_SCALE: f32 = 0.04;
    const Y_SCALE: f32 = 1000.0;
    let u_cubes = queries.q0_mut();
    let gui_state = gui_query.iter().next();
    if gui_state.is_none() {
        error!("gui state not found");
        return;
    }
    let gui_state = gui_state.unwrap();
    let water = water_query.iter().next();
    if water.is_none() {
        error!("failed to find solver");
        return;
    }
    let water = water.unwrap();
    for (mut mesh, mut visible) in u_cubes.iter_mut() {
        if !gui_state.show_velocities {
            visible.is_visible = false;
            continue;
        } else {
            visible.is_visible = true;
        }
        let mut mesh = mesh_assets.get_mut(mesh).unwrap();
        *mesh = Mesh::new(PrimitiveTopology::TriangleList);
        //u
        let u = water.u();
        for x in 0..u.x() {
            for y in 0..u.y() {
                append_cube(
                    &mut mesh,
                    Vec3::new(x as f32 * WATER_SCALE, 1.0, (y as f32 + 0.5) * WATER_SCALE),
                    CUBE_SCALE,
                    u.get(x, y) * Y_SCALE,
                );
            }
        }
    }
    let v_cubes = queries.q1_mut();
    for (mut mesh, mut visible) in v_cubes.iter_mut() {
        if !gui_state.show_velocities {
            visible.is_visible = false;
            continue;
        } else {
            visible.is_visible = true;
        }
        let mut mesh = mesh_assets.get_mut(mesh).unwrap();
        *mesh = Mesh::new(PrimitiveTopology::TriangleList);
        //u
        let v = water.v();
        for x in 0..v.x() {
            for y in 0..v.y() {
                append_cube(
                    &mut mesh,
                    Vec3::new(
                        (x as f32 + 0.5) * WATER_SCALE,
                        1.0,
                        (y as f32) * WATER_SCALE,
                    ),
                    CUBE_SCALE,
                    v.get(x, y) * Y_SCALE,
                );
            }
        }
    }
}
