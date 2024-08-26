use pyo3::prelude::*;

#[derive(Debug)]
#[pyclass]
struct MyPythonRustClass {
    field: String,
}

/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from my-project!".into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_class::<MyPythonRustClass>()?;

    Ok(())
}
