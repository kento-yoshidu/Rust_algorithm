// https://atcoder.jp/contests/abc212/tasks/abc212_b

use itertools::Itertools;

pub fn run(x: &str) -> &'static str {
    let vec: Vec<u32> = x.chars().map(|c| c.to_digit(10).unwrap()).collect();

    if vec.iter().all_equal() {
        return "Weak";
    }

    if (0..3).any(|i| {
        vec[i + 1] != (vec[i] + 1) % 10
    }) {
        "Strong"
    } else {
        "Weak"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc212_b() {
        let tests = [
            TestCase("7777", "Weak"),
            TestCase("0112", "Strong"),
            TestCase("9012", "Weak"),
        ];

        for TestCase(x, expected) in tests {
            assert_eq!(run(x), expected);
        }
    }
}
