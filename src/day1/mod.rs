pub const DATA: &str = include_str!("./input.txt");

pub fn run(input: &str, f: impl Fn((i32, i32), &str) -> (i32, i32)) -> i32 {
    input.lines().map(str::trim).fold((50, 0), f).1
}

pub fn part1((pos, count): (i32, i32), line: &str) -> (i32, i32) {
    let n = line[1..].trim().parse::<i32>().unwrap_or(0);
    let next = (pos + if line.starts_with('L') { -n } else { n }).rem_euclid(100);
    (next, count + (next == 0) as i32)
}

pub fn part2((pos, count): (i32, i32), line: &str) -> (i32, i32) {
    let n = line[1..].parse::<i32>().unwrap_or(0);

    if line.starts_with('L') {
        let p_adj = if pos == 0 { 100 } else { pos };
        ((pos - n).rem_euclid(100), count + (n - p_adj + 100) / 100)
    } else {
        ((pos + n).rem_euclid(100), count + (pos + n) / 100)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        let input: &str = "L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82";

        let result = super::run(input, super::part1);
        assert_eq!(result, 3)
    }

    #[test]
    fn test_2() {
        let input: &str = "L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82";

        let result = super::run(input, super::part2);
        assert_eq!(result, 6)
    }
}
