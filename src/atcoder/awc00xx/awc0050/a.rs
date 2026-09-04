// https://atcoder.jp/contests/awc0050/tasks/awc0050_a

use std::cmp::{min, max};

fn run(_n: usize, a: Vec<isize>, b: Vec<isize>) ->isize {
    a.into_iter()
        .zip(b)
        .map(|(a, b)| max(a, b) - min(a, b))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, Vec<isize>, isize);

    #[test]
    fn awc0050_a() {
        let tests = [
            TestCase(3, vec![1, 3, 5], vec![2, 1, 5], 3),
            TestCase(5, vec![3, -1, 0, 7, -5], vec![-2, 4, 0, 10, -5], 13),
            TestCase(8, vec![1000000000, -999999999, 500000000, 0, -100, 42, -7, 123456789], vec![-1000000000, 999999999, -500000000, 0, 200, -58, 7, -987654321], 6111111522),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
