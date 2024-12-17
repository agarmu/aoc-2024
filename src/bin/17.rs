use itertools::Itertools;

aoc_2024::solution!(17);

type Reg = u64;
#[derive(Debug, Clone)]
struct Machine {
    a: Reg,
    b: Reg,
    c: Reg,
    ip: usize,
    input: Vec<u8>,
    output: Vec<Reg>,
}

impl Machine {
    fn parse(input: &str) -> Option<Self> {
        let (fst, snd) = input.split_once("\n\n")?;

        let mut regvals = fst
            .lines()
            .map(|l| l.split_once(":").unwrap().1.trim())
            .map(|x| x.parse().unwrap());

        let a = regvals.next()?;
        let b = regvals.next()?;
        let c = regvals.next()?;

        let input = snd
            .trim()
            .split_once(":")?
            .1
            .trim()
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();

        Some(Self {
            a,
            b,
            c,
            ip: 0,
            input,
            output: Vec::new(),
        })
    }
    #[inline]
    fn combo(&mut self, val: Reg) -> Reg {
        match val {
            0..=3 => val as Reg,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => unreachable!("Reserved!"),
            x => unreachable!("Invalid input: {}", x),
        }
    }
    #[inline]
    fn read(&mut self) -> Reg {
        let v = self.input[self.ip] as Reg;
        self.ip += 1;
        v
    }

    #[inline]
    fn execute_instruction(&mut self) {
        let opcode = self.read();
        let operand = self.read();
        match opcode {
            0 => {
                // adv
                self.a >>= self.combo(operand);
            }
            1 => {
                // bxl
                self.b ^= operand;
            }
            2 => {
                // bst
                self.b = self.combo(operand) % 8;
            }
            3 => {
                // jnz
                if self.a != 0 {
                    self.ip = operand as usize;
                }
            }
            4 => {
                // bxc
                self.b ^= self.c;
            }
            5 => {
                // output
                let res = self.combo(operand) % 8;
                self.output.push(res);
            }
            6 => {
                // bdv
                self.b = self.a >> self.combo(operand);
            }
            7 => {
                // cdv
                self.c = self.a >> self.combo(operand);
            }
            x => unreachable!("Invalid opcode {}", x),
        }
    }

    fn simulate(&mut self) -> String {
        while self.ip < self.input.len() {
            self.execute_instruction();
        }
        self.output.iter().map(Reg::to_string).join(",")
    }
}

fn parse(input: &str) -> Option<Machine> {
    Machine::parse(input)
}

pub fn part_one(input: &str) -> Option<String> {
    Some(parse(input)?.simulate())
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
