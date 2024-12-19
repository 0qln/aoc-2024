use std::collections::LinkedList;

use itertools::Itertools;
use regex::Regex;

use crate::day_6::{self, Dir, Map, Pos};

pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> (Map, Vec<isize>) {
    let (map, moves) = input.split_once("\r\n\r\n").unwrap();
    let map = day_6::parse(map);
    let moves = moves.chars().filter_map(|c| match c {
        '^' => Some(3),
        '>' => Some(4),
        'v' => Some(1),
        '<' => Some(2),
        _ => None,
    }).collect_vec();
    (map, moves)
}
