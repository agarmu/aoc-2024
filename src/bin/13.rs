use aoc_2024::util::Vec2;

use nalgebra::Vector2;
use num::Float;

aoc_2024::solution!(13);

fn get_nums(x: &str, delim: &str) -> Vec2<i64> {
    let s = x.split_once(":").unwrap().1;
    let (l, r) = s.split_once(",").unwrap();

    let x = l.split_once(delim).unwrap().1;
    let y = r.split_once(delim).unwrap().1;

    let xval = x.parse().unwrap();
    let yval = y.parse().unwrap();

    Vec2::new(xval, yval)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Entry {
    a: Vec2<i64>,
    b: Vec2<i64>,
    result: Vec2<i64>,
}

type Mat = nalgebra::Matrix2<f64>;

impl Entry {
    fn parse(x: &str) -> Self {
        let mut lines = x.lines();
        let a = get_nums(lines.next().unwrap(), "+");
        let b = get_nums(lines.next().unwrap(), "+");
        let result = get_nums(lines.next().unwrap(), "=");

        Self { a, b, result }
    }

    fn solve(&self, offset: i64) -> Option<i64> {
        let a = self.a;
        let b = self.b;
        let result = self.result + Vec2::new(offset, offset);

        /*let mat = array![[a.x as f64, b.x as f64], [a.y as f64, b.y as f64]];
        let target = array![result.x as f64, result.y as f64];

        let res = mat.solve(target);*/

        let mat = Mat::new(a.x as f64, b.x as f64, a.y as f64, b.y as f64);
        let target = Vector2::<f64>::new(result.x as f64, result.y as f64);

        let soln = mat.lu().solve(&target)?;

        let i = soln[0].round() as i64;
        let j = soln[1].round() as i64;

        let x = a * i + b * j;
        if x == result {
            Some(3 * i + j)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Vec<Entry> {
    input.trim().split("\n\n").map(Entry::parse).collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);

    Some(input.iter().filter_map(|x| x.solve(0)).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse(input);
    Some(input.iter().filter_map(|x| x.solve(10000000000000)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));

        assert_eq!(Some(480), result);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
