use aoc_runner_derive::{aoc, aoc_generator};
struct Equation {
    target: u64,
    nums: Vec<u64>,
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(target, nums)| Equation {
                    target: target.parse().unwrap(),
                    nums: nums
                        .split_whitespace()
                        .filter_map(|n| n.parse().ok())
                        .collect(),
                })
                .unwrap()
        })
        .collect()
}

fn dfs(target: u64, current: u64, nums: &[u64]) -> bool {
    if target == current && nums.is_empty() {
        return true;
    }
    if target < current || nums.is_empty() {
        return false;
    }
    dfs(target, current + nums[0], &nums[1..]) || dfs(target, current * nums[0], &nums[1..])
}

#[aoc(day7, part1)]
fn part1(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|eq| dfs(eq.target, 0, &eq.nums))
        .map(|eq| eq.target)
        .sum()
}

const fn concat(n1: u64, n2: u64) -> u64 {
    n1 * 10_u64.pow(n2.ilog10() + 1) + n2
}

fn dfs2(target: u64, current: u64, nums: &[u64]) -> bool {
    if target == current && nums.is_empty() {
        return true;
    }
    if target < current || nums.is_empty() {
        return false;
    }
    dfs2(target, current + nums[0], &nums[1..])
        || dfs2(target, current * nums[0], &nums[1..])
        || dfs2(target, concat(current, nums[0]), &nums[1..])
}

#[aoc(day7, part2)]
fn part2(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|eq| dfs2(eq.target, 0, &eq.nums))
        .map(|eq| eq.target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_concat() {
        assert_eq!(concat(123, 45), 12345);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 11387);
    }
}
