use regex::Regex;

pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> impl Iterator<Item = (i64, i64)> + '_ {
    input.lines().filter_map(|line| {
        let re = Regex::new(r"(\d+)\s+(\d+)").ok()?;
        let nums = re.captures(line)?;
        Some((nums[1].parse().unwrap(), nums[2].parse().unwrap()))
    })
}
