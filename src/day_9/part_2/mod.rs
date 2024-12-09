use std::iter;

use itertools::Itertools;

use super::parse;

#[cfg(test)]
mod test;

struct EmptyBlock {
    idx: usize,
    len: usize,
}

struct FileBlock {
    idx: usize,
    len: usize,
    id: usize,
}

pub fn solve(input: &str) -> usize {
    let data = parse(input);
    let data_idx = data.iter().cloned().enumerate().collect_vec();

    let mut empty_blocks = data_idx
        .chunk_by(|a, b| a.1 == b.1)
        .filter_map(|x| x[0].1.is_none().then_some(EmptyBlock { idx: x[0].0, len: x.len() }))
        .flat_map(|x| (1..=x.len).rev().zip(0..).map(move |(l, i_off)| EmptyBlock { len: l, idx: x.idx + i_off }))
        .collect_vec();

    let file_blocks = data_idx
        .chunk_by(|a, b| a.1 == b.1)
        .filter_map(|x| Some(FileBlock { idx: x[0].0, len: x.len(), id: x[0].1? }))
        .rev();

    file_blocks.flat_map(&mut |file: FileBlock| {
            let new_pos = empty_blocks
                .iter().enumerate()
                .find(|(_, empty)| empty.len >= file.len && empty.idx < file.idx);

            if let Some((empty_i, _)) = new_pos {
                empty_blocks
                    .splice(empty_i..(empty_i + file.len), iter::empty())
                    .map(|used_block| { FileBlock { idx: used_block.idx, ..file } })
                    .collect_vec()
            }
            else {
                (file.idx..(file.idx + file.len))
                    .map(|idx| FileBlock { idx, ..file })
                    .collect_vec()
            }
        })
        .map(|file| file.idx * file.id)
        .sum()
}
