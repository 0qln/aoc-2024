use itertools::Itertools;
use regex::Regex;

#[cfg(test)]
mod test;

enum Token {
    Mul(isize, isize),
    Do,
    Dont,
}

#[derive(Clone, Copy)]
enum MulState {
    Enabled,
    Disabled,
}

fn parse(input: &str) -> Vec<Token> {
    let re = Regex::new(r"(don't\(\))|(do\(\))|(mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let captures = re.captures_iter(input).collect_vec();
    captures.iter()
        .map(|cap| match &cap[0] {
            "do()" => Token::Do,
            "don't()" => Token::Dont,
            _ => Token::Mul(
                cap[4].parse().unwrap(), 
                cap[5].parse().unwrap()
            ),
        })
        .collect_vec()
}

pub fn solve(input: &str) -> isize {
    let pairs = parse(&input);
    pairs.iter()
        .fold((MulState::Enabled, 0), |(state, sum), token| match token {
            Token::Do => (MulState::Enabled, sum),
            Token::Dont => (MulState::Disabled, sum),
            Token::Mul(a, b) => (
                state,
                match state {
                    MulState::Enabled => sum + a * b,
                    MulState::Disabled => sum,
                },                
            )
        }).1
}