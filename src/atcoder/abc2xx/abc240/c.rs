// https://atcoder.jp/contests/abc240/tasks/abc240_c

fn run(n: usize, x: usize, ab: Vec<(usize, usize)>) -> &'static str {
    let mut dp = vec![vec![false; x+1]; n+1];

    dp[0][0] = true;

    for i in 0..n {
        for j in 0..=x {
            if dp[i][j] {
                let (a, b) = ab[i];

                if j + a <= x {
                    dp[i + 1][j + a] = true;
                }
                if j + b <= x {
                    dp[i + 1][j + b] = true;
                }
            }
        }
    }

    if dp[n][x] {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<(usize, usize)>, &'static str);

    #[test]
    fn abc240_c() {
        let tests = [
            TestCase(2, 10, vec![(3, 6), (4, 5)], "Yes"),
            TestCase(2, 10, vec![(10, 100), (10, 100)], "No"),
            TestCase(4, 12, vec![(1, 8), (5, 7), (3, 4), (2, 6)], "Yes"),
        ];

        for TestCase(n, x, ab, expected) in tests {
            assert_eq!(run(n, x, ab), expected);
        }
    }
}
