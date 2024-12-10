use std::{iter, ops::Deref};

use itertools::Itertools;

use crate::{
    day_6::{get_dir, Map, Pos},
    day_8::setoff,
};

use super::{parse, part_1::find_tails};

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> usize {
    let map = parse(input);
    map.find_all(|&ch| ch == '0')
        .map(|head| find_tails(&map, head, '0').count())
        .sum()
}
