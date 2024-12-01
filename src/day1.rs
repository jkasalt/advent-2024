use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn gen(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|l| {
            let mut words = l.split_whitespace();
            let n1: i32 = words.next().and_then(|n| n.parse().ok()).unwrap();
            let n2: i32 = words.next().and_then(|n| n.parse().ok()).unwrap();
            (n1, n2)
        })
        .unzip()
}

#[aoc(day1, part1)]
fn p1((v1, v2): &(Vec<i32>, Vec<i32>)) -> u32 {
    let mut v1 = v1.clone();
    let mut v2 = v2.clone();
    v1.sort();
    v2.sort();
    v1.iter()
        .zip(v2.iter())
        .map(|(n1, n2)| n1.abs_diff(*n2))
        .sum()
}

#[aoc(day1, part2)]
fn p2((v1, v2): &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut count = HashMap::new();
    for n2 in v2 {
        *count.entry(n2).or_insert(0) += 1;
    }
    v1.iter().map(|n1| *n1 * count.get(n1).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        let result = p1(&gen(input));
        assert_eq!(result, 11);
    }

    #[test]
    fn test_p2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        let result = p2(&gen(input));
        assert_eq!(result, 31);
    }
}
