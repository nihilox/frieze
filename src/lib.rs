#![feature(vec_into_raw_parts)]
#![feature(iter_order_by)]

#[macro_use]
extern crate bitflags;
use csv::ReaderBuilder;
use tera::{Context, Tera};
use wasm_bindgen::prelude::*;

mod array2d;
mod frieze;
use array2d::Array2D;
use frieze::{Frieze, FriezeGroup, Point};

#[wasm_bindgen]
pub fn analyse_frieze(content: &[u8]) -> String {
    match parse(content) {
        Ok(f) => f.draw_svg(),
        Err(s) => s.into(),
    }
}

fn parse(content: &[u8]) -> Result<Frieze, &'static str> {
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(content);
    let mut array: Vec<Point> = Vec::with_capacity(content.len() / 2);
    let mut num_rows = 0;
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

impl Frieze {
    pub fn draw_svg(&self) -> String {
        let period = self.period();
        let grids = self.grids();
        let group = self.group();

        let title = format!("{:?} Period({})", group, period);
        let width = grids.num_cols() as f64;
        let height = grids.num_rows() as f64;
        let middle = (grids.num_cols() / period * period / 2) as f64;
        let pad = 2.0;

        use FriezeGroup::*;
        let tips = match group {
            TV(i) | TRVG(i) | TRHVG(i) => {
                let i = middle + i as f64 / 2.0;
                format!("M {} {} L {} {}", i, -pad, i, height + pad)
            }
            TG(offset) => {
                let offset = offset as f64;
                let i = (width - offset) / 2.0;
                let half = height / 2.0;
                format!(
                    "M {} {} L {} {} L {} {} L {} {}",
                    i,
                    -pad,
                    i,
                    half,
                    i + offset,
                    half,
                    i + offset,
                    height + pad
                )
            }
            TR(i) => {
                let i = middle + i as f64 / 2.0;
                let half = height / 2.0 + pad;
                format!("M {} {} L {} {}", i - half, -pad, i + half, height + pad)
            }
            _ => String::new(),
        };

        let mut ctx = Context::new();
        ctx.insert("title", &title);
        ctx.insert("width", &width);
        ctx.insert("height", &height);
        ctx.insert("paths", &self.paths());
        ctx.insert("tips", &tips);

        Tera::one_off(include_str!("graph.svg"), &ctx, true).expect("Could not draw graph")
    }
}
