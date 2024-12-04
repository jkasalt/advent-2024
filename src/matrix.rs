use std::fmt;
use std::ops;

use crate::position::Position;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Matrix<T> {
    pub vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    /// # Panics
    /// if the number of items in the collection is not equal to the width times the height
    pub fn new<I>(items: I, width: usize, height: usize) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let vec: Vec<T> = items.into_iter().collect();
        assert_eq!(vec.len(), width * height);
        Self { vec, width, height }
    }

    #[must_use]
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) else {
            return None;
        };
        if x >= self.width || y >= self.height {
            return None;
        };
        self.vec.get(x + y * self.width)
    }

    #[must_use]
    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) else {
            return None;
        };
        if x >= self.width || y >= self.height {
            return None;
        };
        self.vec.get_mut(x + y * self.width)
    }

    pub fn rook_neighbor_indices(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut res = Vec::new();
        if y > 0 {
            res.push((x, y - 1));
        }
        if x > 0 {
            res.push((x - 1, y));
        }
        if y < self.height - 1 {
            res.push((x, y + 1));
        }
        if x < self.width - 1 {
            res.push((x + 1, y));
        }
        res.into_iter()
    }

    #[must_use]
    pub fn neighbor_indices(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let mut x_vec = vec![x];
        let mut y_vec = vec![y];
        if y > 0 {
            y_vec.push(y - 1);
        }
        if y < self.height - 1 {
            y_vec.push(y + 1);
        }
        if x > 0 {
            x_vec.push(x - 1);
        }
        if x < self.width - 1 {
            x_vec.push(x + 1);
        }
        for yy in y_vec {
            for &xx in &x_vec {
                if xx == x && yy == y {
                    continue;
                }
                result.push((xx, yy));
            }
        }
        result
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.height * self.width
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// # Panics
    /// if the other's width is not equal to self's width
    pub fn insert_row(&mut self, mut other: Self, at: usize) {
        assert!(other.width() == self.width());
        let idx = at * self.width();
        self.height += other.height;
        let mut after = self.vec.split_off(idx);
        self.vec.append(&mut other.vec);
        self.vec.append(&mut after);
    }

    #[must_use]
    pub fn expand_contour(self, n: usize, val: T) -> Self
    where
        T: Clone,
    {
        let height = self.height + n * 2;
        let width = self.width + n * 2;

        let mut new = Self {
            vec: vec![val; height * width],
            height,
            width,
        };
        for x in 0..self.width {
            for y in 0..self.height {
                new[(x + n, y + n)] = self[(x, y)].clone();
            }
        }
        new
    }

    #[must_use]
    pub fn new_default(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self {
            vec: std::iter::repeat_with(T::default)
                .take(height * width)
                .collect(),
            height,
            width,
        }
    }

    pub fn new_with<F>(width: usize, height: usize, f: F) -> Self
    where
        F: Fn() -> T,
    {
        Self {
            vec: std::iter::repeat_with(f).take(height * width).collect(),
            height,
            width,
        }
    }

    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let idx_a = a.1 * self.width() + a.0;
        let idx_b = b.1 * self.width() + b.0;
        self.vec.swap(idx_a, idx_b);
    }

    pub fn position<F>(&self, f: F) -> Option<(usize, usize)>
    where
        F: FnMut(&T) -> bool,
    {
        self.vec
            .iter()
            .position(f)
            .map(|pos| (pos % self.width(), pos / self.width()))
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.vec
            .iter()
            .enumerate()
            .map(|(i, t)| ((i % self.width(), i / self.width()), t))
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        writeln!(f)?;
        for item in &self.vec {
            write!(f, "{item:?}")?;
            count += 1;
            if count == self.width {
                count = 0;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T> ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(
            !(x > self.width() - 1 || y > self.height() - 1),
            "Index ({}, {}) out of range for Matrix with size ({}, {})",
            y,
            x,
            self.height,
            self.width
        );
        &self.vec[x + y * self.width()]
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(
            !(x > self.width() - 1 || y > self.height() - 1),
            "Index ({}, {}) out of range for Matrix with size ({}, {})",
            y,
            x,
            self.height,
            self.width
        );
        &mut self.vec[x + y * self.width]
    }
}

impl<T> ops::Index<(&usize, &usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (&usize, &usize)) -> &Self::Output {
        assert!(
            !(*x > self.width() - 1 || *y > self.height() - 1),
            "Index ({}, {}) out of range for Matrix with size ({}, {})",
            y,
            x,
            self.height,
            self.width
        );
        &self.vec[x + y * self.width()]
    }
}

impl<T> ops::IndexMut<(&usize, &usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (&usize, &usize)) -> &mut Self::Output {
        assert!(
            !(*x > self.width() - 1 || *y > self.height() - 1),
            "Index ({}, {}) out of range for Matrix with size ({}, {})",
            y,
            x,
            self.height,
            self.width
        );
        &mut self.vec[x + y * self.width]
    }
}

#[cfg(test)]
mod test_matrix {
    use super::*;
    #[test]
    fn test_swap() {
        let items = 0..6;
        let mut matrix = Matrix::new(items, 3, 2);
        matrix.swap((0, 0), (0, 1));
        assert_eq!(matrix[(0, 0)], 3);
        assert_eq!(matrix[(0, 1)], 0);
    }

    #[test]
    fn test_iter_pos() {
        let items = 'a'..='f';
        let matrix = Matrix::new(items, 3, 2);
        let mut iter_pos = matrix.iter_pos();
        assert_eq!(
            Some(((0, 0), 'a')),
            iter_pos.next().map(|(pos, c)| (pos, *c))
        );
        assert_eq!(
            Some(((1, 0), 'b')),
            iter_pos.next().map(|(pos, c)| (pos, *c))
        );
        assert_eq!(
            Some(((2, 0), 'c')),
            iter_pos.next().map(|(pos, c)| (pos, *c))
        );
        assert_eq!(
            Some(((0, 1), 'd')),
            iter_pos.next().map(|(pos, c)| (pos, *c))
        );
        assert_eq!(
            Some(((1, 1), 'e')),
            iter_pos.next().map(|(pos, c)| (pos, *c))
        );
        assert_eq!(
            Some(((2, 1), 'f')),
            iter_pos.next().map(|(pos, c)| (pos, *c))
        );
        assert_eq!(None, iter_pos.next());
    }
}
