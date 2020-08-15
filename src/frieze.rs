use crate::array2d::Array2D;

bitflags! {
    pub struct Point: u8 {
        const N  = 0b0001;
        const NE = 0b0010;
        const E  = 0b0100;
        const SE = 0b1000;
    }
}

bitflags! {
    pub struct Grid: u8 {
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
    TR(usize),
    TV(usize),
    TG(usize),
    THG,
    TRVG(usize),
    TRHVG(usize),
}

pub struct Frieze {
    points: Array2D<Point>,
    grids: Array2D<Grid>,
    period: usize,
}

impl Frieze {
    pub fn from_points(points: Array2D<Point>) -> Option<Self> {
        let array = points
            .rows()
            .zip(points.rows().skip(1))
            .flat_map(|(r, nr)| {
                nr.windows(2)
                    .zip(r)
                    .map(|(nr, &u)| Grid::from_points(nr[0], u, nr[1]))
            })
            .collect();
        let num_rows = points.num_rows() - 1;
        let grids = Array2D::from_vec(array, num_rows).unwrap();
        (2..=grids.num_cols() / 2)
            .find(|&p| grids.rows().all(|row| row.starts_with(&row[p..])))
            .map(|period| Self {
                points,
                grids,
                period,
            })
    }

    fn horizontal(&self) -> bool {
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

    fn vertical(&self) -> Option<usize> {
        let period = self.period;
        let sym = self
            .grids
            .rows()
            .flat_map(|row| row[..period].iter().rev().map(Grid::vr))
            .collect::<Vec<_>>();
        let sym = sym.chunks(period);
        let rows = self.grids.rows();

        (period / 2..period)
            .find_map(|i| {
                [i * 2, i * 2 + 1].iter().cloned().find(|&j| {
                    sym.clone()
                        .eq_by(rows.clone(), |a, b| a.ends_with(&b[i..j]))
                })
            })
            .map(|i| i % period)
    }

    fn rotation(&self) -> Option<usize> {
        let row_len = self.period * 2 - 1;
        let half = (self.grids.num_rows() + 1) / 2;
        let sym: Vec<Grid> = self
            .grids
            .rows()
            .rev()
            .take(half)
            .flat_map(|row| row[..row_len].iter().rev().map(Grid::rotate))
            .collect();
        (self.period..=row_len)
            .find(|&i| {
                sym.chunks(row_len)
                    .eq_by(self.grids.rows().take(half), |a, b| a.ends_with(&b[..i]))
            })
            .map(|i| i % self.period)
    }

    fn glied(&self) -> Option<usize> {
        let period = self.period;
        let half = (self.grids.num_rows() + 1) / 2;
        let sym: Vec<Grid> = self
            .grids
            .rows()
            .rev()
            .take(half)
            .flat_map(|row| row[..period].iter().map(Grid::hr))
            .collect();
        (0..period).find(|&i| {
            sym.chunks(period)
                .eq_by(self.grids.rows().take(half), |a, b| b[i..].starts_with(a))
        })
    }

    pub fn group(&self) -> FriezeGroup {
        use FriezeGroup::*;
        if self.horizontal() {
            self.vertical().map_or(THG, |i| TRHVG(i))
        } else if let Some(i) = self.glied() {
            self.vertical().map_or(TG(i), |i| TRVG(i))
        } else if let Some(i) = self.vertical() {
            TV(i)
        } else if let Some(i) = self.rotation() {
            TR(i)
        } else {
            T
        }
    }

    pub fn period(&self) -> usize {
        self.period
    }

    pub fn grids(&self) -> &Array2D<Grid> {
        &self.grids
    }

    pub fn paths(&self) -> Vec<((usize, usize), (usize, usize))> {
        let mut paths = Vec::with_capacity(self.points.num_elems() * 3);
        for (y, row) in self.points.rows().enumerate() {
            for (x, p) in row.iter().enumerate() {
                if p.contains(Point::N) {
                    paths.push(((x, y), (x, y - 1)));
                }
                if p.contains(Point::NE) {
                    paths.push(((x, y), (x + 1, y - 1)));
                }
                if p.contains(Point::E) {
                    paths.push(((x, y), (x + 1, y)));
                }
                if p.contains(Point::SE) {
                    paths.push(((x, y), (x + 1, y + 1)));
                }
            }
        }
        paths
    }
}
