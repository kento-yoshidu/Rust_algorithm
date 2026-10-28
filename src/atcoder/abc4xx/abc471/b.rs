// https://atcoder.jp/contests/abc471/tasks/abc471_b

use std::collections::HashMap;

fn run(_n: usize, s: Vec<&str>) -> usize {
    let mut map = HashMap::new();

    for str in s {
        let s = str.to_lowercase();
        *map.entry(s).or_insert(0) += 1;
    }

    map.into_iter()
        .map(|(_, v)| v)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<&'static str>, usize);

    #[test]
    fn abc471_b() {
        let tests = [
            TestCase(7, vec!["ARC", "abc", "ahc", "ABC", "beginner", "AbC", "ahc"], 3),
            TestCase(10, vec!["x", "x", "x", "x", "x", "x", "x", "x", "x", "x"], 10),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
