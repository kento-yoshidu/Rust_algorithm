// https://yukicoder.me/problems/no/2778

use std::collections::HashMap;
use itertools::Itertools;

fn run(_n: usize, s: &str) -> &'static str {
    if s.chars().all_unique() {
        "No"
    } else {
        "Yes"
    }
}

fn run2(_n: usize, s: &str) -> &'static str {
    let mut hash_map = HashMap::new();

    for c in s.chars() {
        *hash_map.entry(c).or_insert(0) += 1;
    }

    if hash_map.into_values().all(|v| v == 1) {
        "No"
    } else {
        "Yes"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, &'static str);

    #[test]
    fn yuki_2778() {
        let tests = [
            TestCase(5, "TOKYO", "Yes"),
            TestCase(7, "ATCODER", "No"),
            TestCase(19, "QAWSEDRFTGYHUJIKOLP", "No"),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
            assert_eq!(run2(n, s), expected);
        }
    }
}
