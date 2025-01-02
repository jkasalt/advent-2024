use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{matrix::Matrix, pos::Pos, utils};

#[derive(Hash, Ord, PartialOrd, Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    const fn delta(self) -> Pos<isize> {
        match self {
            Self::N => Pos::new(0, -1),
            Self::E => Pos::new(1, 0),
            Self::S => Pos::new(0, 1),
            Self::W => Pos::new(-1, 0),
        }
    }

    const fn turn_clockwise(self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }

    const fn turn_counter_clockwise(self) -> Self {
        match self {
            Self::N => Self::W,
            Self::E => Self::N,
            Self::S => Self::E,
            Self::W => Self::S,
        }
    }
}

struct Info {
    maze: Matrix<char>,
    start_pos: Pos<isize>,
    end_pos: Pos<isize>,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Info {
    let mut start_pos = None;
    let mut end_pos = None;
    let maze = utils::parse_grid_using_pos(input, |pos, c| match c {
        'S' => {
            start_pos = Some(pos.to_isize());
            '.'
        }
        'E' => {
            end_pos = Some(pos.to_isize());
            '.'
        }
        '.' => '.',
        '#' => '#',
        x => panic!("unexpected char in input: {x}"),
    });
    Info {
        start_pos: start_pos.expect("start pos should be set"),
        end_pos: end_pos.expect("end pos should be set"),
        maze,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct State {
    pos: Pos<isize>,
    dir: Direction,
    score: u64,
}

type Node = (Pos<isize>, Direction);

impl State {
    fn next_states(&self) -> [Self; 3] {
        [
            Self {
                pos: self.pos + self.dir.delta(),
                dir: self.dir,
                score: self.score + 1,
            },
            Self {
                pos: self.pos,
                dir: self.dir.turn_clockwise(),
                score: self.score + 1000,
            },
            Self {
                pos: self.pos,
                dir: self.dir.turn_counter_clockwise(),
                score: self.score + 1000,
            },
        ]
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.pos.x.cmp(&other.pos.x))
            .then_with(|| self.pos.y.cmp(&other.pos.y))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Dijkstra {
    real_end_state: Option<State>,
    prev: HashMap<Node, Vec<Node>>,
}

fn dijkstra(info: &Info) -> Dijkstra {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut prev: HashMap<Node, Vec<Node>> = HashMap::new();
    let start_state = State {
        pos: info.start_pos,
        dir: Direction::E,
        score: 0,
    };
    dist.insert((start_state.pos, start_state.dir), start_state.score);
    heap.push(start_state);

    while let Some(state) = heap.pop() {
        let next_states: Vec<_> = state
            .next_states()
            .into_iter()
            .filter(|s| info.maze[s.pos.to_usize()] == '.')
            .collect();

        for next in next_states {
            if next.score <= *dist.get(&(next.pos, next.dir)).unwrap_or(&u64::MAX) {
                heap.push(next);
                dist.insert((next.pos, next.dir), next.score);
                prev.entry((next.pos, next.dir))
                    .or_default()
                    .push((state.pos, state.dir));
            }
        }
    }
    let real_end_state = dist
        .iter()
        .filter(|(k, _)| k.0 == info.end_pos)
        .min_by_key(|(_, v)| *v)
        .map(|(k, v)| State {
            pos: k.0,
            dir: k.1,
            score: *v,
        });
    Dijkstra {
        real_end_state,
        prev,
    }
}

#[aoc(day16, part1)]
fn part1(info: &Info) -> u64 {
    dijkstra(info)
        .real_end_state
        .expect("there should be a path from the start to the end")
        .score
}

#[aoc(day16, part2)]
fn part2(info: &Info) -> usize {
    let result = dijkstra(info);
    let prev = result.prev;
    let real_end_state = result.real_end_state.unwrap();
    let mut visited = HashSet::new();
    let mut to_visit = vec![(real_end_state.pos, real_end_state.dir)];
    while let Some(pos_and_dir) = to_visit.pop() {
        if !visited.contains(&pos_and_dir) && pos_and_dir.0 != info.start_pos {
            to_visit.extend(
                prev.get(&pos_and_dir)
                    .expect("every pos except starting one should have a parent")
                    .clone(),
            );
        }
        visited.insert(pos_and_dir);
    }
    let unique_pos: HashSet<_> = visited.iter().map(|(pos, _)| pos).collect();
    unique_pos.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const EXAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test_case(EXAMPLE => 7036; "smaller maze")]
    #[test_case(EXAMPLE2 => 11048; "bigger maze")]
    fn test_part1(input: &str) -> u64 {
        part1(&parse(input))
    }

    #[test_case(EXAMPLE => 45; "smaller maze")]
    #[test_case(EXAMPLE2 => 64; "bigger maze")]
    fn test_part2(input: &str) -> usize {
        part2(&parse(input))
    }
}
