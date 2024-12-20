use itertools::Itertools;

use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> String {
    let (device, program) = parse(input);
    println!("{:?}", device);
    println!("{:?}", program);
    let result = device.execute(&program);
    result.iter().map(|x| x.to_string()).join(",")
}