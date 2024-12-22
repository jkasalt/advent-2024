use std::{collections::HashMap, string::ToString};

use aoc_runner_derive::{aoc, aoc_generator};

fn aux1(s: &str, towels: &Vec<String>) -> bool {
    if s.is_empty() {
        return true;
    }

    towels.iter().any(|towel| {
        s.strip_prefix(towel)
            .is_some_and(|stripped| aux1(stripped, towels))
    })
}

fn aux2<'a>(s: &'a str, towels: &Vec<String>, memo: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(res) = memo.get(s) {
        return *res;
    }
    if s.is_empty() {
        return 1;
    }

    let res = towels
        .iter()
        .filter_map(|towel| {
            s.strip_prefix(towel)
                .map(|stripped| aux2(stripped, towels, memo))
        })
        .sum();

    memo.insert(s, res);

    res
}

struct Info {
    towels: Vec<String>,
    designs: Vec<String>,
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Info {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").map(ToString::to_string).collect();
    let designs = designs.lines().map(ToString::to_string).collect();
    Info { towels, designs }
}

#[aoc(day19, part1)]
fn part1(info: &Info) -> usize {
    info.designs
        .iter()
        .filter(|design| aux1(design, &info.towels))
        .count()
}

#[aoc(day19, part2)]
fn part2(info: &Info) -> usize {
    let mut memo = HashMap::new();
    info.designs
        .iter()
        .map(|design| aux2(design, &info.towels, &mut memo))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EXAMPLE)), 16);
    }
}
