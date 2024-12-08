use std::collections::HashSet;

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

pub fn check_insert(antinodes: &mut HashSet<Vec2<i64>>, item: Vec2<i64>, w: i64, h: i64) {
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
    for character_nodes in maps.iter() {
        for (c1, c2) in character_nodes.iter().tuple_combinations() {
            let a = *c1;
            let b = *c2;
            let delta = b - a;
            let p1 = b + delta;
            let p2 = a - delta;
            check_insert(&mut antinodes, p1, w, h);
            check_insert(&mut antinodes, p2, w, h);
        }
    }
    Some(antinodes.len())
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
