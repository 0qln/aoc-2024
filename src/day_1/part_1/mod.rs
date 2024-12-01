use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> u64 {
    let input = parse(input);
    let (mut lefts, mut rights): (Vec<i64>, Vec<i64>) = input.unzip();
    lefts.sort_unstable();
    rights.sort_unstable();
    lefts.into_iter().zip(rights.into_iter())
        .map(|(l, r)| { l.abs_diff(r) })
        .sum()
}
