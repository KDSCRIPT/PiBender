use pyo3::prelude::*;
use std::collections::HashSet;

#[pyclass]
pub struct RustSet {
    data: HashSet<i32>,
}

#[pymethods]
impl RustSet {
    #[new]
    pub fn new() -> Self {
        RustSet { data: HashSet::new() }
    }

    pub fn add(&mut self, value: i32) {
        self.data.insert(value);
    }

    pub fn remove(&mut self, value: i32) {
        self.data.remove(&value);
    }

    pub fn contains(&self, value: i32) -> bool {
        self.data.contains(&value)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn to_list(&self) -> Vec<i32> {
        self.data.iter().cloned().collect()
    }
}
