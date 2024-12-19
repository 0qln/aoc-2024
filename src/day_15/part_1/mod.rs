use itertools::Itertools;

use crate::{
    day_6::{self, get_dir, Dir, Map, Pos},
    day_8::setoff,
};

use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> usize {
    let (mut map, moves) = parse(input);
    let mut curr = map.find(|&ch| ch == '@').unwrap();
    for m in moves {
        let m = get_dir(m);
        if move_stuff(&mut map, curr, m) {
            curr = setoff(curr, m);
        }
    }

    map.find_all(|&ch| ch == 'O')
        .map(|(l, t)| t * 100 + l)
        .sum()
}

fn move_stuff(map: &mut Map, curr: Pos, m: Dir) -> bool {
    match map.get(curr) {
        Some('@' | 'O') => {
            let next = setoff(curr, m);
            let can_move = move_stuff(map, next, m);
            if can_move {
                let curr_chr = map.get(curr).unwrap();
                map.set(curr, '.');
                map.set(next, curr_chr);
            }
            can_move
        }
        Some('#') | None => false,
        Some('.') => true,
        c => panic!("Invalid char: {:?}", c),
    }
}
