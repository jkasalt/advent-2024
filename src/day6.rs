use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone)]
struct Info {
    width: i32,
    height: i32,
    wall_pos: Vec<(i32, i32)>,
    s_pos: (i32, i32),
    s_dir: Direction,
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
        .filter_map(|((x, y), c)| (c == '#').then_some((x, y)))
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
    visited(info).tiles.len()
}

struct VisitedInfo {
    tiles: HashSet<(i32, i32)>,
    does_loop: bool,
}

fn visited(info: &Info) -> VisitedInfo {
    let mut visited = HashSet::new();
    let mut visited_with_direction = HashSet::new();
    let mut pos = info.s_pos;
    let mut dir = info.s_dir;
    let in_bounds =
        |pos: (i32, i32)| 0 <= pos.0 && pos.0 < info.width && 0 <= pos.1 && pos.1 < info.height;
    while in_bounds(pos) {
        if visited_with_direction.contains(&(pos, dir)) {
            return VisitedInfo {
                tiles: visited,
                does_loop: true,
            };
        }
        visited.insert(pos);
        visited_with_direction.insert((pos, dir));
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

#[aoc(day6, part2)]
fn part2(info: &Info) -> usize {
    let normally_visited = visited(info).tiles;
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
