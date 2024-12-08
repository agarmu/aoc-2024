aoc_2024::solution!(1);

use hashbrown::HashMap;

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let (l, r): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|x| -> (i64, i64) {
            let mut s = x.split_whitespace();
            let l = s.next().unwrap().parse::<i64>().unwrap();
            let r = s.next().unwrap().parse::<i64>().unwrap();
            (l, r)
        })
        .unzip();
    (l, r)
}

#[must_use]
pub fn part_one(input: &str) -> Option<i64> {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();
    Some(
        left.iter()
            .zip(right.iter())
            .map(|(x, y)| x.abs_diff(*y) as i64)
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<i64> {
    let (left, right) = parse(input);
    let mut counts: HashMap<i64, i64> = HashMap::new();
    for x in right {
        *counts.entry(x).or_insert(0) += 1;
    }
    Some(
        left.iter()
            .map(|&x| {
                if let Some(q) = counts.get(&x) {
                    x * (*q)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
