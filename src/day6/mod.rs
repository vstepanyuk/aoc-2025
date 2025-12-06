use itertools::Itertools;

pub const DATA: &str = include_str!("./input.txt");

type Expr = (Vec<u64>, char);

pub fn run(input: &str, f: fn(&str) -> u64) -> u64 {
    f(input)
}

pub fn part1(input: &str) -> u64 {
    let count = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .count();
    let mut exprs: Vec<Expr> = vec![Default::default(); count];

    for line in input.lines() {
        if line.contains('+') {
            let ops = line
                .split_ascii_whitespace()
                .filter_map(|l| l.chars().next())
                .collect_vec();

            for (i, o) in ops.into_iter().enumerate() {
                exprs[i].1 = o;
            }
        } else {
            let nums = line
                .split_ascii_whitespace()
                .filter_map(|n| n.parse::<u64>().ok())
                .collect_vec();

            for (i, n) in nums.into_iter().enumerate() {
                exprs[i].0.push(n);
            }
        }
    }

    exprs
        .into_iter()
        .map(|expr| {
            let initial = (expr.1 == '*') as u64;

            let result =
                expr.0.into_iter().fold(
                    initial,
                    |acc, n| {
                        if initial == 1 {
                            acc * n
                        } else {
                            acc + n
                        }
                    },
                );
            result
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let last_line = input.lines().rev().next().unwrap(); //.unwrap().chars();

    let mut lens = last_line
        .split(|x| x != ' ')
        .skip(1)
        .map(|x| x.len())
        .collect_vec();

    *lens.last_mut().unwrap() += 1;

    let ops = last_line.chars().filter(|c| *c != ' ').collect_vec();

    let mut nums = vec![];
    for line in input.lines().rev().skip(1) {
        let mut nn = vec![];
        let mut start = 0;

        for l in lens.iter() {
            nn.push(line[start..start + *l].to_string());
            start += *l + 1;
        }

        for (i, n) in nn.iter().enumerate() {
            if nums.len() <= i {
                nums.push(vec![]);
            }
            nums[i].push(n.clone());
        }
    }

    let mut result = 0;
    for (i, op) in ops.iter().enumerate() {
        let a = nums[i].iter().rev().collect_vec();

        let len = nums[i][0].len();
        let mut b = vec![];
        for n in 0..len {
            let c = a
                .iter()
                .map(|s| s.chars().nth(n).unwrap())
                .filter(|c| *c != ' ')
                .collect::<String>()
                .parse::<u64>()
                .unwrap();

            b.push(c);
        }

        if *op == '*' {
            let p = b.into_iter().product::<u64>();
            result += p;
        } else {
            let s = b.into_iter().sum::<u64>();
            result += s;
        }
    }
    result
}

#[cfg(test)]
mod tests {

    static INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_1() {
        let result = super::run(INPUT, super::part1);
        assert_eq!(result, 4277556)
    }

    #[test]
    fn test_2() {
        let result = super::run(INPUT, super::part2);
        assert_eq!(result, 3263827)
    }
}
