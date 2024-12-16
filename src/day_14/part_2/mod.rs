use std::io::stdin;

use itertools::Itertools;

use crate::day_6::{Dir, Map, Pos};

use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> isize {
    let width = 101;
    let height = 103;

    fn apply_bounds(coord: isize, bound: isize) -> usize {
        ((coord % bound + bound) % bound) as usize
    }

    fn robots_to_string(robots: &Vec<(Pos, Dir)>) -> String {
        let mut map = Map::new(vec![vec!['.'; 101]; 101]);
        for (pos, _) in robots {
            map.set_checked(*pos, '#');
        }
        map.to_string()
    }

    fn could_be_xmas_tree(robots: &Vec<(Pos, Dir)>, _width: usize, _height: usize) -> bool {
        // theres probably some kind of straight line in the picture
        let line_len = 8;
        return robots.iter().map(|(pos, _)| pos).any(|t| {
            robots
                .iter()
                .filter(|(pos, _)| pos.1 == t.1 && pos.0.abs_diff(t.0) <= line_len / 2)
                .count()
                >= line_len
        });
    }

    let mut robots = parse(input);

    for i in 0.. {
        println!("Seconds elapsed: {}", i);

        if could_be_xmas_tree(&robots, width as usize, height as usize) {
            println!(
                "Does this look like a Christmas Tree? [y/n] \n{}\n",
                robots_to_string(&robots)
            );
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read line");
            match input.as_str() {
                "y\r\n" => {
                    println!("Christmas Tree!");
                    return i;
                }
                "n\r\n" | _ => println!("Not Christmas Tree :("),
            };
        }

        for (pos, dir) in robots.iter_mut() {
            let left = pos.0 as isize + dir.0;
            let top = pos.1 as isize + dir.1;
            *pos = (apply_bounds(left, width), apply_bounds(top, height))
        }
    }
    return -1;
}
