use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

struct Mul {
    lhs: i64,
    rhs: i64,
}
impl Mul {
    fn compute(&self) -> i64 {
        self.lhs * self.rhs
    }
}

enum Instr {
    Do,
    Dont,
    Multiply(Mul),
}

impl Instr {
    fn compute(&self, enable: &mut bool) -> i64 {
        use Instr::*;
        match self {
            Do => {
                *enable = true;
                0
            }
            Dont => {
                *enable = false;
                0
            }
            Multiply(x) => {
                if *enable {
                    x.compute()
                } else {
                    0
                }
            }
        }
    }
}

#[aoc_generator(day3, part1)]
fn parse(input: &str) -> Vec<Mul> {
    let regex = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    regex
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [lhs, rhs])| {
            let lhs = lhs.parse().unwrap();
            let rhs = rhs.parse().unwrap();
            Mul { lhs, rhs }
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Mul]) -> i64 {
    input.iter().map(Mul::compute).sum()
}

#[aoc_generator(day3, part2)]
fn parse2(input: &str) -> Vec<Instr> {
    let regex = Regex::new(r#"(mul\((\d+),(\d+)\)|do?\(\)|don't\(\))"#).unwrap();
    regex
        .captures_iter(input)
        .map(|c| {
            let mut q = c.iter();
            let fst = q.next().unwrap().unwrap();
            let r = fst.as_str();
            if r.as_bytes()[0] == b'm' {
                // in the mul case
                let _ = q.next();
                let arg1 = q.next().unwrap().unwrap().as_str();
                let arg2 = q.next().unwrap().unwrap().as_str();
                let lhs = arg1.parse().unwrap();
                let rhs = arg2.parse().unwrap();
                Instr::Multiply(Mul { lhs, rhs })
            } else if r.as_bytes()[2] == b'(' {
                Instr::Do
            } else {
                Instr::Dont
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day3, part2)]
fn part2(input: &[Instr]) -> i64 {
    let mut sum = 0;
    let mut enable = true;
    for x in input {
        sum += x.compute(&mut enable);
    }
    sum
}

#[cfg(test)]
mod tests {}
