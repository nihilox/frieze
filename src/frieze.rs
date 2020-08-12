use crate::array2d::Array2D;

bitflags! {
    struct Point: u8 {
        const N  = 0b0001;
        const NE = 0b0010;
        const E  = 0b0100;
        const SE = 0b1000;
    }
}

bitflags! {
    struct Grid: u8 {
        const EW =  0b00000001;
        const SN =  0b00000010;
        const AB =  0b00000100;
        const EWX = 0b00010000;
        const SNX = 0b00100000;
        const ABX = 0b01000000;
    }
}

impl Grid {
    fn hr(&self) -> Self {
        let mut r = *self;
        if self.contains(Self::SNX) {
            r.toggle(Self::SN)
        }
        if self.contains(Self::ABX) {
            r.toggle(Self::AB)
        }
        r
    }

    fn vr(&self) -> Self {
        let mut r = *self;
        if self.contains(Self::EWX) {
            r.toggle(Self::EW)
        }
        if self.contains(Self::ABX) {
            r.toggle(Self::AB)
        }
        r
    }

    fn rotate(&self) -> Self {
        let mut r = *self;
        if self.contains(Self::EWX) {
            r.toggle(Self::EW)
        }
        if self.contains(Self::SNX) {
            r.toggle(Self::SN)
        }
        r
    }

    fn from_points(o: Point, u: Point, r: Point) -> Self {
        let mut g = Grid::empty();
        g.set(Grid::EW, o.contains(Point::N));
        g.set(Grid::EWX, (o ^ r).contains(Point::N));
        g.set(Grid::SN, u.contains(Point::E));
        g.set(Grid::SNX, (o ^ u).contains(Point::E));
        g.set(Grid::AB, o.contains(Point::NE));
        g.set(Grid::ABX, o.contains(Point::NE) ^ u.contains(Point::SE));
        g
    }
}

#[derive(Debug)]
pub enum FriezeGroup {
    T,
    TR,
    TV,
    TG,
    THG,
    TRVG,
    TRHVG,
}

pub struct Frieze {
    grids: Array2D<Grid>,
    period: usize,
}

impl Frieze {
    fn from_grids(grids: Array2D<Grid>) -> Option<Self> {
        (2..=grids.num_cols() / 2)
            .filter(|&p| grids.rows().all(|row| row.starts_with(&row[p..])))
            .next()
            .map(|period| Self { grids, period })
    }

    fn from_points(points: Array2D<Point>) -> Option<Self> {
        let elements = points
            .rows()
            .zip(points.rows().skip(1))
            .flat_map(|(r, nr)| {
                nr.windows(2)
                    .zip(r)
                    .map(|(nr, &u)| Grid::from_points(nr[0], u, nr[1]))
            })
            .collect();
        let num_rows = points.num_rows() - 1;
        let num_cols = points.num_cols() - 1;
        let grids = Array2D::from_row_major(elements, num_rows, num_cols).unwrap();
        Self::from_grids(grids)
    }

    fn is_horizontal_reflections(&self) -> bool {
        let half = (self.grids.num_rows() + 1) / 2;
        self.grids
            .rows()
            .rev()
            .take(half)
            .eq_by(self.grids.rows().take(half), |a, b| {
                a[..self.period]
                    .iter()
                    .map(Grid::hr)
                    .eq(b[..self.period].iter().cloned())
            })
    }

    fn is_vertical_reflections(&self) -> bool {
        let period = self.period;
        let sym: Vec<Grid> = self
            .grids
            .rows()
            .flat_map(|row| row[..period].iter().rev().map(Grid::vr))
            .collect();
        (period / 2..period).any(|i| {
            sym.chunks(period).eq_by(self.grids.rows(), |a, b| {
                let b = &b[i..];
                a.ends_with(&b[..i]) || a.ends_with(&b[..i + 1])
            })
        })
    }

    fn is_rotation(&self) -> bool {
        let row_len = self.period * 2;
        let half = (self.grids.num_rows() + 1) / 2;
        let sym: Vec<Grid> = self
            .grids
            .rows()
            .rev()
            .take(half)
            .flat_map(|row| row[..row_len].iter().rev().map(Grid::rotate))
            .collect();
        (self.period..row_len).any(|size| {
            sym.chunks(row_len)
                .eq_by(self.grids.rows().take(half), |a, b| a.ends_with(&b[..size]))
        })
    }

    fn is_glied_reflections(&self) -> bool {
        let period = self.period;
        let half = (self.grids.num_rows() + 1) / 2;
        let sym: Vec<Grid> = self
            .grids
            .rows()
            .rev()
            .take(half)
            .flat_map(|row| row[..period].iter().map(Grid::hr))
            .collect();
        (0..period).any(|i| {
            sym.chunks(period)
                .eq_by(self.grids.rows().take(half), |a, b| b[i..].starts_with(a))
        })
    }

    pub fn group(&self) -> FriezeGroup {
        if self.is_horizontal_reflections() {
            if self.is_vertical_reflections() {
                FriezeGroup::TRHVG
            } else {
                FriezeGroup::THG
            }
        } else if self.is_glied_reflections() {
            if self.is_vertical_reflections() {
                FriezeGroup::TRVG
            } else {
                FriezeGroup::TG
            }
        } else if self.is_vertical_reflections() {
            FriezeGroup::TV
        } else if self.is_rotation() {
            FriezeGroup::TR
        } else {
            FriezeGroup::T
        }
    }

    pub fn period(&self) -> usize {
        self.period
    }
}

pub fn parse(rows: Vec<u8>, num_rows: usize, num_cols: usize) -> Result<Frieze, &'static str> {
    let points = {
        let (ptr, len, cap) = rows.into_raw_parts();
        let elements = unsafe { Vec::from_raw_parts(ptr as *mut Point, len, cap) };
        Array2D::from_row_major(elements, num_rows, num_cols).ok_or("Incorrect size")?
    };
    let frieze = Frieze::from_points(points).ok_or("Not a frieze")?;

    Ok(frieze)
}
