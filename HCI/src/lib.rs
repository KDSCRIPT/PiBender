use pyo3::prelude::*;
mod rust_list;
mod rust_map;
mod rust_set;

use rust_list::RustList;
use rust_map::RustMap;
use rust_set::RustSet;
use rust_tree:TreeNode;
#[pymodule]
fn hciutils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RustList>()?;
    m.add_class::<RustMap>()?;
    m.add_class::<RustSet>()?;
    Ok(())
}
