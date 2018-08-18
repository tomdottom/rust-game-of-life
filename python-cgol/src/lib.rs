#![feature(use_extern_macros, specialization, extern_prelude, type_ascription)]
extern crate cgol as libcgol;
extern crate pyo3;
extern crate rand;

use rand::{Rng, thread_rng};
use pyo3::prelude::*;

#[pyclass]
struct Universe {
    _universe: libcgol::Universe
}

#[pymethods]
impl Universe {

    #[new]
    #[args(width=32, height=32)]
    fn __new__(obj: &PyRawObject, width: u32, height: u32) -> PyResult<()> {
        obj.init(|_| {
            let choices = [libcgol::Cell::Alive, libcgol::Cell::Dead];
            let mut rng = thread_rng();

            let cells: Vec<libcgol::Cell> = (0..width * height)
                .map(|_|  *rng.choose(&choices).unwrap())
                .collect();

            Universe {
                _universe: libcgol::Universe::new(width, height, cells)
            }
        })
    }

    fn render(&self) -> PyResult<String> {
        Ok(self._universe.render())
    }

    fn tick(&mut self) -> PyResult<()> {
        self._universe.tick();
        Ok(())
    }

}


/// This module is a python moudle implemented in Rust.
#[pymodinit]
fn cgol(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Universe>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        libcgol::Universe::new(None, None);
    }
}
