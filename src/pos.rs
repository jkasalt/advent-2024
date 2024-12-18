use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pos<T> {
    pub fn to_usize(&self) -> Pos<usize>
    where
        T: TryInto<usize> + Copy + Clone,
        T::Error: Debug,
    {
        Pos {
            x: self.x.try_into().unwrap(),
            y: self.y.try_into().unwrap(),
        }
    }
}

impl<T> Pos<T> {
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Pos<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Pos<T>> for (T, T) {
    fn from(val: Pos<T>) -> Self {
        (val.x, val.y)
    }
}

impl<T> Add for Pos<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Mul<T> for Pos<T>
where
    T: Mul<Output = T> + Clone + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Sub for Pos<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
