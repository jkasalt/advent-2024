use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Register {
    a: u32,
    b: u32,
    c: u32,
}

#[derive(Debug, Eq, PartialEq)]
struct Info {
    register: Register,
    instr: Vec<u32>,
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Info {
    let regs = input
        .lines()
        .take(3)
        .map(|l| {
            l.split_once(": ")
                .map(|sp| sp.1)
                .and_then(|t| t.parse().ok())
                .unwrap()
        })
        .collect::<Vec<_>>();
    let [a, b, c] = regs.try_into().unwrap();

    let instr_text = input
        .split("\n\n")
        .nth(1)
        .and_then(|l| l.split_once(": "))
        .map(|sp| sp.1)
        .unwrap();

    let instr: Vec<_> = instr_text
        .split(',')
        .map(|s| {
            s.parse()
                .unwrap_or_else(|err| panic!("{err}: {s} should be a number"))
        })
        .collect();

    Info {
        register: Register { a, b, c },
        instr,
    }
}

fn combo(operand: u32, register: &Register) -> u32 {
    match operand {
        x @ 0..=3 => x,
        4 => register.a,
        5 => register.b,
        6 => register.c,
        x => panic!("invalid operand {x:?}"),
    }
}

#[derive(Debug, PartialEq, Eq)]
enum OpSideEffect {
    None,
    ChangeI(usize),
    Output(u32),
    ChangeRegA(u32),
    ChangeRegB(u32),
    ChangeRegC(u32),
}

fn operate(opcode: u32, operand: u32, register: &Register) -> OpSideEffect {
    let wtf = || register.a >> combo(operand, register);
    match opcode {
        0 => OpSideEffect::ChangeRegA(wtf()),
        1 => OpSideEffect::ChangeRegB(register.b ^ operand),
        2 => OpSideEffect::ChangeRegB(combo(operand, register) % 8),
        3 => {
            if register.a != 0 {
                OpSideEffect::ChangeI(operand as usize)
            } else {
                OpSideEffect::None
            }
        }
        4 => OpSideEffect::ChangeRegB(register.b ^ register.c),
        5 => OpSideEffect::Output(combo(operand, register) % 8),
        6 => OpSideEffect::ChangeRegB(wtf()),
        7 => OpSideEffect::ChangeRegC(wtf()),

        _ => todo!(),
    }
}

fn process(register: &Register, instr: &[u32], cond: impl Fn(&[u32]) -> bool) -> Vec<u32> {
    let mut register = *register;
    let mut output = Vec::new();

    let mut i = 0;
    while i < instr.len() - 1 && cond(&output) {
        let opcode = instr[i];
        let operand = instr[i + 1];

        let side_effect = operate(opcode, operand, &register);
        if let OpSideEffect::ChangeI(next_i) = side_effect {
            i = next_i;
        } else {
            i += 2;
            match side_effect {
                OpSideEffect::None => {}
                OpSideEffect::ChangeI(_) => unreachable!(),
                OpSideEffect::ChangeRegA(new_a) => register.a = new_a,
                OpSideEffect::ChangeRegB(new_b) => register.b = new_b,
                OpSideEffect::ChangeRegC(new_c) => register.c = new_c,
                OpSideEffect::Output(n) => output.push(n),
            }
        }
    }
    output
}

#[aoc(day17, part1)]
fn part1(info: &Info) -> String {
    process(&info.register, &info.instr, |_| true)
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[aoc(day17, part2)]
fn part2(info: &Info) -> u32 {
    for a in 0.. {
        //println!("{a}");
        let this_try_reg = Register { a, ..info.register };
        let output = process(&this_try_reg, &info.instr, |output| {
            output.iter().zip(info.instr.iter()).all(|(a, b)| a == b)
        });
        if output == info.instr {
            return a;
        }
    }
    panic!("How ??")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(EXAMPLE),
            Info {
                register: Register { a: 729, b: 0, c: 0 },
                instr: vec![0, 1, 5, 4, 3, 0]
            }
        );
    }

    #[test_case(EXAMPLE => "4,6,3,5,6,3,5,2,1,0"; "example")]
    fn test_part1(input: &str) -> String {
        part1(&parse(input))
    }

    #[test]
    fn mini_test1() {
        let register = Register { c: 9, a: 42, b: 42 };
        let (opcode, operand) = (2, 6);
        assert_eq!(
            operate(opcode, operand, &register),
            OpSideEffect::ChangeRegB(1)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EXAMPLE2)), 117_440);
    }
}
