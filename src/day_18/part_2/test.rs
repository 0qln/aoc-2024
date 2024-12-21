use super::solve;
use crate::include_using_path;

#[test]
fn dummy_input() {
    let input = include_using_path!("dummy_input.txt");
    let result = solve(input, 7);
    assert_eq!(result, Some((6, 1)));
}

#[test]
fn solve_puzzle() {
    let input = include_using_path!("input.txt");
    let result = solve(input, 71);
    println!("Result: {:?}", result);
}