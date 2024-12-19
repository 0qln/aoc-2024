use super::solve;
use crate::include_using_path;

#[test]
fn dummy_input_0() {
    let input = include_using_path!("dummy_input_0.txt");
    let result = solve(input);
    assert_eq!(result, 2028);
}

#[test]
fn dummy_input_1() {
    let input = include_using_path!("dummy_input_1.txt");
    let result = solve(input);
    assert_eq!(result, 10092);
}

#[test]
fn solve_puzzle() {
    let input = include_using_path!("input.txt");
    let result = solve(input);
    println!("Result: {}", result);
}