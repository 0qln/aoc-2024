use super::solve;
use crate::include_using_path;

#[test]
fn test_input() {
    let input = include_using_path!("test_input.txt");
    let _ = solve(input);
}

#[test]
fn dummy_input() {
    let input = include_using_path!("dummy_input.txt");
    let result = solve(input);
    assert_eq!(result, 9021);
}

#[test]
fn solve_puzzle() {
    let input = include_using_path!("input.txt");
    let result = solve(input);
    println!("Result: {}", result);
}