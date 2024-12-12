use hashbrown::HashMap;
use memoize::memoize;

aoc_2024::solution!(11);

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn total_length(x: u64, n: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    let s = x.to_string();
    if let Some(v) = memo.get(&(x, n)) {
        *v
    } else if n == 0 {
        1
    } else if x == 0 {
        let m = total_length(1, n - 1, memo);
        memo.insert((x, n), m);
        m
    } else if s.len() % 2 == 0 {
        let r = s[(s.len() / 2)..].parse().unwrap();
        let l = s[0..(s.len() / 2)].parse().unwrap();
        let m = total_length(l, n - 1, memo) + total_length(r, n - 1, memo);
        memo.insert((x, n), m);
        m
    } else {
        let m = total_length(x * 2024, n - 1, memo);
        memo.insert((x, n), m);
        m
    }
}
pub fn apply(v: &mut Vec<u64>) {
    let l = v.len();
    for i in 0..l {
        let s = v[i].to_string();
        // try to apply rule 1
        if v[i] == 0 {
            v[i] = 1;
        } else if s.len() % 2 == 0 {
            let r = s[(s.len() / 2)..].parse::<u64>().unwrap();
            let l = s[0..(s.len() / 2)].parse::<u64>().unwrap();
            v[i] = l;
            v.push(r);
        } else {
            v[i] *= 2024;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    let mut memo = HashMap::new();
    Some(input.iter().map(|x| total_length(*x, 25, &mut memo)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);
    let mut memo = HashMap::new();
    Some(input.iter().map(|x| total_length(*x, 75, &mut memo)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
