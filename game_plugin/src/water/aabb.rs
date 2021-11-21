use super::{FiniteSolver, HEIGHT_MULTIPLIER, WATER_SIZE};
use bevy::prelude::*;
use nalgebra::Vector2;
pub struct AABBMaterial {
    pub material: Handle<StandardMaterial>,
}
pub fn insert_aabb_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(Color::rgb(0.1, 0.1, 0.1).into());
    info!("inserting aabb");
    commands.insert_resource(AABBMaterial { material });
}
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
    pub fn from_transform(transform: Transform, water: &FiniteSolver) -> Self {
        let water_x = water.h().x();
        let scaling = water_x as f32 / WATER_SIZE;
        let lower = scaling * (transform.translation - 0.5 * transform.scale);
        let upper = scaling * (transform.translation + 0.5 * transform.scale);
        Self {
            bottom_left: Vector2::new(lower.x as i32, lower.z as i32),
            top_right: Vector2::new(upper.x as i32, upper.z as i32),
        }
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

    let scale_xz = aabb.top_right - aabb.bottom_left;
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
    material: &AABBMaterial,
    meshes: &mut ResMut<Assets<Mesh>>,
    mean_h: f32,
    water_dimensions: Vector2<usize>,
) {
    commands
        .spawn_bundle(build_cube_from_aabb(
            &aabb,
            material.material.clone(),
            meshes,
            mean_h,
            water_dimensions,
        ))
        .insert_bundle(bevy_mod_picking::PickableBundle::default())
        .insert(bevy_transform_gizmo::GizmoTransformable)
        .insert(aabb);
}
pub fn aabb_transform(
    water_query: Query<&FiniteSolver, With<FiniteSolver>>,
    mut box_query: Query<(&mut AABBBArrier, &Transform), Changed<Transform>>,
) {
    let water = if let Some(water) = water_query.iter().next() {
        water
    } else {
        return;
    };
    let water_x = water.h().x();
    let scaling = water_x as f32 / WATER_SIZE;
    for (mut aabb, transform) in box_query.iter_mut() {
        let lower = scaling * (transform.translation - 0.5 * transform.scale);
        let upper = scaling * (transform.translation + 0.5 * transform.scale);

        aabb.bottom_left = Vector2::new(lower.x as i32, lower.z as i32);
        aabb.top_right = Vector2::new(upper.x as i32, upper.z as i32);
    }
}
