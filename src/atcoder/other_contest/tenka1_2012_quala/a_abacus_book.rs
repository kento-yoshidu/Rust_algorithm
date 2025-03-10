// https://atcoder.jp/contests/tenka1-2012-qualA/tasks/tenka1_2012_qualA_1

fn run(n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let mut dp = vec![1; 2];

    for i in 2..=n {
        dp.push(dp[i-1] + dp[i-2]);
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(0, 1),
            TestCase(5, 8),
            TestCase(45, 1836311903),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
