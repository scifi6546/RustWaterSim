use super::{HEIGHT_MULTIPLIER, WATER_SIZE};
use bevy::prelude::*;
use nalgebra::Vector2;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AABBBArrier {
    pub top_right: Vector2<i32>,
    pub bottom_left: Vector2<i32>,
}
impl AABBBArrier {
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        self.top_right.x >= x
            && self.top_right.y >= y
            && self.bottom_left.x <= x
            && self.bottom_left.y <= y
    }
}
fn build_cube_from_aabb(
    aabb: &AABBBArrier,
    material: Handle<StandardMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    y: f32,
    water_dimensions: Vector2<usize>,
) -> PbrBundle {
    let scaling = WATER_SIZE / water_dimensions.x as f32;
    let mesh = meshes.add(shape::Cube::new(1.0).into());
    let center_x = scaling * (aabb.top_right.x + aabb.bottom_left.x) as f32 / 2.0;
    let center_z = scaling * (aabb.top_right.y + aabb.bottom_left.y) as f32 / 2.0;

    let mut transform = Transform::from_translation(Vec3::new(
        center_x,
        y * HEIGHT_MULTIPLIER * scaling,
        center_z,
    ));

    let scale_xz = (aabb.top_right - aabb.bottom_left);
    let scale_xz = scaling * Vector2::new(scale_xz.x as f32, scale_xz.y as f32);
    transform.scale = Vec3::new(scale_xz.x, 2.0, scale_xz.y);

    PbrBundle {
        mesh,
        material,
        transform,
        ..Default::default()
    }
}
pub fn build_barrier(
    commands: &mut Commands,
    aabb: AABBBArrier,
    material: Handle<StandardMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    mean_h: f32,
    water_dimensions: Vector2<usize>,
) {
    commands
        .spawn_bundle(build_cube_from_aabb(
            &aabb,
            material,
            meshes,
            mean_h,
            water_dimensions,
        ))
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable)
        .insert(aabb);
}
