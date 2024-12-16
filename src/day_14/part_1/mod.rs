use crate::{day_6::{Dir, Map, Pos}, day_8::setoff};

use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str, width: isize, height: isize) -> usize {
    fn apply_bounds(coord: isize, bound: isize) -> isize {
        (coord % bound + bound) % bound
    }

    let horz_mid = width / 2;
    let vert_mid = height / 2;
    let robots = parse(input);
    
    fn robots_to_string(robots: &Vec<(Pos, Dir)>, width: usize, height: usize) -> String {
        let mut map = Map::new(vec![vec!['.'; height]; width]);
        for (pos, _) in robots {
            map.set(*pos, '#');
        }
        map.to_string()
    }
    println!("{}\n", robots_to_string(&robots, width as usize, height as usize));

    robots
        .iter()
        .map(|(pos, dir)| {
            let total_dir = (dir.0 * 100, dir.1 * 100);
            let left = pos.0 as isize + total_dir.0;
            let top = pos.1 as isize + total_dir.1;
            (apply_bounds(left, width), apply_bounds(top, height))
        })
        .fold([0; 4], |mut quadrants, pos| {
            if pos.0 != horz_mid && pos.1 != vert_mid {
                let left = if pos.0 < horz_mid { 1 } else { 0 };
                let top = if pos.1 < vert_mid { 2 } else { 0 };
                quadrants[left | top] += 1
            }
            quadrants
        })
        .iter()
        .fold(1, |a, b| a * b)
}
