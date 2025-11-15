// https://atcoder.jp/contests/math-and-algorithm/tasks/dp_a

use std::cmp::min;

fn run(n: usize, h: Vec<isize>) -> usize {
    let mut dp = Vec::new();

    dp.push(0);
    dp.push((h[1] - h[0]).abs());

    for i in 2..n {
        let v1 = dp[i-1] + (h[i] - h[i-1]).abs();
        let v2 = dp[i-2] + (h[i] - h[i-2]).abs();

        dp.push(min(v1, v2));
    }

    dp[n-1] as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, Vec<isize>, usize);

    #[test]
    fn ma028() {
        let tests = [
            TestCase(4, vec![10, 30, 40, 20], 30),
            TestCase(2, vec![10, 10], 0),
            TestCase(6, vec![30, 10, 60, 10, 60, 50], 40),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
