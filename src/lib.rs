use std::{
    ops::Add,
    cmp::Eq,
};

#[derive(Debug)]
pub struct Vec2<T>(T, T);

impl<T> PartialEq for Vec2<T> where T: Eq {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<T> Eq for Vec2<T> where T: Eq {}

impl<LHS, RHS, RESULT> Add<Vec2<RHS>>
for Vec2<LHS>
where LHS: Add<RHS, Output=RESULT> {
    type Output = Vec2<RESULT>;

    fn add(self, rhs: Vec2<RHS>) -> Self::Output {
        Vec2(
            self.0 + rhs.0,
            self.1 + rhs.1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(Vec2(1, 2), Vec2(1, 2));
        assert_ne!(Vec2(1, 2), Vec2(2, 1));
    }

    #[test]
    fn formatting() {
        let v = Vec2(0, 3);
        assert_eq!(format!("{v:?}"), "Vec2(0, 3)");
    }

    #[test]
    fn addition() {
        let result = Vec2(1, 0) + Vec2(-2, 3);
        assert_eq!(result, Vec2(-1, 3));
    }
}
