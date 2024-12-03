use std::sync::LazyLock;

use aoc_runner_derive::aoc;
use regex::Regex;

static RE_MUL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
#[allow(clippy::trivial_regex)] // because although some regexes are trivial we are using it
                                // multiple times, therefore this lint does not apply
static RE_DO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"do\(\)").unwrap());
#[allow(clippy::trivial_regex)]
static RE_DONT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"don't\(\)").unwrap());

#[aoc(day3, part1)]
fn p1(input: &str) -> u64 {
    RE_MUL
        .captures_iter(input)
        .map(|cap| cap.extract())
        .map(|(_, [a, b])| a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap())
        .sum()
}

#[derive(Debug, Clone)]
enum Token {
    Do,
    Dont,
    Mul(u64, u64),
}

#[aoc(day3, part2)]
fn p2(input: &str) -> u64 {
    // find all the tokens and their positions
    let mul_token_positions: Vec<_> = RE_MUL
        .captures_iter(input)
        .map(|cap| {
            let start = cap.get(1).unwrap().start();
            let (_, [a, b]) = cap.extract();
            (start, Token::Mul(a.parse().unwrap(), b.parse().unwrap()))
        })
        .collect();
    let do_token_positions: Vec<_> = RE_DO
        .find_iter(input)
        .map(|m| (m.start(), Token::Do))
        .collect();
    let dont_token_positions: Vec<_> = RE_DONT
        .find_iter(input)
        .map(|m| (m.start(), Token::Dont))
        .collect();

    // order them into an array
    let mut tokens: Vec<_> = [
        mul_token_positions,
        do_token_positions,
        dont_token_positions,
    ]
    .concat();
    tokens.sort_by_key(|(s, _)| *s);

    // then do the needful
    let mut do_mul = true;
    let mut sum = 0;
    for (_, token) in tokens {
        match token {
            Token::Do => do_mul = true,
            Token::Dont => do_mul = false,
            Token::Mul(a, b) => {
                if do_mul {
                    sum += a * b;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, p1(input));
    }

    #[test]
    fn test_p2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, p2(input));
    }
}
