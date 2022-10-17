mod debug_buffer;
mod vector;

use nalgebra::Vector2;

use std::{
    collections::HashMap,
    fs::File,
    io::{Error as IoError, Read, Seek, Write},
    path::Path,
    rc::Rc,
    str::{from_utf8, Utf8Error},
};

pub use debug_buffer::DebugBuffer;
use thiserror::Error;
pub use vector::Vector;
#[derive(Error, Debug, Clone)]
pub enum BoundsCheckError {
    #[error("Invalid Index")]
    InvalidIndex {
        invalid_x: i32,
        invalid_y: i32,
        size_x: i32,
        size_y: i32,
    },
}
#[derive(Error, Debug, Clone)]
pub enum FileError {
    #[error("IoError")]
    IoError(Rc<IoError>),
    #[error("Failed to parse")]
    Utf8Error(#[from] Utf8Error),
    #[error("Invalid Header")]
    InvalidHeader,
    #[error("Invalid Header Json")]
    InvalidHeaderJson,
    #[error("Unsupported Datatype: {0}")]
    UnsupportedDatatype(String),
}
impl From<std::io::Error> for FileError {
    fn from(e: IoError) -> Self {
        Self::IoError(Rc::new(e))
    }
}
use zip::{write::FileOptions as WriterFileOptions, ZipWriter};
#[derive(Clone)]
pub struct Grid<T: Clone + Copy> {
    points: Vec<T>,
    x: usize,
    y: usize,
}
impl<T: Clone + Copy + Default + Vector> Grid<T> {
    pub fn save_several_layers<P: AsRef<Path>>(
        path: P,
        grid_layers: &[&Grid<T>],
    ) -> Result<(), FileError> {
        let mut file = File::create(path)?;
        let _ = Self::save_several_layers_writer(&mut file, grid_layers)?;
        Ok(())
    }
    pub fn get_checked(&self, x: i32, y: i32) -> Result<T, BoundsCheckError> {
        if x < 0 || y < 0 || x >= self.x() as i32 || y >= self.y as i32 {
            return Err(BoundsCheckError::InvalidIndex {
                invalid_x: x,
                invalid_y: y,
                size_x: self.x() as i32,
                size_y: self.y() as i32,
            });
        } else {
            Ok(self.get(x as usize, y as usize))
        }
    }
    pub fn save_several_layers_writer<W: Write + Seek>(
        writer: &mut W,
        grid_layers: &[&Grid<T>],
    ) -> std::io::Result<()> {
        let (shape_x, shape_y) = match grid_layers.len() {
            0 => (0, 0),
            _ => (grid_layers[0].x() as u32, grid_layers[0].y() as u32),
        };
        let mut data =
            Self::make_header([grid_layers.len() as u32, shape_x, shape_y, T::DIM as u32]);
        for layer in grid_layers {
            for x in 0..layer.x() {
                for y in 0..layer.y() {
                    let item = layer.get(x, y);
                    for byte in item.to_le_bytes().iter() {
                        data.push(*byte);
                    }
                }
            }
        }
        let mut zip_writer = ZipWriter::new(writer);
        let options =
            WriterFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        zip_writer
            .start_file("0", options)
            .expect("failed to start file");
        zip_writer.write(&data).expect("failed to write");
        zip_writer.finish().expect("failed to finish zip");

        Ok(())
    }
    pub fn load_layers<P: AsRef<Path>>(path: P) -> Result<Vec<Self>, FileError> {
        let mut f = File::open(path)?;
        Self::load_layers_reader(&mut f)
    }
    /// loads several layers saved as numpy array
    pub fn load_layers_reader<R: Read>(reader: &mut R) -> Result<Vec<Self>, FileError> {
        fn de_quote(s: &str) -> Result<&str, FileError> {
            let s = s.trim();
            let start_char = s.chars().next();
            if start_char.is_none() {
                return Err(FileError::InvalidHeaderJson);
            }
            Ok(s.trim_matches(|c| c == '"' || c == '\''))
        }
        fn parse_tuple(tuple_str: &str) -> Result<Vec<u32>, FileError> {
            let tuple_str = tuple_str.trim();
            let mut is_first_char = true;
            let mut current_num = 0u32;
            let mut out_vec = Vec::new();
            let mut in_beginning_of_num = false;
            for c in tuple_str.chars() {
                if is_first_char {
                    if c != '(' {
                        return Err(FileError::InvalidHeaderJson);
                    }
                    in_beginning_of_num = true;
                } else {
                    if c == ',' {
                        out_vec.push(current_num);
                        current_num = 0;
                        in_beginning_of_num = true;
                    } else if c.is_numeric() {
                        let num = match c {
                            '0' => 0,
                            '1' => 1,
                            '2' => 2,
                            '3' => 3,
                            '4' => 4,
                            '5' => 5,
                            '6' => 6,
                            '7' => 7,
                            '8' => 8,
                            '9' => 9,
                            _ => panic!(),
                        };
                        current_num = current_num * 10 + num;
                        in_beginning_of_num = false;
                    } else if c.is_whitespace() {
                        if !in_beginning_of_num {
                            return Err(FileError::InvalidHeaderJson);
                        }
                    } else if c == ')' {
                        if in_beginning_of_num {
                            return Err(FileError::InvalidHeaderJson);
                        }
                        out_vec.push(current_num);
                        return Ok(out_vec);
                    } else {
                        return Err(FileError::InvalidHeaderJson);
                    }
                }
                is_first_char = false;
            }
            return Err(FileError::InvalidHeaderJson);
        }

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let numpy_str = from_utf8(&buffer[1..6])?;
        if numpy_str != "NUMPY" {
            return Err(FileError::InvalidHeader);
        }
        if buffer[10] != '{' as u8 {
            return Err(FileError::InvalidHeader);
        }
        let header_start_offset = 11;
        let header_end_idx = buffer[11..]
            .iter()
            .enumerate()
            .filter(|(_idx, byte)| **byte == '}' as u8)
            .map(|(idx, _byte)| idx + header_start_offset)
            .next();
        if header_end_idx.is_none() {
            return Err(FileError::InvalidHeader);
        }
        let header_end_idx = header_end_idx.unwrap();

        let dim_str = from_utf8(&buffer[10..header_end_idx + 1])?;

        let mut items: HashMap<String, String> = HashMap::new();
        {
            let mut in_str = false;
            let mut name: String = String::new();

            let mut data = String::new();
            let mut in_name = true;
            let mut in_data = false;
            let mut in_paren = false;
            for c in dim_str.chars().skip(1) {
                if c == '\"' || c == '\'' {
                    in_str = !in_str;
                    if in_name {
                        name.push(c);
                    }
                    if in_data {
                        data.push(c);
                    }
                    if !in_name && !in_data {
                        return Err(FileError::InvalidHeaderJson);
                    }
                } else if c == ':' && !in_str {
                    if in_paren {
                        return Err(FileError::InvalidHeaderJson);
                    }
                    in_name = false;
                    in_data = true;
                    data.clear();
                } else if c == ',' && !in_str && !in_paren {
                    in_name = true;
                    in_data = false;

                    name = de_quote(&name)?.to_string();
                    items.insert(name.to_string(), data.trim().to_string());
                    name.clear();
                    data.clear();
                } else if c == '(' {
                    if in_name || !in_data || data.trim() != "" {
                        return Err(FileError::InvalidHeaderJson);
                    }
                    in_paren = true;
                    data.push(c);
                } else if c == ')' {
                    if in_name || !in_data {
                        return Err(FileError::InvalidHeaderJson);
                    }
                    in_paren = false;
                    data.push(c);
                } else {
                    if in_name {
                        name.push(c);
                    }
                    if in_data {
                        data.push(c);
                    }
                }
            }
        }

        let shape_tuple_str = items.get("shape");
        if shape_tuple_str.is_none() {
            return Err(FileError::InvalidHeaderJson);
        }
        let shape_tuple_str = shape_tuple_str.unwrap();

        let shape_tuple = parse_tuple(&shape_tuple_str)?;
        if !(shape_tuple.len() == 3 || shape_tuple.len() == 4) {
            return Err(FileError::InvalidHeaderJson);
        }

        let format_str = items.get("descr");
        if format_str.is_none() {
            return Err(FileError::InvalidHeaderJson);
        }
        let format_str = format_str.unwrap();

        if de_quote(format_str)? != "<f4" {
            return Err(FileError::UnsupportedDatatype(format_str.clone()));
        }
        let size = std::mem::size_of::<f32>() as u32 * shape_tuple.iter().fold(1, |acc, x| acc * x);
        let avl_bytes = buffer.len() - header_end_idx;
        let copy_buffer = buffer[header_end_idx + 1..buffer.len()]
            .iter()
            .map(|_| 1)
            .fold(0, |acc, x| acc + x);

        let copy_buffer = &buffer[header_end_idx + 1..buffer.len()];

        let (num_layers, size_x, size_y, num_channels) = if shape_tuple.len() == 4 {
            (
                shape_tuple[0] as usize,
                shape_tuple[1] as usize,
                shape_tuple[2] as usize,
                shape_tuple[3] as usize,
            )
        } else if shape_tuple.len() == 3 {
            (
                1,
                shape_tuple[0] as usize,
                shape_tuple[1] as usize,
                shape_tuple[2] as usize,
            )
        } else {
            return Err(FileError::InvalidHeaderJson);
        };
        if num_channels != T::DIM {
            return Err(FileError::InvalidHeaderJson);
        }

        let mut grids: Vec<Grid<T>> = Vec::new();
        grids.reserve(num_layers as usize);
        for layer_num in 0..num_layers {
            let mut data_vec: Vec<T> = Vec::new();
            data_vec.reserve(size_x as usize * size_y as usize);
            for x in 0..size_x {
                for y in 0..size_y {
                    let idx = (num_channels
                        * std::mem::size_of::<f32>()
                        * (y + x * size_y + layer_num * size_x * size_y))
                        as usize;
                    let buff = &copy_buffer[idx..(idx + num_channels * std::mem::size_of::<f32>())];
                    let data = T::from_le_bytes(buff);
                    data_vec.push(data);
                }
            }
            grids.push(Grid::from_vec(
                Vector2::new(size_x as usize, size_y as usize),
                data_vec,
            ));
        }

        Ok(grids)
    }
    pub fn debug_save<P: AsRef<Path>>(&self, save_path: P) -> Result<(), FileError> {
        let data = self.numpy_data();
        let mut file = File::create(save_path).expect("failed to open file");
        file.write(&data)?;
        Ok(())
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
    fn make_header<const SIZE: usize>(shape: [u32; SIZE]) -> Vec<u8> {
        let mut shape_str = String::new();
        for i in 0..SIZE - 1 {
            shape_str += &format!("{}, ", shape[i]);
        }
        shape_str += &format!("{}", shape[SIZE - 1]);
        let header_str = format!(
            "{{'descr': '<f4', 'fortran_order': False, 'shape': ({}), }}",
            shape_str
        );
        let mut out_data = vec![
            0x93, 'N' as u8, 'U' as u8, 'M' as u8, 'P' as u8, 'Y' as u8, 0x01, 0x00,
        ];
        let header_bytes = header_str.as_bytes();
        let header_len = header_bytes.len() as u16;
        for byte in header_len.to_le_bytes().iter() {
            out_data.push(*byte);
        }
        for byte in header_bytes.iter() {
            out_data.push(*byte);
        }
        out_data
    }
    pub fn numpy_data(&self) -> Vec<u8> {
        let mut out_data = Self::make_header([self.x() as u32, self.y() as u32, T::DIM as u32]);
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
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn save_and_load() {
        let dimensions = Vector2::new(10, 10);
        let g = Grid::from_fn(|_, _| 0.0, dimensions);
        Grid::save_several_layers("test", &[&g]).expect("failed to save");
        let g_out: Vec<Grid<f32>> = Grid::load_layers("test").expect("failed to load");
        assert_eq!(g_out.len(), 1);
        let g_out_x = g_out[0].x();
        let g_out_y = g_out[0].y();
        assert_eq!(g_out_x, dimensions.x);
        assert_eq!(g_out_y, dimensions.y);
        std::fs::remove_file("test").expect("failed to delete");
    }
    #[test]
    fn test_big_grid() {
        let dimensions = Vector2::new(100, 200);
        let g = Grid::from_fn(|x, y| (x as f32).powi(2) + y as f32, dimensions);
        let g_layers: Vec<Grid<f32>> = (0..3)
            .map(|l| Grid::from_fn(|x, y| (x as f32).powi(2) + y as f32 + l as f32, dimensions))
            .collect();
        let g_layers_ref: Vec<&Grid<f32>> = g_layers.iter().map(|g| g).collect();
        let mut write: Vec<u8> = Vec::new();
        Grid::save_several_layers_writer(&mut std::io::Cursor::new(&mut write), &g_layers_ref)
            .expect("failed to write");
        let g_load_arr: Vec<Grid<f32>> =
            Grid::load_layers_reader(&mut std::io::Cursor::new(&write)).expect("failed to load");
        assert_eq!(g_layers.len(), g_load_arr.len());
        for i in 0..g_layers.len() {
            assert_eq!(g_layers[i].x(), g_load_arr[i].x());
            assert_eq!(g_layers[i].y(), g_load_arr[i].y());
            let dim_x = g_layers[i].x();
            let dim_y = g_layers[i].y();
            for x in 0..dim_x {
                for y in 0..dim_y {
                    let diff = g_layers[i].get(x, y) - g_load_arr[i].get(x, y);
                    assert!(diff.abs() < 0.01);
                }
            }
        }
        assert_eq!(g_load_arr.len(), 3)
    }
}
