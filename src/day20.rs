use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{matrix::Matrix, pos::Pos, utils};

struct Info {
    maze: Matrix<char>,
    count_if_more_than: usize,
    start: Pos<isize>,
    end: Pos<isize>,
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Info {
    let mut start = None;
    let mut end = None;
    let maze = utils::parse_grid_using_pos(input, |pos, c| {
        if c == 'S' {
            start = Some(pos.to_isize());
        }
        if c == 'E' {
            end = Some(pos.to_isize());
        }
        if c == '#' {
            '#'
        } else {
            '.'
        }
    });

    Info {
        start: start.unwrap(),
        end: end.unwrap(),
        count_if_more_than: 100,
        maze,
    }
}

fn normal_path(info: &Info) -> Vec<Pos<isize>> {
    // find the path without cheats
    let mut visited = HashSet::new();
    visited.insert(info.start);
    let mut path = vec![info.start];
    let mut next = info.start;

    while next != info.end {
        visited.insert(next);
        let neighbors = [
            Pos::new(next.x + 1, next.y),
            Pos::new(next.x - 1, next.y),
            Pos::new(next.x, next.y + 1),
            Pos::new(next.x, next.y - 1),
        ];

        next = *neighbors
            .iter()
            .find(|n| {
                info.maze.get(n.x, n.y).is_some_and(|cell| *cell == '.') && !visited.contains(*n)
            })
            .unwrap();
        path.push(next);
    }
    path
}

fn time_saves(info: &Info, path: &[Pos<isize>], max_dist: isize) -> HashMap<usize, i32> {
    // for each node look at where we can go with a cheat
    let mut time_saves = HashMap::new();
    for (picosec_start, &p) in path.iter().enumerate() {
        println!("{p:?} ---");
        let mut cheatables = HashSet::new();
        let mut queue = VecDeque::from([p]);
        let mut visited = HashSet::new();
        let deltas = [
            Pos::new(1, 0),
            Pos::new(-1, 0),
            Pos::new(0, 1),
            Pos::new(0, -1),
        ];

        while let Some(pp) = queue.pop_front() {
            if visited.contains(&pp) {
                continue;
            }
            visited.insert(pp);
            if info.maze.get(pp.x, pp.y).is_some_and(|c| *c == '.') {
                cheatables.insert(pp);
            }
            for d in deltas {
                let n = pp + d;
                if !visited.contains(&n) && (p - n).l1_norm() <= max_dist {
                    println!("dist: {}, to {n:?}", (p - n).l1_norm());
                    queue.push_back(n);
                }
            }
        }
        //for d in deltas {
        //    let n = p + d;
        //    if info.maze.get(n.x, n.y).is_some_and(|c| *c == '#') {
        //        let nn = p + d * 2;
        //        if info.maze.get(nn.x, nn.y).is_some_and(|c| *c == '.') {
        //            cheatables.insert(nn);
        //        }
        //    }
        //}
        //
        //println!("{p:?} can cheat to {cheatables:?}");
        //println!("{cheatables:?}");

        // find out how much time was saved
        for cheatable in cheatables {
            let cheat_len = (p - cheatable).l1_norm().try_into().unwrap();
            let picosec_arrival = path
                .iter()
                .position(|pp| *pp == cheatable)
                .expect("a cheatable at this point should be in the path");
            if let Some(time_saved) = picosec_arrival
                .checked_sub(picosec_start)
                .and_then(|t| t.checked_sub(cheat_len))
            {
                *time_saves.entry(time_saved).or_insert(0) += 1;
            }
        }
    }
    time_saves
}

#[aoc(day20, part1)]
fn part1(info: &Info) -> i32 {
    let path = normal_path(info);
    let time_saves = time_saves(info, &path, 2);
    time_saves
        .into_iter()
        .filter(|(t, _)| *t >= info.count_if_more_than)
        .map(|(_, c)| c)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn test_normal_path() {
        let result = normal_path(&parse(EXAMPLE));
        assert_eq!(result.len(), 85);
    }

    #[test]
    fn test_part1() {
        let expected_saves: HashMap<_, _> = [
            (0, 84),
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]
        .into_iter()
        .collect();
        let info = &parse(EXAMPLE);
        let result = time_saves(info, &normal_path(info), 2);
        assert_eq!(expected_saves, result);
    }
}
