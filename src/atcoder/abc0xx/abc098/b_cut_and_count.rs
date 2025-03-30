// https://atcoder.jp/contests/abc098/tasks/abc098_b

use std::collections::HashSet;

fn rec(s1: &str, s2: &str) -> usize {
    let set1: HashSet<_> = s1.chars().collect();
    let mut common_characters = HashSet::new();

    for c in s2.chars() {
        if set1.contains(&c) {
            common_characters.insert(c);
        }
    }

    common_characters.len()
}

fn run(n: usize, s: &str) -> usize {
    *(1..n)
        .map(|i| {
            rec(&s[0..i], &s[i..])
        })
        .collect::<Vec<usize>>()
        .iter()
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, &'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(6, "aabbca", 2),
            TestCase(10, "aaaaaaaaaa", 1),
            TestCase(45, "tgxgdqkyjzhyputjjtllptdfxocrylqfqjynmfbfucbir", 9),
        ];

        for TestCase(n, s, expected) in tests {
            assert_eq!(run(n, s), expected);
        }
    }
}
