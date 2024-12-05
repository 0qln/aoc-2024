use std::iter;

use itertools::Itertools;
use regex::Regex;

use crate::day_4::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> usize {
    let grid = parse(input);

    fn check_word(
        (top_dir, left_dir): (isize, isize),
        (mut top, mut left): (usize, usize),
        grid: &Vec<Vec<char>>,
    ) -> bool {
        "XMAS".chars().all(&mut |c| {
            let result = c == grid[top][left];
            top = top.saturating_add_signed(top_dir);
            left = left.saturating_add_signed(left_dir);
            result
        })
    }

    fn find_words(coords: (usize, usize), grid: &Vec<Vec<char>>) -> usize {
        let dirs = (-1..=1).cartesian_product(-1..=1);
        dirs.map(|dir| check_word(dir, coords, grid))
            .filter(|b| *b)
            .count()
    }

    grid.iter()
        .enumerate()
        .flat_map(|(top, row)| (row.iter().enumerate().map(move |(left, _)| (left, top))))
        .map(|coords| find_words(coords, &grid))
        .sum()
}