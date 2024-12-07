aoc_2024::solution!(2);

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[must_use]
pub fn is_safe(x: &[i64]) -> bool {
    if x.len() < 2 {
        return true;
    }
    let increasing = x[0] < x[1];
    let mut prev = x[0];
    for &u in &x[1..] {
        let delta = (u - prev).abs();

        let asc_agree = (prev < u) == increasing;
        let bounds = (1..=3).contains(&delta);
        if !(asc_agree && bounds) {
            return false;
        }
        prev = u;
    }
    true
}

#[must_use]
pub fn is_almost_safe(x: &[i64]) -> bool {
    if is_safe(x) {
        return true;
    }
    for i in 0..x.len() {
        let mut r = x.to_vec();
        r.remove(i);
        if is_safe(&r) {
            return true;
        }
    }
    false
}

#[must_use]
pub fn part_one(dat: &str) -> Option<usize> {
    Some(parse(dat).iter().filter(|x| is_safe(x)).count())
}

#[must_use]
pub fn part_two(dat: &str) -> Option<usize> {
    Some(parse(dat).iter().filter(|x| is_almost_safe(x)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
