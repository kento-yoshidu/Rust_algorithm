// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_s

use std::cmp::max;

fn run(n: usize, w: usize, wv: Vec<(usize, isize)>) -> usize {
    let mut dp = vec![vec![-1; w+1]; n+1];

    dp[0][0] = 0;

    for i in 1..=n {
        let (wi, vi) = wv[i-1];

        for j in 0..=w {
            if dp[i-1][j] != -1 {
                dp[i][j] = max(dp[i-1][j], dp[i][j]);

                if j + wi <= w {
                    dp[i][j + wi] = max(dp[i][j + wi], dp[i-1][j] + vi);
                }
            }
        }
    }

    dp[n].iter().copied().max().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, isize)>, usize);

    #[test]
    fn tessoku_a19() {
        let tests = [
            TestCase(4, 7, vec![(3, 13), (3, 17), (5, 29), (1, 10)], 40),
            TestCase(4, 100, vec![(25, 47), (25, 53), (25, 62), (25, 88)], 250),
            TestCase(10, 285, vec![(29, 8000), (43, 11000), (47, 10000), (51, 13000), (52, 16000), (66, 14000), (72, 25000), (79, 18000), (82, 23000), (86, 27000)], 87000),
        ];

        for TestCase(n, w, wv, expected) in tests {
            assert_eq!(run(n, w, wv), expected);
        }
    }
}
