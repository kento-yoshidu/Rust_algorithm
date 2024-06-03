use std::cmp::min;

pub fn run(n: usize, a: Vec<usize>, b: Vec<usize>) -> usize {
    let mut dp = vec![0];
    dp.push(a[0]);

    for i in 2..n {
        dp.push(min(dp[i-1] + a[i-1], dp[i-2] + b[i-2]));
    }

    dp[n-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<usize>, Vec<usize>, usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, vec![2, 4, 1, 3], vec![5, 3, 7], 8),
            TestCase(10, vec![1, 19, 75, 37, 17, 16, 33, 18, 22], vec![41, 28, 89, 74, 98, 43, 42, 31], 157),
        ];

        for TestCase(n, a, b, expected) in tests {
            assert_eq!(run(n, a, b), expected);
        }
    }
}
