use std::iter;

use itertools::Itertools;
use regex::Regex;

use crate::day_4::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> usize {
    let grid = parse(input);

    fn check_word(
        (top, left): (usize, usize),
        grid: &Vec<Vec<char>>,
    ) -> bool {
        if grid[top][left] != 'A' { return false; }
        let c11 = grid[top.saturating_add_signed(1)][left.saturating_add_signed(1)];
        let c12 = grid[top.saturating_add_signed(-1)][left.saturating_add_signed(-1)];
        let c21 = grid[top.saturating_add_signed(1)][left.saturating_add_signed(-1)];
        let c22 = grid[top.saturating_add_signed(-1)][left.saturating_add_signed(1)];
        let c1_hit = matches!((c11, c12), ('M', 'S') | ('S', 'M'));
        let c2_hit = matches!((c21, c22), ('M', 'S') | ('S', 'M'));
        c1_hit && c2_hit
    }

    grid.iter()
        .enumerate()
        .flat_map(|(top, row)| (row.iter().enumerate().map(move |(left, _)| (left, top))))
        .map(|coords| check_word(coords, &grid))
        .filter(|b| *b)
        .count()
}
