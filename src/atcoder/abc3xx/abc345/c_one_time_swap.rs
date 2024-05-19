// https://atcoder.jp/contests/abc345/tasks/abc345_c

use std::collections::HashMap;

fn calc(n: usize) -> usize {
    n * (n-1) / 2
}

fn run(s: &str) -> usize {
    let mut hash_map = HashMap::new();

    for c in s.chars() {
        *hash_map.entry(c).or_insert(0) += 1;
    }

    let distinct: usize = hash_map.iter()
        .map(|(_, v)| {
            calc(*v)
        })
        .sum();

    let mut ans = calc(s.len()) - distinct;

    if distinct > 0 {
        ans += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("abc", 3),
            TestCase("aaaaa", 1),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
