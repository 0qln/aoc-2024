use std::collections::HashMap;

use super::part_1::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> u64 {
    let input = parse(input);
    let (lefts, rights): (Vec<i64>, Vec<i64>) = input.unzip();
    let rights_occurances = rights.iter().fold(HashMap::new(), |mut acc, r| {
        *acc.entry(*r).or_insert(0) += 1;
        acc
    });
    lefts.iter().map(|l| {
        let r = rights_occurances.get(l).unwrap_or(&0);
        (l * r) as u64
    }).sum()
}
