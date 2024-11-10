use pyo3::prelude::*;

mod mesh;
use mesh::add_mesh3d_module;

mod skeleton2d;
use skeleton2d::add_skeleton2d_module;

/// Python module calling skeletal_structures
#[pymodule]
fn py_skeletal_structures(m: &Bound<'_, PyModule>) -> PyResult<()> {
    add_mesh3d_module(m)?;
    add_skeleton2d_module(m)?;
    Ok(())
}
