use pyo3::prelude::*;
use pyo3::types::{PyList, PySet};

#[pyclass]
struct PQ {
    children: Vec<PyObject>,
}

#[pymethods]
impl PQ {
    #[new]
    fn new(py: Python, seq: Bound<'_, PyList>) -> PyResult<Self> {
        let children = seq.iter().map(|x| x.to_object(py)).collect();
        Ok(PQ { children})
    }

    fn number_of_children(&self) -> usize {
        self.children.len()
    }

    fn ordering(&self, py: Python) -> PyResult<Vec<PyObject>> {
        let mut result = Vec::new();
        for child in &self.children {
            if let Ok(pq_child) = child.extract::<PyRef<PQ>>(py) {
                result.extend(pq_child.ordering(py)?);
            } else {
                result.push(child.to_object(py));
            }
        }
        Ok(result)
    }
    
    fn flatten(&mut self, py: Python) -> PyResult<PyObject> {
        if self.children.len() == 1 {
            return Ok(self.children[0].clone_ref(py));  // Use clone_ref here
        }
        // Instead of moving self, you can clone parts if needed, or restructure this logic.
        Ok(self.clone().into_py(py))  // Clone self to avoid moving it
    }
    

    fn reverse(&mut self, py: Python) {
        for child in &mut self.children {

            if let Ok(mut pq_child) = child.extract::<PyRefMut<PQ>>(py) {

                pq_child.reverse(py);  // Also pass `py` to nested `reverse` calls
            }
        }
        self.children.reverse();
    }
    
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("PQTree with {} children", self.children.len()))
    }

    
}


impl Clone for PQ {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            let children = self.children.iter().map(|child| child.clone_ref(py)).collect();
            PQ { children }
        })
    }
}

#[pyclass]
struct P {
    pq: PQ
}

#[pymethods]
impl P {
    #[new]
    fn new(py: Python, seq: Bound<'_, PyList>) -> PyResult<Self> {
        let pq = PQ::new(py, seq)?;
        Ok(P { pq })
    }

}

#[pyclass]
struct Q {
    pq: PQ
}

#[pymethods]
impl Q {
    #[new]
    fn new(py: Python, seq: Bound<'_, PyList>) -> PyResult<Self> {
        let pq = PQ::new(py, seq)?;
        Ok(Q { pq })
    }

}

#[pymodule]
fn pq_trees(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PQ>()?;
    m.add_class::<P>()?;
    m.add_class::<Q>()?;
    Ok(())
}