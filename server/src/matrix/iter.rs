use std::iter::FusedIterator;
use itertools::Itertools;

use crate::matrix::{Matrix, MatrixSize};
use crate::position::{UNamedPosition, UPosition};

impl<T: Clone + Copy> IntoIterator for Matrix<T> {
    type Item = (T, UPosition);
    type IntoIter = MatrixIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIter::new(self)
    }
}

impl<T: Clone + Copy> FromIterator<(T, UPosition)> for Matrix<T> {
    fn from_iter<I: IntoIterator<Item=(T, UPosition)>>(iter: I) -> Self {
        let iter_contents: Vec<(T, UPosition)> = iter.into_iter().collect();
        let row_size = iter_contents.iter()
            .filter(|(_item, position)| position.1 == 0)
            .sorted_by(|(_item_a, position_a), (_item_b, position_b)| Ord::cmp(&position_b.0, &position_a.0))
            .next()
            .map_or(0, |(_item, position)| position.0 + 1);

        if row_size <= 0 {
            // With how this iterator is implemented, if the width of a matrix is 0, we don't have a
            // way to know how many rows it had from the iterator. Therefore, we return an empty matrix.
            Matrix::empty()
        } else {
            Matrix::new(iter_contents.into_iter()
                .chunks(row_size).into_iter()
                .map(|chunks| chunks.map(|(item, _position)| item).collect_vec())
                .collect())
        }
    }
}

pub struct MatrixIter<T: Clone + Copy> {
    matrix: Matrix<T>,
    pub size: MatrixSize,
    position: UNamedPosition,
}

impl<T: Clone + Copy> MatrixIter<T> {
    pub fn new(matrix: Matrix<T>) -> Self {
        let size = matrix.size();

        MatrixIter {
            matrix,
            size,
            position: (0, 0).into(),
        }
    }
}

impl<T: Clone + Copy> Iterator for MatrixIter<T> {
    type Item = (T, UPosition);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.y >= self.size.h || self.size.w <= 0 {
            None
        } else {
            let item = self.matrix[(self.position.x, self.position.y)];
            let item_position = self.position.clone();

            if self.position.x >= self.size.w - 1 {
                self.position.x = 0;
                self.position.y += 1;
            } else {
                self.position.x += 1;
            }

            Some((item, item_position.into()))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let item_count = self.size.h * self.size.w;
        let items_passed = (self.position.y * self.size.w) + self.position.x;
        let items_remaining = item_count - items_passed;

        (items_remaining, Some(items_remaining))
    }
}

impl<T: Clone + Copy> FusedIterator for MatrixIter<T> {}

impl<T: Clone + Copy> ExactSizeIterator for MatrixIter<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_iter() {
        let mut iter = Matrix::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
        )).into_iter();

        assert_eq!(iter.next(), Some((1, (0, 0))));
        assert_eq!(iter.next(), Some((2, (1, 0))));
        assert_eq!(iter.next(), Some((3, (2, 0))));
        assert_eq!(iter.next(), Some((4, (0, 1))));
        assert_eq!(iter.next(), Some((5, (1, 1))));
        assert_eq!(iter.next(), Some((6, (2, 1))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn matrix_from_iter() {
        let mapped_matrix: Matrix<isize> = Matrix::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
        )).into_iter().map(|(item, position)| (item * 2, position)).collect();

        assert_eq!(mapped_matrix, Matrix::new(vec!(
            vec!(2, 4, 6),
            vec!(8, 10, 12),
        )));
    }

    #[test]
    fn matrix_from_iter_empty_matrix() {
        let result: Matrix<isize> = Matrix::new(Vec::new()).into_iter().collect();

        assert_eq!(result, Matrix::new(Vec::new()));
    }

    #[test]
    fn matrix_from_iter_empty_rows() {
        let result: Matrix<isize> = Matrix::new(vec!(
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )).into_iter().collect();

        assert_eq!(result, Matrix::new(Vec::new()));
    }

    #[test]
    fn matrix_iter_empty_matrix() {
        let mut iter: MatrixIter<usize> = Matrix::new(Vec::new()).into_iter();

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn matrix_iter_empty_rows() {
        let mut iter: MatrixIter<usize> = Matrix::new(vec!(
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )).into_iter();

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn matrix_iter_size_hint() {
        let mut iter = Matrix::new(vec!(
            vec!(1, 2),
            vec!(3, 4),
            vec!(5, 6),
            vec!(7, 8),
        )).into_iter();

        assert_eq!(iter.size_hint(), (8, Some(8)));
        iter.next();
        assert_eq!(iter.size_hint(), (7, Some(7)));
        iter.next();
        assert_eq!(iter.size_hint(), (6, Some(6)));
        iter.next();
        assert_eq!(iter.size_hint(), (5, Some(5)));
        iter.next();
        assert_eq!(iter.size_hint(), (4, Some(4)));
    }
}
