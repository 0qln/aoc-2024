use super::solve;
use crate::include_using_path;

#[test]
fn dummy_input() {
    let input = include_using_path!("dummy_input.txt");
    let result = solve(input, 7, 12);
    assert_eq!(result, 22);
}

#[test]
fn solve_puzzle() {
    let input = include_using_path!("input.txt");
    let result = solve(input, 71, 1024);
    println!("Result: {}", result);
}