use bevy::{
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
const SCALE: f32 = 0.5;
pub fn build_uv_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let x: f32 = 1.0;
    let y: f32 = 1.0;
    let z: f32 = -1.0;
    let mut cube = Mesh::new(PrimitiveTopology::TriangleList);
    let mut position: Vec<[f32; 3]> = vec![];
    let mut normal: Vec<[f32; 3]> = vec![];
    let mut uv: Vec<[f32; 2]> = vec![];

    //face 0

    //vertex 0
    position.push([0.5 * SCALE + x, -0.5 * SCALE + y, 0.5 * SCALE + z]);
    uv.push([2.0 / 6.0, 0.0]);
    normal.push([1.0, 0.0, 0.0]);

    //vertex 1
    position.push([0.5 * SCALE + x, -0.5 * SCALE + y, -0.5 * SCALE + z]);
    uv.push([2.0 / 6.0, 1.0]);
    normal.push([1.0, 0.0, 0.0]);
    // vertex 2
    position.push([0.5 * SCALE + x, 0.5 * SCALE + y, -0.5 * SCALE + z]);
    uv.push([1.0 / 6.0, 1.0]);
    normal.push([1.0, 0.0, 0.0]);
    //vertex 3
    position.push([0.5 * SCALE + x, 0.5 * SCALE + y, 0.5 * SCALE + z]);
    uv.push([1.0 / 6.0, 0.0]);
    normal.push([1.0, 0.0, 0.0]);

    //face 1
    //vertex 4
    position.push([0.5 * SCALE, -0.5 * SCALE, -0.5 * SCALE]);
    uv.push([3.0 / 6.0, 0.0]);
    normal.push([0.0, 1.0, 0.0]);
    // vertex 5
    position.push([-0.5 * SCALE, -0.5 * SCALE, -0.5 * SCALE]);
    uv.push([3.0 / 6.0, 1.0]);
    normal.push([0.0, 1.0, 0.0]);
    //vertex 6
    position.push([0.5 * SCALE, 0.5 * SCALE, -0.5 * SCALE]);
    uv.push([2.0 / 6.0, 0.0]);
    normal.push([0.0, 1.0, 0.0]);
    //vertex 7
    position.push([-0.5 * SCALE, 0.5 * SCALE, -0.5 * SCALE]);
    uv.push([2.0 / 6.0, 1.0]);
    normal.push([0.0, 1.0, 0.0]);
    //face 2
    //vertex 8

    position.push([-0.5 * SCALE, -0.5 * SCALE, -0.5 * SCALE]);
    uv.push([4.0 / 6.0, 0.0]);
    normal.push([-1.0, 0.0, 0.0]);
    //vertex 9
    position.push([-0.5 * SCALE, -0.5 * SCALE, 0.5 * SCALE]);
    uv.push([4.0 / 6.0, 1.0]);
    normal.push([-1.0, 0.0, 0.0]);
    // vertex 10

    position.push([-0.5 * SCALE, 0.5 * SCALE, -0.5 * SCALE]);
    uv.push([3.0 / 6.0, 0.0]);
    normal.push([-1.0, 0.0, 0.0]);
    //vertex 11
    position.push([-0.5 * SCALE, 0.5 * SCALE, 0.5 * SCALE]);
    uv.push([3.0 / 6.0, 1.0]);
    normal.push([-1.0, 0.0, 0.0]);
    //face 3
    //vertex 12

    position.push([-0.5 * SCALE, -0.5 * SCALE, 0.5 * SCALE]);
    uv.push([5.0 / 6.0, 0.0]);
    normal.push([0.0, 0.0, 1.0]);
    // vertex 13

    position.push([0.5 * SCALE, -0.5 * SCALE, 0.5 * SCALE]);
    uv.push([5.0 / 6.0, 1.0]);
    normal.push([0.0, 0.0, 1.0]);
    // vertex 14
    position.push([-0.5 * SCALE, 0.5 * SCALE, 0.5 * SCALE]);
    uv.push([4.0 / 6.0, 0.0]);
    normal.push([0.0, 0.0, 1.0]);
    //vertex 15

    position.push([0.5 * SCALE, 0.5 * SCALE, 0.5 * SCALE]);
    uv.push([4.0 / 6.0, 1.0]);
    normal.push([0.0, 0.0, 1.0]);
    //face 4
    // vertex 16

    position.push([0.5 * SCALE, 0.5 * SCALE, 0.5 * SCALE]);
    uv.push([0.0, 0.0]);
    normal.push([0.0, 1.0, 0.0]);
    // vertex 17
    position.push([0.5 * SCALE, 0.5 * SCALE, -0.5 * SCALE]);
    uv.push([1.0 / 6.0, 0.0]);
    normal.push([0.0, 1.0, 0.0]);
    // vertex 18

    position.push([-0.5 * SCALE, 0.5 * SCALE, 0.5 * SCALE]);
    uv.push([0.0, 1.0]);
    normal.push([0.0, 1.0, 0.0]);
    //vertex 19
    position.push([-0.5 * SCALE, 0.5 * SCALE, -0.5 * SCALE]);
    uv.push([1.0 / 6.0, 1.0]);
    normal.push([0.0, 1.0, 0.0]);
    // face 5
    //vertex 20

    position.push([0.5 * SCALE, -0.5 * SCALE, 0.5 * SCALE]);
    uv.push([5.0 / 6.0, 0.0]);
    normal.push([0.0, -1.0, 0.0]);
    //vertex 21
    position.push([0.5 * SCALE, -0.5 * SCALE, -0.5 * SCALE]);
    uv.push([5.0 / 6.0, 1.0]);
    normal.push([0.0, -1.0, 0.0]);
    //vertex 22

    position.push([-0.5 * SCALE, -0.5 * SCALE, 0.5 * SCALE]);
    uv.push([1.0, 0.0]);
    normal.push([0.0, -1.0, 0.0]);
    //vertex 23
    position.push([-0.5 * SCALE, -0.5 * SCALE, -0.5 * SCALE]);
    uv.push([1.0, 1.0]);
    normal.push([0.0, -1.0, 0.0]);

    let indices: Vec<u32> = vec![
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
    .copied()
    .collect();
    cube.set_attribute(Mesh::ATTRIBUTE_POSITION, position);
    cube.set_attribute(Mesh::ATTRIBUTE_NORMAL, normal);
    cube.set_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    cube.set_indices(Some(Indices::U32(indices)));
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    commands.spawn_bundle(PbrBundle {
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: transform,
        mesh: meshes.add(cube),
        ..Default::default()
    });
}
