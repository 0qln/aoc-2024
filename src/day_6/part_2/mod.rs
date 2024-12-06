use std::collections::{hash_set::Entry, HashSet};

use itertools::Itertools;

use super::{parse, part_1::visit_map, Dir, Guard, Map, Pos};

#[cfg(test)]
mod test;

fn is_cyclic(map: &Map, mut guard: Guard) -> bool {
    let mut pasts = HashSet::<(Pos, Dir)>::new();
    loop {
        match guard.peek_forward(&map) {
            Some('#' | 'O') => guard.rotate90(),
            Some('.' | 'X') => {
                match pasts.entry((guard.pos, guard.get_dir())) {
                    Entry::Occupied(_) => return true,
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert();
                        guard.pos = guard.forward().unwrap()
                    },
                }
            }
            _ => return false
        }
    } 
}

pub fn solve(input: &str) -> usize {
    let map = parse(input);
    let guard = Guard { pos: map.find(|&ch| matches!(ch, '^')).unwrap(), dir: 3 };
    let mut obstacle_positions = 0;
    let mut visited_map = visit_map(map, guard.clone());
    while let Some(obstacle) = visited_map.find(|&ch| ch == 'X') {
        visited_map.set(obstacle, 'O');
        if is_cyclic(&visited_map, guard.clone()) {
            obstacle_positions += 1
        }
        visited_map.set(obstacle, '.');
    }
    return obstacle_positions;
}
