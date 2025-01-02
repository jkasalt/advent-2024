use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::pos::Pos;

struct Info {
    walls: Vec<Pos<i32>>,
    side_len: i32,
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Info {
    let side_len = 70;
    let walls = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| Pos::new(x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    Info { walls, side_len }
}

fn bfs(info: &Info, until: usize) -> Option<usize> {
    let start_pos = Pos::new(0, 0);
    let end_pos = Pos::new(info.side_len, info.side_len);
    let walls = &info.walls[..until];
    let mut dists: HashMap<Pos<i32>, usize> = HashMap::new();
    dists.insert(start_pos, 0);
    let mut to_visit = VecDeque::from([start_pos]);
    let mut visited = HashSet::new();
    while let Some(pos) = to_visit.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        let neighbors = [
            (pos.x + 1, pos.y),
            (pos.x - 1, pos.y),
            (pos.x, pos.y + 1),
            (pos.x, pos.y - 1),
        ]
        .into_iter()
        .map(|(x, y)| Pos::new(x, y))
        .filter(|n| {
            !visited.contains(n)
                && !walls.contains(n)
                && (0..=info.side_len).contains(&n.x)
                && (0..=info.side_len).contains(&n.y)
        });
        let dist = dists[&pos] + 1;
        for n in neighbors {
            let dist_n = dists.entry(n).or_insert(usize::MAX);
            *dist_n = dist.min(*dist_n);
            to_visit.push_back(n);
        }
    }
    dists.get(&end_pos).copied()
}

#[aoc(day18, part1)]
fn part1(info: &Info) -> usize {
    bfs(info, 1024).expect("there should be a path to the end")
}

#[aoc(day18, part2)]
fn part2(info: &Info) -> String {
    (0..info.walls.len())
        .find(|&i| bfs(info, i).is_none())
        .and_then(|i| info.walls.get(i - 1))
        .map(|Pos { x, y }| format!("{x},{y}"))
        .expect("There should be a wall that cuts off the path")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut info = parse(EXAMPLE);
        info.side_len = 6;
        assert_eq!(bfs(&info, 12), Some(22));
    }

    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn test_part2() {
        let mut info = parse(EXAMPLE);
        info.side_len = 6;
        assert_eq!(&part2(&info), "6,1");
    }
}
