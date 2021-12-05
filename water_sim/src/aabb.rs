use nalgebra::Vector2;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AABBBarrier {
    pub top_right: Vector2<i32>,
    pub bottom_left: Vector2<i32>,
}
impl AABBBarrier {
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        self.top_right.x >= x
            && self.top_right.y >= y
            && self.bottom_left.x <= x
            && self.bottom_left.y <= y
    }
}
