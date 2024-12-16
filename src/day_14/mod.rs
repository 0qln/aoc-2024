use std::collections::LinkedList;

use itertools::Itertools;
use regex::Regex;

use crate::day_6::{self, Dir, Pos};

pub mod part_1;
pub mod part_2;

type P = (isize, isize);

pub fn parse(input: &str) -> Vec<(Pos, Dir)> {
    Regex::new(r"p=((\d+),(\d+)) v=((-?\d+),(-?\d+))")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            let pos = (cap[2].parse().unwrap(), cap[3].parse().unwrap());
            let dir = (cap[5].parse().unwrap(), cap[6].parse().unwrap());
            (pos, dir)
        })
        .collect_vec()
}
