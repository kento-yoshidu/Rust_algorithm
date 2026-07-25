// https://atcoder.jp/contests/dp/tasks/dp_d

use std::cmp::max;

fn run(n: usize, w: usize, wv: Vec<(usize, usize)>) -> usize {
    let mut dp = vec![vec![0; w+1]; n+1];

    for i in 1..=n {
        let (wi, vi) = wv[i-1];

        for j in 0..=w {
            dp[i][j] = dp[i-1][j];

            if wi <= j {
                dp[i][j] = max(dp[i][j], dp[i-1][j - wi] + vi);
            }
        }
    }

    *dp[n].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, usize);

    #[test]
    fn edpc_d() {
        let tests = [
            TestCase(3, 8, vec![(3, 30), (4, 50), (5, 60)], 90),
            TestCase(5, 5, vec![(1, 1000000000), (1, 1000000000), (1, 1000000000), (1, 1000000000), (1, 1000000000)], 5000000000),
            TestCase(6, 15, vec![(6, 5), (5, 6), (6, 4), (6, 6), (3, 5), (7, 2)], 17),
        ];

        for TestCase(n, w, wv, expected) in tests {
            assert_eq!(run(n, w, wv), expected);
        }
    }
}
