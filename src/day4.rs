use aoc_runner_derive::{aoc, aoc_generator};

use crate::util::*;

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|x| x.as_bytes().to_vec())
        .collect::<Vec<_>>()
}

fn discover_dir(data: &[Vec<u8>], string: &[u8], base_pt: Vec2<i64>, dir: Vec2<i64>) -> bool {
    for (t, x) in string.iter().enumerate() {
        let p = base_pt + dir * (t as i64);
        let Some(c) = data.try_access(p) else {
            return false;
        };
        if c != *x {
            return false;
        }
    }
    true
}
fn discover(data: &[Vec<u8>], string: &[u8], base_pt: Vec2<i64>) -> usize {
    if string.is_empty() || data.access(base_pt) != string[0] {
        return 0;
    }
    Vec2::<i64>::MOORE
        .iter()
        .filter(|dir| discover_dir(data, string, base_pt, **dir))
        .count()
}

fn check_dia(chara: u8, charb: u8) -> bool {
    (chara == b'M' && charb == b'S') || (chara == b'S' && charb == b'M')
}
fn discover_2(data: &[Vec<u8>], base_pt: Vec2<i64>) -> bool {
    let process = |x| data.try_access(base_pt + x).unwrap_or(b'-');
    let a = process(Vec2::<i64>::NE);
    let b = process(Vec2::<i64>::SE);
    let c = process(Vec2::<i64>::NW);
    let d = process(Vec2::<i64>::SW);
    let x = process(Vec2::<i64>::ZZ);
    check_dia(a, d) && check_dia(b, c) && x == b'A'
}

#[aoc(day4, part1)]
fn part1(input: &[Vec<u8>]) -> usize {
    let search_string = "XMAS".as_bytes();
    Vec2::<i64>::cover(input)
        .map(|v| discover(input, search_string, v))
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Vec<u8>]) -> usize {
    Vec2::<i64>::cover(input)
        .filter(|v| discover_2(input, *v))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        let s = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let p = parse(s);
        assert_eq!(part1(&p), 18);
    }

    #[test]
    fn part2_example() {
        let s = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let p = parse(s);
        let q: &[Vec<u8>] = &p;
        assert_eq!(part2(q), 9);
    }
}
