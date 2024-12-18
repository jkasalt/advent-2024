use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{matrix::Matrix, pos::Pos};

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    const fn delta(&self) -> Pos<isize> {
        match self {
            Self::N => Pos::new(0, -1),
            Self::E => Pos::new(1, 0),
            Self::S => Pos::new(0, 1),
            Self::W => Pos::new(-1, 0),
        }
    }
}

struct Info {
    warehouse: Matrix<char>,
    instructions: Vec<Direction>,
}

#[aoc_generator(day15, part1)]
fn parse(input: &str) -> Info {
    let (warehouse, instructions) = input.split_once("\n\n").unwrap();
    let width = warehouse.lines().next().unwrap().len();
    let height = warehouse.lines().count();
    let items = warehouse
        .chars()
        .filter(|c| matches!(c, '@' | '.' | 'O' | '#'));
    let warehouse = Matrix::new(items, width, height);
    let instructions = instructions
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::N),
            '>' => Some(Direction::E),
            'v' => Some(Direction::S),
            '<' => Some(Direction::W),
            _ => None,
        })
        .collect();
    Info {
        warehouse,
        instructions,
    }
}

#[aoc_generator(day15, part2)]
fn parse2(input: &str) -> Info {
    let (warehouse, instructions) = input.split_once("\n\n").unwrap();
    let width = 2 * warehouse.lines().next().unwrap().len();
    let height = warehouse.lines().count();
    let items = warehouse
        .chars()
        .filter_map(|c| match c {
            'O' => Some(vec!['[', ']']),
            '#' => Some(vec!['#', '#']),
            '@' => Some(vec!['@', '.']),
            '.' => Some(vec!['.', '.']),
            _ => None,
        })
        .flatten();
    let warehouse = Matrix::new(items, width, height);
    let instructions = instructions
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::N),
            '>' => Some(Direction::E),
            'v' => Some(Direction::S),
            '<' => Some(Direction::W),
            _ => None,
        })
        .collect();
    Info {
        warehouse,
        instructions,
    }
}

fn simulate(info: &Info) -> usize {
    let mut warehouse = info.warehouse.clone();
    let mut robot_pos = warehouse
        .position(|cell| *cell == '@')
        .map(|pos| (pos.0.try_into().unwrap(), pos.1.try_into().unwrap()))
        .map(Pos::from)
        .unwrap();
    'dir: for d in &info.instructions {
        let mut wants_to_move: HashSet<_> = std::iter::once(robot_pos).collect();
        let mut needs_to_be_free: HashSet<_> = std::iter::once(robot_pos + d.delta()).collect();
        loop {
            let all_free = needs_to_be_free
                .iter()
                .filter_map(|pos| warehouse.get(pos.x, pos.y))
                .all(|c| *c == '.');
            if all_free {
                // to do the move, start from the furthermost positions, say (x,y), and
                let mut sorted_moving: Vec<_> = wants_to_move.into_iter().collect();
                sorted_moving.sort_unstable_by_key(|&Pos { x, y }| match d {
                    Direction::N => y,
                    Direction::E => -x,
                    Direction::S => -y,
                    Direction::W => x,
                });
                for &pos in &sorted_moving {
                    let other = pos + d.delta();
                    let x = pos.x.try_into().unwrap();
                    let y = pos.y.try_into().unwrap();
                    let xo = other.x.try_into().unwrap();
                    let yo = other.y.try_into().unwrap();
                    warehouse.swap((x, y), (xo, yo));
                }
                // also remember to update the robot's position
                robot_pos = robot_pos + d.delta();
                break;
            }
            // otherwise check those that need to be free
            let mut to_switch = Vec::new();
            for cell_pos in &needs_to_be_free {
                match warehouse.get(cell_pos.x, cell_pos.y) {
                    // if wall: stop everything and nobody moves
                    None | Some('#') => continue 'dir,
                    // if boulder:
                    // say that him and his neighbor wants to move
                    Some('O') => to_switch.push(*cell_pos),
                    Some('[') => {
                        to_switch.push(*cell_pos);
                        to_switch.push(*cell_pos + (1, 0).into());
                    }
                    Some(']') => {
                        to_switch.push(*cell_pos);
                        to_switch.push(*cell_pos + (-1, 0).into());
                    }
                    Some('@') => panic!("you are not supposed to be here"),
                    Some('.') => {}
                    Some(x) => panic!("unexpected char {x} at {cell_pos:?}"),
                }
            }
            // remove it from the needs_to_be_free and add to it the spaces the boulder would move
            for pos in to_switch {
                wants_to_move.insert(pos);
                needs_to_be_free.remove(&pos);
                let moving_into = pos + d.delta();
                if !wants_to_move.contains(&moving_into) {
                    needs_to_be_free.insert(moving_into);
                }
            }
        }
    }
    warehouse
        .iter_pos()
        .filter(|(_, cell)| **cell == 'O' || **cell == '[')
        .map(|(pos, _)| pos.1 * 100 + pos.0)
        .sum()
}

#[aoc(day15, part1)]
fn part1(info: &Info) -> usize {
    simulate(info)
}

#[aoc(day15, part2)]
fn part2(info: &Info) -> usize {
    simulate(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const EXAMPLE2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test_case(EXAMPLE1 => 2028; "small example")]
    #[test_case(EXAMPLE2 => 10092; "larger example")]
    fn test_part1(input: &str) -> usize {
        part1(&parse(input))
    }

    #[test_case(EXAMPLE2 => 9021; "larger example")]
    fn test_part2(input: &str) -> usize {
        part2(&parse2(input))
    }
}
