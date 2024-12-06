use std::collections::HashSet;

use itertools::Itertools;

use super::{is_valid, parse, RuleSet, Update};

#[cfg(test)]
mod test;

type Comparisons = HashSet<(u8, u8)>;

pub fn revise(update: &mut Update, cmp: &Comparisons) {
    update.sort_by(|a, b| {
        let kvp = (*a, *b);
        match cmp.contains(&kvp) {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Equal,
        }
    });
}

pub fn solve(input: &str) -> usize {
    let (rules, mut updates) = parse(input);
    let comparisons = rules
        .iter()
        .flat_map(|(k, vs)| vs.iter().map(|v| (*k, *v)))
        .collect::<Comparisons>();
    let mut invalid_updates = updates
        .iter_mut()
        .filter(|update| !is_valid(update, &rules))
        .collect_vec();
    for x in &mut invalid_updates {
        revise(x, &comparisons)
    }
    invalid_updates
        .iter()
        .map(|x| x[x.len() / 2] as usize)
        .sum()
}
