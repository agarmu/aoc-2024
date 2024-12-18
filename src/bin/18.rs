use std::collections::VecDeque;

use aoc_2024::util::Vec2;
use bit_vec::BitVec;
use hashbrown::HashSet;
use itertools::Itertools;
use partitions::{partition_vec, PartitionVec};

aoc_2024::solution!(18);

fn parse_heatmap(order: impl Iterator<Item = Vec2<i64>>, width: usize, height: usize) -> Vec<i64> {
    let mut h = vec![i64::MAX; width * height];
    let size = Vec2::new(width as i64, height as i64);
    for (i, v) in order.enumerate() {
        h[vec_to_idx(v, size)] = i as i64 + 1;
    }
    h
}

fn parse_order(input: &str) -> impl use<'_> + Iterator<Item = Vec2<i64>> {
    input.trim().lines().map(|l| {
        let (x, y) = l.split_once(",").unwrap();
        let x = x.trim().parse().ok().unwrap();
        let y = y.trim().parse().ok().unwrap();
        Vec2::new(x, y)
    })
}

const fn out_of_bounds(v: Vec2<i64>, size: Vec2<i64>) -> bool {
    v.x < 0 || v.x >= size.x || v.y < 0 || v.y >= size.y
}

#[inline]
fn part_one_inner(input: &str, width: usize, height: usize, max_level: i64) -> Option<usize> {
    let blocks = parse_heatmap(parse_order(input), width, height);
    bfs(&blocks, width, height, max_level)
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_inner(input, 71, 71, 1024)
}

const fn vec_to_idx(v: Vec2<i64>, size: Vec2<i64>) -> usize {
    (v.x * size.y + v.y) as usize
}

#[inline]
fn bfs(blocks: &[i64], width: usize, height: usize, max_level: i64) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back(Vec2::new(0, 0));
    let mut visited = BitVec::from_elem(width * height, false);
    //let mut visited = HashSet::<Vec2<i64>>::new();
    let target = Vec2::new(width as i64 - 1, height as i64 - 1);
    let size = Vec2::new(width as i64, height as i64);
    for level in 1..(width * height) {
        let l = queue.len();
        for _ in 0..l {
            let u = queue.pop_front()?;
            for dir in &Vec2::<i64>::CARDINALS {
                let v = u + *dir;
                if v == target {
                    return Some(level);
                } else if out_of_bounds(v, size) {
                    continue;
                }
                let idx = vec_to_idx(v, Vec2::new(width as i64, height as i64));
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

#[inline]
fn join_at_locus(
    locus: Vec2<i64>,
    connected_components: &mut PartitionVec<()>,
    size: Vec2<i64>,
    heatmap: &[i64],
) {
    let idx = vec_to_idx(locus, size);
    if heatmap[idx] != i64::MAX {
        return; // i am blocked :(
    }
    for dir in Vec2::<i64>::CARDINALS {
        let adj = locus + dir;
        let adj_idx = vec_to_idx(adj, size);
        if out_of_bounds(adj, size) || heatmap[adj_idx] != i64::MAX {
            continue; // neighbor is blocked :(
        }
        connected_components.union(idx, adj_idx);
    }
}

fn part_two_inner(input: &str, width: usize, height: usize) -> Option<String> {
    let order = parse_order(input).collect_vec();
    let mut heatmap = parse_heatmap(order.iter().copied(), width, height);
    let size = Vec2::new(width as i64, height as i64);
    let mut disjoint_set = partition_vec![(); width * height];

    // compute connected components
    // with all blocks
    for x in 0..width {
        for y in 0..height {
            join_at_locus(
                Vec2::new(x as i64, y as i64),
                &mut disjoint_set,
                size,
                &heatmap,
            );
        }
    }
    // remove blocks one by one
    let start_idx = vec_to_idx(Vec2::new(0, 0), size);
    let target_idx = vec_to_idx(size - Vec2::new(1, 1), size);

    for j in (0..order.len()).rev() {
        heatmap[vec_to_idx(order[j], size)] = i64::MAX;
        join_at_locus(order[j], &mut disjoint_set, size, &heatmap);
        if disjoint_set.same_set(start_idx, target_idx) {
            return Some(format!("{},{}", order[j].x, order[j].y));
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_inner(input, 71, 71)
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
        let result = part_two_inner(&aoc_2024::template::read_file("examples", DAY), 7, 7);
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
