// https://atcoder.jp/contests/abc386/tasks/abc386_b

use itertools::Itertools;

fn run(s: &str) -> usize {
    s.chars()
        .dedup_with_count()
        .map(|(count, c)| {
            match c {
                '0' => (count + 1) / 2,
                _ => count
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase("1000000007", 6),
            TestCase("998244353", 9),
            TestCase("32000", 4),
        ];

        for TestCase(s, expected) in tests {
            assert_eq!(run(s) ,expected);
        }
    }
}
