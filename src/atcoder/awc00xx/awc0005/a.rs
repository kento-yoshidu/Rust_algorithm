// https://atcoder.jp/contests/awc0005/tasks/awc0005_a

fn run(_n: usize, k: usize, a: Vec<usize>) -> usize {
    a.into_iter()
        .filter(|n| {
            *n % k == 0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0005_a() {
        let tests = [
            TestCase(5, 3, vec![6, 7, 9, 12, 5], 27),
            TestCase(8, 5, vec![10, 25, 7, 15, 30, 8, 100, 3], 180),
            TestCase(10, 1000000, vec![500000, 1000000, 2000000, 3000000, 750000, 4000000, 1234567, 5000000, 999999, 6000000], 21000000),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}
