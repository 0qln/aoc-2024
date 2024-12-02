use std::default;

use crate::day_2::part_1::{is_save, Direction};

use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> usize {
    let input = parse(input);
    input
        .iter()
        .filter_map(|report| {
            report
                .windows(2).map(|pair| pair[0] - pair[1])
                .try_fold(Direction::None, is_save)
                .or_else(|| {
                    (0..report.len()).map(|i| {
                        report.into_iter().enumerate()
                            .filter(move |&(idx, _)| idx != i).map(|(_, r)| r)
                    })
                    .any(|report_variant| {
                        report_variant.collect::<Vec<_>>()
                            .windows(2).map(|pair| pair[0] - pair[1])
                            .try_fold(Direction::None, is_save)
                            .is_some()
                    }).then_some(Direction::None)        
                })
        })
        .count()
}
