use pathfinding::prelude::*;
use tap::Tap;

pub const DATA: &str = include_str!("./input.txt");

pub fn run(input: &str, f: fn(Grid) -> usize) -> usize {
    f(
        Grid::from_iter(input.lines().enumerate().flat_map(|(y, row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, v)| **v == b'@')
                .map(move |(x, _)| (x, y))
        }))
        .tap_mut(Grid::enable_diagonal_mode),
    )
}

pub fn part1(grid: Grid) -> usize {
    grid.iter()
        .filter(|(x, y)| grid.neighbours((*x, *y)).len() < 4)
        .count()
}

pub fn part2(mut grid: Grid) -> usize {
    let mut to_remove = Vec::new();
    std::iter::from_fn(|| {
        to_remove.extend(
            grid.iter()
                .filter(|&(x, y)| grid.neighbours((x, y)).len() < 4),
        );

        let removed = to_remove.len();
        to_remove.drain(..).for_each(|v| {
            grid.remove_vertex(v);
        });

        (removed > 0).then_some(removed)
    })
    .sum()
}

#[cfg(test)]
mod tests {

    static INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_1() {
        let result = super::run(INPUT, super::part1);
        assert_eq!(result, 13)
    }

    #[test]
    fn test_2() {
        let result = super::run(INPUT, super::part2);
        assert_eq!(result, 43)
    }
}
