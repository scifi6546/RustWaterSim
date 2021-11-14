use super::AABBBArrier;
use bevy::prelude::*;
pub fn build_cube_from_aabb(aabb: &AABBBArrier) -> Mesh {
    let shape_box = shape::Box::new(1.0, 1.0, 1.0);
    shape_box.into()
}
