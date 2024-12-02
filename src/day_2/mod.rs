use regex::Regex;

pub mod part_1;
pub mod part_2;

pub fn parse(input: &str) -> Vec<Vec<isize>> {
    let re = Regex::new(r"(\d+)").unwrap();
    input
        .lines()
        .map(|line| re
            .captures_iter(line)
            .map(|cap| cap[1].parse().unwrap())
            .collect())
        .collect()
}
