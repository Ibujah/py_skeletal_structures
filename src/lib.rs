use pyo3::prelude::*;

mod mesh;
use mesh::add_mesh_module;

/// Python module calling skeletal_structures
#[pymodule]
fn py_skeletal_structures(m: &Bound<'_, PyModule>) -> PyResult<()> {
    add_mesh_module(m)?;
    Ok(())
}
