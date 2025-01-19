use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use skeletal_structures::graph_structure::simplicial2::{self, Simplicial2};

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

#[pymethods]
impl PyIterHalfEdge2 {
    /// Gets halfedge index
    /// /!\ Can be modified if simplicial is modified
    pub fn index(&self) -> usize {
        self.ind_halfedge
    }

    /// First node
    pub fn first_node(&self) -> PyIterNode2 {
        PyIterNode2 {
            simpl2: self.simpl2.clone(),
            ind_halfedge: self.ind_halfedge,
        }
    }

    /// Last node
    pub fn last_node(&self) -> PyIterNode2 {
        self.next().first_node()
    }

    /// Next halfedge on same triangle
    pub fn next(&self) -> PyIterHalfEdge2 {
        let ind_next = self
            .simpl2
            .get_halfedge_from_index(self.ind_halfedge)
            .unwrap()
            .next()
            .index();
        PyIterHalfEdge2 {
            simpl2: self.simpl2.clone(),
            ind_halfedge: ind_next,
        }
    }

    /// Previous halfedge on same triangle
    pub fn previous(&self) -> PyIterHalfEdge2 {
        let ind_prev = self
            .simpl2
            .get_halfedge_from_index(self.ind_halfedge)
            .unwrap()
            .previous()
            .index();
        PyIterHalfEdge2 {
            simpl2: self.simpl2.clone(),
            ind_halfedge: ind_prev,
        }
    }

    /// Opposite halfedge: Same vertices in opposite order (on neighbor triangle)
    pub fn opposite(&self) -> PyIterHalfEdge2 {
        let ind_opp = self
            .simpl2
            .get_halfedge_from_index(self.ind_halfedge)
            .unwrap()
            .opposite()
            .index();
        PyIterHalfEdge2 {
            simpl2: self.simpl2.clone(),
            ind_halfedge: ind_opp,
        }
    }

    /// Triangle containing halfedge
    pub fn triangle(&self) -> PyIterTriangle2 {
        let ind_tri = self
            .simpl2
            .get_halfedge_from_index(self.ind_halfedge)
            .unwrap()
            .triangle()
            .index();
        PyIterTriangle2 {
            simpl2: self.simpl2.clone(),
            ind_triangle: ind_tri,
        }
    }

    // /// Get dual cell halfedge
    // pub fn dual(&self) -> IterDiagHalfEdge2<'a> {
    //     IterDiagHalfEdge2::new(self.simplicial, self.ind_halfedge)
    // }

    /// Halfedge to string
    pub fn to_string(&self) -> String {
        self.simpl2
            .get_halfedge_from_index(self.ind_halfedge)
            .unwrap()
            .to_string()
    }
}

#[pymethods]
impl PyIterTriangle2 {
    /// Gets triangle index
    /// /!\ Can be modified if simplicial is modified
    pub fn index(&self) -> usize {
        self.ind_triangle
    }

    /// Surrounding halfedges (array of halfedge iterators)
    pub fn halfedges(&self) -> [PyIterHalfEdge2; 3] {
        let [he0, he1, he2] = self
            .simpl2
            .get_triangle_from_index(self.ind_triangle)
            .unwrap()
            .halfedges();
        [
            PyIterHalfEdge2 {
                simpl2: self.simpl2.clone(),
                ind_halfedge: he0.index(),
            },
            PyIterHalfEdge2 {
                simpl2: self.simpl2.clone(),
                ind_halfedge: he1.index(),
            },
            PyIterHalfEdge2 {
                simpl2: self.simpl2.clone(),
                ind_halfedge: he2.index(),
            },
        ]
    }

    /// usizes(array of nodes)
    pub fn node_values(&self) -> [usize; 3] {
        self.simpl2
            .get_triangle_from_index(self.ind_triangle)
            .unwrap()
            .node_values()
    }

    /// array of nodes
    pub fn nodes(&self) -> [PyIterNode2; 3] {
        let [he0, he1, he2] = self.halfedges();
        [he0.first_node(), he1.first_node(), he2.first_node()]
    }

    // /// Get dual cell node
    // pub fn dual(&self) -> IterDiagNode2<'a> {
    //     IterDiagNode2::new(self.simplicial, self.ind_triangle)
    // }

    /// Triangle to string
    pub fn to_string(&self) -> String {
        self.simpl2
            .get_triangle_from_index(self.ind_triangle)
            .unwrap()
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
