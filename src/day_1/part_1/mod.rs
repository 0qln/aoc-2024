use itertools::Itertools;

use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> u64 {
    let input = parse(input);
    let (lefts, rights): (Vec<i64>, Vec<i64>) = input.unzip();
    lefts.into_iter().sorted()
        .zip(rights.into_iter().sorted())
        .map(|(l, r)| { l.abs_diff(r) })
        .sum()
}
