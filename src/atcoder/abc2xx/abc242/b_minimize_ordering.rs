// https://atcoder.jp/contests/abc242/tasks/abc242_b

use itertools::Itertools;

fn run(s: &str) -> String {
    s.chars().sorted().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc242_b() {
        let tests = [
            TestCase("aba", "aab"),
            TestCase("zzzz", "zzzz"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
