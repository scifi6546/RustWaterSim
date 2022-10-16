use super::{FileError, Grid, Vector};
use std::path::Path;
pub struct DebugBuffer<T: Copy + Clone + Vector + Default> {
    items: Vec<Grid<T>>,
    buffer_size: u32,
    current_idx: u32,
}
impl<T: Copy + Clone + Vector + Default> DebugBuffer<T> {
    pub fn new(buffer_size: u32) -> Self {
        Self {
            items: vec![],
            buffer_size,
            current_idx: 0,
        }
    }
    pub fn push(&mut self, grid: Grid<T>) {
        if (self.items.len() as u32) < self.buffer_size {
            self.items.push(grid);
        } else if (self.items.len() as u32) == self.buffer_size {
            let new_idx = (self.current_idx as u32 + 1) % self.buffer_size;
            self.items[new_idx as usize] = grid;
            self.current_idx = new_idx;
        } else {
            panic!(
                "invalid state, buffer length: {} is greater then max buffer size: {}",
                self.items.len(),
                self.buffer_size
            );
        }
    }
    pub fn save<P: AsRef<Path>>(&self, save_path: P) -> Result<(), FileError> {
        let mut save_vec = vec![&self.items[self.current_idx as usize]];
        let mut idx = (self.current_idx + 1) % self.buffer_size;
        loop {
            if idx == self.current_idx || idx >= self.items.len() as u32 {
                break;
            }

            save_vec.push(&self.items[idx as usize]);
            idx = (idx + 1) % self.buffer_size;
        }
        Grid::save_several_layers(save_path, &save_vec)
    }
}
