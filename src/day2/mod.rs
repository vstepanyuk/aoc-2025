use itertools::Itertools;
use rayon::prelude::*;
use std::ops::RangeInclusive;

pub const DATA: &str = include_str!("./input.txt");

pub fn run(input: &str, f: fn(RangeInclusive<u64>) -> Vec<u64>) -> u64 {
    input
        .split(',')
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            let start = start.parse::<u64>().ok()?;
            let end = end.parse::<u64>().ok()?;

            Some(start..=end)
        })
        .par_bridge()
        .flat_map(f)
        .sum()
}

pub fn part1(range: RangeInclusive<u64>) -> Vec<u64> {
    range
        .into_par_iter()
        .filter(|i| {
            let digits_str = i.to_string();
            let len = digits_str.len();

            len % 2 == 0 && digits_str[0..len / 2] == digits_str[len / 2..]
        })
        .collect()
}

pub fn part2(range: RangeInclusive<u64>) -> Vec<u64> {
    range
        .into_par_iter()
        .filter(|n| {
            let digits_str = n.to_string();

            (1..=digits_str.len() / 2).any(|i| digits_str.as_bytes().chunks(i).all_equal())
        })
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result = super::run(input, super::part1);
        assert_eq!(result, 1227775554)
    }

    #[test]
    fn test_2() {
        let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result = super::run(input, super::part2);
        assert_eq!(result, 4174379265)
    }
}
