use std::ops::ControlFlow;

use super::parse;

#[cfg(test)]
mod test;

enum Direction {
    None,
    Desc,
    Asc,
}

pub fn solve(input: &str) -> usize {
    let input = parse(input);
    input
        .iter()
        .filter_map(|report| {
            // report.windows(2).map(|pair| pair[0] - pair[1]).try_reduce(
            //     |prev_diff, level_diff| {
            //         if (prev_diff < 0) != (level_diff < 0) {
            //             return None;
            //         }
            //         match level_diff.abs() {
            //             1..=3 => Some(level_diff),
            //             _ => None,
            //         }
            //     },
            // )
            report
                .windows(2)
                .map(|pair| pair[0] - pair[1])
                .try_fold(Direction::None, |dir, level_diff| {
                    match dir {
                        Direction::None => {
                                match level_diff {
                                1..=3 => Some(Direction::Asc),
                                -3..=-1 => Some(Direction::Desc),
                                _ => None,
                            }
                        },
                        Direction::Asc =>  {
                            match level_diff {
                                1..=3 => Some(Direction::Asc),
                                _ => None,
                                
                            }
                        },
                        Direction::Desc => {
                            match level_diff {
                                -3..=-1 => Some(Direction::Desc),
                                _ => None,
                            }
                        },
                    }
                })
        })
        .count()
}
