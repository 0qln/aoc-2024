use std::{collections::LinkedList, iter, ops::Deref};

use ilog::IntLog;
use itertools::Itertools;

use super::parse;

#[cfg(test)]
mod test;
    
pub fn split(stone: u64) -> (u64, u64) {
    let digits = 1 + stone.log10() as u32;
    let shift = 10u64.pow(digits / 2);
    let left = stone / shift;
    let right = stone - left * shift;
    (left, right)
}
    
pub fn apply_rules(stones: &mut LinkedList<u64>) {
    let mut cursor = stones.cursor_front_mut();
    while let Some(stone) = cursor.current() {
        if *stone == 0 {
            *stone = 1;
        }
        else if (1 + stone.log10()) % 2 == 0 {
                let (left, right) = split(*stone);
                *stone = left;
                cursor.insert_after(right);
                cursor.move_next();
                assert_eq!(right, *cursor.current().unwrap());
        }
        else {
            *stone *= 2024
        }
        cursor.move_next()
    }
}

pub fn solve(input: &str) -> usize {
    let mut stones = parse(input);
    for _ in 0..25 {
        apply_rules(&mut stones);
    }
    stones.len()
}
