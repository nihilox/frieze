use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2D<T> {
    array: Vec<T>,
    num_rows: usize,
    num_cols: usize,
}

impl<T> Array2D<T> {
    pub fn from_row_major(elements: Vec<T>, num_rows: usize, num_cols: usize) -> Option<Self>
    where
        T: Clone,
    {
        if num_rows * num_cols == elements.len() {
            Some(Array2D {
                array: elements,
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

    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.get_index(row, column).map(|index| &self.array[index])
    }

    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.get_index(row, column)
            .map(move |index| &mut self.array[index])
    }

    pub fn rows(&self) -> std::slice::Chunks<'_, T> {
        self.array.chunks(self.num_cols)
    }

    fn get_index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.num_rows && col < self.num_cols {
            Some(row * self.num_cols() + col)
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Array2D<T> {
    type Output = T;
    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        self.get(row, column)
            .unwrap_or_else(|| panic!("Index indices {}, {} out of bounds", row, column))
    }
}

impl<T> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        self.get_mut(row, column)
            .unwrap_or_else(|| panic!("Index mut indices {}, {} out of bounds", row, column))
    }
}
