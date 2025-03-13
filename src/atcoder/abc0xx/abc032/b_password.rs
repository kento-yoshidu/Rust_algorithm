// https://atcoder.jp/contests/abc032/tasks/abc032_a

use std::collections::HashSet;

fn run(s: &str, k: usize) -> usize {
    let mut hash_set = HashSet::new();

    let chars: Vec<char> = s.chars().collect();

    for arr in chars.windows(k) {
        hash_set.insert(arr.iter().collect::<String>());
    }

    hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("abcabc", 2, 3),
            TestCase("aaaaa", 1, 1),
            TestCase("hello", 10, 0),
        ];

        for TestCase(s, k, expected) in tests {
            assert_eq!(run(s, k), expected);
        }
    }
}
