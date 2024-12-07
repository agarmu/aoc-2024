aoc_2024::solution!(4);

use aoc_2024::util::*;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|x| x.as_bytes().to_vec())
        .collect::<Vec<_>>()
}

fn discover_dir(data: &[Vec<u8>], string: &[u8], base_pt: Vec2<i64>, dir: Vec2<i64>) -> bool {
    for (t, x) in string.iter().enumerate() {
        let p = base_pt + dir * (t as i64);
        let Some(c) = data.try_access(p) else {
            return false;
        };
        if c != *x {
            return false;
        }
    }
    true
}
fn discover(data: &[Vec<u8>], string: &[u8], base_pt: Vec2<i64>) -> usize {
    if string.is_empty() || data.access(base_pt) != string[0] {
        return 0;
    }
    Vec2::<i64>::MOORE
        .iter()
        .filter(|dir| discover_dir(data, string, base_pt, **dir))
        .count()
}

const fn check_dia(a: u8, b: u8) -> bool {
    (a == b'M' && b == b'S') || (a == b'S' && b == b'M')
}
fn discover_2(data: &[Vec<u8>], base_pt: Vec2<i64>) -> bool {
    let process = |x| data.try_access(base_pt + x).unwrap_or(b'-');
    let ne = process(Vec2::<i64>::NE);
    let se = process(Vec2::<i64>::SE);
    let nw = process(Vec2::<i64>::NW);
    let sw = process(Vec2::<i64>::SW);
    let center = process(Vec2::<i64>::ZZ);
    check_dia(ne, sw) && check_dia(se, nw) && center == b'A'
}

fn part_one(input: &str) -> Option<usize> {
    let search_string = b"XMAS";
    let input = parse(input);
    Some(
        Vec2::<i64>::cover(&input)
            .map(|v| discover(&input, search_string, v))
            .sum(),
    )
}

fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(
        Vec2::<i64>::cover(&input)
            .filter(|v| discover_2(&input, *v))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
