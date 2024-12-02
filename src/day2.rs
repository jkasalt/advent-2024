use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_skipping(it: &[i32]) -> bool {
    let safe_interval = 1..=3;
    (0..it.len()).any(|to_skip| {
        let diff = it
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if i == to_skip { None } else { Some(*x) })
            .map_windows(|[n0, n1]| n1 - n0);
        diff.clone().all(|d| safe_interval.contains(&d))
            || diff.clone().all(|d| safe_interval.contains(&-d))
    })
}

fn is_safe(it: &[i32]) -> bool {
    let safe_interval = 1..=3;
    let diff = it.windows(2).map(|window| window[1] - window[0]);
    diff.clone().all(|d| safe_interval.contains(&d))
        || diff.clone().all(|d| safe_interval.contains(&-d))
}

#[aoc(day2, part1)]
fn p1(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|r| is_safe(r)).count()
}

#[aoc(day2, part2)]
fn p2(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|r| is_safe_skipping(r)).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(2, p1(&gen(input)));
    }

    #[test]
    fn test_p2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(4, p2(&gen(input)));
    }
}
