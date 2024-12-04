use aoc_runner_derive::{aoc, aoc_generator};

use crate::matrix::Matrix;

#[aoc_generator(day4)]
fn parse(input: &str) -> Matrix<char> {
    let width = input.lines().next().map(str::len).unwrap();
    let height = input.lines().count();
    let items = input.chars().filter(char::is_ascii_graphic);
    Matrix::new(items, width, height)
}

#[aoc(day4, part1)]
fn p1(letters: &Matrix<char>) -> usize {
    let deltas = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];
    let x_pos = letters
        .iter_pos()
        .filter(|(_, letter)| **letter == 'X')
        .map(|(pos, _)| pos);

    // for X, we look at the deltas
    // for delta we want (1, M) -> (2, A) -> (3, S)
    // count the number of times the chain gets to the end
    let chain = [(1, 'M'), (2, 'A'), (3, 'S')];
    x_pos
        .into_iter()
        .map(|x_pos| {
            deltas
                .iter()
                .filter(|delta| {
                    chain.iter().all(|&(coeff, wanting)| {
                        let new_pos = (
                            i32::try_from(x_pos.0).unwrap() + delta.0 * coeff,
                            i32::try_from(x_pos.1).unwrap() + delta.1 * coeff,
                        );
                        letters
                            .get(new_pos.0.try_into().unwrap(), new_pos.1.try_into().unwrap())
                            .is_some_and(|&letter| letter == wanting)
                    })
                })
                .count()
        })
        .sum()
}

#[aoc(day4, part2)]
fn p2(letters: &Matrix<char>) -> usize {
    // for A we look at the cross neighbors
    // if they are all M and S and M's are not crossed, we gucci, we tamagucci
    let a_pos = letters
        .iter_pos()
        .filter(|(_, letter)| **letter == 'A')
        .map(|(pos, _)| pos);

    let deltas = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
    a_pos
        .filter(|a_pos| {
            let nbs: Vec<_> = deltas
                .iter()
                .map(|d| {
                    (
                        isize::try_from(a_pos.0).unwrap() + d.0,
                        isize::try_from(a_pos.1).unwrap() + d.1,
                    )
                })
                .collect();
            let all_good_letters = nbs
                .iter()
                .all(|&(nx, ny)| matches!(letters.get(nx, ny), Some('M' | 'S')));
            let no_cross_letters = (letters.get(nbs[0].0, nbs[0].1)
                != letters.get(nbs[3].0, nbs[3].1))
                && letters.get(nbs[1].0, nbs[1].1) != letters.get(nbs[2].0, nbs[2].1);

            all_good_letters && no_cross_letters
        })
        .count()
}

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
