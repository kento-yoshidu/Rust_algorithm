// https://atcoder.jp/contests/abc260/tasks/abc260_c

fn run(n: usize, x: usize, y: usize) -> usize {
    let mut r_dp = vec![0; n+1];
    let mut b_dp = vec![0; n+1];

    r_dp[n] = 1;

    for i in (2..=n).rev() {
        r_dp[i-1] += r_dp[i];
        b_dp[i] += r_dp[i] * x;

        r_dp[i-1] += b_dp[i];
        b_dp[i-1] += b_dp[i] * y;
    }

    b_dp[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, usize, usize);

    #[test]
    fn abc260_c() {
        let tests = [
            TestCase(2, 3, 4, 12),
            TestCase(1, 5, 5, 0),
            TestCase(10, 5, 5, 3942349900),
        ];

        for TestCase(n, x, y, expected) in tests {
            assert_eq!(run(n, x, y), expected);
        }
    }
}
