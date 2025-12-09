use geo::contains::Contains;
use geo::geometry::{LineString, Point, Polygon};
use std::sync::atomic::{AtomicUsize, Ordering};

use itertools::Itertools;
use rayon::prelude::*;

pub const DATA: &str = include_str!("./input.txt");

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter_map(|l| {
            let (x, y) = l.split_once(',')?;
            Some((x.parse().ok()?, y.parse().ok()?))
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .tuple_combinations()
        .map(area)
        .max()
        .unwrap_or(0) as usize
}

#[inline]
fn area((p1, p2): ((usize, usize), (usize, usize))) -> usize {
    (((p1.0 as isize - p2.0 as isize).abs() + 1) * ((p1.1 as isize - p2.1 as isize).abs() + 1))
        as usize
}

pub fn part2(input: &str) -> usize {
    let parsed = parse(input);

    let points = parsed
        .iter()
        .map(|(x, y)| Point::new(*x as f32, *y as f32).0)
        .collect_vec();

    let p = Polygon::new(LineString::new(points), vec![]);

    let max_area = AtomicUsize::new(0);
    parsed
        .into_iter()
        .tuple_combinations()
        .par_bridge()
        .filter_map(|(p1, p2)| {
            let area = area((p1, p2));

            if area <= max_area.load(Ordering::Relaxed) {
                return None;
            }

            let rect = geo::geometry::Rect::new(
                Point::new(p1.0 as f32, p1.1 as f32),
                Point::new(p2.0 as f32, p2.1 as f32),
            );

            if p.contains(&rect) {
                max_area.fetch_max(area, Ordering::Relaxed);
                Some(area)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {

    static INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_1() {
        let result = super::part1(INPUT);
        assert_eq!(result, 50)
    }

    #[test]
    fn test_2() {
        let result = super::part2(INPUT);
        assert_eq!(result, 24)
    }
}
