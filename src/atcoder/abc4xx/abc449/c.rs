// https://atcoder.jp/contests/abc449/tasks/abc449_c

use std::collections::HashMap;

fn run(_n: usize, l: usize, r: usize, s: &str) -> usize {
    let mut map = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        map.entry(c).or_insert_with(Vec::new).push(i);
    }

    let mut ans = 0;

    for (_, v) in map {
        let mut left = 0;
        let mut right = 0;

        for j in 0..v.len() {
            while left < v.len() && v[left] + r < v[j] {
                left += 1;
            }

            while right < v.len() && v[right] + l <= v[j] {
                right += 1;
            }

            if right > left {
                ans += right - left;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

use super::*;

    struct TestCase(usize, usize, usize, &'static str, usize);

    #[test]
    fn abc449_c() {
        let tests = [
            TestCase(6, 2, 4, "aabcba", 2),
            TestCase(9, 3, 6, "aaaaaaaaa", 18),
            TestCase(10, 2, 6, "aabbccaabb", 6),
        ];

        for TestCase(n, l, r, s, expected) in tests {
            assert_eq!(run(n, l, r, s), expected);
        }
    }
}
