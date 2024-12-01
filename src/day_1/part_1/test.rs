use super::*;

#[test]
fn dummy_input() {
    let input = include_str!("dummy_input.txt");
    let result = solve(input);
    assert_eq!(result, 11);
}

#[test]
fn solve_puzzle() {
    let input = include_str!("input.txt");
    let result = solve(input);
    println!("Result: {}", result);
}
