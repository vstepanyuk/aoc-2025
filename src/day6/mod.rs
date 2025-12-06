use itertools::Itertools;

pub const DATA: &str = include_str!("./input.txt");

pub fn run(
    input: &str,
    f: impl for<'a> Fn(&'a [Vec<String>], usize) -> Box<dyn Iterator<Item = String> + 'a>,
) -> u64 {
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

    ops.iter()
        .enumerate()
        .map(|(i, op)| {
            let nn = f(&nums, i)
                .into_iter()
                .filter_map(|x| x.trim().parse::<u64>().ok());

            if *op == '*' {
                nn.product::<u64>()
            } else {
                nn.sum::<u64>()
            }
        })
        .sum()
}

pub fn part1(nums: &[Vec<String>], i: usize) -> Box<dyn Iterator<Item = String> + '_> {
    Box::new(nums[i].iter().cloned())
}

pub fn part2(nums: &[Vec<String>], i: usize) -> Box<dyn Iterator<Item = String> + '_> {
    let len = nums[i][0].len();

    Box::new((0..len).map(move |n| {
        nums[i]
            .iter()
            .filter_map(|s| s.chars().nth(n))
            .rev()
            .collect::<String>()
    }))
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
