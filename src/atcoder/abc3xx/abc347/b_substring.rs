// https://atcoder.jp/contests/abc347/tasks/abc347_b

use std::collections::HashSet;

pub fn run(s: &str) -> usize {
    let mut hash_set = HashSet::new();

    for i in 0..s.len() {
        for j in i..s.len() {
            hash_set.insert(&s[i..=j]);
        }
    }

    hash_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("yay", 5),
            TestCase("aababc", 17),
            TestCase("abracadabra", 54),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
