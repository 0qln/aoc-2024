use super::solve;
use crate::include_using_path;

#[test]
fn dummy_input() {
    let input = include_using_path!("dummy_input.txt");
    let result = solve(input);
    assert!(result > (100 + 100 * 3) + (100 + 100 * 3));
}

#[test]
fn solve_puzzle() {
    let input = include_using_path!("input.txt");
    let result = solve(input);
    println!("Result: {}", result);
}