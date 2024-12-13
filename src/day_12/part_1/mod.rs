use std::{
    collections::{hash_set::Entry, HashSet, LinkedList},
    default,
    iter::{self, Sum},
    ops::{Add, Deref},
};

use ilog::IntLog;
use itertools::Itertools;

use crate::{
    day_6::{get_dir, Map, Pos},
    day_8::setoff,
};

use super::parse;

#[cfg(test)]
mod test;

#[derive(Default, PartialEq, Eq, Debug)]
struct PriceInfo {
    area: usize,
    perimiter: usize,
}

impl Sum for PriceInfo {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(PriceInfo::default(), |acc, x| acc + x)
    }
}

impl Add for PriceInfo {
    type Output = PriceInfo;

    fn add(self, rhs: Self) -> Self::Output {
        PriceInfo {
            area: self.area + rhs.area,
            perimiter: self.perimiter + rhs.perimiter,
        }
    }
}

type PlotInfo = (Pos, char);

fn price_info(map: &Map, plot: PlotInfo, visitied: &mut HashSet<Pos>) -> PriceInfo {
    match visitied.entry(plot.0) {
        Entry::Occupied(_) => PriceInfo::default(),
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert();
            let mut perimiter = 0;
            let next: PriceInfo = (0..4)
                .map(|dir| {
                    let next_plot = (setoff(plot.0, get_dir(dir)), plot.1);
                    match map.get(next_plot.0) {
                        Some(x) if x == next_plot.1 => price_info(map, next_plot, visitied),
                        _ => {
                            perimiter += 1;
                            PriceInfo::default()
                        }
                    }
                })
                .sum();
            next + PriceInfo {
                area: 1,
                perimiter: perimiter,
            }
        }
    }
}

pub fn solve(input: &str) -> usize {
    let map = parse(input);
    let mut visited = HashSet::new();
    map.enumerate()
        .map(|plot| price_info(&map, plot, &mut visited))
        .inspect(|info| if *info != Default::default() { println!("{:?}", info) })
        .map(|info| info.area * info.perimiter)
        .sum()
}
