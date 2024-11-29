use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use nalgebra::base::*;

use skeletal_structures::mesh_structure::skeleton2d::io::*;
use skeletal_structures::mesh_structure::skeleton2d::Skeleton2D;

/// Rust struct for PySkeleton2D class, representing a 2D skeleton.
#[pyclass]
struct PySkeleton2D {
    skel: Skeleton2D,
}

#[pymethods]
impl PySkeleton2D {
    #[new]
    /// Initializes a new empty PySkeleton2D object.
    fn new() -> Self {
        PySkeleton2D {
            skel: Skeleton2D::new(),
        }
    }

    /// Returns the number of vertices in the skel.
    fn get_nb_vertices(&self) -> usize {
        self.skel.get_nb_vertex()
    }

    /// Returns the vertex coordinates at a given index as a list of 2 floats.
    fn get_vertex_coords(&self, ind_vertex: usize) -> PyResult<[f64; 2]> {
        match self.skel.get_vertex_coords(ind_vertex) {
            Ok(vertex) => Ok([vertex[0], vertex[1]]),
            Err(err) => Err(PyException::new_err(err.to_string())),
        }
    }

    /// Gets all vertices
    fn get_all_vertices_coords(&self) -> PyResult<Vec<[f64; 2]>> {
        let mut vec_vert = Vec::new();
        for ind_vertex in 0..self.skel.get_nb_vertex() {
            match self.skel.get_vertex_coords(ind_vertex) {
                Ok(vertex) => vec_vert.push([vertex[0], vertex[1]]),
                Err(err) => return Err(PyException::new_err(err.to_string())),
            }
        }
        return Ok(vec_vert);
    }

    /// Returns the vertex radius at a given index as a float.
    fn get_vertex_radius(&self, ind_vertex: usize) -> PyResult<f64> {
        match self.skel.get_vertex_radius(ind_vertex) {
            Ok(radius) => Ok(radius),
            Err(err) => Err(PyException::new_err(err.to_string())),
        }
    }

    /// Gets all vertices
    fn get_all_vertices_radii(&self) -> PyResult<Vec<f64>> {
        let mut vec_radii = Vec::new();
        for ind_vertex in 0..self.skel.get_nb_vertex() {
            match self.skel.get_vertex_radius(ind_vertex) {
                Ok(radius) => vec_radii.push(radius),
                Err(err) => return Err(PyException::new_err(err.to_string())),
            }
        }
        return Ok(vec_radii);
    }

    /// Returns the number of edges strating at a given vertex in the skel.
    fn get_nb_neighbors(&self, ind_vertex: usize) -> PyResult<usize> {
        match self.skel.get_vertex_neighbors(ind_vertex) {
            Ok(edg) => Ok(edg.len()),
            Err(err) => Err(PyException::new_err(err.to_string())),
        }
    }

    /// Returns the neighbor indices at a given index as a list
    fn get_neighbors(&self, ind_vertex: usize) -> PyResult<Vec<usize>> {
        match self.skel.get_vertex_neighbors(ind_vertex) {
            Ok(edg) => Ok(edg),
            Err(err) => Err(PyException::new_err(err.to_string())),
        }
    }

    /// Gets all neighbors
    fn get_all_neighbors(&self) -> PyResult<Vec<Vec<usize>>> {
        let mut vec_neigh = Vec::new();
        for ind_vertex in 0..self.skel.get_nb_vertex() {
            match self.skel.get_vertex_neighbors(ind_vertex) {
                Ok(neigh) => vec_neigh.push(neigh),
                Err(err) => return Err(PyException::new_err(err.to_string())),
            }
        }
        return Ok(vec_neigh);
    }

    /// Inserts a new vertex (coordinates + radius) into the skeleton and returns its index.
    fn insert_vertex(&mut self, coords: [f64; 2], radius: f64) -> PyResult<usize> {
        let coords_vec = Vector2::new(coords[0], coords[1]);
        match self.skel.insert_vertex(coords_vec, radius) {
            Ok(ind_vertex) => Ok(ind_vertex),
            Err(err) => Err(PyException::new_err(err.to_string())),
        }
    }

    /// Inserts a new edge into the skeleton.
    fn insert_edge(&mut self, v1: usize, v2: usize) -> PyResult<()> {
        match self.skel.insert_edge(v1, v2) {
            Ok(()) => Ok(()),
            Err(err) => Err(PyException::new_err(err.to_string())),
        }
    }
}

#[pyfunction]
fn load_pyskeleton2d_ply(filename: &str) -> PyResult<PySkeleton2D> {
    match load_skeleton2d_ply(filename) {
        Ok(skel) => Ok(PySkeleton2D { skel }),
        Err(err) => Err(PyException::new_err(err.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (filename, pyskel, header=None))]
fn save_pyskeleton2d_ply(
    filename: &str,
    pyskel: &PySkeleton2D,
    header: Option<String>,
) -> PyResult<()> {
    match save_skeleton2d_ply(filename, &pyskel.skel, header) {
        Ok(()) => Ok(()),
        Err(err) => Err(PyException::new_err(err.to_string())),
    }
}

pub fn add_skeleton2d_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let skel_module = PyModule::new_bound(m.py(), "skeleton2d")?;
    skel_module.add_class::<PySkeleton2D>()?;
    skel_module.add_function(wrap_pyfunction!(load_pyskeleton2d_ply, &skel_module)?)?;
    skel_module.add_function(wrap_pyfunction!(save_pyskeleton2d_ply, &skel_module)?)?;
    m.add_submodule(&skel_module)?;

    m.py()
        .import_bound("sys")?
        .getattr("modules")?
        .set_item("py_skeletal_structures.skeleton2d", skel_module)
}
