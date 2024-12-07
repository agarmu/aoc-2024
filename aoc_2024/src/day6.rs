use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use util::Access;
use util::Vec2;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cell {
    Blocked,
    Empty,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn dir(&self) -> Vec2<i64> {
        match self {
            Dir::North => Vec2::<i64>::N,
            Dir::South => Vec2::<i64>::S,
            Dir::East => Vec2::<i64>::E,
            Dir::West => Vec2::<i64>::W,
        }
    }
    fn next(&mut self) {
        use Dir::*;
        *self = match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(Debug, Clone)]
struct Parse {
    pub cells: Vec<Vec<Cell>>,
    pub start_pos: Vec2<i64>,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Parse {
    use Cell::*;
    let mut res = Vec::new();
    let mut v = Vec2::<i64>::ZZ;
    for (y, line) in input.trim().lines().map(|x| x.as_bytes()).enumerate() {
        let w = line.len() + 2;
        let mut r = Vec::with_capacity(w);
        for (x, c) in line.iter().enumerate() {
            r.push(match c {
                b'#' => Blocked,
                b'^' => {
                    v = Vec2 {
                        x: x as i64,
                        y: y as i64,
                    };
                    Empty
                }
                _ => Empty,
            });
        }
        res.push(r);
    }

    Parse {
        cells: res,
        start_pos: v,
    }
}

#[aoc(day6, part1)]
fn part1(input: &Parse) -> usize {
    run_nocheckloop(input).len()
}

fn run_nocheckloop(input: &Parse) -> HashSet<Vec2<i64>> {
    use Cell::*;
    use Dir::*;
    let mut visited: HashSet<Vec2<i64>> = HashSet::new();
    let mut current_pos = input.start_pos;
    let mut current_dir = North;
    let cells: &[Vec<Cell>] = &input.cells;

    loop {
        match cells.try_access(current_pos) {
            None => {
                break;
            }
            Some(Blocked) => {
                current_pos -= current_dir.dir();
                current_dir.next();
            }
            Some(Empty) => {
                visited.insert(current_pos);
                current_pos += current_dir.dir();
            }
        }
    }
    visited
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Visit {
    loc: Vec2<i64>,
    dir: Dir,
}

impl Visit {
    fn new(loc: Vec2<i64>, dir: Dir) -> Self {
        Self { loc, dir }
    }
}
fn induces_loop(input: &Parse, obstacle_added: Vec2<i64>) -> bool {
    use Cell::*;
    use Dir::*;
    let mut visited_dir: HashSet<Visit> = HashSet::new();
    let mut current_pos = input.start_pos;
    let mut current_dir = North;
    let cells: &[Vec<Cell>] = &input.cells;
    loop {
        match cells.try_access(current_pos) {
            None => {
                break;
            }
            Some(x) => {
                if x == Blocked || current_pos == obstacle_added {
                    current_pos -= current_dir.dir();
                    current_dir.next();
                } else {
                    // check if we've already visited
                    let s = visited_dir.len();
                    visited_dir.insert(Visit::new(current_pos, current_dir));
                    if visited_dir.len() == s {
                        return true;
                    }
                    current_pos += current_dir.dir();
                }
            }
        }
    }
    false
}

#[aoc(day6, part2)]
fn part2(input: &Parse) -> usize {
    // first run the part one solution to determine which cells I am allowed to
    // edit (these are, naturally, only the cells where the guard naturally goes)
    let mut cells_blockable = run_nocheckloop(input);
    cells_blockable.remove(&input.start_pos); // cannot drop an obstacle on the guard
    cells_blockable
        .iter()
        .filter(|x| induces_loop(input, **x))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    const TEST_INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    const TEST_E: &str = "
###
.^#
###
";

    const TEST_W: &str = "
###
.^.
#.#
";

    const TEST_N: &str = "
...
.^.
...
";
    const TEST_S: &str = "
###
.^#
#.#
";
    #[test]
    fn test_parse() {
        let parsed = parse(TEST_INPUT);
        assert_eq!(parsed.start_pos, Vec2 { x: 4, y: 6 });

        let parsed_n = parse(TEST_N);
        assert_eq!(parsed_n.start_pos, Vec2 { x: 1, y: 1 });
        for l in parsed_n.cells.iter() {
            for c in l.iter() {
                assert_eq!(Cell::Empty, *c);
            }
        }
    }
    #[test]
    fn part1_example() {
        let parsed = parse(TEST_INPUT);
        let output = part1(&parsed);
        assert_eq!(41, output);
        for (name, dir) in &[
            ("North", TEST_N),
            ("South", TEST_S),
            ("East", TEST_E),
            ("West", TEST_W),
        ] {
            assert_eq!(2, part1(&parse(dir)));
        }
    }

    #[test]
    fn part2_example() {}
}
