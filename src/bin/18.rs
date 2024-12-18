use std::collections::VecDeque;

use aoc_2024::util::Vec2;
use bit_vec::BitVec;
use hashbrown::{HashMap, HashSet};

aoc_2024::solution!(18);

fn parse(input: &str) -> HashMap<Vec2<i64>, usize> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let x = x.trim().parse().ok().unwrap();
            let y = y.trim().parse().ok().unwrap();
            Vec2::new(x, y)
        })
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect()
}

const fn out_of_bounds(v: Vec2<i64>, target: Vec2<i64>) -> bool {
    v.x < 0 || v.x > target.x || v.y < 0 || v.y > target.y
}

pub fn part_one_inner(input: &str, width: usize, height: usize, max_level: usize) -> Option<usize> {
    let blocks = parse(input);
    let mut queue = VecDeque::new();
    queue.push_back(Vec2::new(0, 0));
    let mut visited = BitVec::from_elem(width * height, false);
    //let mut visited = HashSet::<Vec2<i64>>::new();
    let target = Vec2::new(width as i64 - 1, height as i64 - 1);
    for level in 1..(width * height) {
        let l = queue.len();
        for _ in 0..l {
            let u = queue.pop_front()?;
            for dir in &Vec2::<i64>::CARDINALS {
                let v = u + *dir;
                if v == target {
                    return Some(level);
                } else if out_of_bounds(v, target) {
                    continue;
                }
                let idx = (v.x as usize) * height + (v.y as usize);
                if visited[idx] || blocks.get(&v).is_some_and(|v| *v < max_level) {
                    continue;
                }
                queue.push_back(v);
                visited.set(idx, true);
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_inner(input, 71, 71, 1024)
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
        let result = part_one_inner(&aoc_2024::template::read_file("examples", DAY), 7, 7, 12);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
