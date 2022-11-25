pub mod iter;
pub mod index;

#[derive(Debug, Eq, PartialEq)]
pub struct MatrixSize {
    pub h: usize,
    pub w: usize,
}

impl MatrixSize {
    pub fn new(h: usize, w: usize) -> Self {
        MatrixSize {
            h,
            w,
        }
    }
}

impl From<(usize, usize)> for MatrixSize {
    fn from(size: (usize, usize)) -> Self {
        Self {
            h: size.0,
            w: size.1,
        }
    }
}

pub enum MatrixRotation {
    None,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Matrix<T: Clone + Copy> {
    rows: Vec<Vec<T>>,
}

impl<T: Clone + Copy> Matrix<T> {
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        let first_row_len = rows.get(0).map_or(0, |row| row.len());
        if rows.iter().any(|row| row.len() != first_row_len) {
            panic!("All rows of a matrix must be the same length");
        }

        Self {
            rows
        }
    }

    pub fn filled_with(size: MatrixSize, element: T) -> Self {
        Self::new((0..size.h).into_iter().map(|_| (0..size.w).into_iter().map(|_| element).collect()).collect())
    }

    pub fn empty() -> Self {
        Self {
            rows: Vec::new()
        }
    }

    pub fn size(&self) -> MatrixSize {
        MatrixSize::new(self.rows.len(), self.rows.get(0).map_or(0, |row| row.len()))
    }

    pub fn rotate_clockwise(self, r: MatrixRotation) -> Self {
        match r {
            MatrixRotation::None => self,
            MatrixRotation::Deg90 => {
                let first_row_len = self.rows.get(0).map_or(0, |row| row.len());
                Self::new((0..first_row_len).into_iter().map(|index| {
                    self.rows.iter().map(|row| row[index]).rev().collect()
                }).collect())
            }
            MatrixRotation::Deg180 => {
                Self::new(self.rows.clone().into_iter().map(|row| row.into_iter().rev().collect()).rev().collect())
            }
            MatrixRotation::Deg270 => {
                let first_row_len = self.rows.get(0).map_or(0, |row| row.len());
                Self::new((0..first_row_len).into_iter().map(|index| {
                    self.rows.iter().map(|row| row[index]).collect()
                }).rev().collect())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "must be the same length")]
    fn uneven_row_sizes() {
        Matrix::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
            vec!(7, 8)
        ));
    }

    #[test]
    fn matrix_rotate_clockwise_0() {
        let result = Matrix::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
        )).rotate_clockwise(MatrixRotation::None);

        assert_eq!(result, Matrix::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
        )));
    }

    #[test]
    fn matrix_rotate_clockwise_90() {
        let result = Matrix::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
        )).rotate_clockwise(MatrixRotation::Deg90);

        assert_eq!(result, Matrix::new(vec!(
            vec!(4, 1),
            vec!(5, 2),
            vec!(6, 3),
        )));
    }

    #[test]
    fn matrix_rotate_clockwise_180() {
        let result = Matrix::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
        )).rotate_clockwise(MatrixRotation::Deg180);

        assert_eq!(result, Matrix::new(vec!(
            vec!(6, 5, 4),
            vec!(3, 2, 1),
        )));
    }

    #[test]
    fn matrix_rotate_clockwise_270() {
        let result = Matrix::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
        )).rotate_clockwise(MatrixRotation::Deg270);

        assert_eq!(result, Matrix::new(vec!(
            vec!(3, 6),
            vec!(2, 5),
            vec!(1, 4),
        )));
    }

    #[test]
    fn matrix_size() {
        let matrix = Matrix::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
        ));
        let size = matrix.size();

        assert_eq!(size, MatrixSize::new(2, 3));
    }

    #[test]
    fn matrix_size_empty() {
        let matrix: Matrix<usize> = Matrix::new(Vec::new());
        let size = matrix.size();

        assert_eq!(size, MatrixSize::new(0, 0));
    }

    #[test]
    fn matrix_filled_with() {
        let result = Matrix::filled_with(MatrixSize::new(3, 2), 9);

        assert_eq!(result, Matrix::new(vec!(
            vec!(9, 9),
            vec!(9, 9),
            vec!(9, 9),
        )));
    }
}
