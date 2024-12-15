use std::cmp::max;

use aoc_2024::util::{Access, Vec2};
use itertools::Itertools;
use num::{traits::Euclid, Integer};

aoc_2024::solution!(14);

const SIZE: Vec2<usize> = Vec2::new(101, 103);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Robot {
    position: Vec2<usize>,
    velocity: Vec2<usize>,
}

impl Robot {
    fn parse(x: &str, size: Vec2<usize>) -> Option<Self> {
        let (p, v) = x.split_once(" ")?;
        let position = parse_single(p, size)?;
        let velocity = parse_single(v, size)?;
        Some(Self { position, velocity })
    }

    fn elapse(mut self, dt: usize, size: Vec2<usize>) -> Self {
        self.position = (self.position + self.velocity * dt) % size;
        self
    }

    const fn quadrant(self, size: Vec2<usize>) -> Option<usize> {
        if (size.y % 2 == 1 && self.position.y == size.y / 2)
            || (size.x % 2 == 1 && self.position.x == size.x / 2)
        {
            None
        } else {
            Some(
                match (
                    (self.position.x < size.x / 2),
                    (self.position.y < size.y / 2),
                ) {
                    (true, true) => 0,
                    (true, false) => 1,
                    (false, true) => 2,
                    (false, false) => 3,
                },
            )
        }
    }
}

fn parse_single(x: &str, size: Vec2<usize>) -> Option<Vec2<usize>> {
    let (_, x) = x.split_once("=")?;
    let (l, r) = x.split_once(",")?;
    let x = l.parse::<i64>().ok()?.rem_euclid(size.x as i64) as usize;
    let y = r.parse::<i64>().ok()?.rem_euclid(size.y as i64) as usize;
    Some(Vec2::new(x, y))
}

fn parse(input: &str, size: Vec2<usize>) -> Vec<Robot> {
    input
        .trim()
        .lines()
        .filter_map(|x| Robot::parse(x, size))
        .collect()
}

pub fn part_one_inner(input: &str, size: Vec2<usize>) -> usize {
    parse(input, size)
        .into_iter()
        .map(|x| x.elapse(100, size))
        .filter_map(|x| x.quadrant(size))
        .fold([0usize; 4], |mut lst, i| {
            lst[i] += 1;
            lst
        })
        .into_iter()
        .product()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(part_one_inner(input, SIZE))
}

pub fn part_two_inner(input: &str, size: Vec2<usize>) -> Option<usize> {
    let mut robots = parse(input, size);
    // cannot calculate using Chinese Remainder Theorem in this case
    let egcd = (size.x as i64).extended_gcd(&(size.y as i64));
    if egcd.gcd != 1 {
        None
    } else {
        let mut varx = usize::MAX;
        let mut vary = usize::MAX;

        let mut ix = 0;
        let mut iy = 0;

        for time in 0..max(size.x, size.y) {
            // integer division is close enough
            let xavg = robots.iter().map(|x| x.position.x).sum::<usize>() / robots.len();
            let yavg = robots.iter().map(|x| x.position.y).sum::<usize>() / robots.len();

            let xerr = robots
                .iter()
                .map(|r| r.position.x - xavg)
                .map(|x| x * x)
                .sum::<usize>();

            let yerr = robots
                .iter()
                .map(|r| r.position.y - yavg)
                .map(|y| y * y)
                .sum::<usize>();

            if time < size.x && xerr < varx {
                varx = xerr;
                ix = time;
            }
            if time < size.y && yerr < vary {
                vary = yerr;
                iy = time;
            }
            robots.iter_mut().for_each(|x| *x = x.elapse(1, size));
        }

        let prod = size.x * size.y;
        let a = (size.x as i64 * egcd.x).rem_euclid(prod as i64) as usize;
        let b = (size.y as i64 * egcd.y).rem_euclid(prod as i64) as usize;

        // Combine indices using the Chinese Remainder Theorem to get index mod 10403.
        let t = (a * iy + b * ix) % prod;

        // reset to print properly
        //print_grid(&robots, size, size.x * size.y + t - max(size.x, size.y));
        Some(t)
    }
}

fn print_grid(robots: &[Robot], size: Vec2<usize>, t: usize) {
    let mut g = vec![vec!['.'; size.x]; size.y];

    for robot in robots.iter().map(|x| x.elapse(t, size)) {
        *g.mut_access(robot.position) = '#';
    }

    let s = g.into_iter().map(|x| x.into_iter().join("")).join("\n");
    eprintln!("{}", s);
}

pub fn part_two(input: &str) -> Option<usize> {
    part_two_inner(input, SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part_one() {
        let result = part_one_inner(
            &aoc_2024::template::read_file("examples", DAY),
            Vec2::new(11, 7),
        );
        assert_eq!(result, 12);
    }
}
