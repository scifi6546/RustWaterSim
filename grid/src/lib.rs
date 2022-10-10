mod vector;
use nalgebra::Vector2;

use std::{
    collections::HashMap,
    fs::File,
    io::{Error as IoError, Read, Write},
    path::Path,
    str::{from_utf8, Utf8Error},
};

use thiserror::Error;
pub use vector::Vector;
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IoError")]
    IoError(#[from] IoError),
    #[error("Failed to parse")]
    Utf8Error(#[from] Utf8Error),
    #[error("Invalid Header")]
    InvalidHeader,
    #[error("Invalid Header Json")]
    InvalidHeaderJson,
    #[error("Unsupported Datatype: {0}")]
    UnsupportedDatatype(String),
}
#[derive(Clone)]
pub struct Grid<T: Clone + Copy> {
    points: Vec<T>,
    x: usize,
    y: usize,
}
impl<T: Clone + Copy + Default + Vector> Grid<T> {
    pub fn save_several_layers<P: AsRef<Path>>(
        save_path: P,
        grid_layers: &[&Grid<T>],
    ) -> std::io::Result<()> {
        let mut data = Self::make_header([
            grid_layers.len() as u32,
            grid_layers[0].x() as u32,
            grid_layers[0].y() as u32,
            T::DIM as u32,
        ]);
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
        let mut file = File::create(save_path)?;
        let s = file.write(&data)?;
        if s != data.len() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to write entire file",
            ))
        } else {
            Ok(())
        }
    }
    pub fn load_layers<P: AsRef<Path>>(path: P) -> Result<Vec<Self>, ParseError> {
        let mut f = File::open(path)?;
        Self::load_layers_reader(&mut f)
    }
    /// loads several layers saved as numpy array
    pub fn load_layers_reader<R: Read>(reader: &mut R) -> Result<Vec<Self>, ParseError> {
        fn de_quote(s: &str) -> Result<&str, ParseError> {
            let s = s.trim();
            let start_char = s.chars().next();
            if start_char.is_none() {
                return Err(ParseError::InvalidHeaderJson);
            }
            Ok(s.trim_matches(|c| c == '"' || c == '\''))
        }
        fn parse_tuple(tuple_str: &str) -> Result<Vec<u32>, ParseError> {
            let tuple_str = tuple_str.trim();
            println!("tuple to parse: \"{}\"", tuple_str);
            let mut is_first_char = true;
            let mut current_num = 0u32;
            let mut out_vec = Vec::new();
            let mut in_beginning_of_num = false;
            for c in tuple_str.chars() {
                if is_first_char {
                    if c != '(' {
                        return Err(ParseError::InvalidHeaderJson);
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
                            return Err(ParseError::InvalidHeaderJson);
                        }
                    } else if c == ')' {
                        if in_beginning_of_num {
                            return Err(ParseError::InvalidHeaderJson);
                        }
                        out_vec.push(current_num);
                        return Ok(out_vec);
                    } else {
                        return Err(ParseError::InvalidHeaderJson);
                    }
                }
                is_first_char = false;
            }
            return Err(ParseError::InvalidHeaderJson);
        }

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let numpy_str = from_utf8(&buffer[1..6])?;
        if numpy_str != "NUMPY" {
            return Err(ParseError::InvalidHeader);
        }
        if buffer[10] != '{' as u8 {
            return Err(ParseError::InvalidHeader);
        }
        let header_start_offset = 11;
        let header_end_idx = buffer[11..]
            .iter()
            .enumerate()
            .filter(|(_idx, byte)| **byte == '}' as u8)
            .map(|(idx, _byte)| idx + header_start_offset)
            .next();
        if header_end_idx.is_none() {
            return Err(ParseError::InvalidHeader);
        }
        let header_end_idx = header_end_idx.unwrap();
        println!("end idx: {:?}", header_end_idx);
        let dim_str = from_utf8(&buffer[10..header_end_idx + 1])?;

        println!("start\n{}", dim_str);
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
                        return Err(ParseError::InvalidHeaderJson);
                    }
                } else if c == ':' && !in_str {
                    if in_paren {
                        return Err(ParseError::InvalidHeaderJson);
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
                        println!("start paren!!");
                        println!(
                            "in_name: {}, in_data: {},data: \"{}\"",
                            in_name, in_data, data
                        );

                        return Err(ParseError::InvalidHeaderJson);
                    }
                    in_paren = true;
                    data.push(c);
                } else if c == ')' {
                    if in_name || !in_data {
                        println!("end paren!!");
                        return Err(ParseError::InvalidHeaderJson);
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
        for (k, v) in items.iter() {
            println!("({},{})", k, v);
        }
        let shape_tuple_str = items.get("shape");
        if shape_tuple_str.is_none() {
            return Err(ParseError::InvalidHeaderJson);
        }
        let shape_tuple_str = shape_tuple_str.unwrap();

        let shape_tuple = parse_tuple(&shape_tuple_str)?;
        if !(shape_tuple.len() == 3 || shape_tuple.len() == 4) {
            return Err(ParseError::InvalidHeaderJson);
        }
        println!("shape: {:#?}", shape_tuple);
        let format_str = items.get("descr");
        if format_str.is_none() {
            return Err(ParseError::InvalidHeaderJson);
        }
        let format_str = format_str.unwrap();
        println!("format: {}", format_str);
        if de_quote(format_str)? != "<f4" {
            return Err(ParseError::UnsupportedDatatype(format_str.clone()));
        }
        let size = std::mem::size_of::<f32>() as u32 * shape_tuple.iter().fold(1, |acc, x| acc * x);
        let avl_bytes = buffer.len() - header_end_idx;
        let copy_buffer = buffer[header_end_idx + 1..buffer.len()]
            .iter()
            .map(|_| 1)
            .fold(0, |acc, x| acc + x);

        println!(
            "predicted size: {}, available bytes: {}, copy buffer size: {}",
            size, avl_bytes, copy_buffer
        );

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
            return Err(ParseError::InvalidHeaderJson);
        };
        if num_channels != T::DIM {
            return Err(ParseError::InvalidHeaderJson);
        }
        println!(
            "size_x: {} size_y: {}, num_channels: {}",
            size_x, size_y, num_channels
        );
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
        for (idx, c) in dim_str.chars().enumerate() {
            println!("{}:\t{}", idx + 10, c);
        }
        Ok(grids)
    }
    pub fn debug_save<P: AsRef<Path>>(&self, save_path: P) -> Result<(), ParseError> {
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
    }
}
