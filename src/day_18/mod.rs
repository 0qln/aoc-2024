use itertools::Itertools;
use regex::Regex;

use crate::day_6::Pos;

pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> Vec<Pos> {
    Regex::new(r"(\d+),(\d+)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
        .collect_vec()
}
