// https://atcoder.jp/contests/abc164/tasks/abc164_c

use std::collections::HashSet;

fn run(_n: usize, s: Vec<&str>) -> usize {
    s.into_iter().collect::<HashSet<_>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(3, vec!["apple", "orange", "apple"], 2),
            TestCase(5, vec!["grape", "grape", "grape", "grape", "grape"], 1),
            TestCase(4, vec!["aaaa", "a", "aaa", "aa"], 4),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
