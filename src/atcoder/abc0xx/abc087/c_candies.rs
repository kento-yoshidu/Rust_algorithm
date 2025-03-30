// https://atcoder.jp/contests/abc087/tasks/arc090_a

fn run(n: usize, a: [Vec<usize>; 2]) -> usize {
    let mut dp: Vec<Vec<usize>> = vec![vec![], vec![]];

    dp[0].push(a[0][0]);

    for i in 1..n {
        let prev = dp[0][i-1];
        dp[0].push(prev + a[0][i]);
    }

    dp[1].push(a[0][0] + a[1][0]);

    for i in 1..n {
        let prev = dp[1][i-1];
        let next = dp[0][i];
        dp[1].push(prev.max(next) + a[1][i]);
    }

    dp[1][n-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(usize, [Vec<usize>; 2], usize);

    #[test]
    fn test() {
        let tests = [
            TestCase(5, [vec![3, 2, 2, 4, 1], vec![1, 2, 2, 2, 1]], 14),
            TestCase(4, [vec![1, 1, 1, 1], vec![1, 1, 1, 1]], 5),
            TestCase(7, [vec![3, 3, 4, 5, 4, 5, 3], vec![5, 3, 4, 4, 2, 3, 2]], 29),
            TestCase(1, [vec![2], vec![3]], 5),
        ];

        for TestCase(n, a, expected) in tests {
            assert_eq!(run(n, a), expected);
        }
    }
}
