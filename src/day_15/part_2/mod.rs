use std::{collections::HashSet, iter};

use itertools::{enumerate, Itertools};

use crate::{
    day_6::{get_dir, Dir, Map, Pos},
    day_8::setoff,
};

use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> usize {
    let (mut map, moves) = parse(input);
    map.map_chars_h(|chr| match chr {
        '#' => vec!['#', '#'],
        'O' => vec!['[', ']'],
        '.' => vec!['.', '.'],
        '@' => vec!['@', '.'],
        _ => panic!("Invalid char."),
    });
    let mut curr = map.find(|&ch| ch == '@').unwrap();
    for &m in moves.iter() {
        let is_horizontal = m % 2 == 0;
        if is_horizontal {
            let m = get_dir(m);
            if move_horz(&mut map, curr, m) {
                curr = setoff(curr, m);
            }
        } else {
            let mut moves = Vec::new();
            if move_vert(&mut map, curr, get_dir(m), &mut moves) {
                for next in moves {
                    let prev = setoff(next, get_dir(m + 2));
                    let prev_chr = map.get(prev).unwrap();
                    map.set(prev, '.');
                    map.set(next, prev_chr);
                }
                curr = setoff(curr, get_dir(m));
            }
        }
    }

    map.find_all(|&ch| ch == '[')
        .map(|(l, t)| t * 100 + l)
        .sum()
}

fn move_horz(map: &mut Map, curr: Pos, m: Dir) -> bool {
    match map.get(curr) {
        Some('@' | '[' | ']') => {
            let next = setoff(curr, m);
            let can_move = move_horz(map, next, m);
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

fn move_vert(map: &mut Map, curr: Pos, m: Dir, moves: &mut Vec<Pos>) -> bool {
    if moves.contains(&curr) {
        return true;
    }
    let next = setoff(curr, m);
    let can_move = match map.get(curr) {
        Some('#') | None => return false,
        Some('@') => return move_vert(map, next, m, moves),
        Some('[') => move_vert(map, next, m, moves) && move_vert(map, setoff(next, get_dir(4)), m, moves),
        Some(']') => move_vert(map, next, m, moves) && move_vert(map, setoff(next, get_dir(2)), m, moves),
        Some('.') => true,
        c => panic!("Invalid char: {:?}", c),
    };
    if can_move {
        moves.push(curr);
    }
    can_move
}
