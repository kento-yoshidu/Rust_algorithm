// https://atcoder.jp/contests/abc426/tasks/abc426_b

use std::collections::HashMap;

fn run(s: &str) -> char {
    let mut hash_map = HashMap::new();

    for c in s.chars() {
        *hash_map.entry(c).or_insert(0) += 1;
    }

    hash_map.into_iter()
        .find(|(_, v)| *v == 1)
        .map(|(c, _)| c)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, char);

    #[test]
    fn abc426_b() {
        let tests = [
            TestCase("odd", 'o'),
            TestCase("dad", 'a'),
            TestCase("wwwwwwwwwv", 'v'),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
