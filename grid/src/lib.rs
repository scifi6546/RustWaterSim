mod vector;
use nalgebra::Vector2;
use std::{fs::File, io::Write};
pub use vector::Vector;
#[derive(Clone)]
pub struct Grid<T: Clone + Copy> {
    points: Vec<T>,
    x: usize,
    y: usize,
}
impl<T: Clone + Copy + Default + Vector> Grid<T> {
    pub fn debug_save<P: AsRef<std::path::Path>>(&self, save_path: P) {
        let data = self.numpy_data();
        let mut file = File::create(save_path).expect("failed to open file");
        file.write(&data);
    }
    pub fn from_vec(dimensions: Vector2<usize>, points: Vec<T>) -> Self {
        assert_eq!(dimensions.x * dimensions.y, points.len());
        Self {
            points,
            x: dimensions.x,
            y: dimensions.y,
        }
    }
    /// X dimensions
    pub fn x(&self) -> usize {
        self.x
    }
    /// Y dimensions
    pub fn y(&self) -> usize {
        self.y
    }
    ///
    pub fn get(&self, x: usize, y: usize) -> T {
        self.points[self.y * x + y]
    }
    /// gets value at index or gets other value
    pub fn get_or(&self, x: i32, y: i32, other_value: T) -> T {
        if x >= 0 && y >= 0 {
            let idx = self.get_idx(x as usize, y as usize);
            if idx < self.points.len() {
                self.points[idx]
            } else {
                other_value
            }
        } else {
            other_value
        }
    }
    /// gets idx of coords
    fn get_idx(&self, x: usize, y: usize) -> usize {
        self.y * x + y
    }
    /// gets mut unchecked
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.points[self.y * x + y]
    }
    /// gets points unchecked at point
    pub fn get_unchecked(&self, dim: Vector2<i64>) -> T {
        self.get(dim.x as usize, dim.y as usize)
    }
    /// gets unchecked mut
    pub fn get_mut_unchecked(&mut self, dim: Vector2<i64>) -> &mut T {
        self.get_mut(dim.x as usize, dim.y as usize)
    }
    /// builds grid from function
    pub fn from_fn<F: Fn(usize, usize) -> T>(f: F, dimensions: Vector2<usize>) -> Self {
        let mut s = Self::from_vec(dimensions, vec![T::default(); dimensions.x * dimensions.y]);
        for x in 0..dimensions.x {
            for y in 0..dimensions.y {
                *s.get_mut(x, y) = f(x, y);
            }
        }
        s
    }
    pub fn numpy_data(&self) -> Vec<u8> {
        let header_str = format!(
            "{{'descr': '<f4', 'fortran_order': False, 'shape': ({}, {}, {}), }}",
            self.x(),
            self.y(),
            T::DIM
        );
        let header_bytes = header_str.as_bytes();
        let header_len = header_bytes.len() as u16;
        let mut out_data = vec![
            0x93, 'N' as u8, 'U' as u8, 'M' as u8, 'P' as u8, 'Y' as u8, 0x01, 0x00,
        ];
        for byte in header_len.to_le_bytes().iter() {
            out_data.push(*byte);
        }
        for byte in header_bytes.iter() {
            out_data.push(*byte);
        }
        for x in 0..self.x() {
            for y in 0..self.y() {
                let h = self.get(x, y);
                for byte in h.to_le_bytes().iter() {
                    out_data.push(*byte);
                }
            }
        }

        return out_data;
    }
}
impl<T: std::ops::Add + std::ops::AddAssign + Clone + Copy> std::ops::Add for Grid<T> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        assert_eq!(self.x, other.x);
        assert_eq!(self.y, other.y);
        for i in 0..self.points.len() {
            self.points[i] += other.points[i];
        }
        Self {
            points: self.points,
            x: self.x,
            y: self.y,
        }
    }
}