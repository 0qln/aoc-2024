use itertools::Itertools;

use crate::day_6::{self, Map};

pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> Map {
    day_6::parse(input)
}