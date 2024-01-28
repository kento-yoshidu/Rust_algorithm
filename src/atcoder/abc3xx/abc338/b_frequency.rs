// https://atcoder.jp/contests/abc338/tasks/abc338_b

use std::collections::HashMap;

use itertools::Itertools;

pub fn run(s: &str) -> char {
    let mut hash_map = HashMap::new();

    for c in s.chars() {
        *hash_map.entry(c).or_insert(0) += 1;
    }

    let max = hash_map.values().max().unwrap();

    *hash_map.iter()
        .filter(|(_, v)| *v == max)
        .map(|(c, _)| c)
        .sorted()
        .collect::<Vec<&char>>()
        [0]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, char);

    #[test]
    fn test() {
        let tests = [
            TestCase("frequency", 'e'),
            TestCase("atcoder", 'a'),
            TestCase("pseudopseudohypoparathyroidism", 'o'),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
