use std::ops::{Index, IndexMut};
use super::vector::Vec2;
#[allow(non_camel_case_types)]
type usize2 = Vec2<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<Item> {
    internal_vector: Vec<Item>,
    size: usize2,
}

impl<Item> Matrix<Item> where Item: Clone {
    pub fn filled(filler: &Item, size: usize2) -> Matrix<Item> {
        let length = size.0 * size.1;
        let mut internal_vector = Vec::<Item>::with_capacity(length);
        for _ in 0..length {
            internal_vector.push(filler.clone());
        }

        Self {
            internal_vector,
            size
        }
    }
}

impl<Item> Matrix<Item> {
    pub fn new<const W: usize, const H: usize>(array: [[Item; W]; H]) -> Matrix<Item> {
        let mut internal_vector = Vec::<Item>::with_capacity(W * H);
        for row in array {
            internal_vector.extend(Vec::from(row))
        }

        Self {
            internal_vector,
            size: Vec2(W, H),
        }
    }

    pub fn size(&self) -> usize2 {
        self.size
    }

    fn to_scalar_index(&self, index: usize2) -> usize {
        index.1 * self.size.0 + index.0
    }

    fn check_index(&self, index: usize2) {
        if !(index < self.size) {
            panic!("Index should be below size of the matrix")
        }
    }

    pub fn get_unchecked(&self, index: usize2) -> &Item {
        &self.internal_vector[self.to_scalar_index(index)]
    }

    pub fn get_unchecked_mut(&mut self, index: usize2) -> &mut Item {
        let index = self.to_scalar_index(index);
        &mut self.internal_vector[index]
    }

    pub fn get(&self, index: usize2) -> Option<&Item> {
        if index < self.size {
            Some(self.get_unchecked(index))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize2) -> Option<&mut Item> {
        if index < self.size {
            Some(self.get_unchecked_mut(index))
        } else {
            None
        }
    }

    pub fn swap(&mut self, first_index: usize2, second_index: usize2) {
        self.check_index(first_index);
        self.check_index(second_index);

        let first_index = self.to_scalar_index(first_index);
        let second_index = self.to_scalar_index(second_index);

        self.internal_vector.swap(first_index, second_index)
    }
}

impl<Item> Index<usize2> for Matrix<Item> {
    type Output = Item;

    fn index(&self, index: usize2) -> &Self::Output {
        self.check_index(index);
        self.get_unchecked(index)
    }
}

impl<Item> IndexMut<usize2> for Matrix<Item> {
    fn index_mut(&mut self, index: usize2) -> &mut Self::Output {
        self.check_index(index);
        self.get_unchecked_mut(index)
    }
}


#[cfg(test)]
mod tests {
    use rstest::{rstest, fixture};
    use super::*;

    #[rstest]
    fn creating_filled() {
        let matrix = Matrix::filled(&0, Vec2(4, 3));

        assert_eq!(matrix, Matrix {
            internal_vector: vec![
                0, 0, 0, 0,
                0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            size: Vec2(4, 3),
        })
    }

    #[rstest]
    fn creating_new() {
        let matrix = Matrix::new([
            [1,  4,  9],
            [16, 25, 36],
            [49, 64, 81]
        ]);

        assert_eq!(matrix.internal_vector, vec![1, 4, 9, 16, 25, 36, 49, 64, 81])
    }

    #[fixture]
    fn small_matrix() -> Matrix<i32> {
        Matrix {
            internal_vector: vec![0, 1, 2, 3],
            size: Vec2(2, 2),
        }
    }

    #[rstest]
    fn size(small_matrix: Matrix<i32>) {
        assert_eq!(small_matrix.size(), Vec2::<usize>(2, 2))
    }

    #[rstest]
    fn getting(small_matrix: Matrix<i32>) {
        assert_eq!(small_matrix.get(Vec2(1, 0)), Some(&1));
        assert_eq!(small_matrix.get(Vec2(0, 2)), None);
    }

    #[rstest]
    fn mut_getting(mut small_matrix: Matrix<i32>) {
        match small_matrix.get_mut(Vec2(1, 0)) {
            Some(x) => {
                *x = 8;
                assert_eq!(small_matrix.internal_vector[1], 8);
            }
            None => assert!(false)
        }
    }

    #[rstest]
    fn swapping(mut small_matrix: Matrix<i32>) {
        small_matrix.swap(Vec2(0, 0), Vec2(1, 1));
        assert_eq!(small_matrix.internal_vector, vec![3, 1, 2, 0]);
    }

    #[rstest]
    fn indexing(small_matrix: Matrix<i32>) {
        assert_eq!(small_matrix[Vec2(1, 0)], 1);
        assert_eq!(small_matrix[Vec2(0, 1)], 2);
    }

    #[rstest]
    fn mut_indexing(mut small_matrix: Matrix<i32>) {
        small_matrix[Vec2(1, 0)] = 8;
        assert_eq!(small_matrix[Vec2(1, 0)], 8);
    }
}