// https://atcoder.jp/contests/abc225/tasks/abc225_a

use itertools::Itertools;

fn run(s: &str) -> usize {
    s.chars().permutations(3).unique().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc225_a() {
        let tests = [
            TestCase("aba", 3),
            TestCase("ccc", 1),
            TestCase("xyz", 6),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
