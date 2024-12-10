use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Tiles = HashSet<TileAndDirection>;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Eq, Hash, PartialEq)]
struct TileAndDirection {
    pos: (i32, i32),
    dir_visited: Direction,
}

#[derive(Debug, Clone)]
struct Info {
    width: i32,
    height: i32,
    wall_pos: Vec<(i32, i32)>,
    s_pos: (i32, i32),
    s_dir: Direction,
}

struct VisitedInfo {
    tiles: Tiles,
    does_loop: bool,
}

impl VisitedInfo {
    fn unique_tiles(&self) -> HashSet<(i32, i32)> {
        self.tiles.iter().map(|t| t.pos).collect()
    }
}

fn visited(info: &Info) -> VisitedInfo {
    let mut visited = HashSet::new();
    let mut pos = info.s_pos;
    let mut dir = info.s_dir;
    let in_bounds =
        |(x, y): (i32, i32)| (0..info.width).contains(&x) && (0..info.height).contains(&y);
    while in_bounds(pos) {
        let tile_and_direction = TileAndDirection {
            pos,
            dir_visited: dir,
        };
        if visited.contains(&tile_and_direction) {
            return VisitedInfo {
                tiles: visited,
                does_loop: true,
            };
        }
        visited.insert(tile_and_direction);
        // try to go forward
        let maybe_next_pos = match dir {
            Direction::N => (pos.0, pos.1 - 1),
            Direction::E => (pos.0 + 1, pos.1),
            Direction::S => (pos.0, pos.1 + 1),
            Direction::W => (pos.0 - 1, pos.1),
        };
        // if next pos is a wall instead turn 90 degrees instead
        if info.wall_pos.contains(&maybe_next_pos) {
            dir = match dir {
                Direction::N => Direction::E,
                Direction::E => Direction::S,
                Direction::S => Direction::W,
                Direction::W => Direction::N,
            };
        } else {
            // otherwise just go forward
            pos = maybe_next_pos;
        }
    }
    VisitedInfo {
        tiles: visited,
        does_loop: false,
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Info {
    let width = input
        .lines()
        .next()
        .map(str::len)
        .and_then(|n| n.try_into().ok())
        .unwrap();
    let height = input.lines().count().try_into().unwrap();
    let mut pos_iter = input
        .chars()
        .filter(char::is_ascii_graphic)
        .enumerate()
        .map(|(i, c)| {
            let i = i32::try_from(i).unwrap();
            ((i % width, i / width), c)
        });
    let wall_pos = pos_iter
        .clone()
        .filter(|(_, c)| *c == '#')
        .map(|(pos, _)| pos)
        .collect();
    let (s_pos, s_dir) = pos_iter
        .find_map(|((x, y), c)| match c {
            '^' => Some(((x, y), Direction::N)),
            '>' => Some(((x, y), Direction::E)),
            'v' => Some(((x, y), Direction::S)),
            '<' => Some(((x, y), Direction::W)),
            _ => None,
        })
        .unwrap();
    Info {
        width,
        height,
        wall_pos,
        s_pos,
        s_dir,
    }
}

#[aoc(day6, part1)]
fn part1(info: &Info) -> usize {
    visited(info).unique_tiles().len()
}

#[aoc(day6, part2)]
fn part2(info: &Info) -> usize {
    let normally_visited = visited(info).unique_tiles();
    normally_visited
        .into_iter()
        .filter(|tile| {
            let mut added_wall = info.wall_pos.clone();
            added_wall.push(*tile);
            let info = Info {
                wall_pos: added_wall,
                ..info.clone()
            };
            visited(&info).does_loop
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test1() {
        assert_eq!(part1(&parse(EXAMPLE)), 41);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(EXAMPLE)), 6);
    }
}
