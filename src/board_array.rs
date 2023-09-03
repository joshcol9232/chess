use std::ops::{Index, IndexMut};
use std::slice::{IterMut, Iter};
use std::convert::From;

/// \brief A 2D array with [i, j] indexing.
// More just an excuse to try out generics :)
pub struct BoardArray<T> {
    data: [T; 8 * 8]
}

impl<T> BoardArray<T> {
    pub fn new(data: [T; 8 * 8]) -> Self { Self { data } }
    pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }
}

pub fn xy_to_board_idx(xy: [u8; 2]) -> usize {
    //  0  1  2  3  4  5  6  7  8
    //  9 10 11 ...
    (xy[0] + xy[1] * 8) as usize
}

impl<T> Index<[u8; 2]> for BoardArray<T> {
    type Output = T;

    fn index(&self, idx: [u8; 2]) -> &Self::Output {
        &self.data[xy_to_board_idx(idx)]
    }
}

impl<T> IndexMut<[u8; 2]> for BoardArray<T> {
    fn index_mut(&mut self, idx: [u8; 2]) -> &mut Self::Output {
        &mut self.data[xy_to_board_idx(idx)]
    }
}

impl<T> From<[T; 8 * 8]> for BoardArray<T> {
    fn from(data: [T; 8 * 8]) -> Self {
        Self { data }
    }
}

