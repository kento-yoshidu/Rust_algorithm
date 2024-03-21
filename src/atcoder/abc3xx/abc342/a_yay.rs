// https://atcoder.jp/contests/abc342/tasks/abc342_a

use std::collections::HashMap;

pub fn run(s: &str) -> usize {
    let mut hash_map = HashMap::new();

    for c in s.chars() {
        *hash_map.entry(c).or_insert(0) += 1;
    }

    let mut char = 'x';

    for (k, v) in hash_map {
        if v == 1 {
            char = k;
        }
    }

    s.chars()
        .position(|c| {
            c == char
        })
        .unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("yay", 2),
            TestCase("egg", 1),
            TestCase("zzzzzwz", 6),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
