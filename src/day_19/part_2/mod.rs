use std::collections::HashMap;

use itertools::Itertools;

use super::parse;

#[cfg(test)]
mod test;

fn num_compositions<'a>(
    available: &Vec<&'a str>,
    desired: &'a str,
    compositions: &mut HashMap<(&'a str, &'a str), usize>,
) -> usize {
    available
        .iter()
        .map(move |towel| {
            let key = (desired, *towel);
            if compositions.contains_key(&key) {
                *compositions.get(&key).unwrap()
            } else {
                let value = if *towel == desired {
                    1
                } else if desired.starts_with(towel) {
                    num_compositions(available, desired[towel.len()..].as_ref(), compositions)
                } else {
                    0
                };
                compositions.insert(key, value);
                value
            }
        })
        .sum()
}

pub fn solve(input: &str) -> usize {
    let (available, desired) = parse(input);
    desired
        .iter()
        .map(|desired| num_compositions(&available, desired, &mut HashMap::<_, _>::new()))
        .sum()
}
