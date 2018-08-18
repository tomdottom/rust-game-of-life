#![feature(use_extern_macros, extern_prelude, custom_attribute)]

extern crate cgol;
extern crate wasm_bindgen;
extern crate wbg_rand;

use wasm_bindgen::prelude::*;
use wbg_rand::{Rng, wasm_rng};

#[wasm_bindgen]
pub struct Universe {
    pub width: u32,
    pub height: u32,
    _universe: cgol::Universe
}

#[wasm_bindgen]
impl Universe {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Universe {
        let choices = [cgol::Cell::Alive, cgol::Cell::Dead];
        let mut rng = wasm_rng();

        let cells: Vec<cgol::Cell> = (0..32 * 32)
            .map(|_|  *rng.choose(&choices).unwrap())
            .collect();

        Universe {
            width: 32,
            height: 32,
            _universe: cgol::Universe::new(32, 32, cells),
        }
    }

    pub fn render(&self) -> String {
        self._universe.render()
    }

    pub fn as_array(&self) -> Vec<u8> {
        self._universe.as_array()
    }

    pub fn tick(&mut self) {
        self._universe.tick();
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_a_universe() {
        Universe::new();
    }
}
