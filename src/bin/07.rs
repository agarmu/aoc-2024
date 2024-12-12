aoc_2024::solution!(7);

struct Data {
    pub target: u64,
    pub numbers: Vec<u64>,
}
fn parse(input: &str) -> Vec<Data> {
    input
        .trim()
        .lines()
        .map(|x| {
            let (l, r) = x.split_once(":").unwrap();

            let des = l.trim().parse().unwrap();

            let r = r
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .rev()
                .collect::<Vec<_>>();

            Data {
                target: des,
                numbers: r,
            }
        })
        .collect()
}

pub fn solvable_pt1(target: u64, numbers: &[u64]) -> bool {
    // base case: if numbers len = 0
    if numbers.len() == 1 {
        numbers[0] == target
    } else if numbers[0] <= target && solvable_pt1(target - numbers[0], &numbers[1..]) {
        true
    } else if target == 0 && numbers[0] == 0 {
        true // mul by 0
    } else if target != 0 && numbers[0] != 0 && target % numbers[0] == 0 {
        solvable_pt1(target / numbers[0], &numbers[1..])
    } else {
        false
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    Some(
        input
            .iter()
            .filter(|x| solvable_pt1(x.target, &x.numbers))
            .map(|x| x.target)
            .sum(),
    )
}

pub const fn suffix_strict(mut a: u64, mut b: u64) -> Option<u64> {
    if a <= b {
        return None;
    }
    while (b % 10) == (a % 10) && b != 0 {
        b /= 10;
        a /= 10;
    }
    if b == 0 {
        Some(a)
    } else {
        None
    }
}

pub fn solvable_pt2(target: u64, numbers: &[u64]) -> bool {
    // base case: if numbers len = 0
    if numbers.len() == 1 {
        numbers[0] == target
    } else if numbers[0] <= target && solvable_pt2(target - numbers[0], &numbers[1..]) {
        true
    } else if target == 0 && numbers[0] == 0 {
        true // mul by 0
    } else if target != 0
        && numbers[0] != 0
        && target % numbers[0] == 0
        && solvable_pt2(target / numbers[0], &numbers[1..])
    {
        true
    } else if let Some(x) = suffix_strict(target, numbers[0]) {
        solvable_pt2(x, &numbers[1..])
    } else {
        false
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);
    Some(
        input
            .iter()
            .filter(|x| solvable_pt2(x.target, &x.numbers))
            .map(|x| x.target)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_suffix() {
        assert_eq!(Some(21), suffix_strict(2112, 12));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
