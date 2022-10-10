use grid::Grid;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use std::fmt::write;
use std::{fs::File, path::Path};

#[derive(Debug, Clone)]
pub enum StackError {
    InvalidDimensions {
        old_x: u32,
        new_x: u32,
        old_y: u32,
        new_y: u32,
    },
}
impl std::fmt::Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDimensions {
                old_x,
                new_x,
                old_y,
                new_y,
            } => write!(
                f,
                "invalid dimensions requested dimensions: ({}, {}) correct dimensions: ({}, {})",
                old_x, old_y, new_x, new_y
            ),
        }
    }
}
impl From<StackError> for PyErr {
    fn from(err: StackError) -> Self {
        PyIndexError::new_err(format!("{}", err))
    }
}
#[pyclass(name = "GridStack")]
struct PyGridLayers {
    layers: Vec<Grid<f32>>,
}
#[pymethods]
impl PyGridLayers {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self { layers: Vec::new() })
    }
    fn push_grid(&mut self, grid: PyGrid) -> Result<(), StackError> {
        let layers_len = self.layers.len();
        if layers_len > 0 {
            let size_x = self.layers[0].x();
            let size_y = self.layers[0].y();
            if grid.grid.x() != size_x || grid.grid.y() != size_y {
                return Err(StackError::InvalidDimensions {
                    old_x: size_x as u32,
                    old_y: size_y as u32,
                    new_x: grid.grid.x() as u32,
                    new_y: grid.grid.y() as u32,
                });
            }
            self.layers.push(grid.grid)
        }
        Ok(())
    }
}
#[pyfunction]
fn load_stack(p: &str) -> PyResult<PyGridLayers> {
    let layers = Grid::load_layers(Path::new(p))?;
    Ok(PyGridLayers { layers })
}
#[derive(Clone)]
#[pyclass(name = "Grid")]
struct PyGrid {
    grid: Grid<f32>,
}
fn py_dict_to_i32x2(py: Python<'_>, tuple: Py<PyTuple>) -> PyResult<(i32, i32)> {
    let tuple_ref = tuple.as_ref(py);
    let len = tuple_ref.len();
    if len != 2 {
        return Err(PyIndexError::new_err("invalid tuple length"));
    }
    let x0 = tuple_ref.get_item(0)?.extract()?;
    let x1 = tuple_ref.get_item(1)?.extract()?;
    Ok((x0, x1))
}
impl PyGrid {
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x < self.grid.x() as i32 && y < self.grid.y() as i32 && x >= 0 && y >= 0
    }
}
#[pymethods]
impl PyGrid {
    #[new]
    fn new(x: usize, y: usize) -> Self {
        Self {
            grid: Grid::from_fn(|_, _| 0.0, nalgebra::Vector2::new(x, y)),
        }
    }

    fn __getitem__(&self, py: Python<'_>, x: Py<PyTuple>) -> PyResult<f32> {
        let (idx_0, idx_1) = py_dict_to_i32x2(py, x)?;
        if self.in_bounds(idx_0, idx_1) {
            Ok(self.grid.get(idx_0 as usize, idx_1 as usize))
        } else {
            Err(PyIndexError::new_err("index out of bounds"))
        }
    }
    fn __setitem__(&mut self, py: Python<'_>, x: Py<PyTuple>, item: f32) -> PyResult<()> {
        let (idx_0, idx_1) = py_dict_to_i32x2(py, x)?;
        if self.in_bounds(idx_0, idx_1) {
            *self.grid.get_mut(idx_0 as usize, idx_1 as usize) = item;
            Ok(())
        } else {
            Err(PyIndexError::new_err("index out of bounds"))
        }
    }
}
/// A Python module implemented in Rust.
#[pymodule]
fn py_watersim(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_stack, m)?)?;
    m.add_class::<PyGrid>()?;
    m.add_class::<PyGridLayers>()?;
    Ok(())
}
