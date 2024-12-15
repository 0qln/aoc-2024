use std::{
    collections::{hash_set::Entry, HashSet, LinkedList},
    default,
    iter::{self, Sum},
    ops::{Add, Deref},
};

use ilog::IntLog;
use itertools::Itertools;

use crate::{
    day_6::{get_dir, Dir, Map, Pos},
    day_8::setoff,
};

use super::parse;

#[cfg(test)]
mod test;

#[derive(Default, PartialEq, Eq, Debug)]
struct PriceInfo {
    area: usize,
    sides: usize,
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
            sides: self.sides + rhs.sides,
        }
    }
}

type PlotInfo = (Pos, char);

fn price_info(
    map: &Map,
    plot: PlotInfo,
    visitied: &mut HashSet<Pos>,
    sides_cache: &mut HashSet<(Pos, isize)>,
) -> PriceInfo {
    match visitied.entry(plot.0) {
        Entry::Occupied(_) => PriceInfo::default(),
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert();
            // next
            (0..4)
                .map(|dir| {
                    let next_plot = (setoff(plot.0, get_dir(dir)), plot.1);
                    if map.get(next_plot.0) == Some(next_plot.1) {
                        price_info(map, next_plot, visitied, sides_cache)
                    } else {
                        PriceInfo {
                            area: 0,
                            sides: sides_cache
                                .insert((side_entry(map, plot, dir), dir))
                                .then_some(1)
                                .unwrap_or(0),
                        }
                    }
                })
                // curr
                .chain(iter::once(PriceInfo { area: 1, sides: 0 }))
                .sum()
        }
    }
}

fn side_entry(map: &Map, mut plot: PlotInfo, dir: isize) -> Pos {
    let side_dir = get_dir(dir + 1);
    let dir = get_dir(dir);
    let chr = Some(plot.1);
    loop {
        let next_plot_0 = setoff(plot.0, side_dir);
        if map.get(next_plot_0) != chr || map.get(setoff(next_plot_0, dir)) == chr {
            return plot.0;                
        } 
        plot.0 = next_plot_0;
    }
}

pub fn solve(input: &str) -> usize {
    let map = parse(input);
    let mut visited = HashSet::new();
    map.enumerate()
        .map(|plot| price_info(&map, plot, &mut visited, &mut HashSet::new()))
        .filter(|i| *i != Default::default())
        .inspect(|info| println!("{:?}", info))
        .map(|info| info.area * info.sides)
        .sum()
}
