use std::ops::{Index, IndexMut};
use super::vector::Vec2;
#[allow(non_camel_case_types)]
type usize2 = Vec2<usize>;

#[derive(Debug, Clone)]
pub struct Matrix<Item> {
    internal_vector: Vec<Item>,
    size: usize2,
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

    pub fn size(self) -> usize2 {
        self.size
    }

    pub fn get_unchecked(&self, index: usize2) -> &Item {
        &self.internal_vector[index.1 * self.size.0 + index.0]
    }

    pub fn get_unchecked_mut(&mut self, index: usize2) -> &mut Item {
        &mut self.internal_vector[index.1 * self.size.0 + index.0]
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
}

impl<Item> Index<usize2> for Matrix<Item> {
    type Output = Item;

    fn index(&self, index: usize2) -> &Self::Output {
        if !(index < self.size) {
            panic!("Index should be below size of the matrix")
        }

        self.get_unchecked(index)
    }
}

impl<Item> IndexMut<usize2> for Matrix<Item> {
    fn index_mut(&mut self, index: usize2) -> &mut Self::Output {
        if !(index < self.size) {
            panic!("Index should be below size of the matrix")
        }

        self.get_unchecked_mut(index)
    }
}


#[cfg(test)]
mod tests {
    use rstest::{rstest, fixture};
    use super::*;

    #[rstest]
    fn creating() {
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