// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_r

fn run(n: usize, s: usize, a: Vec<usize>) -> &'static str {
    let mut dp = vec![vec![false; s+1]; n+1];

    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            if dp[i-1][j] {
                dp[i][j] = true;

                if j + a[i-1] <= s {
                    dp[i][j + a[i-1]] = true;
                }
            }
        }
    }

    if dp[n][s] {
        "Yes"
    } else {
        "No"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, &'static str);

    #[test]
    fn tessoku_a18() {
        let tests = [
            TestCase(3, 7, vec![2, 2, 3], "Yes"),
            TestCase(4, 11, vec![3, 1, 4, 5], "No"),
        ];

        for TestCase(n, s, a, expected) in tests {
            assert_eq!(run(n, s, a), expected);
        }
    }
}
