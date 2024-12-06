use std::collections::HashMap;

use regex::Regex;

pub mod part_1;
pub mod part_2;

type RuleSet = HashMap<u8, Vec<u8>>;
type Update = Vec<u8>;

fn parse_rules(input: &str) -> RuleSet {
    let re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    re.captures_iter(input)
        .fold(HashMap::new(), |mut acc, caps| {
            let key = caps[1].parse().unwrap();
            let val = caps[2].parse().unwrap();
            acc.entry(key).or_insert_with(Vec::new).push(val);
            acc
        })
}

fn parse_updates(input: &str) -> Vec<Update> {
    let re = Regex::new(r"(\d+,)+\d+").unwrap();
    re.captures_iter(input)
        .map(|cap| cap[0].split(',').map(|x| x.parse().unwrap()).collect())
        .collect()
}

pub fn parse(input: &str) -> (RuleSet, Vec<Update>) {
    (parse_rules(input), parse_updates(input))
}

pub fn is_valid(update: &Update, rules: &RuleSet) -> bool {
    update.iter().enumerate().all(|(i, &x)| {
        if let Some(xs_forbidden_before) = rules.get(&x) {
            return update[..i].iter().all(|y| !xs_forbidden_before.contains(y));
        }
        return true;
    })
}

