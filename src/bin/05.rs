aoc_2024::solution!(5);

use hashbrown::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
    edges: HashMap<i64, HashSet<i64>>,
    sorts: Vec<Vec<i64>>,
}

fn parse(input: &str) -> Input {
    let (lhs, rhs) = input.split_once("\n\n").expect("Could not parse");

    let edgelist = lhs.lines().map(|x| {
        let (from, to) = x.split_once('|').expect("Could not parse line");
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
            x.split(',')
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

fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);
    let is_lesseq = |x, y| input.edges.get(y).is_none_or(|s| !s.contains(x));
    Some(
        input
            .sorts
            .iter()
            .filter(|x| x.is_sorted_by(is_lesseq))
            .map(|x| {
                x[x.len() / 2] // assume odd length
            })
            .sum(),
    )
}

fn part_two(input: &str) -> Option<i64> {
    let input = parse(input);
    let is_lesseq = |x, y| input.edges.get(y).is_none_or(|s| !s.contains(x));
    let is_lesseq2 = |x, y| input.edges.get(&y).is_none_or(|s| !s.contains(&x));

    let mut sum = 0;
    for l in &input.sorts {
        if l.is_sorted_by(is_lesseq) {
            continue;
        }
        let mut l = l.clone();
        quick_sort(&mut l, &is_lesseq2);
        sum += l[l.len() / 2];
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
