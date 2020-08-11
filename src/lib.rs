mod frieze;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn analyse_frieze(rows: Vec<u8>, num_rows: usize, num_cols: usize) -> String {
    match frieze::parse(rows, num_rows, num_cols) {
        Ok(f) => format!("Group: {:?}, Period: {}", f.group(), f.period()),
        Err(s) => "Err: ".to_owned() + s,
    }
}
