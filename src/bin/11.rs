aoc_2024::solution!(11);

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
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

pub fn part_one(input: &str) -> Option<usize> {
    let mut input = parse(input);
    for _ in 0..25 {
        apply(&mut input);
    }
    Some(input.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);

    None
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

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
