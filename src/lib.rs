extern crate serde;
extern crate serde_json;
use core::fmt;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct MyError {
    pub msg: &'static str,
}

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error from Rust: {}", self.msg)
    }
}

impl std::convert::From<MyError> for PyErr {
    fn from(value: MyError) -> Self {
        PyValueError::new_err(value.to_string())
    }
}

#[pyfunction]
fn greater_than_2(number: isize) -> Result<isize, MyError> {
    if number <= 2 {
        return Err(MyError {
            msg: "number is less than or equal to 2",
        });
    } else {
        return Ok(number);
    }
}

#[pyfunction]
fn human_says_hi(human_data: String) {
    println!("{}", human_data);
    let human: Human = serde_json::from_str(&human_data).unwrap();

    println!(
        "Now we can work with the struct:\n {:#?}.\n {} is {} years old.",
        human, human.name, human.age,
    )
}

#[derive(Debug, Serialize, Deserialize)]
struct Human {
    name: String,
    age: u8,
}

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from my-project!!!".into())
}

#[pyclass]
pub struct RustStruct {
    #[pyo3(get, set)]
    pub data: String,
    #[pyo3(get, set)]
    pub vector: Vec<u8>,
}

#[pymethods]
impl RustStruct {
    #[new]
    pub fn new(data: String, vector: Vec<u8>) -> RustStruct {
        RustStruct { data, vector }
    }
    pub fn printer(&self) {
        println!("{}", self.data);
        for i in &self.vector {
            println!("{}", i);
        }
    }
    pub fn extend_vector(&mut self, extension: Vec<u8>) {
        println!("{}", self.data);
        for i in extension {
            self.vector.push(i);
        }
    }
}

#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(human_says_hi, m)?)?;
    m.add_function(wrap_pyfunction!(greater_than_2, m)?)?;
    m.add_class::<RustStruct>()?;
    Ok(())
}
