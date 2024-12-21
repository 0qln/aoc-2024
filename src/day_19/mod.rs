use itertools::Itertools;
use regex::Regex;

pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (available, desired) = input.split_once("\r\n\r\n").unwrap();
    let available = available.split(", ").collect_vec();
    let desired = desired.lines().collect_vec();
    (available, desired)
}
