use std::iter;

use itertools::Itertools;

use crate::day_6::{Dir, Map, Pos};

use super::{delta, mul, parse, setoff};

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> usize {
    let f_maps = parse(input);
    let mut anti_nodes = f_maps[0].clone_empty();
    for f_map in f_maps.iter() {
        for antenna in f_map.find_all(|&ch| ch == '@') {
            for other_antenna in f_map.find_all(|&ch| ch == '@') {
                if antenna == other_antenna {
                    continue;
                }
                for d in [
                    delta(antenna, other_antenna), 
                    delta(other_antenna, antenna)
                ].iter() {
                    let mut dist = 0;
                    while {
                        let d = mul(*d, dist);
                        dist += 1;
                        let anti_node = setoff(antenna, d);
                        anti_nodes.set_checked(anti_node, '#').is_some()
                    } {}
                }
            }
        }
    }
    anti_nodes.count(|&ch| matches!(ch, '#'))
}
