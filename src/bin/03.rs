aoc_2024::solution!(3);

use regex::Regex;

struct Mul {
    lhs: i64,
    rhs: i64,
}
impl Mul {
    const fn compute(&self) -> i64 {
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
        use Instr::{Do, Dont, Multiply};
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

fn parse_one(input: &str) -> Vec<Mul> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
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

fn part_one(input: &str) -> Option<i64> {
    Some(parse_one(input).iter().map(Mul::compute).sum())
}

fn parse_two(input: &str) -> Vec<Instr> {
    let regex = Regex::new(r"(mul\((\d+),(\d+)\)|do?\(\)|don't\(\))").unwrap();
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
                return Instr::Do;
            } else {
                return Instr::Dont;
            }
        })
        .collect::<Vec<_>>()
}

fn part_two(input: &str) -> Option<i64> {
    let mut sum = 0;
    let mut enable = true;
    for x in parse_two(input) {
        sum += x.compute(&mut enable);
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
