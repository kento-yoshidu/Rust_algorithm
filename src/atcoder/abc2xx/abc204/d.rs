// https://atcoder.jp/contests/abc204/tasks/abc204_d

use std::cmp::{min, max};

fn run(n: usize, t: Vec<usize>) -> usize {
    let total = t.iter().sum();

    let mut dp = vec![vec![false; total+1]; n+1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=total {
            if dp[i-1][j] {
                dp[i][j] = true;

                if t[i-i] + j <= total {
                    dp[i][t[i-1] + j] = true;
                }
            }
        }
    }

    let mut ans = std::usize::MAX;

    for j in 1..=total {
        if dp[n][j] {
            ans = min(ans, max(j, total - j));
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, usize);

    #[test]
    fn abc204_d() {
        let tests = [
            TestCase(5, vec![8, 3, 7, 2, 5], 13),
            TestCase(2, vec![1000, 1], 1000),
            TestCase(9, vec![3, 14, 15, 9, 26, 5, 35, 89, 79], 138),
        ];

        for TestCase(n, t, expected) in tests {
            assert_eq!(run(n, t), expected);
        }
    }
}
