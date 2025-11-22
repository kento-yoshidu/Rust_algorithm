// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ac

use std::cmp::max;

fn run(n: usize, a: Vec<usize>) -> usize {
    let mut dp = vec![0; n];

    dp[0] = a[0];
    dp[1] = max(a[0], a[1]);

    for i in 2..n {
        dp[i] = max(dp[i-1], dp[i-2] + a[i]);
    }

    dp[n-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn ma031() {
        let tests = [
            TestCase(5, vec![2, 5, 3, 3, 1], 8),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
