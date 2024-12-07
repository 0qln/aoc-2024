use std::iter;
use ilog::IntLog;

use itertools::Itertools;

use super::{parse, solutions};

#[cfg(test)]
mod test;

pub fn concatenate(l: u64, r: u64) -> u64 {
    r + (l * 10u64.pow((1 + r.log10()) as u32))
}

pub fn solve(input: &str) -> u64 {
    let equations = parse(input);
    const OPS: [fn(u64, u64) -> u64; 3] = [
        u64::saturating_mul, 
        u64::saturating_add,
        concatenate,
    ];
    equations
        .iter()
        .filter(|eq| solutions(&eq.1, &OPS).any(|sol| sol == eq.0))
        .map(|eq| eq.0)
        .sum()
}