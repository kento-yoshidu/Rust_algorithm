// https://atcoder.jp/contests/abc458/tasks/abc458_c

use std::cmp::min;

fn run(s: &str) -> usize {
    s.chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c == 'C' {
                Some(min(i+1, s.len() - i))
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn abc458_c() {
        let tests = [
            TestCase("ABCCA", 5),
            TestCase("XYZ", 0),
            TestCase("SMBCPROGRAMMINGCONTEST", 11),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s), expected);
        }
    }
}
