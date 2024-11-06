use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct RustList {
    data: Vec<i32>,
}

#[pymethods]
impl RustList {
    #[new]
    pub fn new() -> Self {
        RustList { data: Vec::new() }
    }

    pub fn append(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn insert(&mut self, index: usize, value: i32) {
        if index <= self.data.len() {
            self.data.insert(index, value);
        }
    }
    pub fn count_parallel(&self, value: i32) -> usize {
        self.data.par_iter().filter(|&&x| x == value).count()
    }

    pub fn sort_parallel(&mut self) {
        self.data.par_sort();
    }
    pub fn extend(&mut self, values: Vec<i32>) {
        self.data.extend(values);
    }
    pub fn remove(&mut self, value: i32) {
        if let Some(pos) = self.data.iter().position(|&x| x == value) {
            self.data.remove(pos);
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn index(&self, value: i32) -> Option<usize> {
        self.data.iter().position(|&x| x == value)
    }

    pub fn count(&self, value: i32) -> usize {
        self.data.par_iter().filter(|&&x| x == value).count()
    }

    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    pub fn sort(&mut self) {
        self.data.par_sort();
    }

    pub fn perform_operation(
        &mut self,
        operation_number: u32,
        args: HashMap<String, i32>,
    ) -> Vec<i32> {
        match operation_number {
            1 => {
                if let Some(&value) = args.get("value") {
                    self.append(value);
                }
            }
            2 => {
                if let (Some(&index), Some(&value)) = (args.get("index"), args.get("value")) {
                    self.insert(index as usize, value);
                }
            }
            3 => {
                if let Some(&value) = args.get("value") {
                    self.remove(value);
                }
            }
            4 => {
                self.pop();
            }
            5 => {
                self.clear();
            }
            6 => {
                if let Some(&value) = args.get("value") {
                    self.index(value);
                }
            }
            7 => {
                if let Some(&value) = args.get("value") {
                    self.count(value);
                }
            }
            8 => {
                self.reverse();
            }
            9 => {
                self.sort();
            }
            _ => {}
        }
        self.data.clone()
    }

    pub fn get_data(&self) -> Vec<i32> {
        self.data.clone()
    }
}
