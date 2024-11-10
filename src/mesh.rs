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

    /// Gets all vertices
    fn get_all_vertices(&self) -> PyResult<Vec<[f64; 3]>> {
        let mut vec_vert = Vec::new();
        for ind_vertex in 0..self.mesh.get_nb_vertices() {
            match self.mesh.get_vertex(ind_vertex) {
                Ok(vertex) => vec_vert.push([vertex[0], vertex[1], vertex[2]]),
                Err(err) => return Err(PyException::new_err(err.to_string())),
            }
        }
        return Ok(vec_vert);
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

    /// Gets all faces
    fn get_all_faces(&self) -> PyResult<Vec<[usize; 3]>> {
        let mut vec_face = Vec::new();
        for ind_face in 0..self.mesh.get_nb_faces() {
            match self.mesh.get_face(ind_face) {
                Ok(face) => vec_face.push([face[0], face[1], face[2]]),
                Err(err) => return Err(PyException::new_err(err.to_string())),
            }
        }
        return Ok(vec_face);
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

pub fn add_mesh3d_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let mesh_module = PyModule::new_bound(m.py(), "mesh3d")?;
    mesh_module.add_class::<PyMesh3D>()?;
    mesh_module.add_function(wrap_pyfunction!(load_pymesh_obj, &mesh_module)?)?;
    mesh_module.add_function(wrap_pyfunction!(load_pymesh_off, &mesh_module)?)?;
    mesh_module.add_function(wrap_pyfunction!(load_pymesh_ply, &mesh_module)?)?;
    mesh_module.add_function(wrap_pyfunction!(save_pymesh_ply, &mesh_module)?)?;
    m.add_submodule(&mesh_module)?;

    m.py()
        .import_bound("sys")?
        .getattr("modules")?
        .set_item("py_skeletal_structures.mesh3d", mesh_module)
}
