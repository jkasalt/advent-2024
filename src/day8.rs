use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Antennas = HashMap<char, Vec<(i64, i64)>>;

#[derive(Debug)]
struct Infos {
    width: i64,
    height: i64,
    antennas: Antennas,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Infos {
    let width = input.lines().next().unwrap().trim().len();
    let height = input.lines().count();
    let mut antennas: Antennas = HashMap::new();
    input
        .chars()
        .filter(char::is_ascii_graphic)
        .enumerate()
        .map(|(i, c)| ((i % width, i / width), c))
        .filter(|(_, c)| *c != '.')
        .for_each(|((x, y), c)| {
            antennas
                .entry(c)
                .or_default()
                .push((x.try_into().unwrap(), y.try_into().unwrap()));
        });

    Infos {
        width: width.try_into().unwrap(),
        height: height.try_into().unwrap(),
        antennas,
    }
}

#[aoc(day8, part1)]
fn part1(input: &Infos) -> usize {
    let mut antinodes = HashSet::new();
    for antenna_pos in input.antennas.values() {
        for (i, a) in antenna_pos[..antenna_pos.len() - 1].iter().enumerate() {
            for b in &antenna_pos[i + 1..] {
                let diff = (b.0 - a.0, b.1 - a.1);
                let anti1 = (b.0 + diff.0, b.1 + diff.1);
                let anti2 = (a.0 - diff.0, a.1 - diff.1);
                antinodes.insert(anti1);
                antinodes.insert(anti2);
            }
        }
    }
    antinodes
        .iter()
        .filter(|(x, y)| (0..input.width).contains(x) && (0..input.height).contains(y))
        .count()
}

#[aoc(day8, part2)]
#[allow(clippy::maybe_infinite_iter)] // it's OK
fn part2(input: &Infos) -> usize {
    let mut antinodes = HashSet::new();
    let in_bounds = |(x, y)| (0..input.width).contains(&x) && (0..input.height).contains(&y);
    for antenna_pos in input.antennas.values() {
        for (i, a) in antenna_pos[..antenna_pos.len() - 1].iter().enumerate() {
            for b in &antenna_pos[i + 1..] {
                let diff = (b.0 - a.0, b.1 - a.1);
                (0..)
                    .map(|lambda| (b.0 + lambda * diff.0, b.1 + lambda * diff.1))
                    .take_while(|antinode| in_bounds(*antinode))
                    .for_each(|antinode| {
                        antinodes.insert(antinode);
                    });
                (0..)
                    .map(|lambda| (a.0 - lambda * diff.0, a.1 - lambda * diff.1))
                    .take_while(|antinode| in_bounds(*antinode))
                    .for_each(|antinode| {
                        antinodes.insert(antinode);
                    });
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 34);
    }
}
