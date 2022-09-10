use std::cmp::Ordering;
use std::ops::Add;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vec2<T>(pub T, pub T);

impl<T> fmt::Display for Vec2<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}; {}}}", self.0, self.1)
    }
}

impl<T> PartialOrd for Vec2<T> where Vec2<T>: Eq, T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        }
        else if self > other {
            Some(Ordering::Greater)
        }
        else if self < other {
            Some(Ordering::Less)
        }
        else {
            None
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0 && self.1 < other.1
    }

    fn le(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 <= other.1
    }

    fn gt(&self, other: &Self) -> bool {
        self.0 > other.0 && self.1 > other.1
    }

    fn ge(&self, other: &Self) -> bool {
        self.0 >= other.0 && self.1 >= other.1
    }
}

impl<Lhs, Rhs, Result> Add<Vec2<Rhs>>
for Vec2<Lhs>
    where Lhs: Add<Rhs, Output=Result> {
    type Output = Vec2<Result>;

    fn add(self, rhs: Vec2<Rhs>) -> Self::Output {
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
    fn ordering() {
        assert!(Vec2(2, 2) > Vec2(1, 1));
        assert!(Vec2(2, 1) >= Vec2(1, 1));
    }

    #[test]
    fn addition() {
        let result = Vec2(1, 0) + Vec2(-2, 3);
        assert_eq!(result, Vec2(-1, 3));
    }
}
