use aoc_2024::util::{Access, Vec2};
use hashbrown::HashSet;
use itertools::Itertools;

aoc_2024::solution!(10);

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|x| x.trim().bytes().map(|x| x - b'0').collect())
        .collect()
}

fn dfs(input: &[Vec<u8>], memo: &mut [Vec<Option<HashSet<Vec2<i64>>>>], point: Vec2<i64>) {
    if memo.access(point).is_some() {
        return;
    }
    let v = *input.access(point);

    let mut h = HashSet::new();
    if v == 9 {
        h.insert(point);
    } else {
        for dir in [
            Vec2::<i64>::N,
            Vec2::<i64>::S,
            Vec2::<i64>::E,
            Vec2::<i64>::W,
        ] {
            if let Some(&v2) = input.try_access(point + dir) {
                if v2 == v + 1 {
                    // discover there!!
                    dfs(input, memo, point + dir);
                    let q = memo.access(point + dir).as_ref().unwrap();
                    h.extend(q);
                }
            }
        }
    }

    *memo.mut_access(point) = Some(h);
}

fn dfs_rating(input: &[Vec<u8>], memo: &mut [Vec<Option<usize>>], point: Vec2<i64>) -> usize {
    memo.access(point).as_ref().copied().map_or_else(
        || {
            let v = *input.access(point);
            if v == 9 {
                1
            } else {
                let r = [
                    Vec2::<i64>::N,
                    Vec2::<i64>::S,
                    Vec2::<i64>::E,
                    Vec2::<i64>::W,
                ]
                .into_iter()
                .map(|dir| point + dir)
                .filter(|x| input.try_access(*x).copied() == Some(v + 1))
                .map(|x| dfs_rating(input, memo, x))
                .sum();
                *memo.mut_access(point) = Some(r);
                r
            }
        },
        |q| q,
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut memo = input
        .iter()
        .map(|x| x.iter().map(|_| None).collect_vec())
        .collect_vec();

    Some(
        Vec2::<i64>::cover(&input)
            .map(|locus| {
                let c = input.access(locus);
                if *c == 0 {
                    dfs(&input, &mut memo, locus);
                    memo.access(locus).as_ref().map(|x| x.len()).unwrap_or(0)
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let mut memo = input
        .iter()
        .map(|x| x.iter().map(|_| None).collect_vec())
        .collect_vec();

    Some(
        Vec2::<i64>::cover(&input)
            .map(|locus| {
                let c = input.access(locus);
                if *c == 0 {
                    dfs_rating(&input, &mut memo, locus)
                } else {
                    0
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
