use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[pyclass(name = "Grid")]
struct PyGrid {
    grid: grid::Grid<f32>,
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
            grid: grid::Grid::from_fn(|_, _| 0.0, nalgebra::Vector2::new(x, y)),
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
    m.add_class::<PyGrid>()?;
    Ok(())
}
