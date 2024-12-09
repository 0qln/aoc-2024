use std::{iter, ops::Deref};

use itertools::Itertools;

use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> usize {
    let files = parse(input);
    let front = files.iter();
    let mut back = files.iter().rev().filter(|c| c.is_some());
    front
        .map_while(&mut |maybe_file: &Option<_>| {
            match maybe_file { None => *back.next()?, x => *x }
        })
        // todo: theres probably a more efficient way to control the bounds
        .take(files.iter().filter(|c| c.is_some()).count())
        .enumerate()
        .map(|(i, id)| (i * id))
        .sum()
}