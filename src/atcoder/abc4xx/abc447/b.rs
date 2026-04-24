// https://atcoder.jp/contests/abc447/tasks/abc447_b

use std::collections::HashMap;

fn run(s: &str) -> String {
    let mut map = HashMap::new();

    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let max = map.values().copied().max().unwrap();

    s.chars()
        .filter(|c| map.get(c).unwrap() != &max)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str);

    #[test]
    fn abc447_b() {
        let tests = [
            TestCase("mississippi", "mpp"),
            TestCase("atcoder", ""),
            TestCase("beginner", "bgir"),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
