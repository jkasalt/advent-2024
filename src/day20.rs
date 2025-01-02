use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{matrix::Matrix, pos::Pos, utils};

struct Info {
    maze: Matrix<char>,
    count_if_more_than: usize,
    start: Pos<isize>,
    end: Pos<isize>,
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Info {
    let mut start = None;
    let mut end = None;
    let maze = utils::parse_grid_using_pos(input, |pos, c| {
        if c == 'S' {
            start = Some(pos.to_isize());
        }
        if c == 'E' {
            end = Some(pos.to_isize());
        }
        if c == '#' {
            '#'
        } else {
            '.'
        }
    });

    Info {
        start: start.unwrap(),
        end: end.unwrap(),
        count_if_more_than: 100,
        maze,
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Node {
    pos: Pos<isize>,
    cheat_start: Option<Cheat>,
    cheat_end: Option<Cheat>,
}

impl Node {
    const fn is_cheating(&self) -> Option<Cheat> {
        if self.cheat_start.is_some() && self.cheat_end.is_none() {
            self.cheat_start
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Cheat {
    at_time: usize,
    at_pos: Pos<isize>,
}

#[derive(Debug)]
struct State {
    node: Node,
    time: usize,
}

fn bfs(info: &Info, starting_state: State, give_up_time: Option<usize>) -> Vec<State> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut times = Vec::new();

    queue.push_back(starting_state);

    while let Some(mut state) = queue.pop_front() {
        let pos = state.node.pos;
        if pos == info.end {
            times.push(state);
            continue;
        }
        if visited.contains(&state.node) {
            continue;
        }
        if give_up_time.is_some_and(|t| t < state.time) {
            continue;
        }
        // if we are cheating...
        if let Some(Cheat { at_time, .. }) = state.node.is_cheating() {
            // stop it if it has been 2 ps
            if (state.time - at_time) >= 2 {
                state.node.cheat_end = Some(Cheat {
                    at_time: state.time,
                    at_pos: state.node.pos,
                });
                // then give up on path if our position is invalid
                if info
                    .maze
                    .get(state.node.pos.x, state.node.pos.y)
                    .is_some_and(|c| *c == '#')
                {
                    continue;
                }
            }
        }
        visited.insert(state.node);
        let neighbors = [
            Pos::new(pos.x + 1, pos.y),
            Pos::new(pos.x - 1, pos.y),
            Pos::new(pos.x, pos.y + 1),
            Pos::new(pos.x, pos.y - 1),
        ];
        for npos in neighbors {
            let Some(ncell) = info.maze.get(npos.x, npos.y) else {
                continue;
            };
            // if we are cheating all neighbors are valid
            if state.node.is_cheating().is_some() {
                queue.push_back(State {
                    node: Node {
                        pos: npos,
                        ..state.node
                    },
                    time: state.time + 1,
                });
            } else if state.node.cheat_start.is_none() && *ncell == '#' {
                // if we haven't cheated yet, and the neighbor is a wall, try to follow a path where we cheat into it
                debug_assert!(state.node.cheat_end.is_none());
                queue.push_back(State {
                    node: Node {
                        pos: npos,
                        cheat_start: Some(Cheat {
                            at_time: state.time,
                            at_pos: npos,
                        }),
                        ..state.node
                    },
                    time: state.time + 1,
                });
            } else if *ncell == '.' {
                // if the neighbor is free, try to follow a path where we just go there
                queue.push_back(State {
                    node: Node {
                        pos: npos,
                        ..state.node
                    },
                    time: state.time + 1,
                });
            }
        }
    }

    times
}

fn time_saves(info: &Info) -> Vec<usize> {
    let no_cheat_start = State {
        node: Node {
            pos: info.start,
            cheat_end: Some(Cheat {
                at_time: 0,
                at_pos: info.start,
            }),
            cheat_start: Some(Cheat {
                at_time: 0,
                at_pos: info.start,
            }),
        },
        time: 0,
    };

    let no_cheat_time = bfs(info, no_cheat_start, None)
        .into_iter()
        .map(|s| s.time)
        .min()
        .unwrap();

    let starting_state = State {
        node: Node {
            pos: info.start,
            cheat_start: None,
            cheat_end: None,
        },
        time: 0,
    };
    let times = bfs(info, starting_state, Some(no_cheat_time));

    times
        .iter()
        .filter(|s| s.time < no_cheat_time)
        .inspect(|s| println!("{s:?}"))
        .map(|s| no_cheat_time - s.time)
        .collect()
}

#[aoc(day20, part1)]
fn part1(info: &Info) -> usize {
    time_saves(info)
        .into_iter()
        .filter(|&delta| delta > info.count_if_more_than)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test_part1() {
        let expected_saves = [
            vec![2; 14],
            vec![4; 14],
            vec![6; 2],
            vec![8; 4],
            vec![10; 2],
            vec![12; 3],
            vec![20],
            vec![36],
            vec![38],
            vec![40],
            vec![64],
        ]
        .concat();
        let result = time_saves(&parse(EXAMPLE));
        assert_eq!(expected_saves.len(), result.len());
        for save in expected_saves {
            assert!(result.contains(&save));
        }
    }
}
