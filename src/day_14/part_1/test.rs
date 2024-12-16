use super::solve;
use crate::include_using_path;

#[test]
fn dummy_input() {
    let input = include_using_path!("dummy_input.txt");
    let result = solve(input, 7, 11);
    assert_eq!(result, 12);
}

#[test]
fn solve_puzzle() {
    let input = include_using_path!("input.txt");
    let result = solve(input, 101, 103);
    println!("Result: {}", result);
}