use std::collections::{HashMap, HashSet};

use pathfinding::grid::Grid;

pub const DATA: &str = include_str!("./input.txt");

fn parse(input: &str) -> (Grid, (usize, usize)) {
    let lines = input.lines().collect::<Vec<_>>();
    let height = lines.len();
    let width = lines[0].len();
    let mut start = None;
    let mut grid = Grid::new(width, height);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                grid.add_vertex((x, y));
            } else if c == 'S' {
                start = Some((x, y));
            }
        }
    }

    (grid, start.unwrap())
}

pub fn part1(input: &str) -> usize {
    fn count_split(grid: &Grid, (x, y): (usize, usize), visited: &mut HashSet<(usize, usize)>) {
        let ny = y + 1;

        if ny == grid.height {
            return;
        }

        if grid.has_vertex((x, ny)) {
            if visited.insert((x, ny)) {
                count_split(grid, (x - 1, ny), visited);
                count_split(grid, (x + 1, ny), visited);
            }
        } else {
            count_split(grid, (x, ny), visited);
        }
    }

    let (grid, start) = parse(input);
    let mut visited = HashSet::new();
    count_split(&grid, start, &mut visited);

    visited.len()
}

pub fn part2(input: &str) -> usize {
    fn count_paths(
        grid: &Grid,
        (x, y): (usize, usize),
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if y == grid.height - 1 {
            return 1;
        }

        if let Some(&cached) = memo.get(&(x, y)) {
            return cached;
        }

        let ny = y + 1;
        let result = if grid.has_vertex((x, ny)) {
            count_paths(grid, (x - 1, ny), memo) + count_paths(grid, (x + 1, ny), memo)
        } else {
            count_paths(grid, (x, ny), memo)
        };

        memo.insert((x, y), result);
        result
    }

    let (grid, start) = parse(input);
    count_paths(&grid, start, &mut HashMap::new())
}

#[cfg(test)]
mod tests {

    static INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_1() {
        let result = super::part1(INPUT);
        assert_eq!(result, 21)
    }

    #[test]
    fn test_2() {
        let result = super::part2(INPUT);
        assert_eq!(result, 40)
    }
}
