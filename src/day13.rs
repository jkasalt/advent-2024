use crate::pos::Pos;
use std::{collections::HashMap, sync::LazyLock};

use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

static RE_DIFF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap());
static RE_TARGET: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"X=(\d+), Y=(\d+)").unwrap());

#[derive(Clone, Copy)]
struct ClawMachine {
    a: Pos,
    b: Pos,
    target: Pos,
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<ClawMachine> {
    fn parse_diff(line: &str) -> Pos {
        let [x, y] = RE_DIFF.captures(line).unwrap().extract().1;
        Pos::new(x.parse().unwrap(), y.parse().unwrap())
    }
    fn parse_target(line: &str) -> Pos {
        let [x, y] = RE_TARGET.captures(line).unwrap().extract().1;
        Pos::new(x.parse().unwrap(), y.parse().unwrap())
    }
    input
        .split("\n\n")
        .map(|section| {
            let mut lines = section.lines();
            let a = lines.next().map(parse_diff).unwrap();
            let b = lines.next().map(parse_diff).unwrap();
            let target = lines.next().map(parse_target).unwrap();
            ClawMachine { a, b, target }
        })
        .collect()
}

fn search(machine: &ClawMachine) -> Option<u64> {
    fn aux(
        machine: &ClawMachine,
        a_amount: isize,
        b_amount: isize,
        memo: &mut HashMap<(isize, isize), Option<u64>>,
    ) -> Option<u64> {
        let current = machine.a * a_amount + machine.b * b_amount;
        let tokens = (a_amount * 3 + b_amount).try_into().unwrap();
        if current == machine.target {
            memo.insert((a_amount, b_amount), Some(tokens));
            return Some(tokens);
        }
        if current.x > machine.target.x || current.y > machine.target.y {
            return None;
        }
        if let Some(res) = memo.get(&(a_amount, b_amount)) {
            return *res;
        }
        let going_a = aux(machine, a_amount + 1, b_amount, memo);
        let going_b = aux(machine, a_amount, b_amount + 1, memo);

        let res = match (going_a, going_b) {
            (Some(ra), Some(rb)) => Some(ra.min(rb)),
            (None, Some(rb)) => Some(rb),
            (Some(ra), None) => Some(ra),
            (None, None) => None,
        };
        memo.insert((a_amount, b_amount), res);
        res
    }
    aux(machine, 0, 0, &mut HashMap::new())
}

#[aoc(day13, part1)]
fn part1(machines: &[ClawMachine]) -> u64 {
    machines.iter().filter_map(search).sum()
}

#[aoc(day13, part2)]
fn part2(machines: &[ClawMachine]) -> u64 {
    let added = 10_000_000_000_000;
    machines
        .iter()
        .map(|m| ClawMachine {
            target: Pos::new(m.target.x + added, m.target.y + added),
            ..*m
        })
        .filter_map(|m| search(&m))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
";

    #[test_case(EXAMPLE => 280; "example")]
    fn part1_example(input: &str) -> u64 {
        part1(&parse(input))
    }
}
