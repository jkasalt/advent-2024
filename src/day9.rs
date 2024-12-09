use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};

type Memory = Vec<Option<usize>>;

#[aoc_generator(day9, part1)]
fn parse1(input: &str) -> Memory {
    input
        .chars()
        .filter(char::is_ascii_graphic)
        .enumerate()
        .flat_map(|(i, c)| {
            let is_file = i % 2 == 0;
            let num_repeat = c.to_digit(10).unwrap() as usize;
            let id = i / 2;
            if is_file {
                vec![Some(id); num_repeat]
            } else {
                vec![None; num_repeat]
            }
        })
        .collect()
}

fn checksum(input: Memory) -> usize {
    input
        .into_iter()
        .enumerate()
        .map(|(i, o)| i * o.unwrap_or(0))
        .sum()
}

#[aoc(day9, part1)]
fn part1(input: &Memory) -> usize {
    let mut input = input.to_owned();
    loop {
        let last_file_pos = input.iter().rposition(Option::is_some).unwrap();
        let first_free_pos = input.iter().position(Option::is_none).unwrap();
        if last_file_pos < first_free_pos {
            return checksum(input);
        }
        input.swap(last_file_pos, first_free_pos);
    }
}

#[derive(Clone, Debug)]
struct MemoryShard {
    id: Option<usize>,
    size: usize,
}

impl MemoryShard {
    fn raw_memory(&self) -> Memory {
        vec![self.id; self.size]
    }
}

#[aoc_generator(day9, part2)]
fn parse2(input: &str) -> Vec<MemoryShard> {
    input
        .chars()
        .filter(char::is_ascii_graphic)
        .enumerate()
        .map(|(i, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            let id = (i % 2 == 0).then_some(i / 2);
            MemoryShard { id, size }
        })
        .collect()
}

#[aoc(day9, part2)]
fn part2(input: &[MemoryShard]) -> usize {
    let mut input = input.to_owned();
    let max_id = input.iter().filter_map(|shard| shard.id).max().unwrap();
    for id in (0..=max_id).rev() {
        let to_move_pos = input
            .iter()
            .rposition(|shard| shard.id == Some(id))
            .unwrap();
        let space_needed = input[to_move_pos].size;
        let free_space_pos = input[..to_move_pos]
            .iter()
            .position(|shard| shard.id.is_none() && space_needed <= shard.size);

        if let Some(free_space_pos) = free_space_pos {
            input[free_space_pos].size -= input[to_move_pos].size;
            let to_move = input[to_move_pos].clone();
            input[to_move_pos].id = None;
            input.insert(free_space_pos, to_move);
        }
    }
    checksum(input.iter().flat_map(MemoryShard::raw_memory).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse1(EXAMPLE)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse2(EXAMPLE)), 2858);
    }
}
