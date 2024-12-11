use aoc_runner_derive::{aoc, aoc_generator};

use crate::matrix::Matrix;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect()
}

fn blink_process(stones: &mut Vec<i64>, max_iter: usize) -> usize {
    for _ in 0..max_iter {
        let mut will_insert = Vec::new();
        for (i, stone) in stones.iter_mut().enumerate() {
            if *stone == 0 {
                *stone = 1;
            } else if stone.ilog10() % 2 == 1 {
                let split_magnitude = 10_i64.pow(1 + (stone.ilog10() / 2));
                let split_left = *stone / split_magnitude;
                let split_right = *stone % split_magnitude;
                *stone = split_right;
                will_insert.push((i, split_left));
            } else {
                *stone *= 2024;
            }
        }
        for (i, stone) in will_insert {
            stones.insert(i, stone);
        }
    }
    stones.len()
}

#[aoc(day11, part1)]
fn part1(stones: &[i64]) -> usize {
    let mut stones = stones.to_owned();
    blink_process(&mut stones, 25)
}

#[aoc(day11, part2)]
fn part2(topography: &[i64]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = "125 17";

    #[test_case(EXAMPLE => 22; "normal example")]
    fn part1_example(input: &str) -> usize {
        blink_process(&mut parse(input), 6)
    }
    //
    //fn part2_example(input: &str) -> usize {
    //    part2(&parse(input))
    //}
}
