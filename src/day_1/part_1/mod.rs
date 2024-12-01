use regex::Regex;

#[cfg(test)]
mod test;

pub fn parse(input: &str) -> impl Iterator<Item = (i64, i64)> + '_ {
    input.lines().filter_map(|line| {
        let re = Regex::new(r"(\d+)\s+(\d+)").ok()?;
        let nums = re.captures(line)?;
        Some((nums[1].parse().unwrap(), nums[2].parse().unwrap()))
    })
}

#[no_mangle]
pub fn solve(input: &str) -> u64 {
    let input = parse(input);
    let (mut lefts, mut rights): (Vec<i64>, Vec<i64>) = input.unzip();
    lefts.sort_unstable();
    rights.sort_unstable();
    lefts.into_iter().zip(rights.into_iter())
        .map(|(l, r)| { l.abs_diff(r) })
        .sum()
}
