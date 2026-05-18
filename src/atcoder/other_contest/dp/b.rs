// https://atcoder.jp/contests/dp/tasks/dp_b

fn run(n: usize, k: usize, a: Vec<usize>) -> usize {
    let mut dp = vec![std::usize::MAX; n];
    dp[0] = 0;

    for i in 1..n {
        let s = i.saturating_sub(k);

        for j in s..i {
            dp[i] = dp[i].min(dp[j] + (a[i] as isize - a[j] as isize).abs() as usize);
        }
    }

    dp[n-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize, Vec<usize>, usize);

    #[test]
    fn dp_b() {
        let tests = [
            TestCase(5, 3, vec![10, 30, 40, 50, 20], 30),
            TestCase(3, 1, vec![10, 20, 10], 20),
            TestCase(2, 100, vec![10, 10], 0),
            TestCase(10, 4, vec![40, 10, 20, 70, 80, 10, 20, 70, 80, 60], 40),
        ];

        for TestCase(n, k, a, expected) in tests {
            assert_eq!(run(n, k, a), expected);
        }
    }
}