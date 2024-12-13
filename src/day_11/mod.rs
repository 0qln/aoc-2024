use std::collections::LinkedList;

use itertools::Itertools;
use regex::Regex;

pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> LinkedList<u64> {
    let re = Regex::new(r"(\d+)").unwrap();
    re.captures_iter(input).map(|cap| cap[1].parse().unwrap()).collect()
}