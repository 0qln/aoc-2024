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

pub fn solve(input: &str, size: usize, simulate_bytes: usize) -> usize {
    let falling_bytes = parse(input);
    let falling_bytes = falling_bytes
        .iter()
        .take(simulate_bytes)
        .cloned()
        .collect::<FxHashSet<Pos>>();
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
    shortest_path.unwrap().1
}
