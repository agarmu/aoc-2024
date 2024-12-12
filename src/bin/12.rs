use aoc_2024::util::{Access, Vec2};
use hashbrown::HashSet;
use itertools::iproduct;
use partitions::{partition_vec, PartitionVec};

aoc_2024::solution!(12);

fn to_idx(v: Vec2<i64>, size: Vec2<i64>) -> usize {
    (v.y * size.x + v.x) as usize
}

fn from_idx(k: usize, size: Vec2<i64>) -> Vec2<i64> {
    Vec2::new(k as i64 % size.x, (k as i64) / size.x)
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.trim().lines().map(|x| x.bytes().collect()).collect()
}

fn count_neighbors_matching(locus: Vec2<i64>, data: &[Vec<u8>], m: Option<u8>) -> usize {
    [
        Vec2::<i64>::N,
        Vec2::<i64>::S,
        Vec2::<i64>::E,
        Vec2::<i64>::W,
    ]
    .into_iter()
    .filter(|dir| data.try_access(locus + *dir) == m)
    .count()
}

fn is_corner(locus: Vec2<i64>, data: &[Vec<u8>], x: Vec2<i64>, y: Vec2<i64>) -> bool {
    let my = data.try_access(locus);
    let dx = data.try_access(locus + x);
    let dy = data.try_access(locus + y);
    let dc = data.try_access(locus + x + y);

    let is_ext = (dy != my) && (dx != my);
    let is_int = (dy == my) && (dx == my) && (dc != my);

    is_ext || is_int
}

fn count_corners(locus: Vec2<i64>, data: &[Vec<u8>]) -> usize {
    iproduct!(
        [Vec2::<i64>::N, Vec2::<i64>::S].iter(),
        [Vec2::<i64>::E, Vec2::<i64>::W].iter()
    )
    .filter(|(y, x)| is_corner(locus, data, **x, **y))
    .count()
}

fn count_external_neighbors(locus: Vec2<i64>, data: &[Vec<u8>]) -> usize {
    let s = data.try_access(locus);
    4 - count_neighbors_matching(locus, data, s)
}

fn collect_regions(input: &[Vec<u8>]) -> PartitionVec<Vec2<i64>> {
    let size = Vec2::new(input[0].len() as i64, input.len() as i64);
    let mut sets = (0..(size.x * size.y))
        .map(|x| from_idx(x as usize, size))
        .collect::<PartitionVec<_>>();

    for locus in Vec2::<i64>::cover(input) {
        let x = input.try_access(locus);
        for dir in [
            Vec2::<i64>::N,
            Vec2::<i64>::S,
            Vec2::<i64>::E,
            Vec2::<i64>::W,
        ] {
            if x == input.try_access(locus + dir) {
                sets.union(to_idx(locus, size), to_idx(locus + dir, size));
            }
        }
    }
    sets
}

fn part_one(input: &str) -> Option<usize> {
    let input: &[Vec<u8>] = &parse(input);
    let sets = collect_regions(input);
    Some(
        sets.all_sets()
            .map(|x| {
                let (area, perimeter) = x.fold((0, 0), |(count, perim), (_, v)| {
                    (count + 1, perim + count_external_neighbors(*v, input))
                });
                area * perimeter
            })
            .sum(),
    )
}

fn part_two(input: &str) -> Option<usize> {
    let input: &[Vec<u8>] = &parse(input);
    let sets = collect_regions(input);
    Some(
        sets.all_sets()
            .map(|x| {
                let (area, sides) = x.fold((0, 0), |(count, sides), (_, v)| {
                    (count + 1, sides + count_corners(*v, input))
                });
                area * sides
            })
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
