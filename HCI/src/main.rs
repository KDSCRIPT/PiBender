use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug)]
pub struct RustList {
    data: Vec<i32>, // You can replace i32 with other types if needed
}

impl RustList {
    pub fn new() -> Self {
        RustList { data: Vec::new() }
    }

    // Append operation (sequential)
    fn append(&mut self, value: i32) {
        self.data.push(value);
    }

    // Insert operation (sequential)
    fn insert(&mut self, index: usize, value: i32) {
        if index <= self.data.len() {
            self.data.insert(index, value);
        }
    }

    // Remove operation (sequential)
    fn remove(&mut self, value: i32) {
        if let Some(pos) = self.data.iter().position(|&x| x == value) {
            self.data.remove(pos);
        }
    }

    // Pop operation (sequential)
    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    // Clear operation (sequential)
    fn clear(&mut self) {
        self.data.clear();
    }

    // Index operation (sequential)
    fn index(&self, value: i32) -> Option<usize> {
        self.data.iter().position(|&x| x == value)
    }

    // Count operation (parallelized)
    fn count(&self, value: i32) -> usize {
        self.data.par_iter().filter(|&&x| x == value).count()
    }

    // Reverse operation (parallelized)
    fn reverse(&mut self) {
        self.data.par_iter_mut().rev().collect::<Vec<_>>();
    }

    // Sort operation (parallelized)
    fn sort(&mut self) {
        self.data.par_sort();
    }

    // Perform an operation based on the operation number
    pub fn perform_operation(
        &mut self,
        operation_number: u32,
        args: HashMap<String, i32>,
    ) -> Vec<i32> {
        match operation_number {
            1 => {
                // Append
                if let Some(&value) = args.get("value") {
                    self.append(value);
                }
            }
            2 => {
                // Insert
                if let (Some(&index), Some(&value)) = (args.get("index"), args.get("value")) {
                    self.insert(index as usize, value);
                }
            }
            3 => {
                // Remove
                if let Some(&value) = args.get("value") {
                    self.remove(value);
                }
            }
            4 => {
                // Pop
                self.pop();
            }
            5 => {
                // Clear
                self.clear();
            }
            6 => {
                // Index
                if let Some(&value) = args.get("value") {
                    if let Some(idx) = self.index(value) {
                        println!("Index of {} is {}", value, idx);
                    } else {
                        println!("Value {} not found", value);
                    }
                }
            }
            7 => {
                // Count (parallelized)
                if let Some(&value) = args.get("value") {
                    let count = self.count(value);
                    println!("Count of {} is {}", value, count);
                }
            }
            8 => {
                // Reverse (parallelized)
                self.reverse();
            }
            9 => {
                // Sort (parallelized)
                self.sort();
            }
            _ => {
                println!("Invalid operation number");
            }
        }
        self.data.clone()
    }
}

fn main() {
    let mut my_list = RustList::new();

    // Append 5
    let mut args = HashMap::new();
    args.insert("value".to_string(), 5);
    println!("After append: {:?}", my_list.perform_operation(1, args.clone()));

    // Append 10
    args.insert("value".to_string(), 10);
    println!("After append: {:?}", my_list.perform_operation(1, args.clone()));

    // Insert 15 at index 1
    args.insert("index".to_string(), 1);
    args.insert("value".to_string(), 15);
    println!("After insert: {:?}", my_list.perform_operation(2, args.clone()));

    // Remove 10
    args.insert("value".to_string(), 10);
    println!("After remove: {:?}", my_list.perform_operation(3, args.clone()));

    // Reverse the list
    args.clear();
    println!("After reverse: {:?}", my_list.perform_operation(8, args.clone()));

    // Sort the list
    println!("After sort: {:?}", my_list.perform_operation(9, args.clone()));
}
