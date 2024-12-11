use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse(input: &str) -> HashMap<u64, usize> {
    let mut map = HashMap::new();
    input
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .for_each(|n| {
            *map.entry(n).or_default() += 1;
        });
    map
}

fn blink_process(stones: &mut HashMap<u64, usize>, max_iter: usize) -> usize {
    for _ in 0..max_iter {
        let mut will_add = Vec::new();
        for (stone, count) in &mut *stones {
            let turns_into = if *stone == 0 {
                vec![1]
            } else if stone.ilog10() % 2 == 1 {
                let split_magnitude = 10_u64.pow(1 + (stone.ilog10() / 2));
                let split_left = *stone / split_magnitude;
                let split_right = *stone % split_magnitude;
                vec![split_left, split_right]
            } else {
                vec![stone * 2024]
            };
            for new_stone in turns_into {
                will_add.push((new_stone, *count));
            }
            *count = 0;
        }
        for (stone, count) in will_add {
            *stones.entry(stone).or_default() += count;
        }
    }
    stones.values().sum()
}

#[aoc(day11, part1)]
fn part1(stones: &HashMap<u64, usize>) -> usize {
    let mut stones = stones.to_owned();
    blink_process(&mut stones, 25)
}

#[aoc(day11, part2)]
fn part2(stones: &HashMap<u64, usize>) -> usize {
    let mut stones = stones.to_owned();
    blink_process(&mut stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    const EXAMPLE: &str = "125 17";

    #[test_case(EXAMPLE, 6 => 22; "normal example")]
    #[test_case(EXAMPLE, 25 => 55312; "normal example bigger")]
    fn part1_example(input: &str, blink_times: usize) -> usize {
        blink_process(&mut parse(input), blink_times)
    }
}
