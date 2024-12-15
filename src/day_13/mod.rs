use std::collections::LinkedList;

use itertools::Itertools;
use regex::Regex;

use crate::day_6::{self, Map};

pub mod part_1;
pub mod part_2;

type P = (isize, isize);

pub fn parse(input: &str) -> Vec<(P, P, P)> {
    input
        .split("\r\n\r\n")
        .map(|mac| {
            let p_re = Regex::new(r"(\d+), Y.(\d+)").unwrap();
            p_re.captures_iter(mac)
                .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}
