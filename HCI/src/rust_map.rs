use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

#[pyclass]
pub struct RustMap {
    data: HashMap<i32, i32>,
}

#[pymethods]
impl RustMap {
    #[new]
    pub fn new() -> Self {
        RustMap { data: HashMap::new() }
    }

    pub fn insert(&mut self, key: i32, value: i32) {
        self.data.insert(key, value);
    }

    pub fn remove(&mut self, key: i32) {
        self.data.remove(&key);
    }

    pub fn get(&self, key: i32) -> Option<i32> {
        self.data.get(&key).cloned()
    }

    pub fn contains_key(&self, key: i32) -> bool {
        self.data.contains_key(&key)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn to_dict<'py>(&self, py: Python<'py>) -> &'py PyDict {
        let dict = PyDict::new(py);
        for (k, v) in &self.data {
            dict.set_item(k, v).unwrap();
        }
        dict
    }
}
