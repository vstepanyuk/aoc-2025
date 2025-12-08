use std::collections::HashSet;

use itertools::Itertools;

pub const DATA: &str = include_str!("./input.txt");

type Point3 = (i64, i64, i64);

fn parse(input: &str) -> (Vec<Point3>, Vec<(i64, Point3, Point3)>) {
    let points = input
        .lines()
        .filter_map(|l| {
            let mut s = l.split(',');
            Some((
                s.next()?.parse().ok()?,
                s.next()?.parse().ok()?,
                s.next()?.parse().ok()?,
            ))
        })
        .collect_vec();

    let distances = points
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| (distance(p1, p2), *p1, *p2))
        .sorted_by_key(|(d, _, _)| *d)
        .collect_vec();

    (points, distances)
}

#[inline]
fn distance(a: &Point3, b: &Point3) -> i64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

pub fn part1(input: &str, limit: usize) -> i64 {
    let (points, distances) = parse(input);

    let mut clusters: Vec<HashSet<Point3>> =
        points.iter().map(|p| HashSet::from_iter([*p])).collect();

    for (_, p1, p2) in distances.into_iter().take(limit) {
        let ids = find_clusters(&clusters, &p1, &p2);
        if ids.len() == 2 {
            merge(&mut clusters, &ids);
        }
    }

    clusters
        .iter()
        .map(|c| c.len() as i64)
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub fn part2(input: &str) -> i64 {
    let (points, distances) = parse(input);

    let mut clusters: Vec<HashSet<Point3>> =
        points.iter().map(|p| HashSet::from_iter([*p])).collect();

    for (_, p1, p2) in distances.into_iter() {
        let ids = find_clusters(&clusters, &p1, &p2);

        if ids.len() == 2 {
            merge(&mut clusters, &ids);
        }

        if clusters.len() == 1 {
            return p1.0 * p2.0;
        }
    }

    unreachable!()
}

fn find_clusters(clusters: &[HashSet<Point3>], p1: &Point3, p2: &Point3) -> Vec<usize> {
    clusters
        .iter()
        .enumerate()
        .filter(|(_, c)| c.contains(&p1) || c.contains(&p2))
        .map(|(i, _)| i)
        .sorted()
        .rev()
        .collect_vec()
}

fn merge(clusters: &mut Vec<HashSet<Point3>>, ids: &[usize]) {
    let mut a = clusters.remove(ids[0]);
    let b = clusters.remove(ids[1]);

    a.extend(b);

    clusters.push(a);
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
