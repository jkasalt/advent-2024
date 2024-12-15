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
    for d in &info.instructions {
        let mut wants_to_move = vec![robot_pos];
        let mut needs_to_be_free = vec![robot_pos + d.delta()];
        loop {
            wants_to_move.push(needs_to_be_free);
            match warehouse.get(needs_to_be_free.x, needs_to_be_free.y){
                Some('O') => needs_to_be_free = needs_to_be_free + d.delta(),
                Some('#') | None => {
                    println!("Move {d:?}:\n{warehouse:?}");
                    break;
                },
                Some('.') => {
                    robot_pos = wants_to_move[1];
                    let mut rotated_pos: Vec<Pos<isize>> = vec![*wants_to_move.last().unwrap()];
                    rotated_pos.extend(&wants_to_move[..wants_to_move.len()]);
                    let rotated_items:Vec<_> = rotated_pos.iter()
                        .filter_map(|pos| warehouse.get(pos.x, pos.y).copied())
                        .collect();
                    for (pos, new_cell) in wants_to_move.iter().zip(rotated_items.into_iter()) {
                        *warehouse.get_mut(pos.x, pos.y).unwrap() = new_cell;
                    }
                    break;
                },
                Some('@') => panic!("found robot at {needs_to_be_free:?}, but there is already one at {robot_pos:?}\n\n{warehouse:?}"),
                Some(x) => panic!("unexpected char in matrix: {x} at {needs_to_be_free:?}\n{warehouse:?}"),
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
