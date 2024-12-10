use std::{iter, ops::Deref};

use itertools::Itertools;

use crate::{
    day_6::{get_dir, Map, Pos},
    day_8::setoff,
};

use super::parse;

#[cfg(test)]
mod test;

pub fn find_tails(map: &Map, pos: Pos, expected: char) -> Box<dyn Iterator<Item = Pos> + '_> {
    match map.get(pos) {
        Some('9') if expected == '9' => Box::new(iter::once(pos)),
        Some(level) if expected == level => {
            let expected = level.to_digit(10).unwrap() + 1;
            let expected = char::from_digit(expected, 10).unwrap();
            Box::new((0..4).flat_map(move |dir| {
                let dir = get_dir(dir);
                let new_pos = setoff(pos, dir);
                find_tails(map, new_pos, expected)
            }))
        }
        Some('.') | None | _ => Box::new(iter::empty()),
    }
}

pub fn solve(input: &str) -> usize {
    let map = parse(input);
    map.find_all(|&ch| ch == '0')
        .map(|head| find_tails(&map, head, '0').unique().count())
        .sum()
}
