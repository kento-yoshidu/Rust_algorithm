// https://atcoder.jp/contests/abc012/tasks/abc012_3

use std::fmt::format;

fn run(n: usize) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    for i in 1..=9 {
        for j in 1..=9 {
            if i*j == 2025 - n {
                vec.push(format!("{} * {}", i, j));
            }
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase(2013, vec!["2 * 6", "3 * 4", "4 * 3", "6 * 2"]),
            TestCase(2024, vec!["1 * 1"]),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
