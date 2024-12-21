use std::collections::HashSet;

use itertools::Itertools;

use super::parse;

#[cfg(test)]
mod test;

fn can_be_composed<'a>(
    available: &Vec<&'a str>,
    desired: &'a str,
    impossible: &mut HashSet<(&'a str, &'a str)>,
) -> bool {
    available.iter().any(move |towel| {
        let entry = (desired, *towel);
        if !impossible.contains(&entry) && {
            *towel == desired || {
                desired.starts_with(towel)
                    && can_be_composed(available, desired[towel.len()..].as_ref(), impossible)
            }
        } {
            true
        } else {
            impossible.insert(entry);
            false
        }
    })
}

pub fn solve(input: &str) -> usize {
    let (available, desired) = parse(input);
    desired
        .iter()
        .filter(|desired| can_be_composed(&available, desired, &mut HashSet::<_>::new()))
        .count()
}
