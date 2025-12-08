use std::collections::HashMap;

use itertools::Itertools;
use petgraph::unionfind::UnionFind;

pub const DATA: &str = include_str!("./input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3(i64, i64, i64);
impl Point3 {
    #[inline]
    fn distance(&self, other: &Self) -> i64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        let dz = self.2 - other.2;

        dx * dx + dy * dy + dz * dz
    }
}

fn parse(input: &str) -> Vec<Point3> {
    input
        .lines()
        .filter_map(|l| {
            let mut s = l.split(',');
            Some(Point3(
                s.next()?.parse().ok()?,
                s.next()?.parse().ok()?,
                s.next()?.parse().ok()?,
            ))
        })
        .collect()
}

pub fn part1(input: &str, limit: usize) -> usize {
    let points = parse(input);

    let uf = (0..points.len())
        .tuple_combinations()
        .map(|(i, j)| (points[i].distance(&points[j]), i, j))
        .sorted_by_key(|(d, _, _)| *d)
        .take(limit)
        .fold(UnionFind::new(points.len()), |mut uf, (_, i, j)| {
            uf.union(i, j);
            uf
        });

    (0..points.len())
        .fold(HashMap::new(), |mut acc, i| {
            *acc.entry(uf.find(i)).or_insert(0) += 1;
            acc
        })
        .values()
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub fn part2(input: &str) -> i64 {
    let points = parse(input);

    let distances = (0..points.len())
        .tuple_combinations()
        .map(|(i, j)| (points[i].distance(&points[j]), i, j))
        .sorted_by_key(|(d, _, _)| *d);

    let mut len: usize = points.len();
    let mut uf = UnionFind::new(len);

    for (_, i, j) in distances {
        if uf.union(i, j) {
            len -= 1;
        }

        if len == 1 {
            return points[i].0 * points[j].0;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {

    static INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_1() {
        let result = super::part1(INPUT, 10);
        assert_eq!(result, 40)
    }

    #[test]
    fn test_2() {
        let result = super::part2(INPUT);
        assert_eq!(result, 25272)
    }
}
