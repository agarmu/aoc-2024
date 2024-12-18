use itertools::Itertools;

aoc_2024::solution!(17);

type Reg = u64;
#[derive(Debug, Clone)]
struct Machine<'a> {
    a: Reg,
    b: Reg,
    c: Reg,
    ip: usize,
    rom: &'a [Reg],
}

impl<'a> Machine<'a> {
    #[inline]
    fn combo(&self, val: Reg) -> Reg {
        match val {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => unreachable!("Reserved!"),
            x => unreachable!("Invalid input: {}", x),
        }
    }
    #[inline]
    fn read(&mut self) -> Reg {
        let v = self.rom[self.ip] as Reg;
        self.ip += 1;
        v
    }

    #[inline]
    fn execute_instruction(&mut self) -> Option<u64> {
        let opcode = self.read();
        let operand = self.read();
        match opcode {
            0 => {
                // adv
                let c = self.combo(operand);
                self.a >>= c;
            }
            1 => {
                // bxl
                self.b ^= operand;
            }
            2 => {
                let c = self.combo(operand);
                // bst
                self.b = c % 8;
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
                let c = self.combo(operand);
                return Some(c % 8);
            }
            6 => {
                // bdv
                let c = self.combo(operand);
                self.b = self.a >> c;
            }
            7 => {
                // adv
                let c = self.combo(operand);
                self.c = self.a >> c;
            }
            x => unreachable!("Invalid opcode {}", x),
        }
        None
    }

    const fn new(a: Reg, b: Reg, c: Reg, rom: &'a [Reg]) -> Self {
        let ip = 0;
        Self { a, b, c, ip, rom }
    }
}

impl Iterator for Machine<'_> {
    type Item = Reg;
    fn next(&mut self) -> Option<Self::Item> {
        while self.ip < self.rom.len() {
            let next = self.execute_instruction();
            if next.is_some() {
                return next;
            }
        }
        None
    }
}

fn parse(input: &str) -> Option<(Reg, Reg, Reg, Vec<Reg>)> {
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

    Some((a, b, c, input))
}

pub fn part_one(input: &str) -> Option<String> {
    let (a, b, c, rom) = parse(input)?;
    let machine = Machine::new(a, b, c, &rom);
    Some(machine.map(|x| x.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, _, _, rom) = parse(input)?;
    let v = backtrack_search(&rom, rom.len() - 1, 0);
    v.into_iter().min()
}

fn backtrack_search(rom: &[u64], idx: usize, a: u64) -> Vec<u64> {
    let k = (0..8).map(move |x| a * 8 + x).filter(move |a| {
        let m = Machine::new(*a, 0, 0, rom);
        m.eq(rom[idx..].iter().copied())
    });
    if idx == 0 {
        k.collect()
    } else {
        k.flat_map(move |a| backtrack_search(rom, idx - 1, a).into_iter())
            .collect()
    }
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
}
