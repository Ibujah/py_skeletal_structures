use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use nalgebra::base::*;

use skeletal_structures::mesh_structure::mesh3d::io::*;
use skeletal_structures::mesh_structure::mesh3d::Mesh3D;

/// Rust struct for PyMesh3D class, representing a 3D mesh.
#[pyclass]
struct PyMesh3D {
    mesh: Mesh3D,
}

#[pymethods]
impl PyMesh3D {
    #[new]
    /// Initializes a new empty PyMesh3D object.
    fn new() -> Self {
        PyMesh3D {
            mesh: Mesh3D::new(),
        }
    }

    /// Returns the number of vertices in the mesh.
    fn get_nb_vertices(&self) -> usize {
        self.mesh.get_nb_vertices()
    }

    /// Returns the vertex coordinates at a given index as a list of 3 floats.
    fn get_vertex(&self, ind_vertex: usize) -> PyResult<[f64; 3]> {
        match self.mesh.get_vertex(ind_vertex) {
            Ok(vertex) => Ok([vertex[0], vertex[1], vertex[2]]),
            Err(err) => Err(PyException::new_err(err.to_string())),
        }
    }

    /// Returns the number of faces in the mesh.
    fn get_nb_faces(&self) -> usize {
        self.mesh.get_nb_faces()
    }

    /// Returns the face indices at a given index as a list of 3 integers.
    fn get_face(&self, ind_face: usize) -> PyResult<[usize; 3]> {
        match self.mesh.get_face(ind_face) {
            Ok(face) => Ok([face[0], face[1], face[2]]),
            Err(err) => Err(PyException::new_err(err.to_string())),
        }
    }

    /// Inserts a new vertex into the mesh and returns its index.
    fn insert_vertex(&mut self, vertex: [f64; 3]) -> PyResult<usize> {
        let vertex_vec = Vector3::new(vertex[0], vertex[1], vertex[2]);
        match self.mesh.insert_vertex(vertex_vec) {
            Ok(ind_vertex) => Ok(ind_vertex),
            Err(err) => Err(PyException::new_err(err.to_string())),
        }
    }

    /// Inserts a new face into the mesh and returns its index.
    fn insert_face(&mut self, face: [usize; 3]) -> PyResult<usize> {
        match self.mesh.insert_face(face.to_vec()) {
            Ok(ind_face) => Ok(ind_face),
            Err(err) => Err(PyException::new_err(err.to_string())),
        }
    }
}

#[pyfunction]
fn load_pymesh_obj(filename: &str) -> PyResult<PyMesh3D> {
    match load_mesh_obj(filename) {
        Ok(mesh) => Ok(PyMesh3D { mesh }),
        Err(err) => Err(PyException::new_err(err.to_string())),
    }
}

#[pyfunction]
fn load_pymesh_off(filename: &str) -> PyResult<PyMesh3D> {
    match load_mesh_off(filename) {
        Ok(mesh) => Ok(PyMesh3D { mesh }),
        Err(err) => Err(PyException::new_err(err.to_string())),
    }
}

#[pyfunction]
fn load_pymesh_ply(filename: &str) -> PyResult<PyMesh3D> {
    match load_mesh_ply(filename) {
        Ok(mesh) => Ok(PyMesh3D { mesh }),
        Err(err) => Err(PyException::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (filename, pymesh, header=None))]
fn save_pymesh_ply(filename: &str, pymesh: &PyMesh3D, header: Option<String>) -> PyResult<()> {
    match save_mesh_ply(filename, &pymesh.mesh, header) {
        Ok(()) => Ok(()),
        Err(err) => Err(PyException::new_err(err.to_string())),
    }
}

/// Python module calling skeletal_structures
#[pymodule]
fn py_skeletal_structures(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMesh3D>()?;
    m.add_function(wrap_pyfunction!(load_pymesh_obj, m)?)?;
    m.add_function(wrap_pyfunction!(load_pymesh_off, m)?)?;
    m.add_function(wrap_pyfunction!(load_pymesh_ply, m)?)?;
    m.add_function(wrap_pyfunction!(save_pymesh_ply, m)?)?;
    Ok(())
}
