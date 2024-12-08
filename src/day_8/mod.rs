use core::f64;
use std::{collections::HashSet, iter};

use itertools::Itertools;
use regex::Regex;

use crate::day_6::{self, Dir, Map, Pos};

pub mod part_1;
pub mod part_2;

fn delta(a: Pos, b: Pos) -> Dir {
    (a.0 as isize - b.0 as isize, a.1 as isize - b.1 as isize)
}

fn setoff(pos: Pos, dir: Dir) -> Pos {
    let left = pos.0 as isize + dir.0;
    let right = pos.1 as isize + dir.1;
    (left as usize, right as usize)
}

fn mul(a: Dir, b: isize) -> Dir {
    (a.0 * b, a.1 * b)
}

pub fn parse(input: &str) -> Vec<Map> {
    let map = day_6::parse(input);
    map
    // find frequenceis
    .map(|&ch| match ch { '.' => None, _ => Some(ch), })
    .filter_map(|f| f)
    .unique()
    // create map with only the frequencies
    .map(|f| {
        map.find_all(|&ch| ch == f).fold(map.clone_empty(), |mut f_map, antenna| {
            f_map.set(antenna, '@');
            f_map
        })
    })
    .collect_vec()
}
