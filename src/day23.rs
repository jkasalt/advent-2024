use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    let triangles = input.lines().enumerate().flat_map(|(i, line)| {
        let (left, right) = line
            .split_once('-')
            .unwrap_or_else(|| panic!("line `{line}` does not contain a `-`"));
        let connects_left: HashSet<_> = input
            .lines()
            .skip(i + 1)
            .filter_map(|ll| {
                let (left2, right2) = ll.split_once('-').unwrap();
                if left2 == left {
                    Some(right2)
                } else if right2 == left {
                    Some(left2)
                } else {
                    None
                }
            })
            .collect();
        let connects_right: HashSet<_> = input
            .lines()
            .skip(i + 1)
            .filter_map(|ll| {
                let (left2, right2) = ll.split_once('-').unwrap();
                if left2 == right {
                    Some(right2)
                } else if right2 == right {
                    Some(left2)
                } else {
                    None
                }
            })
            .collect();
        connects_left
            .intersection(&connects_right)
            .map(|other| [left, right, other])
            .collect::<Vec<_>>()
    });

    triangles
        .filter(|triangle: &[&str; 3]| triangle.iter().any(|corner| corner.starts_with('t')))
        .count()
}

#[aoc(day23, part2)]
fn part2(input: &str) -> String {
    let mut adj_list = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();
        adj_list.entry(left).or_insert_with(Vec::new).push(right);
        adj_list.entry(right).or_insert_with(Vec::new).push(left);
    }

    let mut cliques: Vec<Vec<&str>> = Vec::new();
    for &node1 in adj_list.keys() {
        let mut clique = vec![node1];
        for &node2 in adj_list.keys() {
            if clique
                .iter()
                .all(|clique_node| adj_list[*clique_node].contains(&node2))
            {
                clique.push(node2);
            }
        }
        cliques.push(clique);
    }

    let mut biggest_clique = cliques.into_iter().max_by_key(std::vec::Vec::len).unwrap();
    biggest_clique.sort_unstable();
    biggest_clique.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(EXAMPLE), "co,de,ka,ta");
    }
}
