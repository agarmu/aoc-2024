use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
    edges: HashMap<i64, HashSet<i64>>,
    sorts: Vec<Vec<i64>>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let (lhs, rhs) = input.split_once("\n\n").expect("Could not parse");

    let edgelist = lhs.lines().map(|x| {
        let (from, to) = x.split_once("|").expect("Could not parse line");
        (
            from.parse::<i64>().expect("Invalid Number"),
            to.parse::<i64>().expect("Invalid Number"),
        )
    });

    let mut edges = HashMap::new();
    for (from, to) in edgelist {
        edges
            .entry(from)
            .or_insert(HashSet::<i64>::new())
            .insert(to);
    }

    let sorts = rhs
        .lines()
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<i64>().expect("Invalid Number"))
                .collect()
        })
        .collect();

    Input { edges, sorts }
}

fn quick_sort<T, F>(v: &mut [T], f: &F)
where
    T: Copy,
    F: Fn(T, T) -> bool,
{
    let len = v.len();
    if len >= 2 {
        let pivot_index = partition(v, f);
        quick_sort(&mut v[0..pivot_index], f);
        quick_sort(&mut v[pivot_index + 1..len], f);
    }
}

fn partition<T, F>(v: &mut [T], f: &F) -> usize
where
    T: Copy,
    F: Fn(T, T) -> bool,
{
    let len = v.len();
    let pivot_index = len / 2;
    let last_index = len - 1;

    v.swap(pivot_index, last_index);

    let mut store_index = 0;
    for i in 0..last_index {
        let q = f(v[i], v[last_index]);
        if q {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> i64 {
    let is_lesseq = |x, y| input.edges.get(y).map(|s| !s.contains(x)).unwrap_or(true);
    input
        .sorts
        .iter()
        .filter(|x| x.is_sorted_by(is_lesseq))
        .map(|x| {
            x[x.len() / 2] // assume odd length
        })
        .sum()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> i64 {
    let e = input.edges.clone();
    let is_lesseq = |x, y| e.get(y).map(|s| !s.contains(x)).unwrap_or(true);
    let is_lesseq2 = |x, y| e.get(&y).map(|s| !s.contains(&x)).unwrap_or(true);

    let mut sum = 0;
    for l in &input.sorts {
        if l.is_sorted_by(is_lesseq) {
            continue;
        }
        let mut l = l.clone();
        quick_sort(&mut l, &is_lesseq2);
        sum += l[l.len() / 2];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const TEST_INPUT: &str = include_str!("../input/2024/day5_sample.txt");
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_INPUT)), 143);
    }
    //
    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
