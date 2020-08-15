#![feature(vec_into_raw_parts)]
#![feature(iter_order_by)]

#[macro_use]
extern crate bitflags;
use csv::ReaderBuilder;
use wasm_bindgen::prelude::*;

mod array2d;
mod frieze;
use array2d::Array2D;
use frieze::{Frieze, Point};

#[wasm_bindgen]
pub fn analyse_frieze(content: &[u8]) -> String {
    match parse(content) {
        Ok(f) => f.draw_svg(),
        Err(s) => s.into(),
    }
}

fn parse(content: &[u8]) -> Result<Frieze, &'static str> {
    let mut array: Vec<Point> = Vec::with_capacity(content.len() / 2);
    let mut num_rows = 0;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(content);
    for record in rdr.records() {
        for field in record.map_err(|_| "Ragged rows")?.iter() {
            let bits = field.parse::<u8>().map_err(|_| "Invalid point")?;
            let point = Point::from_bits(bits).ok_or("Invalid point")?;
            array.push(point);
        }
        num_rows += 1;
    }
    let points = Array2D::from_vec(array, num_rows).unwrap();

    Frieze::from_points(points).ok_or("Not a frieze")
}
