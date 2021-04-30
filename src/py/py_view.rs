use pyo3::pycell::PyCell;
use pyo3::types::{PyBytes, PyTuple};
use pyo3::{PyObject, PyResult, Python, ToPyObject};

use super::clvm_object::CLVMObject;

#[derive(Clone)]
pub enum PyView {
    Atom(PyObject),
    Pair(PyObject),
}

impl PyView {
    pub fn new_atom(py: Python, atom: &PyBytes) -> Self {
        PyView::Atom(atom.to_object(py))
    }

    pub fn new_pair(py: Python, pair: &PyTuple) -> PyResult<Self> {
        if pair.len() != 2 {
            py.eval("raise ValueError('new_pair requires 2-tuple')", None, None)?;
        }
        let _p0: &PyCell<CLVMObject> = pair.get_item(0).extract()?;
        let _p1: &PyCell<CLVMObject> = pair.get_item(1).extract()?;

        Ok(PyView::Pair(pair.to_object(py)))
    }
}
