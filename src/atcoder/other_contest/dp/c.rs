// https://atcoder.jp/contests/dp/tasks/dp_c

use std::cmp::max;

fn run(n: usize, abc: Vec<(usize, usize, usize)>) -> usize {
    let mut dp = vec![vec![0; 3]; n];

    for (i, (a, b, c)) in abc.into_iter().enumerate() {
        if i == 0 {
            dp[0][0] = a;
            dp[0][1] = b;
            dp[0][2] = c;
        } else {
            dp[i][0] = max(dp[i-1][1], dp[i-1][2]) + a;
            dp[i][1] = max(dp[i-1][0], dp[i-1][2]) + b;
            dp[i][2] = max(dp[i-1][0], dp[i-1][1]) + c;
        }
    }

    *dp[n-1].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize, usize)>, usize);

    #[test]
    fn dp_c() {
        let tests = [
            TestCase(3, vec![(10, 40, 70), (20, 50, 80), (30, 60, 90)], 210),
            TestCase(1, vec![(100, 10, 1)], 100),
            TestCase(7, vec![(6, 7, 8), (8, 8, 3), (2, 5, 2), (7, 8, 6), (4, 6, 8), (2, 3, 4), (7, 5, 1)], 46),
        ];

        for TestCase(n, abc, expected) in tests {
            assert_eq!(run(n, abc), expected);
        }
    }
}
