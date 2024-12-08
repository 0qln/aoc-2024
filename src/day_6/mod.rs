use core::f64;

use itertools::Itertools;

pub mod part_1;
pub mod part_2;

pub type Pos = (usize, usize);
pub type Dir = (isize, isize);

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

#[derive(Clone, Default)]
pub struct Map {
    v: Vec<Vec<char>>,
}

impl Map {
    pub fn v(self) -> Vec<Vec<char>> { self.v }

    pub fn find(&self, pred: impl Fn(&char) -> bool) -> Option<Pos> {
        self.v.iter().enumerate().find_map(|(top, line)| {
            line.iter()
                .enumerate()
                .find_map(|(left, &ch)| match pred(&ch) {
                    true => Some((left, top)),
                    false => None,
                })
        })
    }

    pub fn find_all<'a, F: Fn(&char) -> bool + Copy>(&'a self, pred: F) -> impl Iterator<Item = Pos> + use<'a, F> {
        self.v.iter().enumerate().flat_map(move |(top, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(left, &ch)| if pred(&ch) { Some((left, top)) } else { None })
        })
    }

    pub fn fold<T>(&self, init: T, mut f: impl FnMut(T, &char) -> T) -> T {
        self.v.iter().fold(init, |acc, line| {
            line.iter().fold(acc, |acc, &ch| {
                f(acc, &ch)
            })
        })
    }
    
    pub fn map<'a, T, F: Fn(&char) -> T + Copy>(&'a self, f: F) -> impl Iterator<Item = T> + use<'a, F, T> {
        self.v.iter().flat_map(move |line| line.iter().map(f))
    }

    pub fn count(&self, pred: impl Fn(&char) -> bool) -> usize {
        self.v.iter().flatten().filter(|x| pred(x)).count()
    }

    pub fn get(&self, (left, top): Pos) -> Option<char> {
        self.v.get(top)?.get(left).copied()
    }

    pub fn set(&mut self, (left, top): Pos, c: char) {
        self.v[top][left] = c;
    }
    
    pub fn set_checked(&mut self, (left, top): Pos, c: char) -> Option<()> { 
        self.v.get_mut(top)?.get_mut(left).map(|v| *v = c)
    }
    
    pub fn clone_empty(&self) -> Map {
        Map { v: vec![vec!['.'; self.v[0].len()]; self.v.len()] }
    }
}

pub fn parse(input: &str) -> Map {
    Map {
        v: input
            .lines()
            .map(|line: &str| line.chars().collect_vec())
            .collect_vec(),
    }
}
