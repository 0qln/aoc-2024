use itertools::Itertools;

use super::{is_valid, parse};

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> usize {
    let (rules, updates) = parse(input);
    let valid_updates = updates
        .iter()
        .filter(|update| is_valid(update, &rules))
        .collect_vec();

    valid_updates.iter().map(|&x| x[x.len() / 2] as usize).sum()
}
