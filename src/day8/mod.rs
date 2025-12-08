use std::collections::HashSet;

use itertools::Itertools;

pub const DATA: &str = include_str!("./input.txt");

type Point3 = (i64, i64, i64);

fn parse(input: &str) -> impl Iterator<Item = Point3> + '_ {
    input.lines().filter_map(|l| {
        let mut s = l.split(',');
        let x = s.next()?.parse().ok()?;
        let y = s.next()?.parse().ok()?;
        let z = s.next()?.parse().ok()?;
        Some((x, y, z))
    })
}

fn distance(a: &Point3, b: &Point3) -> f32 {
    ((a.0 - b.0).pow(2) as f32 + (a.1 - b.1).pow(2) as f32 + (a.2 - b.2).pow(2) as f32).sqrt()
}

pub fn part1(input: &str, limit: usize) -> i64 {
    let mut distances = vec![];
    let points = parse(input).collect_vec();

    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            distances.push((distance(p1, p2), *p1, *p2));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut clusters: Vec<HashSet<Point3>> = vec![];

    for (_, p1, p2) in distances.into_iter().take(limit) {
        let ids = clusters
            .iter()
            .enumerate()
            .filter(|(_, c)| c.contains(&p1) || c.contains(&p2))
            .map(|(i, _)| i)
            .sorted()
            .rev()
            .collect_vec();

        match ids.len() {
            1 => {
                clusters[ids[0]].insert(p1);
                clusters[ids[0]].insert(p2);
            }
            2 => {
                let mut nn = HashSet::new();

                let a = clusters.remove(ids[0]);
                let b = clusters.remove(ids[1]);
                nn.extend(a);
                nn.extend(b);
                clusters.push(nn);
            }
            _ => clusters.push(HashSet::from_iter([p1, p2])),
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
    let mut distances = vec![];
    let points = parse(input).collect_vec();

    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            distances.push((distance(p1, p2), *p1, *p2));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut clusters: Vec<HashSet<Point3>> =
        points.iter().map(|p| HashSet::from_iter([*p])).collect();

    for (_, p1, p2) in distances.into_iter() {
        let ids = clusters
            .iter()
            .enumerate()
            .filter(|(_, c)| c.contains(&p1) || c.contains(&p2))
            .map(|(i, _)| i)
            .sorted()
            .rev()
            .collect_vec();

        match ids.len() {
            1 => {
                clusters[ids[0]].insert(p1);
                clusters[ids[0]].insert(p2);
            }
            2 => {
                let mut nn = HashSet::new();

                let a = clusters.remove(ids[0]);
                let b = clusters.remove(ids[1]);
                nn.extend(a);
                nn.extend(b);
                clusters.push(nn);

                if clusters.len() == 1 {
                    return p1.0 * p2.0;
                }
            }
            _ => clusters.push(HashSet::from_iter([p1, p2])),
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
