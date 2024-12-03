use itertools::Itertools;
use regex::Regex;

#[cfg(test)]
mod test;

pub fn parse(input: &str) -> Vec<(isize, isize)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let captures = re.captures_iter(input).collect_vec();
    captures.iter()
        .map(|cap| (cap[1].parse().unwrap(), cap[2].parse().unwrap()))
        .collect_vec()
}

pub fn solve(input: &str) -> isize {
    let pairs = parse(&input);
    pairs.iter().map(|(a, b)| a * b).sum()        
}