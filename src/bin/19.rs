use hashbrown::HashMap;

aoc_2024::solution!(19);

fn parse(input: &str) -> Option<(Vec<&str>, Vec<&str>)> {
    input.trim().split_once("\n\n").map(|(l, r)| {
        let patterns = l.split(",").map(|x| x.trim()).collect();
        let constructions = r.lines().map(|x| x.trim()).collect();
        (patterns, constructions)
    })
}

fn solvable<'a>(patterns: &[&str], string: &'a str, memo: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(c) = memo.get(string) {
        return *c;
    }
    let res = string.is_empty()
        || (patterns)
            .iter()
            .any(|&p| string.starts_with(p) && solvable(patterns, &string[p.len()..], memo));

    memo.insert(string, res);
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    let (patterns, constructions) = parse(input)?;
    let mut memo = HashMap::new();
    Some(
        constructions
            .iter()
            .filter(|&&x| solvable(&patterns, x, &mut memo))
            .count(),
    )
}

fn solvable2<'a>(patterns: &[&str], string: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(v) = memo.get(string) {
        return *v;
    }
    let res = if string.is_empty() {
        1
    } else {
        (patterns)
            .iter()
            .filter(|&p| string.starts_with(p))
            .map(|p| solvable2(patterns, &string[p.len()..], memo))
            .sum()
    };
    memo.insert(string, res);
    res
}

pub fn part_two(input: &str) -> Option<usize> {
    let (patterns, constructions) = parse(input)?;
    let mut memo = HashMap::new();
    Some(
        constructions
            .iter()
            .map(|x| solvable2(&patterns, x, &mut memo))
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
