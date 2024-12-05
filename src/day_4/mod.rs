use std::iter;

use itertools::Itertools;

pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> Vec<Vec<char>> {
    let line_len = input.lines().next().unwrap().len();
    let edge = (0..line_len + 2).map(|_| '.').collect_vec();
    iter::once(edge.clone())
        .chain(input.lines().map(|line: &str| {
            iter::once('.')
                .chain(line.chars())
                .chain(iter::once('.'))
                .collect_vec()
        }))
        .chain(iter::once(edge))
        .collect_vec()
}