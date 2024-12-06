use core::f64;

use itertools::Itertools;

pub mod part_1;
pub mod part_2;

type Pos = (usize, usize);
type Dir = (isize, isize);

#[derive(Clone)]
struct Guard {
    pos: Pos,
    dir: isize,
}

impl Guard {
    fn rotate90(&mut self) {
        self.dir += 1;
    }

    fn get_dir(&self) -> Dir {
        let x = self.dir as f64 * f64::consts::PI / 2f64;
        (f64::cos(x).round() as isize, f64::sin(x).round() as isize)
    }

    fn forward(&self) -> Option<Pos> {
        let (dl, dt) = self.get_dir();
        let left = self.pos.0.checked_add_signed(dl)?;
        let top = self.pos.1.checked_add_signed(dt)?;
        Some((left, top))
    }

    fn peek_forward(&self, map: &Map) -> Option<char> {
        let pos = self.forward()?;
        map.get(pos)
    }
}

#[derive(Clone)]
struct Map {
    v: Vec<Vec<char>>,
}

impl Map {
    fn find(&self, pred: impl Fn(&char) -> bool) -> Option<Pos> {
        self.v.iter().enumerate().find_map(|(top, line)| {
            line.iter()
                .enumerate()
                .find_map(|(left, &ch)| match pred(&ch) {
                    true => Some((left, top)),
                    false => None,
                })
        })
    }

    fn count(&self, pred: impl Fn(&char) -> bool) -> usize {
        self.v.iter().flatten().filter(|x| pred(x)).count()
    }

    fn get(&self, (left, top): Pos) -> Option<char> {
        self.v.get(top)?.get(left).copied()
    }

    fn set(&mut self, (left, top): Pos, c: char) {
        self.v[top][left] = c;
    }
}

fn parse(input: &str) -> Map {
    Map {
        v: input
            .lines()
            .map(|line: &str| line.chars().collect_vec())
            .collect_vec(),
    }
}
