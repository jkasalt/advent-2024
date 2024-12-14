use std::io::{self, BufRead};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::pos::Pos;

struct Robot {
    pos: Pos<i64>,
    vel: Pos<i64>,
}

struct Info {
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Info {
    let robots = input
        .lines()
        .map(|line| {
            let (p, v) = line
                .split_once(' ')
                .and_then(|(p, v)| {
                    p.split_once('=')
                        .map(|p| p.1)
                        .zip(v.split_once('=').map(|v| v.1))
                })
                .unwrap();
            let pos = p
                .split_once(',')
                .and_then(|(px, py)| px.parse().ok().zip(py.parse().ok()))
                .unwrap()
                .into();
            let vel = v
                .split_once(',')
                .and_then(|(vx, vy)| vx.parse().ok().zip(vy.parse().ok()))
                .unwrap()
                .into();
            Robot { pos, vel }
        })
        .collect();
    Info {
        robots,
        width: 101,
        height: 103,
    }
}

fn simulate(info: &Info, max_iter: usize, is_part2: bool) -> Vec<Pos<i64>> {
    let width = info.width;
    let height = info.height;
    let mut positions: Vec<_> = info.robots.iter().map(|r| r.pos).collect();
    for n in 0..max_iter {
        info.robots
            .iter()
            .zip(positions.iter_mut())
            .for_each(|(Robot { vel, .. }, pos)| {
                pos.x += vel.x;
                pos.x = pos.x.rem_euclid(width);
                pos.y += vel.y;
                pos.y = pos.y.rem_euclid(height);
            });
        if is_part2 {
            let mut picture = String::new();
            for y in 0..height {
                for x in 0..width {
                    if positions.iter().any(|pos| pos.x == x && pos.y == y) {
                        picture.push('#');
                    } else {
                        picture.push('.');
                    }
                }
                picture.push('\n');
            }

            if picture.contains("#########") {
                println!("{picture}");
                println!("{}", n + 1);
                let mut user_input = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut user_input).unwrap();
                if !user_input.trim().is_empty() {
                    return positions;
                }
            }
        }
    }
    positions
}

#[aoc(day14, part1)]
fn part1(info: &Info) -> usize {
    let width = info.width;
    let height = info.height;
    let positions = simulate(info, 100, false);
    let quadrants = [
        ((0..width / 2), (0..height / 2)),
        ((width / 2 + 1..width), (0..height / 2)),
        ((0..width / 2), (height / 2 + 1..height)),
        ((width / 2 + 1..width), (height / 2 + 1..height)),
    ];

    quadrants
        .iter()
        .map(|q| {
            positions
                .iter()
                .filter(|pos| q.0.contains(&pos.x) && q.1.contains(&pos.y))
                .count()
        })
        .product()
}

#[aoc(day14, part2)]
fn part2(info: &Info) -> usize {
    simulate(info, 10000, true);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_name() {
        let mut info = parse(EXAMPLE);
        info.width = 11;
        info.height = 7;
        assert_eq!(part1(&info), 12);
    }
}
