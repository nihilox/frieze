#![feature(vec_into_raw_parts)]
#![feature(iter_order_by)]

#[macro_use]
extern crate bitflags;
use wasm_bindgen::prelude::*;

mod array2d;
mod frieze;

#[wasm_bindgen]
pub fn analyse_frieze(rows: Vec<u8>, num_rows: usize, num_cols: usize) -> String {
    match frieze::parse(rows, num_rows, num_cols) {
        Ok(f) => format!("Group: {:?}, Period: {}", f.group(), f.period()),
        Err(s) => "Err: ".to_owned() + s,
    }
}
