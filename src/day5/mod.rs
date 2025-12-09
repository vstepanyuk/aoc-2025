use std::ops::RangeInclusive;

use itertools::Itertools;
use rangemap::RangeInclusiveSet;

pub const DATA: &str = include_str!("./input.txt");

pub fn run(input: &str, f: fn(Vec<RangeInclusive<usize>>, &[usize]) -> usize) -> usize {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .filter_map(|l| {
            let (start, end) = l.split_once('-').unwrap();
            let start = start.parse::<usize>().ok()?;
            let end = end.parse::<usize>().ok()?;

            Some(RangeInclusive::new(start, end))
        })
        .collect_vec();

    let ids = ids
        .lines()
        .filter_map(|l| l.parse::<usize>().ok())
        .collect_vec();

    f(ranges, &ids)
}

pub fn part1(ranges: Vec<RangeInclusive<usize>>, ids: &[usize]) -> usize {
    ids.iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count()
}

pub fn part2(ranges: Vec<RangeInclusive<usize>>, _id: &[usize]) -> usize {
    RangeInclusiveSet::from_iter(ranges)
        .into_iter()
        .map(RangeInclusive::count)
        .sum()
}

#[cfg(test)]
mod tests {

    static INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_1() {
        let result = super::run(INPUT, super::part1);
        assert_eq!(result, 3)
    }

    #[test]
    fn test_2() {
        let result = super::run(INPUT, super::part2);
        assert_eq!(result, 14)
    }
}
