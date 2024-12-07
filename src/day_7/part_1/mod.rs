use std::iter;

use itertools::Itertools;

use super::{parse, solutions};

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> u64 {
    let equations = parse(input);
    const OPS: [fn(u64, u64) -> u64; 2] = [
        u64::saturating_mul, 
        u64::saturating_add
    ];
    equations
        .iter()
        .filter(|eq| solutions(&eq.1, &OPS).any(|sol| sol == eq.0))
        .map(|eq| eq.0)
        .sum()
}