use std::ops::Add;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T>(pub T, pub T);

impl<T> fmt::Display for Vec2<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}; {}}}", self.0, self.1)
    }
}

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
    fn formatting() {
        let v = Vec2(0, 3);
        assert_eq!(format!("{v:?}"), "Vec2(0, 3)");
    }

    #[test]
    fn copying() {
        let original = Vec2(1, 3);
        let _copy = original;
        assert_eq!(original, Vec2(1, 3));
    }

    #[test]
    fn displaying() {
        let v = Vec2(2, 1);
        assert_eq!(format!("{v}"), "{2; 1}");
    }

    #[test]
    fn equality() {
        assert_eq!(Vec2(1, 2), Vec2(1, 2));
        assert_ne!(Vec2(1, 2), Vec2(2, 1));
    }

    #[test]
    fn addition() {
        let result = Vec2(1, 0) + Vec2(-2, 3);
        assert_eq!(result, Vec2(-1, 3));
    }
}
