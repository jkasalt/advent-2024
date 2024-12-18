use crate::{matrix::Matrix, pos::Pos};

fn width_height(input: &str) -> (usize, usize) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    (width, height)
}

pub fn parse_grid<F, T>(input: &str, map_each_char: F) -> Matrix<T>
where
    F: Fn(char) -> T,
{
    let (width, height) = width_height(input);
    let items = input
        .chars()
        .filter(char::is_ascii_graphic)
        .map(map_each_char);
    Matrix::new(items, width, height)
}

pub fn parse_grid_using_pos<F, T>(input: &str, map_each_char: F) -> Matrix<T>
where
    F: FnMut((Pos<usize>, char)) -> T,
{
    let (width, height) = width_height(input);
    let items = input
        .chars()
        .filter(char::is_ascii_graphic)
        .enumerate()
        .map(|(i, c)| (Pos::new(i % width, i / width), c))
        .map(map_each_char);
    Matrix::new(items, width, height)
}
