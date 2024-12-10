use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::matrix::Matrix;

#[aoc_generator(day10)]
fn parse(input: &str) -> Matrix<u32> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let items = input.chars().filter(char::is_ascii_graphic).map(|c| {
        if c == '.' {
            100
        } else {
            c.to_digit(10).unwrap()
        }
    });
    Matrix::new(items, width, height)
}

fn dfs(topography: &Matrix<u32>, (x, y): (isize, isize), found_tops: &mut HashSet<(isize, isize)>) {
    let cur = topography.get(x, y).unwrap();
    if *cur == 9 {
        found_tops.insert((x, y));
    }
    let neighbors = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
    neighbors
        .into_iter()
        .filter(|(nx, ny)| topography.get(*nx, *ny).is_some_and(|n| *n == 1 + cur))
        .for_each(|pos| {
            dfs(topography, pos, found_tops);
        });
}

#[aoc(day10, part1)]
fn part1(topography: &Matrix<u32>) -> usize {
    topography
        .iter_pos()
        .filter(|(_, n)| **n == 0)
        .map(|(pos, _)| (pos.0.try_into().unwrap(), pos.1.try_into().unwrap()))
        .map(|pos| {
            let mut found_tops = HashSet::new();
            dfs(topography, pos, &mut found_tops);
            found_tops.len()
        })
        .sum()
}

fn dfs2(topography: &Matrix<u32>, (x, y): (isize, isize)) -> usize {
    let cur = topography.get(x, y).unwrap();
    if *cur == 9 {
        return 1;
    }
    let neighbors = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
    neighbors
        .into_iter()
        .filter(|(nx, ny)| topography.get(*nx, *ny).is_some_and(|n| *n == 1 + cur))
        .map(|pos| dfs2(topography, pos))
        .sum()
}

#[aoc(day10, part2)]
fn part2(topography: &Matrix<u32>) -> usize {
    topography
        .iter_pos()
        .filter(|(_, n)| **n == 0)
        .map(|(pos, _)| (pos.0.try_into().unwrap(), pos.1.try_into().unwrap()))
        .map(|pos| dfs2(topography, pos))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    const EXAMPLE2: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
";

    const EXAMPLE3: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

    const EXAMPLE4: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
";

    #[test_case(EXAMPLE1 => 36; "normal example")]
    #[test_case(EXAMPLE2 => 2; "simple example")]
    #[test_case(EXAMPLE3 => 4; "unreachable top")]
    #[test_case(EXAMPLE4 => 3; "two trailheads")]
    fn part1_example(input: &str) -> usize {
        part1(&parse(input))
    }

    #[test_case(EXAMPLE1 => 81; "normal example")]
    fn part2_example(input: &str) -> usize {
        part2(&parse(input))
    }
}
