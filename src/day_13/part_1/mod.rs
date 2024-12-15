use super::parse;

#[cfg(test)]
mod test;

pub fn solve(input: &str) -> isize {
    let machines = parse(input);
    machines
        .iter()
        .map(|(a, b, prize)| {
            let (a_1, a_2) = a;
            let (b_1, b_2) = b;
            let (p_1, p_2) = prize;
            let x_a = (p_2 * b_1 - p_1 * b_2) / (a_2 * b_1 - a_1 * b_2);
            let x_b = (p_1 * a_2 - p_2 * a_1) / (a_2 * b_1 - a_1 * b_2);
            let r_1 = a_1 * x_a + b_1 * x_b;
            let r_2 = a_2 * x_a + b_2 * x_b;
            let playable = x_a <= 100 && x_b <= 100;
            let winnable = r_1 == *p_1 && r_2 == *p_2;
            let cost = 3 * x_a + 1 * x_b;
            if playable && winnable { cost } else { 0 }
        })
        .sum()
}
