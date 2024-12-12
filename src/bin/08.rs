use hashbrown::HashSet;
use std::cmp::max;

use aoc_2024::util::Vec2;
use itertools::Itertools;
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

pub fn generate_maps(input: &[Vec<u8>]) -> Vec<Vec<Vec2<i64>>> {
    let mut maps = (0..256)
        .map(|_| Vec::<Vec2<i64>>::new())
        .collect::<Vec<_>>();
    for (y, l) in input.iter().enumerate() {
        for (x, c) in l.iter().enumerate() {
            if *c != b'.' {
                maps[*c as usize].push(Vec2::new(x as i64, y as i64));
            }
        }
    }

    maps
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let w = input[0].len() as i64;
    let h = input.len() as i64;
    let maps = generate_maps(&input);
    // compute the relevalt antinodes
    Some(
        maps.iter()
            .flat_map(|charnodes| {
                charnodes.iter().tuple_combinations().flat_map(|(&a, &b)| {
                    let delta = b - a;
                    let p1 = b + delta;
                    let p2 = a - delta;
                    [p1, p2].into_iter()
                })
            })
            .filter(|item| (0..w).contains(&item.x) && (0..h).contains(&item.y))
            .sorted()
            .dedup()
            .count(),
    )
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let w = input[0].len() as i64;
    let h = input.len() as i64;
    let m = max(w, h);
    let maps = generate_maps(&input);
    let mut antinodes = HashSet::<Vec2<i64>>::new();
    // compute the relevalt antinodes
    for character_nodes in maps.iter() {
        for (c1, c2) in character_nodes.iter().tuple_combinations() {
            let delta = *c2 - *c1;
            let l = gcd(delta.x.abs(), delta.y.abs());
            let delta_norm = delta / l;
            for t in 0..m {
                let locus = *c1 + delta_norm * t;
                if locus.x < 0 || locus.x >= w || locus.y < 0 || locus.y >= h {
                    break;
                }
                antinodes.insert(locus);
            }
            for t in 0..m {
                let locus = *c1 - delta_norm * t;
                if locus.x < 0 || locus.x >= w || locus.y < 0 || locus.y >= h {
                    break;
                }
                antinodes.insert(locus);
            }
        }
    }
    Some(antinodes.len())
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
        assert_eq!(result, Some(34));
    }
}
