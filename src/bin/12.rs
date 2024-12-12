use aoc_2024::util::{Access, Vec2};
use hashbrown::HashSet;

aoc_2024::solution!(12);

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.trim().lines().map(|x| x.bytes().collect()).collect()
}

#[derive(Debug, PartialEq, Eq)]
enum DiscoveryResult {
    UnvisitedEqual,
    UnvisitedUnequal,
    Visited { area: i64, perimeter: i64 },
}

impl DiscoveryResult {
    fn cost(&self) -> i64 {
        match self {
            DiscoveryResult::Visited { area, perimeter } => {
                if *area < 0 || *perimeter < 0 {
                    unimplemented!()
                }
                area * perimeter
            }
            _ => 0,
        }
    }
}

fn cost(visited: &mut [Vec<bool>], data: &[Vec<u8>], point: Vec2<i64>) -> i64 {
    let x = discover(visited, data, point, data.access(point));
    x.cost()
}

fn discover(
    visited: &mut [Vec<bool>],
    data: &[Vec<u8>],
    point: Vec2<i64>,
    value: u8,
) -> DiscoveryResult {
    use DiscoveryResult::*;
    let value_match = data.try_access(point) == Some(value);
    let already_visited = visited.try_access(point) == Some(true);
    match (value_match, already_visited) {
        (false, _) => UnvisitedUnequal,
        (true, true) => UnvisitedEqual,
        (true, false) => {
            *visited.mut_access(point) = true;
            let mut cur_perim = 0;
            let mut cur_area = 1;
            for dir in [
                Vec2::<i64>::N,
                Vec2::<i64>::S,
                Vec2::<i64>::E,
                Vec2::<i64>::W,
            ] {
                match discover(visited, data, point + dir, value) {
                    UnvisitedUnequal => {
                        cur_perim += 1;
                    } // this is a boundary
                    UnvisitedEqual => {} // this is NOT a boundary, do nothing
                    Visited { area, perimeter } => {
                        cur_area += area;
                        cur_perim += perimeter;
                    }
                }
            }
            Visited {
                area: cur_area,
                perimeter: cur_perim,
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let input: &[Vec<u8>] = &parse(input);
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let unvisited = Vec2::<i64>::cover(input);
    let mut sum = 0;
    for point in unvisited {
        sum += cost(&mut visited, input, point);
    }
    Some(sum)
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
