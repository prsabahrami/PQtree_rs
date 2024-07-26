use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyclass]
struct PQ {
    children: Vec<PyObject>,
}

#[pymethods]
impl PQ {
    #[new]
    fn new(seq: Bound<'_, PyList>) -> PyResult<Self> {
        let children = seq.iter().map(|x| x.to_object(seq.py())).collect();
        Ok(PQ { children })
    }
}

#[pyclass(extends=PQ)]
struct P {}

#[pymethods]
impl P {
    #[new]
    fn new(seq: Bound<'_, PyList>) -> PyResult<(Self, PQ)> {
        let pq = PQ::new(seq)?;
        Ok((P {}, pq))
    }

}

#[pyclass(extends=PQ)]
struct Q {}

#[pymethods]
impl Q {
    #[new]
    fn new(seq: Bound<'_, PyList>) -> PyResult<(Self, PQ)> {
        let pq = PQ::new(seq)?;
        Ok((Q {}, pq))
    }

}

#[pymodule]
fn pq_trees(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PQ>()?;
    m.add_class::<P>()?;
    m.add_class::<Q>()?;
    Ok(())
}