use itertools::Itertools;
use pathfinding::directed::astar;
use rustc_hash::FxHashSet;

use crate::{
    day_6::{get_dir, Pos},
    day_8::setoff,
};

use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str, size: usize) -> Option<Pos> {
    let falling_bytes = parse(input);
    let mut falling_bytes_gen = falling_bytes.iter();
    let mut falling_bytes = FxHashSet::<Pos>::default();
    let mut blocking_byte = None;
    while {
        let target = size - 1;
        let cost = |pos: &Pos| target.abs_diff(pos.0) + target.abs_diff(target);
        let done = |pos: &Pos| pos.0 == target && pos.1 == target;
        let succ = |pos: &Pos| {
            (0..4)
                .filter_map(|dir| {
                    let succ = setoff(*pos, get_dir(dir));
                    let in_bounds = succ.1 < size && succ.0 < size;
                    (!falling_bytes.contains(&succ) && in_bounds).then_some((succ, 1))
                })
                .collect_vec()
        };
        let shortest_path = astar::astar(&(0, 0), succ, cost, done);
        shortest_path.is_some()
    } {
        blocking_byte = falling_bytes_gen.next().map(|pos| *pos);
        while !falling_bytes.insert(blocking_byte.unwrap()) {}
    }
    blocking_byte
}
