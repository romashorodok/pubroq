use pyo3::prelude::*;

#[derive(Debug)]
#[pyclass]
pub struct CPythonAgent {}

#[pymethods]
impl CPythonAgent {
    #[new]
    pub fn new() -> CPythonAgent {
        CPythonAgent {}
    }

    pub fn ufrag(&self) -> String {
        protocol::Agent::ufrag(self)
    }
}

impl protocol::Agent for CPythonAgent {
    fn ufrag(&self) -> String {
        "test".into()
    }
}
