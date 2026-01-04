// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ab

fn run(n: usize) -> usize {
    let mut dp = Vec::new();

    dp.push(1);
    dp.push(1);

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
    fn ma029() {
        let tests = [
            TestCase(4, 5),
            TestCase(45, 1836311903),
        ];

        for TestCase(n, expected) in tests {
            assert_eq!(run(n), expected);
        }
    }
}
