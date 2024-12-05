use std::collections::HashMap;

#[aoc_generator(day1)]
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

#[aoc(day1, part1)]
pub fn part1((left, right): &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut left = left.clone();
    let mut right = right.clone();
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(x, y)| x.abs_diff(*y) as i64)
        .sum()
}

#[aoc(day1, part2)]
pub fn part2((left, right): &(Vec<i64>, Vec<i64>)) -> i64 {
    let mut counts: HashMap<i64, i64> = HashMap::new();
    for x in right {
        *counts.entry(*x).or_insert(0) += 1;
    }
    left.iter()
        .map(|&x| {
            if let Some(q) = counts.get(&x) {
                x * (*q)
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p1_sample() {
        let mut l = vec![3, 4, 2, 1, 3, 3];
        let mut r = vec![4, 3, 5, 3, 9, 3];
        l.sort();
        r.sort();
        let c = (l, r);
        assert_eq!(super::part1(&c), 11);
    }
    #[test]
    fn test_p2_sample() {
        let mut l = vec![3, 4, 2, 1, 3, 3];
        let mut r = vec![4, 3, 5, 3, 9, 3];
        l.sort();
        r.sort();
        let c = (l, r);
        assert_eq!(super::part2(&c), 31);
    }
}
