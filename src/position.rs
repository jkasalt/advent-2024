use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Position {
    fn from(value: (isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: isize::try_from(value.0).unwrap(),
            y: isize::try_from(value.1).unwrap(),
        }
    }
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: isize::try_from(value.0).unwrap(),
            y: isize::try_from(value.1).unwrap(),
        }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<isize> for Position {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<usize> for Position {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x * isize::try_from(rhs).unwrap(),
            y: self.y * isize::try_from(rhs).unwrap(),
        }
    }
}
