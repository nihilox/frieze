use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2D<T> {
    array: Vec<T>,
    num_rows: usize,
    num_cols: usize,
}

impl<T> Array2D<T> {
    pub fn from_vec(array: Vec<T>, num_rows: usize) -> Option<Self> {
        let num_cols = array.len() / num_rows;
        if num_rows * num_cols == array.len() {
            Some(Array2D {
                array,
                num_rows,
                num_cols,
            })
        } else {
            None
        }
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn num_elems(&self) -> usize {
        self.array.len()
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.get_index(row, col).map(|index| &self.array[index])
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.get_index(row, col)
            .map(move |index| &mut self.array[index])
    }

    pub fn rows(&self) -> std::slice::Chunks<'_, T> {
        self.array.chunks(self.num_cols)
    }

    fn get_index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.num_rows && col < self.num_cols {
            Some(row * self.num_cols + col)
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Array2D<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.get(row, col)
            .unwrap_or_else(|| panic!("Index indices {}, {} out of bounds", row, col))
    }
}

impl<T> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, col)
            .unwrap_or_else(|| panic!("Index mut indices {}, {} out of bounds", row, col))
    }
}
