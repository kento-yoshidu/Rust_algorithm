// https://atcoder.jp/contests/abc219/tasks/abc219_c

use std::collections::HashMap;
use itertools::Itertools;

fn run(x: &'static str, _n: usize, s: Vec<&'static str>) -> Vec<&'static str> {
    let hash_map: HashMap<char, usize> = x.chars().zip(0..).collect();

    s.iter()
        .sorted_by_key(|s| {
            s.chars()
                .map(|c| hash_map[&c])
                .collect::<Vec<_>>()
        })
        .cloned()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, Vec<&'static str>, Vec<&'static str>);

    #[test]
    fn test() {
        let tests = [
            TestCase("bacdefghijklmnopqrstuvwxzy", 4, vec!["abx", "bzz", "bzy", "caa"], vec!["bzz", "bzy", "abx", "caa"]),
            TestCase("zyxwvutsrqponmlkjihgfedcba", 5, vec!["a", "ab", "abc", "ac", "b"], vec!["b", "a", "ac", "ab", "abc"]),
        ];

        for TestCase(x, n, s, expected) in tests {
            assert_eq!(run(x, n, s), expected);
        }
    }
}
