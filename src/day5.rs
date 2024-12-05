use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Rules = HashMap<u32, HashSet<u32>>;

struct Instructions {
    rules: Rules,
    updates: Vec<Vec<u32>>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Instructions {
    let (rules_part, updates_part) = input.split_once("\n\n").unwrap();
    let mut rules = HashMap::new();
    for rule in rules_part.lines() {
        let (before, after) = rule
            .split_once('|')
            .map(|(a, b)| (a.parse().unwrap(), b.parse::<u32>().unwrap()))
            .unwrap();
        rules
            .entry(before)
            .and_modify(|set: &mut HashSet<u32>| {
                set.insert(after);
            })
            .or_insert_with(|| {
                let mut s = HashSet::new();
                s.insert(after);
                s
            });
        rules.entry(after).or_insert_with(HashSet::new);
    }
    let updates = updates_part
        .lines()
        .map(|l| l.split(',').map(|w| w.parse().unwrap()).collect())
        .collect();
    Instructions { rules, updates }
}

fn is_sorted(update: &[u32], rules: &Rules) -> bool {
    update.iter().enumerate().all(|(i, n)| {
        // get the number and its rules
        let Some(ruleset) = rules.get(n) else {
            // If the number has no rules we gucci
            return true;
        };
        // make sure that before it, no number appears in its rulebook
        !update[..i].iter().any(|m| ruleset.contains(m))
    })
}

#[aoc(day5, part1)]
fn part1(input: &Instructions) -> u32 {
    input
        .updates
        .iter()
        .filter(|update| is_sorted(update, &input.rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn sorted(update: &[u32], rules: &Rules) -> Vec<u32> {
    let mut l = Vec::with_capacity(update.len());
    let update: HashSet<_> = update.iter().copied().collect();
    // first generation are those who are not on right side of anyone
    // second generation are those whose right side only contains those from the first generation
    // etc...
    while l.len() != update.len() {
        let new_gen: Vec<_> = update
            .iter()
            .filter(|&n| !l.contains(n))
            .filter(|n| rules[n].intersection(&update).all(|m| l.contains(m)))
            .collect();
        for n in new_gen {
            l.push(*n);
        }
    }
    l
}

#[aoc(day5, part2)]
fn part2(input: &Instructions) -> u32 {
    input
        .updates
        .iter()
        .filter(|update| !is_sorted(update, &input.rules))
        .map(|update| sorted(update, &input.rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 123);
    }
}
