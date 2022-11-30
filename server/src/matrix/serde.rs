use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer};
use serde::de::{SeqAccess, Visitor};
use crate::matrix::Matrix;

struct MatrixVisitor<'de, T: Clone + Copy + Deserialize<'de>> {
    marker: PhantomData<&'de T>
}

impl<'de, T: Clone + Copy + Deserialize<'de>> MatrixVisitor<'de, T> {
    fn new() -> Self {
        Self {
            marker: PhantomData
        }
    }
}

impl<'de, T: Clone + Copy + Deserialize<'de>> Visitor<'de> for MatrixVisitor<'de, T> {
    type Value = Matrix<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a matrix (list within a list)")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'de> {
        let mut rows = Vec::new();

        while let Some(row) = seq.next_element()? {
            rows.push(row);
        }

        Ok(Matrix::new(rows))
    }
}

impl<'de, T: Clone + Copy + Deserialize<'de> + 'de> Deserialize<'de> for Matrix<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_seq(MatrixVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct StructWithMatrix {
        content: Matrix<u32>
    }

    #[test]
    fn deserialize() {
        let json = "{\"content\": [[1, 2, 3], [4, 5, 6]]}";
        let result: StructWithMatrix = serde_json::from_str(json).unwrap();

        assert_eq!(result.content, Matrix::new(vec!(
            vec!(1, 2, 3),
            vec!(4, 5, 6),
        )))
    }

    #[test]
    #[should_panic(expected = "expected a sequence")]
    fn deserialize_not_enough_dimensions() {
        let json = "{\"content\": [1, 2, 3]}";
        let result: StructWithMatrix = serde_json::from_str(json).unwrap();
    }

    #[test]
    #[should_panic(expected = "expected a matrix (list within a list)")]
    fn deserialize_incorrect_type() {
        let json = "{\"content\": \"test\"}";
        let result: StructWithMatrix = serde_json::from_str(json).unwrap();
    }
}
