// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_u

use std::cmp::max;

fn run(n: usize, pa: Vec<(usize, usize)>) -> usize {
    let mut dp = vec![vec![0; n+1]; n+1];

    // 区間の長さ
    for len in 2..=n {
        // 左端
        for l in 1..=(n - len + 1) {
            let r = l + len - 1;

            let score_l = if l + 1 <= pa[l-1].0 && pa[l-1].0 <= r {
                pa[l-1].1
            } else {
                0
            };

            let score_r = if l <= pa[r-1].0 && pa[r-1].0 <= r - 1 {
                pa[r-1].1
            } else {
                0
            };

            dp[l][r] = max(dp[l+1][r] + score_l, dp[l][r-1] + score_r);
        }

    }

    dp[1][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<(usize, usize)>, usize);

    #[test]
    fn tessoku_a21() {
        let tests = [
            TestCase(4, vec![(4, 20), (3, 30), (2, 40), (1, 10)], 60),
            TestCase(8, vec![(8, 100), (7, 100), (6, 100), (5, 100), (4, 100), (3, 100), (2, 100), (1, 100)], 400),
        ];

        for TestCase(n, pa, expected) in tests {
            assert_eq!(run(n, pa), expected);
        }
    }
}
