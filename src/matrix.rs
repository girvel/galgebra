// use std::ops::{Index};
use super::vector::Vec2;
#[allow(non_camel_case_types)]
type usize2 = Vec2<usize>;

pub mod order {
    pub struct Normal;
    pub struct Mathematical;
}

#[derive(Debug, Clone)]
pub struct Matrix<Item> {
    internal_vector: Vec<Item>,
    size: usize2,
}

impl<Item> Matrix<Item> {
    pub fn size(self) -> usize2 {
        self.size
    }
}

// impl<'a, Item> Index<usize2> for Matrix<Item> {
//     type Output = Option<&'a Item>;
//
//     fn index(&self, index: usize2) -> Self::Output {
//         if index < self.size {
//             Some(&self.internal_vector[index.1 * self.size.0 + index.0])
//         }
//         else {
//             None
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use rstest::{rstest, fixture};
    use super::*;

    #[fixture]
    fn small_matrix() -> Matrix<i32> {
        Matrix {
            internal_vector: vec![1, 2, 3, 4],
            size: Vec2(2, 2),
        }
    }

    #[rstest]
    fn size(small_matrix: Matrix<i32>) {
        assert_eq!(small_matrix.size(), Vec2::<usize>(2, 2))
    }

    // #[rstest]
    // fn indexing(small_matrix: Matrix<i32>) {
    //     assert_eq!(small_matrix[Vec2(0, 1)], 2);
    //     assert_eq!(small_matrix[Vec2(1, 0)], 3);
    // }
}