use aoc_runner_derive::{aoc, aoc_generator};

use crate::{matrix::Matrix, position::Position};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl Letter {
    const fn next(self) -> Option<Self> {
        match self {
            Self::X => Some(Self::M),
            Self::M => Some(Self::A),
            Self::A => Some(Self::S),
            Self::S => None,
        }
    }
}

impl std::str::FromStr for Letter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::X),
            "M" => Ok(Self::M),
            "A" => Ok(Self::A),
            "S" => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Matrix<Letter> {
    let width = input.lines().next().map(str::len).unwrap();
    let height = input.lines().count();
    let items = input.chars().filter_map(|c| format!("{c}").parse().ok());
    Matrix::new(items, width, height)
}

#[aoc(day4, part1)]
fn p1(letters: &Matrix<Letter>) -> usize {
    let deltas = [
        Position::from((1, 0)),
        Position::from((-1, 0)),
        Position::from((0, 1)),
        Position::from((0, -1)),
        Position::from((1, 1)),
        Position::from((-1, 1)),
        Position::from((1, -1)),
        Position::from((-1, -1)),
    ];
    let x_pos = letters
        .iter_pos()
        .filter(|(_, letter)| **letter == Letter::X)
        .map(|(pos, _)| Position::from(pos));

    // for X, we look at the deltas
    // for delta we want (1, M) -> (2, A) -> (3, S)
    // if that is the case we increase count by 1
    let chain = [Letter::X, Letter::M, Letter::A, Letter::S];
    x_pos
        .into_iter()
        .map(|x_pos| {
            deltas
                .iter()
                .filter(|delta| {
                    chain.iter().enumerate().all(|(coeff, wanting)| {
                        let new_pos: Position = x_pos + (**delta * coeff);
                        letters
                            .get(new_pos.x, new_pos.y)
                            .is_some_and(|letter| letter == wanting)
                    })
                })
                .count()
        })
        .sum()
}

#[aoc(day4, part2)]
fn p2(letters: &Matrix<Letter>) -> usize {
    // for A we look at the cross neighbors
    // if they are all M and S and M's are not crossed, we gucci, we tamagucci
    let a_pos = letters
        .iter_pos()
        .filter(|(_, letter)| **letter == Letter::A)
        .map(|(pos, _)| pos);

    let mut count = 0;
    let deltas = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
    for a_pos in a_pos {
        let nbs: Vec<_> = deltas
            .iter()
            .map(|d| (a_pos.0 as isize + d.0, a_pos.1 as isize + d.1))
            .collect();
        let all_good_letters = nbs
            .iter()
            .all(|&(nx, ny)| matches!(letters.get(nx, ny), Some(Letter::M | Letter::S)));
        let no_cross_letters = (letters.get(nbs[0].0, nbs[0].1) != letters.get(nbs[3].0, nbs[3].1))
            && letters.get(nbs[1].0, nbs[1].1) != letters.get(nbs[2].0, nbs[2].1);
        //println!(
        //    "{a_pos:?} - good_letters: {all_good_letters}, no_cross_letters: {no_cross_letters}"
        //);
        if all_good_letters && no_cross_letters {
            count += 1;
        }
    }
    count
}

//fn search(letters: &Matrix<Letter>, (x, y): Position) -> usize {
//    let sample = letters[(x, y)];
//    println!("({x},{y}) - {sample:?}");
//    if sample == Letter::S {
//        return 1;
//    }
//    letters
//        .neighbor_indices(x, y)
//        .into_iter()
//        .filter(|(xx, yy)| sample.next().is_some_and(|s| s == letters[(xx, yy)]))
//        .map(|pos| search(letters, pos))
//        .sum()
//}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(18, p1(&parse(input)));
    }
    #[test]
    fn test_p2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(9, p2(&parse(input)));
    }
}
