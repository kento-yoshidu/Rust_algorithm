// https://atcoder.jp/contests/joi2022yo1a/tasks/joi2022_yo1a_c

use std::collections::HashSet;
use itertools::Itertools;

fn run(_n: usize, s: &str) -> &'static str {
    let len = s.chars()
        .sorted()
        .dedup()
        .collect::<Vec<char>>()
        .len();

    if len >= 3 {
        "Yes"
    } else {
        "No"
    }
}

fn run2(_n: usize, s: &str) -> &'static str {
    let mut hash_set = HashSet::new();

    for c in s.chars() {
        hash_set.insert(c);
    }

    if hash_set.len() >= 3 {
        "Yes"
    } else {
        "No"
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn test() {
        let tests = [
            TestCase(4, "BABE", "Yes"),
            TestCase(3, "DAD", "No"),
            TestCase(5, "BACED", "Yes"),
            TestCase(28, "EEEEEEEEEEEEEEEEEEEEEEEEEEEE", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
