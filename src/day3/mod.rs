pub const DATA: &str = include_str!("./input.txt");

pub fn run(input: &str, f: fn(&[u8]) -> u64) -> u64 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect::<Vec<_>>()
        })
        .map(|v| f(&v))
        .sum()
}

pub fn part1(digits: &[u8]) -> u64 {
    find_max(digits, 2)
}

pub fn part2(digits: &[u8]) -> u64 {
    find_max(digits, 12)
}

fn find_max(digits: &[u8], remaining: usize) -> u64 {
    let mut to_drop = digits.len() - remaining;
    let mut stack: Vec<u8> = Vec::with_capacity(digits.len());

    for &digit in digits {
        while to_drop > 0 && stack.last().is_some_and(|&last| last < digit) {
            stack.pop();
            to_drop -= 1;
        }
        stack.push(digit);
    }

    stack.truncate(remaining);
    stack.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        let input: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

        let result = super::run(input, super::part1);
        assert_eq!(result, 357)
    }

    #[test]
    fn test_2() {
        let input: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

        let result = super::run(input, super::part2);
        assert_eq!(result, 3121910778619)
    }
}
