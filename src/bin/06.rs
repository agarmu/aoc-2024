aoc_2024::solution!(6);

use aoc_2024::util::Access as _;
use aoc_2024::util::Vec2;
use hashbrown::{HashMap, HashSet};
use rayon::iter::IntoParallelRefIterator as _;
use rayon::iter::ParallelIterator as _;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Blocked,
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    const fn dir(self) -> Vec2<i64> {
        match self {
            Self::North => Vec2::<i64>::N,
            Self::South => Vec2::<i64>::S,
            Self::East => Vec2::<i64>::E,
            Self::West => Vec2::<i64>::W,
        }
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn next(&mut self) {
        use Dir::{East, North, South, West};
        *self = match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(Clone)]
struct Parse {
    pub cells: Vec<Vec<Cell>>,
    pub start_pos: Vec2<i64>,
}

fn parse(input: &str) -> Parse {
    use Cell::{Blocked, Empty};
    let mut res = Vec::new();
    let mut v = Vec2::<i64>::ZZ;
    for (y, line) in input.trim().lines().map(str::as_bytes).enumerate() {
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

fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(run_nocheckloop(&input.cells, input.start_pos, Dir::North).len())
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pred {
    pub loc: Vec2<i64>,
    pub dir: Dir,
}

#[inline(always)]
fn run_nocheckloop(
    cells: &[Vec<Cell>],
    start_pos: Vec2<i64>,
    start_dir: Dir,
) -> HashMap<Vec2<i64>, Pred> {
    use Cell::{Blocked, Empty};
    let mut pred: HashMap<Vec2<i64>, Pred> = HashMap::new();
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;
    let mut prev_pos = start_pos;
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
                pred.entry(current_pos).or_insert(Pred {
                    loc: prev_pos,
                    dir: current_dir,
                });
                prev_pos = current_pos;
                current_pos += current_dir.dir();
            }
        }
    }
    pred
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Visit {
    loc: Vec2<i64>,
    dir: Dir,
}

impl Visit {
    const fn new(loc: Vec2<i64>, dir: Dir) -> Self {
        Self { loc, dir }
    }
}
fn induces_loop(
    cells: &[Vec<Cell>],
    start_pos: Vec2<i64>,
    start_dir: Dir,
    obstacle_added: Vec2<i64>,
) -> bool {
    use Cell::Blocked;
    let mut obstacles_hit: HashSet<Visit> = HashSet::new();
    let mut current_pos = start_pos;
    let mut current_dir = start_dir;
    loop {
        match cells.try_access(current_pos) {
            None => {
                break;
            }
            Some(x) => {
                if *x == Blocked || current_pos == obstacle_added {
                    // check if we've already visited
                    let s = obstacles_hit.len();
                    obstacles_hit.insert(Visit::new(current_pos, current_dir));
                    if obstacles_hit.len() == s {
                        return true;
                    }
                    current_pos -= current_dir.dir();
                    current_dir.next();
                } else {
                    current_pos += current_dir.dir();
                }
            }
        }
    }
    false
}

fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    // first run the part one solution to determine which cells I am allowed to
    // edit (these are, naturally, only the cells where the guard naturally goes)
    let mut preds = run_nocheckloop(&input.cells, input.start_pos, Dir::North);
    let cells: &[Vec<Cell>] = &input.cells;
    preds.remove(&input.start_pos); // cannot drop an obstacle on the guard
    Some(
        preds
            .par_iter()
            .filter(|(x, prev)| induces_loop(cells, prev.loc, prev.dir, **x))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
