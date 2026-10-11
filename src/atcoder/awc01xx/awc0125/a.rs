// https://atcoder.jp/contests/awc0125/tasks/awc0125_a

use std::cmp::min;

fn run(n: usize, d: usize, s: usize, t: Vec<usize>) -> usize {
    let sum: usize = t.into_iter().sum();

    let dist = min(s - 1, n - s) + (n - 1);

    sum + dist * d
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, Vec<usize>, usize);

    #[test]
    fn awc0125_a() {
        let tests = [
            TestCase(4, 2, 2, vec![3, 1, 4, 2], 18),
            TestCase(5, 3, 5, vec![2, 8, 1, 6, 4], 33),
            TestCase(12, 5, 6, vec![7, 2, 9, 4, 6, 3, 8, 5, 1, 10, 2, 7], 144),
            TestCase(30, 100000000, 17, vec![12, 999999999, 345678901, 1, 500000000, 234567890, 876543210, 111111111, 222222222, 333333333, 444444444, 555555555, 666666666, 777777777, 888888888, 999999998, 123456789, 987654321, 314159265, 271828182, 161803398, 141421356, 173205080, 223606797, 707106781, 1000000000, 42, 424242424, 606060606, 808080808], 18099415856),
            TestCase(1, 1000000000, 1, vec![1000000000], 1000000000),
        ];

        for TestCase(n, d, s, t, expected) in tests {
            assert_eq!(run(n, d, s, t), expected);
        }
    }
}
