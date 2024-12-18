use std::collections::VecDeque;

use aoc_2024::util::Vec2;
use bit_vec::BitVec;
use itertools::Itertools;

aoc_2024::solution!(18);

fn parse(input: &str, width: usize, height: usize) -> (Vec<usize>, Vec<Vec2<usize>>) {
    let mut h = vec![usize::MAX; width * height];

    let v = input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let x: usize = x.trim().parse().ok().unwrap();
            let y: usize = y.trim().parse().ok().unwrap();
            Vec2::new(x, y)
        })
        .collect_vec();

    for (i, v) in v.iter().enumerate() {
        h[v.x * height + v.y] = i + 1;
    }

    (h, v)
}

const fn out_of_bounds(v: Vec2<i64>, target: Vec2<i64>) -> bool {
    v.x < 0 || v.x > target.x || v.y < 0 || v.y > target.y
}

#[inline]
fn part_one_inner(input: &str, width: usize, height: usize, max_level: usize) -> Option<usize> {
    let (blocks, _) = parse(input, width, height);
    bfs(&blocks, width, height, max_level)
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_inner(input, 71, 71, 1024)
}

#[inline]
fn bfs(blocks: &[usize], width: usize, height: usize, max_level: usize) -> Option<usize> {
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
                if visited[idx] || blocks[idx] <= max_level {
                    continue;
                }
                queue.push_back(v);
                visited.set(idx, true);
            }
        }
    }
    None
}

fn part_two_inner(input: &str, width: usize, height: usize, min_level: usize) -> Option<String> {
    let (blocks, blockorder) = parse(input, width, height);
    let max_level = blocks.len();
    (min_level..=max_level)
        .find(|x| bfs(&blocks, width, height, *x).is_none())
        .map(|i| blockorder[i - 1])
        .map(|v| format!("{},{}", v.x, v.y))
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_inner(input, 71, 71, 1024)
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
        let result = part_two_inner(&aoc_2024::template::read_file("examples", DAY), 7, 7, 12);
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
