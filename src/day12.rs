use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::matrix::Matrix;

type Pos = (isize, isize);

#[derive(Debug, Clone, Default)]
struct RegionInfo {
    perimeter: HashSet<Border>,
    area: usize,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Border {
    from: Pos,
    to: Pos,
}

impl Border {
    fn neighbors(&self) -> [Self; 2] {
        match (self.to.0 - self.from.0, self.to.1 - self.from.1) {
            (1 | -1, 0) => [
                Self {
                    from: (self.from.0, self.from.1 + 1),
                    to: (self.to.0, self.to.1 + 1),
                },
                Self {
                    from: (self.from.0, self.from.1 - 1),
                    to: (self.to.0, self.to.1 - 1),
                },
            ],
            (0, 1 | -1) => [
                Self {
                    from: (self.from.0 + 1, self.from.1),
                    to: (self.to.0 + 1, self.to.1),
                },
                Self {
                    from: (self.from.0 - 1, self.from.1),
                    to: (self.to.0 - 1, self.to.1),
                },
            ],
            x => panic!("Unexpected diff: {x:?}"),
        }
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Matrix<char> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let items = input.chars().filter(char::is_ascii_graphic);
    Matrix::new(items, width, height)
}

fn dfs_one_region(
    plots: &Matrix<char>,
    (x, y): Pos,
    region_info: &mut RegionInfo,
    visited: &mut HashSet<Pos>,
) {
    if visited.contains(&(x, y)) {
        return;
    }
    let color = plots.get(x, y).unwrap();
    visited.insert((x, y));
    region_info.area += 1;
    let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    for (nx, ny) in neighbors {
        if plots.get(nx, ny) == Some(color) {
            dfs_one_region(plots, (nx, ny), region_info, visited);
        } else {
            region_info.perimeter.insert(Border {
                from: (x, y),
                to: (nx, ny),
            });
        }
    }
}

fn regions(plots: &Matrix<char>) -> Vec<RegionInfo> {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    for x in 0..plots.width() {
        for y in 0..plots.height() {
            let (x, y) = (x.try_into().unwrap(), y.try_into().unwrap());
            let mut region_info = RegionInfo::default();
            dfs_one_region(plots, (x, y), &mut region_info, &mut visited);
            regions.push(region_info);
        }
    }
    regions
}

#[aoc(day12, part1)]
fn part1(plots: &Matrix<char>) -> usize {
    regions(plots)
        .iter()
        .map(|region_info| region_info.area * region_info.perimeter.len())
        .sum()
}

#[aoc(day12, part2)]
fn part2(plots: &Matrix<char>) -> usize {
    regions(plots)
        .into_iter()
        .map(|region| {
            let mut visited = HashSet::new();
            let mut count = 0;
            loop {
                let Some(next_start) = region
                    .perimeter
                    .iter()
                    .find(|border| !visited.contains(*border))
                else {
                    return count * region.area;
                };
                count += 1;
                let mut queue = vec![*next_start];
                while let Some(border) = queue.pop() {
                    visited.insert(border);
                    border
                        .neighbors()
                        .into_iter()
                        .filter(|n| !visited.contains(n) && region.perimeter.contains(n))
                        .for_each(|n| queue.push(n));
                }
            }
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = "AAAA
BBCD
BBCC
EEEC
";
    const EXAMPLE2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const EXAMPLE3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test_case(EXAMPLE1 => 140; "small example")]
    #[test_case(EXAMPLE2 => 772; "medium example")]
    #[test_case(EXAMPLE3 => 1930; "large example")]
    fn part1_example(input: &str) -> usize {
        part1(&parse(input))
    }

    #[test_case(EXAMPLE1 => 80; "small example")]
    #[test_case("AAAA" => 16; "mini example")]
    #[test_case(EXAMPLE2 => 436; "medium example")]
    #[test_case(EXAMPLE3 => 1206; "large example")]
    fn part2_example(input: &str) -> usize {
        part2(&parse(input))
    }
}
