use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyAny;
use std::fmt;
use std::fmt::Debug;

#[pyclass(module = "pqtree", get_all)]
struct P {
    children: Vec<PyObject>,
}

impl Debug for P {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P{{{:?}}}", self.children)
    }
}

impl Debug for Q {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Q{{{:?}}}", self.children)
    }
}

#[pyclass(module = "pqtree", get_all)]
struct Q {
    children: Vec<PyObject>,
}

fn _flatten(obj: PyObject, py: Python) -> PyResult<Py<PyAny>> {
    if let Ok(p_obj) = obj.downcast_bound::<P>(py) {
        p_obj.borrow_mut().flatten(py)
    } else if let Ok(q_obj) = obj.downcast_bound::<Q>(py) {
        q_obj.borrow_mut().flatten(py)
    } else {
        Ok(obj)
    }
}

fn factorial(n: u128) -> u128 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

#[pymethods]
impl P {
    #[new]
    fn new(children: Vec<PyObject>) -> Self {
        P { children }
    }

    fn reverse(&mut self, py: Python) {
        for child in &mut self.children {
            if let Ok(py_obj) = child.extract::<PyObject>(py) {
                // Check if the object is an instance of Q
                if let Ok(q_obj) = py_obj.downcast_bound::<Q>(py) {
                    q_obj.borrow_mut().reverse(py);
                }
                // Check if the object is an instance of P
                else if let Ok(p_obj) = py_obj.downcast_bound::<P>(py) {
                    p_obj.borrow_mut().reverse(py);
                }
            }
        }

        self.children.reverse();
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let mut result = String::from("P{");
        let mut first = true;

        for child in &self.children {
            if !first {
                result.push_str(", ");
            }
            first = false;

            // Recursively format if the child is an instance of P or Q

            if let Ok(py_obj) = child.extract::<PyObject>(py) {
                if let Ok(p_obj) = py_obj.downcast_bound::<P>(py) {
                    result.push_str(&p_obj.borrow().__repr__(py)?);
                } else if let Ok(q_obj) = py_obj.downcast_bound::<Q>(py) {
                    result.push_str(&q_obj.borrow().__repr__(py)?);
                } else if let Ok(obj) = py_obj.downcast_bound::<PyAny>(py) {
                    result.push_str(obj.str()?.to_str()?);
                }
            }
        }

        result.push('}');
        Ok(result)
    }

    fn __str__(&self, py: Python) -> PyResult<String> {
        let mut result = String::from("P{");
        let mut first = true;

        for child in &self.children {
            if !first {
                result.push_str(", ");
            }
            first = false;

            // Recursively format if the child is an instance of P or Q

            if let Ok(py_obj) = child.extract::<PyObject>(py) {
                if let Ok(p_obj) = py_obj.downcast_bound::<P>(py) {
                    result.push_str(&p_obj.borrow().__str__(py)?);
                } else if let Ok(q_obj) = py_obj.downcast_bound::<Q>(py) {
                    result.push_str(&q_obj.borrow().__str__(py)?);
                } else if let Ok(obj) = py_obj.downcast_bound::<PyAny>(py) {
                    result.push_str(obj.str()?.to_str()?);
                }
            }
        }

        result.push('}');
        Ok(result)
    }

    fn number_of_children(&self) -> usize {
        self.children.len()
    }

    fn get_children(&self, py: Python) -> Vec<PyObject> {
        let mut copy_children = Vec::new();
        for child in &self.children {
            if let Ok(ch) = child.extract::<PyObject>(py) {
                copy_children.push(ch);
            }
        }
        copy_children
    }

    fn flatten(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        if self.number_of_children() == 1 {
            if let Ok(obj) = self
                .get_children(py)
                .first()
                .unwrap()
                .extract::<PyObject>(py)
            {
                _flatten(obj, py)
            } else {
                Err(PyErr::new::<PyTypeError, _>("Error"))
            }
        } else {
            for child in &mut self.children {
                if let Ok(obj) = child.extract::<PyObject>(py) {
                    *child = _flatten(obj, py)?;
                }
            }
            Ok(Self {
                children: self.get_children(py),
            }
            .into_py(py))
        }
    }

    fn ordering(&self, py: Python) -> Vec<PyObject> {
        let mut value = Vec::new();
        for child in &self.children {
            if let Ok(py_obj) = child.extract::<PyObject>(py) {
                if let Ok(p_obj) = py_obj.downcast_bound::<P>(py) {
                    value.extend(p_obj.borrow().ordering(py));
                } else if let Ok(q_obj) = py_obj.downcast_bound::<Q>(py) {
                    value.extend(q_obj.borrow().ordering(py));
                } else {
                    value.push(py_obj);
                }
            }
        }
        value
    }

    fn cardinality(&self, py: Python) -> u128 {
        let mut n: u128 = factorial(self.number_of_children() as u128);
        for child in &self.children {
            if let Ok(py_obj) = child.extract::<PyObject>(py) {
                if let Ok(p_obj) = py_obj.downcast_bound::<P>(py) {
                    n *= p_obj.borrow().cardinality(py);
                } else if let Ok(q_obj) = py_obj.downcast_bound::<Q>(py) {
                    n *= q_obj.borrow().cardinality(py);
                }
            }
        }
        n
    }
}

#[pymethods]
impl Q {
    #[new]
    fn new(children: Vec<PyObject>) -> Self {
        Q { children }
    }

    fn reverse(&mut self, py: Python) {
        for child in &mut self.children {
            if let Ok(py_obj) = child.extract::<PyObject>(py) {
                // Check if the object is an instance of Q or P
                if let Ok(q_obj) = py_obj.downcast_bound::<Q>(py) {
                    q_obj.borrow_mut().reverse(py);
                } else if let Ok(p_obj) = py_obj.downcast_bound::<P>(py) {
                    p_obj.borrow_mut().reverse(py);
                }
            }
        }

        self.children.reverse();
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let mut result = String::from("Q{");
        let mut first = true;

        for child in &self.children {
            if !first {
                result.push_str(", ");
            }
            first = false;

            // Recursively format if the child is an instance of P or Q

            if let Ok(py_obj) = child.extract::<PyObject>(py) {
                if let Ok(p_obj) = py_obj.downcast_bound::<P>(py) {
                    result.push_str(&p_obj.borrow().__repr__(py)?);
                } else if let Ok(q_obj) = py_obj.downcast_bound::<Q>(py) {
                    result.push_str(&q_obj.borrow().__repr__(py)?);
                } else if let Ok(obj) = py_obj.downcast_bound::<PyAny>(py) {
                    result.push_str(obj.str()?.to_str()?);
                }
            }
        }

        result.push('}');
        Ok(result)
    }

    fn __str__(&self, py: Python) -> PyResult<String> {
        let mut result = String::from("Q{");
        let mut first = true;

        for child in &self.children {
            if !first {
                result.push_str(", ");
            }
            first = false;

            // Recursively format if the child is an instance of P or Q

            if let Ok(py_obj) = child.extract::<PyObject>(py) {
                if let Ok(p_obj) = py_obj.downcast_bound::<P>(py) {
                    result.push_str(&p_obj.borrow().__str__(py)?);
                } else if let Ok(q_obj) = py_obj.downcast_bound::<Q>(py) {
                    result.push_str(&q_obj.borrow().__str__(py)?);
                } else if let Ok(obj) = py_obj.downcast_bound::<PyAny>(py) {
                    result.push_str(obj.str()?.to_str()?);
                }
            }
        }

        result.push('}');
        Ok(result)
    }

    fn number_of_children(&self) -> usize {
        self.children.len()
    }

    fn get_children(&self, py: Python) -> Vec<PyObject> {
        let mut copy_children = Vec::new();
        for child in &self.children {
            if let Ok(ch) = child.extract::<PyObject>(py) {
                copy_children.push(ch);
            }
        }
        copy_children
    }

    fn flatten(&mut self, py: Python) -> PyResult<Py<PyAny>> {
        if self.number_of_children() == 1 {
            if let Ok(obj) = self
                .get_children(py)
                .first()
                .unwrap()
                .extract::<PyObject>(py)
            {
                _flatten(obj, py)
            } else {
                Err(PyErr::new::<PyTypeError, _>("Error"))
            }
        } else {
            for child in &mut self.children {
                if let Ok(obj) = child.extract::<PyObject>(py) {
                    *child = _flatten(obj, py)?;
                }
            }
            Ok(Self {
                children: self.get_children(py),
            }
            .into_py(py))
        }
    }

    fn ordering(&self, py: Python) -> Vec<PyObject> {
        let mut value = Vec::new();
        for child in &self.children {
            if let Ok(py_obj) = child.extract::<PyObject>(py) {
                if let Ok(p_obj) = py_obj.downcast_bound::<P>(py) {
                    value.extend(p_obj.borrow().ordering(py));
                } else if let Ok(q_obj) = py_obj.downcast_bound::<Q>(py) {
                    value.extend(q_obj.borrow().ordering(py));
                } else {
                    value.push(py_obj);
                }
            }
        }
        value
    }

    fn cardinality(&self, py: Python) -> u128 {
        let mut n: u128 = 1;
        for child in &self.children {
            if let Ok(py_obj) = child.extract::<PyObject>(py) {
                if let Ok(p_obj) = py_obj.downcast_bound::<P>(py) {
                    n *= p_obj.borrow().cardinality(py);
                } else if let Ok(q_obj) = py_obj.downcast_bound::<Q>(py) {
                    n *= q_obj.borrow().cardinality(py);
                }
            }
        }
        if self.number_of_children() == 1 {
            n
        } else {
            n * 2
        }
    }
}

#[pymodule(name = "pqtree")]
fn pqtrees_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<P>()?;
    m.add_class::<Q>()?;
    Ok(())
}
