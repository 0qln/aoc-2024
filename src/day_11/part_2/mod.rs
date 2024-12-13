use std::{
    array,
    collections::{HashMap, LinkedList},
    iter,
    ops::Deref,
};

use ilog::IntLog;
use itertools::Itertools;
use rustc_hash::FxHashMap;

use super::{
    parse,
    part_1::{self, split},
};

#[cfg(test)]
mod test;

pub fn apply_rules<const TIMES: usize>(stone: u64) -> u64 {
    let mut cache: [FxHashMap<u64, u64>; TIMES] = array::from_fn(|_| FxHashMap::default());
    
    return apply_rules_rec(stone, TIMES, &mut cache);

    fn apply_rules_rec(stone: u64, times: usize, cache: &mut [FxHashMap<u64, u64>]) -> u64 {
        if times == 0 {
            return 1;
        }
        
        let times = times - 1;

        if cache[times].contains_key(&stone) {
            return *cache[times].get(&stone).unwrap();
        }

        let value = {
            if stone == 0 {
                apply_rules_rec(1, times, cache)
            } else if stone.log10() % 2 == 1 {
                let (left, right) = split(stone);
                let stones_left = apply_rules_rec(left, times, cache);
                let stones_right = apply_rules_rec(right, times, cache);
                stones_left + stones_right
            } else {
                apply_rules_rec(stone * 2024, times, cache)
            }
        };

        cache[times].insert(stone, value);
        value
    }
}

pub fn solve(input: &str) -> u64 {
    let stones = parse(input);
    stones.iter().map(|stone| apply_rules::<75>(*stone)).sum()
}
