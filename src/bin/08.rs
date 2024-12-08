use std::{cmp::max, collections::HashSet};

use aoc_2024::util::Vec2;
use itertools::{Combinations, Itertools};
aoc_2024::solution!(8);

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|x| x.as_bytes().to_owned())
        .collect()
}

pub fn check_insert(antinodes: &mut HashSet<Vec2<i64>>, item: Vec2<i64>, w: i64, h: i64, _c: u8) {
    if item.x >= 0 && item.x < w && item.y >= 0 && item.y < h {
        antinodes.insert(item);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let w = input[0].len() as i64;
    let h = input.len() as i64;
    let mut maps: Vec<Vec<Vec2<i64>>> = vec![Vec::new(); 256];
    let mut antinodes = HashSet::<Vec2<i64>>::new();
    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c != b'.' {
                maps[*c as usize].push(Vec2::new(x as i64, y as i64));
            }
        }
    }

    // compute the relevalt antinodes
    for (c, character_nodes) in maps.iter().enumerate() {
        for (c1, c2) in character_nodes.iter().tuple_combinations() {
            let a = *c1;
            let b = *c2;
            let delta = b - a;
            let p1 = b + delta;
            let p2 = a - delta;
            check_insert(&mut antinodes, p1, w, h, c as u8);
            check_insert(&mut antinodes, p2, w, h, c as u8);
        }
    }
    Some(antinodes.len())
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn part_two_inner(input: &[Vec<u8>]) -> HashSet<Vec2<i64>> {
    let w = input[0].len() as i64;
    let h = input.len() as i64;
    let m = max(w, h);
    let mut maps: Vec<Vec<Vec2<i64>>> = vec![Vec::new(); 256];
    let mut antinodes = HashSet::<Vec2<i64>>::new();
    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c != b'.' {
                maps[*c as usize].push(Vec2::new(x as i64, y as i64));
            }
        }
    }

    // compute the relevalt antinodes
    for (c, character_nodes) in maps.iter().enumerate() {
        for (c1, c2) in character_nodes.iter().tuple_combinations() {
            let delta = *c2 - *c1;
            let l = gcd(delta.x.abs(), delta.y.abs());
            let delta_norm = delta / l;
            for t in -m..=m {
                check_insert(&mut antinodes, *c1 + (delta_norm * t), w, h, c as u8);
            }
        }
    }
    antinodes
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(part_two_inner(&input).len())
}

#[cfg(test)]
mod tests {
    use core::str;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let file_contents = &aoc_2024::template::read_file("examples", DAY);
        let mut p = parse(file_contents);
        let i = part_two_inner(&p);
        for l in &i {
            p[l.y as usize][l.x as usize] = b'#';
        }
        eprintln!(
            "{}",
            p.iter().map(|x| str::from_utf8(x).unwrap()).join("\n")
        );

        assert_eq!(i.len(), 34);
    }
}
