// https://atcoder.jp/contests/abc232/tasks/abc232_d

use std::cmp::max;

fn run(h: usize, w: usize, c: Vec<&str>) -> usize {
    let c: Vec<Vec<char>> = c.into_iter().map(|s| s.chars().collect()).collect();

    let mut dp = vec![vec![0; w]; h];

     if c[0][0] == '.' {
        dp[0][0] = 1;
    }

    let mut ans = 0;

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                continue;
            }

            if i > 0 && dp[i-1][j] > 0 {
                dp[i][j] = max(dp[i][j], dp[i-1][j] + 1);
            }

            if j > 0 && dp[i][j-1] > 0 {
                dp[i][j] = max(dp[i][j], dp[i][j-1] + 1);
            }

            ans = ans.max(dp[i][j]);
        }
    }

    dp.into_iter().map(|v| v.into_iter().max().unwrap()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<&'static str>, usize);

    #[test]
    fn abc232_d() {
        let tests = [
            TestCase(3, 4, vec![".#..", "..#.", "..##"], 4),
            TestCase(5, 5, vec![".....", ".....", ".....", ".....", "....."], 9),
        ];

        for TestCase(h, w, c, expected) in tests {
            assert_eq!(run(h, w, c), expected);
        }
    }
}
