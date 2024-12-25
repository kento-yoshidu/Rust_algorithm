// https://atcoder.jp/contests/abc310/tasks/abc310_c

use std::collections::HashSet;

fn run(_n: usize, s: Vec<&str>) -> usize {
    let mut hash_set = HashSet::new();

    for str in s {
        let rev: String = str.chars().rev().collect();

        if !hash_set.contains(&rev) && !hash_set.contains(&str.to_string()) {
            hash_set.insert(rev);
        }
    }

    hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6,vec!["a", "abc", "de", "cba", "de", "abc"], 3),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
