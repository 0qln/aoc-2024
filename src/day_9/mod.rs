use itertools::Itertools;

pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> Vec<Option<usize>> {
    input.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let is_file = i % 2 == 0;
            let n = c.to_digit(10).unwrap();
            let ds = is_file.then_some(i / 2);
            (0..n).map(move |_| ds)
        })
        .collect_vec()
}
