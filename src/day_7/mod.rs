use core::f64;
use std::iter;

use itertools::Itertools;
use regex::Regex;

pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    let re_groups = Regex::new(r"(?P<test_val>\d+): (?P<nums>[\d ]+)").unwrap();
    re_groups
        .captures_iter(input)
        .map(|cap| {
            let test_val = cap["test_val"].parse().unwrap();
            let nums = cap["nums"].split(' ').map(|x| x.parse().unwrap()).collect_vec();
            (test_val, nums)
        })
        .collect_vec()
}

pub fn solutions<'a, 'b, 'c>(
    vals: &'a [u64],
    ops: &'b [fn(u64, u64) -> u64],
) -> Box<dyn Iterator<Item = u64> + 'c>
where
    'a: 'c,
    'b: 'c,
{
    match vals.len() {
        1 => Box::new(iter::once(vals[0])),
        n => {
            let val = vals[n - 1];
            Box::new(ops.iter().flat_map(move |op| {
                let lefts = solutions(&vals[..n - 1], ops);
                lefts.map(move |l| op(l, val))
            }))
        }
    }
}
