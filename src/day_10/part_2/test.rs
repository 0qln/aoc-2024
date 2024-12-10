use super::solve;
use crate::include_using_path;

#[test]
fn dummy_input_0() {
    let input = include_using_path!("dummy_input_0.txt");
    let result = solve(input);
    assert_eq!(result, 3);
}

#[test]
fn dummy_input_1() {
    let input = include_using_path!("dummy_input_1.txt");
    let result = solve(input);
    assert_eq!(result, 13);
}

#[test]
fn dummy_input_2() {
    let input = include_using_path!("dummy_input_2.txt");
    let result = solve(input);
    assert_eq!(result, 227);
}  

#[test]
fn dummy_input_3() {
    let input = include_using_path!("dummy_input_3.txt");
    let result = solve(input);
    assert_eq!(result, 81);
}

#[test]
fn solve_puzzle() {
    let input = include_using_path!("input.txt");
    let result = solve(input);
    println!("Result: {}", result);
}