use std::iter;

use itertools::Itertools;

use crate::day_6::{Dir, Map, Pos};

use super::{delta, parse, setoff};

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
                let d1 = delta(antenna, other_antenna);
                let anti_node1 = setoff(antenna, d1);
                anti_nodes.set_checked(anti_node1, '#');
                
                let d2 = delta(other_antenna, antenna);
                let anti_node2 = setoff(other_antenna, d2);
                anti_nodes.set_checked(anti_node2, '#');
            }
        }
    }
    anti_nodes.count(|&ch| matches!(ch, '#'))
}