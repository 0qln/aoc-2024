use itertools::Itertools;

use super::{parse, Guard, Map};

#[cfg(test)]
mod test;

pub(super) fn visit_map(mut map: Map, mut guard: Guard) -> Map {
    loop {
        match guard.peek_forward(&map) {
            Some('#') => {
                guard.rotate90();
            }
            Some('.' | _) => {
                map.set(guard.pos, 'X');
                guard.pos = guard.forward().unwrap()
            }
            None => {
                map.set(guard.pos, 'X');
                break;
            }
        }
    }
    map
}

pub fn solve(input: &str) -> usize {
    let map = parse(input);
    let guard = Guard {
        pos: map.find(|&ch| matches!(ch, '^')).unwrap(),
        dir: 3,
    };
    visit_map(map, guard).count(|&ch| ch == 'X')
}

