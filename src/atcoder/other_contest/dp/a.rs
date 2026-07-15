// https://atcoder.jp/contests/dp/tasks/dp_a

use std::cmp::min;

fn run(n: usize, h: Vec<isize>) -> usize {
    let mut dp = vec![0];
    dp.push((h[1] - h[0]).abs());

    for i in 2..n {
        let a = (h[i] - h[i-1]).abs() + dp[i-1];
        let b = (h[i] - h[i-2]).abs() + dp[i-2];

        dp.push(min(a, b));
    }

    dp[n-1] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, usize);

    #[test]
    fn dp_a() {
        let tests = [
            TestCase(4, vec![10, 30, 40, 20], 30),
            TestCase(2, vec![10, 10], 0),
            TestCase(6, vec![30, 10, 60, 10, 60, 50], 40),
        ];

        for TestCase(n, h, expected) in tests {
            assert_eq!(run(n, h), expected);
        }
    }
}
