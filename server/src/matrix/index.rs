use std::ops::{Index, IndexMut, Range, RangeInclusive};
use crate::matrix::Matrix;
use crate::position::{UNamedPosition, UPosition};

trait Slice<Idx> {
    fn slice(self, index: Idx) -> Self;
}

impl<T: Clone + Copy> Slice<Range<UPosition>> for Matrix<T> {
    fn slice(self, index: Range<UPosition>) -> Self {
        Matrix::new(self.rows[index.start.1..index.end.1].iter()
            .map(|row| row[index.start.0..index.end.0].to_vec())
            .collect())
    }
}

impl<T: Clone + Copy> Slice<RangeInclusive<UPosition>> for Matrix<T> {
    fn slice(self, index: RangeInclusive<UPosition>) -> Self {
        Matrix::new(self.rows[index.start().1..=index.end().1].iter()
            .map(|row| row[index.start().0..=index.end().0].to_vec())
            .collect())
    }
}

impl<T: Clone + Copy> Matrix<T> {
    pub fn replace(&mut self, start: UNamedPosition, items: Matrix<T>) {
        items.into_iter().for_each(|(item, position)| {
            self[(start.x + position.0, start.y + position.1)] = item;
        });
    }
}

impl<T: Clone + Copy> Index<UPosition> for Matrix<T> {
    type Output = T;

    fn index(&self, index: UPosition) -> &Self::Output {
        &self.rows[index.1][index.0]
    }
}

impl<T: Clone + Copy> IndexMut<UPosition> for Matrix<T> {
    fn index_mut(&mut self, index: UPosition) -> &mut Self::Output {
        &mut self.rows[index.1][index.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_index() {
        let matrix = Matrix::new(vec!(
            vec!(6, 5, 4),
            vec!(3, 2, 1),
        ));

        assert_eq!(matrix[(0, 1)], 3);
        assert_eq!(matrix[(1, 0)], 5);
        assert_eq!(matrix[(2, 0)], 4);
        assert_eq!(matrix[(0, 0)], 6);
        assert_eq!(matrix[(2, 1)], 1);
    }

    #[test]
    fn matrix_replace() {
        let mut matrix = Matrix::new(vec!(
            vec!(16, 15, 14, 13),
            vec!(12, 11, 10, 9),
            vec!(8, 7, 6, 5),
            vec!(4, 3, 2, 1),
        ));

        matrix.replace(UNamedPosition::new(1, 0), Matrix::new(vec!(
            vec!(99, 98),
            vec!(97, 96),
            vec!(95, 94),
        )));

        assert_eq!(matrix, Matrix::new(vec!(
            vec!(16, 99, 98, 13),
            vec!(12, 97, 96, 9),
            vec!(8, 95, 94, 5),
            vec!(4, 3, 2, 1),
        )));
    }

    #[test]
    fn matrix_slice() {
        let matrix = Matrix::new(vec!(
            vec!(1, 2, 3, 4),
            vec!(5, 6, 7, 8),
            vec!(9, 10, 11, 12),
            vec!(13, 14, 15, 16),
        ));

        assert_eq!(matrix.slice((0, 1)..(2, 3)), Matrix::new(vec!(
            vec!(5, 6),
            vec!(9, 10),
        )));
    }

    #[test]
    fn matrix_slice_inclusive() {
        let matrix = Matrix::new(vec!(
            vec!(1, 2, 3, 4),
            vec!(5, 6, 7, 8),
            vec!(9, 10, 11, 12),
            vec!(13, 14, 15, 16),
        ));

        assert_eq!(matrix.slice((0, 1)..=(2, 3)), Matrix::new(vec!(
            vec!(5, 6, 7),
            vec!(9, 10, 11),
            vec!(13, 14, 15),
        )));
    }

    #[test]
    fn matrix_index_mut() {
        let mut matrix = Matrix::new(vec!(
            vec!(6, 5, 4),
            vec!(3, 2, 1),
        ));

        matrix[(1, 1).into()] = 9;
        matrix[(2, 0).into()] = 100;

        assert_eq!(matrix, Matrix::new(vec!(
            vec!(6, 5, 100),
            vec!(3, 9, 1),
        )));
    }
}
