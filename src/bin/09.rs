#![feature(array_repeat)]

use std::{
    array::repeat,
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, VecDeque},
    iter::repeat_n,
};

use itertools::Itertools;
use num::traits::ops::overflowing::OverflowingSub;

aoc_2024::solution!(9);

fn parse(input: &str) -> (Vec<Option<u16>>, VecDeque<usize>) {
    let mut v = Vec::with_capacity(100_000);

    let mut i = 0;
    let mut queue = VecDeque::new();

    for (id, byte) in input.trim().bytes().enumerate() {
        let size = (byte - b'0') as usize;
        if id % 2 != 0 {
            for u in 0..size {
                queue.push_back(i + u);
                v.push(None);
            }
            i += size;
        } else {
            let id = (id / 2) as u16;
            v.reserve(size);
            for _ in 0..size {
                v.push(Some(id));
            }
            i += size;
        }
    }

    (v, queue)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut v, mut q) = parse(input);

    for j in (0..v.len()).rev() {
        if v[j].is_none() {
            continue;
        }

        let Some(next_free) = q.pop_front() else {
            break;
        };
        if j <= next_free {
            break;
        }

        v.swap(j, next_free);
    }

    Some(
        v.iter()
            .enumerate()
            .map(|(i, v)| (v.unwrap_or(0) as usize) * i)
            .sum(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FreeSpace {
    idx: usize,
    size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct File {
    idx: usize,
    id: usize,
    size: usize,
}

impl File {
    fn checksum(&self) -> usize {
        let end = self.idx + self.size;
        let e = end * end.overflowing_sub(1).0;
        let s = self.idx * self.idx.overflowing_sub(1).0;
        ((e - s) / 2) * self.id
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.idx.cmp(&other.idx))
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.idx.cmp(&other.idx)
    }
}

fn parse2(input: &str) -> (Vec<File>, [BinaryHeap<Reverse<usize>>; 10]) {
    let mut v = Vec::with_capacity(100_000);

    let mut idx = 0;
    let mut free_space = repeat(BinaryHeap::new());

    for (id, byte) in input.trim().bytes().enumerate() {
        let size = (byte - b'0') as usize;
        if id % 2 != 0 {
            free_space[size].push(Reverse(idx));
            idx += size;
        } else {
            let id = id / 2;
            v.push(File { idx, id, size });
            idx += size;
        }
    }

    (v, free_space)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut files, mut free_space) = parse2(input);

    for j in (0..files.len()).rev() {
        let size = files[j].size;
        // (interval_size, interval_idx)
        let Some((interval_size, interval_idx)) = (size..=9)
            .flat_map(|i| free_space[i].peek().map(|x| (i, x.0)))
            .next()
        else {
            continue; // no interval found
        };
        // remove that interval
        free_space[interval_size].pop();

        // move file
        files[j].idx = interval_idx;

        let size_left = interval_size - size;
        let interval_idx_new = interval_idx + size;
        free_space[size_left].push(Reverse(interval_idx_new));
    }
    Some(files.iter().map(File::checksum).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    fn get_lst(v: &[Option<u16>]) -> String {
        v.iter()
            .map(|x| x.map(|x| x.to_string()).unwrap_or(".".to_owned()))
            .join("")
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }

    #[test]
    fn test_parse() {
        let d = &aoc_2024::template::read_file("examples", DAY);
        let p = parse(d);
        let q = get_lst(&p.0);
        assert_eq!(q, "00...111...2...333.44.5555.6666.777.888899");
    }
}
