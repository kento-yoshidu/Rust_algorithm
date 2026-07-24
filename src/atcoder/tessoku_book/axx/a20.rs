// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_t

use std::cmp::max;

fn run(s: &str, t: &str) -> usize {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut dp = vec![vec![0; t.len()+1]; s.len()+1];

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i-1] == t[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
            } else {
                if i > 0 && j > 0 {
                    dp[i][j] = max(dp[i-1][j], dp[i][j-1]);
                } else if i > 0 {
                    dp[i][j] = dp[i-1][j];
                } else {
                    dp[i][j] = dp[i][j-1];
                }
            }
        }
    }

    dp[s.len()][t.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(&'static str, &'static str, usize);

    #[test]
    fn tessoku_a20() {
        let tests = [
            TestCase("mynavi", "monday", 3),
            TestCase("tokyo", "kyoto", 3),
        ];

        for TestCase(s, t, expected) in tests {
            assert_eq!(run(s, t), expected);
        }
    }
}
