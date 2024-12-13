use crate::pos::Pos;
use std::sync::LazyLock;

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

// can't be bothered to add num-traits dependency
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
fn search(machine: &ClawMachine) -> Option<u64> {
    let det = (machine.a.x * machine.b.y - machine.a.y * machine.b.x) as f64;
    let sol = (
        (machine.b.y * machine.target.x - machine.b.x * machine.target.y) as f64 / det,
        (-machine.a.y * machine.target.x + machine.a.x * machine.target.y) as f64 / det,
    );

    if (sol.0.round() - sol.0).abs() < f64::EPSILON && (sol.1.round() - sol.1).abs() < f64::EPSILON
    {
        Some(sol.0 as u64 * 3 + sol.1 as u64)
    } else {
        None
    }
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
        .map(|&m| ClawMachine {
            target: Pos::new(m.target.x + added, m.target.y + added),
            ..m
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

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test_case(EXAMPLE => 480; "example")]
    fn part1_example(input: &str) -> u64 {
        part1(&parse(input))
    }
}
