// https://atcoder.jp/contests/abc032/tasks/abc032_d

use std::cmp::max;

pub fn run(n: usize, w: usize, vw: Vec<(usize, usize)>) -> usize {
    let mut dp = vec![vec![0; w+1]; n+1];


    for (i, (value, weight)) in vw.into_iter().enumerate() {
        let i = i + 1;

        for j in 1..=w {
            if j < weight {
                dp[i][j] = dp[i-1][j];
            } else {
                dp[i][j] = max(dp[i-1][j], dp[i-1][j - weight] + value);
            }
        }
    }

    *dp[n].iter().max().unwrap()
}
