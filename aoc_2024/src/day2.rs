#[aoc_generator(day2)]
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

#[must_use] pub fn is_safe(x: &[i64]) -> bool {
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

#[must_use] pub fn is_almost_safe(x: &[i64]) -> bool {
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

#[aoc(day2, part1)]
#[must_use] pub fn part1(dat: &[Vec<i64>]) -> usize {
    dat.iter().filter(|x| is_safe(x)).count()
}

#[aoc(day2, part2)]
#[must_use] pub fn part2(dat: &[Vec<i64>]) -> usize {
    dat.iter().filter(|x| is_almost_safe(x)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_p1_sample() {
        let dat = parse(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );

        assert_eq!(super::part1(&dat), 2);
    }
    #[test]
    fn test_p2_sample() {
        let dat = parse(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );

        assert_eq!(super::part2(&dat), 4);
    }
}
