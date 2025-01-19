use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use skeletal_structures::graph_structure::simplicial2::{
    self, IterHalfEdge2, IterNode2, IterTriangle2, Simplicial2,
};

/// Rust struct for PySimplicial2 class, representing a 2-Simplcial.
#[pyclass]
struct PySimplicial2 {
    simpl2: Box<Simplicial2>,
}

/// Rust struct for PyIterNode2 class, representing a 2-Node.
#[pyclass]
struct PyIterNode2 {
    simpl2: Box<Simplicial2>,
    ind_halfedge: usize,
}

/// Rust struct for PyIterHalfEdge2 class, representing a 2-Halfedge.
#[pyclass]
struct PyIterHalfEdge2 {
    simpl2: Box<Simplicial2>,
    ind_halfedge: usize,
}

/// Rust struct for PyIterTriangle2 class, representing a 2-Triangle.
#[pyclass]
struct PyIterTriangle2 {
    simpl2: Box<Simplicial2>,
    ind_triangle: usize,
}

#[pymethods]
impl PySimplicial2 {
    #[new]
    /// Initializes a new empty PySimplicial2 object.
    fn new(register_node_halfedges: bool) -> Self {
        PySimplicial2 {
            simpl2: Box::new(Simplicial2::new(register_node_halfedges)),
        }
    }

    /// Gets halfedge iterator from index
    pub fn get_halfedge_from_index(&self, ind_he: usize) -> PyResult<PyIterHalfEdge2> {
        if let Err(err) = self.simpl2.get_halfedge_from_index(ind_he) {
            return Err(PyException::new_err(err.to_string()));
        };

        let phe = PyIterHalfEdge2 {
            simpl2: self.simpl2.clone(),
            ind_halfedge: ind_he,
        };
        Ok(phe)
    }

    /// Gets triangle iterator from index
    pub fn get_triangle_from_index(&self, ind_tri: usize) -> PyResult<PyIterTriangle2> {
        if let Err(err) = self.simpl2.get_triangle_from_index(ind_tri) {
            return Err(PyException::new_err(err.to_string()));
        };

        let phe = PyIterTriangle2 {
            simpl2: self.simpl2.clone(),
            ind_triangle: ind_tri,
        };
        Ok(phe)
    }

    /// Gets total number of halfedges
    pub fn get_nb_halfedges(&self) -> usize {
        self.simpl2.get_nb_halfedges()
    }

    /// Gets number of triangles
    pub fn get_nb_triangles(&self) -> usize {
        self.simpl2.get_nb_triangles()
    }

    /// Checks if a node is in the simplicial
    ///
    /// Returns halfedge iterator if found
    pub fn find_node(&self, node0: usize) -> Option<PyIterNode2> {
        if let Some(nod) = self.simpl2.find_node(node0) {
            return Some(PyIterNode2 {
                simpl2: self.simpl2.clone(),
                ind_halfedge: nod.halfedges()[0].index(),
            });
        }

        None
    }
    /// Checks if an edge is in the simplicial
    ///
    /// Returns halfedge iterator if found
    pub fn find_halfedge(&self, node0: usize, node1: usize) -> Option<PyIterHalfEdge2> {
        if let Some(he) = self.simpl2.find_halfedge(node0, node1) {
            return Some(PyIterHalfEdge2 {
                simpl2: self.simpl2.clone(),
                ind_halfedge: he.index(),
            });
        }

        None
    }

    /// Checks if a triangle is in the simplicial
    ///
    /// Returns triangle iterator if found
    pub fn find_triangle(
        &self,
        node0: usize,
        node1: usize,
        node2: usize,
    ) -> Option<PyIterTriangle2> {
        if let Some(tri) = self.simpl2.find_triangle(node0, node1, node2) {
            return Some(PyIterTriangle2 {
                simpl2: self.simpl2.clone(),
                ind_triangle: tri.index(),
            });
        }

        None
    }
}

#[pymethods]
impl PyIterNode2 {
    pub fn value(&self) -> usize {
        self.simpl2
            .get_halfedge_from_index(self.ind_halfedge)
            .unwrap()
            .first_node()
            .value()
    }

    /// Gets list of halfedges starting at this vertex
    pub fn halfedges(&self) -> Vec<PyIterHalfEdge2> {
        self.simpl2
            .get_halfedge_from_index(self.ind_halfedge)
            .unwrap()
            .first_node()
            .halfedges()
            .iter()
            .map(|&he| PyIterHalfEdge2 {
                simpl2: self.simpl2.clone(),
                ind_halfedge: he.index(),
            })
            .collect()
    }

    // /// Get dual cell
    // pub fn dual(&self) -> PyIterDiagCell2 {}

    /// Node to string
    pub fn to_string(&self) -> String {
        self.simpl2
            .get_halfedge_from_index(self.ind_halfedge)
            .unwrap()
            .first_node()
            .to_string()
    }
}

#[pyfunction]
fn build_from_triangle_list(
    triangles: Vec<[usize; 3]>,
    register_node_halfedges: bool,
) -> PyResult<PySimplicial2> {
    match simplicial2::build_from_triangle_list(triangles, register_node_halfedges) {
        Ok(simpl2) => Ok(PySimplicial2 {
            simpl2: Box::new(simpl2),
        }),
        Err(err) => Err(PyException::new_err(err.to_string())),
    }
}

pub fn add_simplicial2_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let simpl2_module = PyModule::new_bound(m.py(), "simplicial2")?;
    simpl2_module.add_class::<PySimplicial2>()?;
    simpl2_module.add_function(wrap_pyfunction!(build_from_triangle_list, &simpl2_module)?)?;
    m.add_submodule(&simpl2_module)?;

    m.py()
        .import_bound("sys")?
        .getattr("modules")?
        .set_item("py_skeletal_structures.simplicial2", simpl2_module)
}
