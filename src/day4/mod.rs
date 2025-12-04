use itertools::Itertools;
use pathfinding::prelude::*;

pub const DATA: &str = include_str!("./input.txt");

pub fn run(input: &str, f: fn(Grid) -> usize) -> usize {
    // let grid = Diagram::from_rows(input.lines().map(|l| l.bytes())).unwrap();
    let mut grid = Grid::from_iter(input.lines().enumerate().flat_map(|(y, row)| {
        row.as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == b'@')
            .map(move |(x, _)| (x, y))
    }));

    grid.enable_diagonal_mode();
    f(grid)
}

pub fn part1(grid: Grid) -> usize {
    grid.iter()
        .filter(|(x, y)| grid.neighbours((*x, *y)).len() < 4)
        .count()
}

pub fn part2(mut grid: Grid) -> usize {
    let mut count = 0;
    loop {
        let to_remove = grid
            .iter()
            .filter(|(x, y)| grid.neighbours((*x, *y)).len() < 4)
            .collect_vec();

        count += to_remove.len();
        if to_remove.is_empty() {
            break;
        }

        for vertex in to_remove.into_iter() {
            grid.remove_vertex(vertex);
        }
    }

    count
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_1() {
        let input: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let result = super::run(input, super::part1);
        assert_eq!(result, 13)
    }

    #[test]
    fn test_2() {
        let input: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let result = super::run(input, super::part2);
        assert_eq!(result, 43)
    }
}
